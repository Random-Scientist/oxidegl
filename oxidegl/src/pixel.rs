//! # Pixel conversion utilities
//! This module emulates the OpenGL pixel pipeline on the CPU (and possibly the GPU in the future), as well as providing various traits
//! and types for working with GL pixel formats.
//!
//!
//!
use std::{
    any::TypeId,
    borrow::Cow,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    num::NonZeroU32,
    ptr, slice,
};

use bytemuck::{BoxBytes, Pod, Zeroable, box_bytes_of, cast_mut};
use half::f16;
use strum_macros::FromRepr;

use crate::{
    debug::{gl_trace, gl_warn},
    error::{GlFallible, gl_assert},
    gl_enums::{InternalFormat, PixelFormat, PixelType},
    pixel::internal_formats::{ConvertibleToInternalFormat, DepthComponent, InternalFormatInfo},
};
mod internal_formats;

trait ChannelType: Copy + 'static {}
macro_rules! decl_channels {
    ($($i:ident),+) => {
        $(
            #[derive(Clone, Copy, Debug, )]
            enum $i {}
            impl ChannelType for $i {}
        )+
    };
}

decl_channels!(Red, Green, Blue, Alpha, Depth, Stencil);
/// Represents a pixel format that can provide a certain type of channel information, represented as the type Channel (see above for all of the valid types)
trait ChannelSource<Type: ChannelType>: Sized + Copy {
    type Channel;
    fn channel(self) -> Self::Channel;
    fn get<T>(self) -> T
    where
        Self::Channel: ConvertChannel<T>,
    {
        self.channel().convert_channel()
    }
}

/// Performs a pixel channel format conversion as specified by OpenGL
trait ConvertChannel<T> {
    fn convert_channel(self) -> T;
}
// identity impl
impl<T> ConvertChannel<T> for T {
    fn convert_channel(self) -> T {
        self
    }
}

// TODO better on_unimplemented diagnostic
trait ConvertPixel<T>: Sized {
    fn convert_pixel(self) -> T;
}
/// Represents the default value for this channel according to the OpenGL pixel interchange specification
struct DefaultChannelValue<Ctype>(PhantomData<Ctype>);
impl<T> DefaultChannelValue<T> {
    const fn new() -> Self {
        Self(PhantomData)
    }
}
// stencil should never get a default channel value
impl<Int: GlIntegerType> ConvertChannel<Int> for DefaultChannelValue<Stencil> {
    fn convert_channel(self) -> Int {
        unreachable!()
    }
}
// depth should never get a default channel value
impl ConvertChannel<f32> for DefaultChannelValue<Depth> {
    fn convert_channel(self) -> f32 {
        unreachable!()
    }
}
impl ConvertChannel<f32> for DefaultChannelValue<Alpha> {
    fn convert_channel(self) -> f32 {
        1.0
    }
}
impl ConvertChannel<f16> for DefaultChannelValue<Alpha> {
    fn convert_channel(self) -> f16 {
        f16::from_f32(1.0)
    }
}
trait IsRgb {}
impl IsRgb for Red {}
impl IsRgb for Green {}
impl IsRgb for Blue {}
impl<Itype: GlIntegerType> ConvertChannel<NormalizedIntChannel<Itype>>
    for DefaultChannelValue<Alpha>
{
    fn convert_channel(self) -> NormalizedIntChannel<Itype> {
        NormalizedIntChannel(Itype::from_bits(Itype::MAX_BITS))
    }
}
impl<Itype: GlIntegerType> ConvertChannel<Itype> for DefaultChannelValue<Alpha> {
    fn convert_channel(self) -> Itype {
        Itype::from_bits(1)
    }
}
impl<Itype: GlIntegerType, C: IsRgb> ConvertChannel<NormalizedIntChannel<Itype>>
    for DefaultChannelValue<C>
{
    fn convert_channel(self) -> NormalizedIntChannel<Itype> {
        NormalizedIntChannel(Itype::from_bits(0))
    }
}
impl<Itype: GlIntegerType, C: IsRgb> ConvertChannel<Itype> for DefaultChannelValue<C> {
    fn convert_channel(self) -> Itype {
        Itype::from_bits(0)
    }
}
impl<C: IsRgb> ConvertChannel<f32> for DefaultChannelValue<C> {
    fn convert_channel(self) -> f32 {
        1.0
    }
}
impl<C: IsRgb> ConvertChannel<f16> for DefaultChannelValue<C> {
    fn convert_channel(self) -> f16 {
        f16::from_f32(1.0)
    }
}

trait AlignInfo: Sized {
    const CHANNEL_ALIGN: Option<GlAlignment> = None;
    const SHOULD_SWAP_BYTES: bool = const { matches!(Self::SIZE, 1 | 2 | 4) };
    const CHECK_ALIGN: () = {
        if let Some(a) = Self::CHANNEL_ALIGN {
            assert!(align_of::<Self>() == a as u8 as usize);
        }
    };
    const SIZE: usize = size_of::<Self>();
}
impl GlAlignment {
    const fn from_size(size: usize) -> Option<Self> {
        match size {
            1 => Some(GlAlignment::One),
            2 => Some(GlAlignment::Two),
            4 => Some(GlAlignment::Four),
            8 => Some(GlAlignment::Eight),
            _ => None,
        }
    }
}

macro_rules! channelsource_default {
    (
        $(
            default impl $(  ($($imp:tt)*) )? ChannelSource<$ty:ty> for $struct:ty {}
        )+
    ) => {
        $(
            impl$(<$($imp)*>)? $crate::pixel::ChannelSource<$ty> for $struct {
                type Channel = $crate::pixel::DefaultChannelValue<$ty>;
                fn channel(self) -> Self::Channel {
                    $crate::pixel::DefaultChannelValue::new()
                }
            }
        )+
    };
}
pub(crate) use channelsource_default;
const fn const_arr_max<const N: usize>(arr: &[usize; N]) -> usize {
    let mut g = 0;
    let mut index = 0;
    while index < arr.len() {
        if arr[index] > g {
            g = arr[index];
        }
        index += 1;
    }
    g
}
const fn assert_covered<const N: usize>(arr: &[usize; N]) {
    let mut v = 0;
    let l = const_arr_max(arr);
    while v <= l {
        assert!(
            arr_contains(v, arr),
            "Bad simple color format definition, all array indices must be occupied"
        );
        v += 1;
    }
}
const fn arr_contains(val: usize, arr: &[usize]) -> bool {
    let mut v = 0;
    while v < arr.len() {
        if arr[v] == val {
            return true;
        }
        v += 1;
    }
    false
}
macro_rules! decl_simple_formats {
    (   $(
            $( #[doc = $doc:expr] )*
            $vis:vis struct $fmt_name:ident  {
                $( ($idx:literal : $chan:ty) ),+ default: [$($dty:ty),+]
            }
        ),+
        $(,)?
    ) => {
        $(
            $( #[doc = $doc] )*
            #[derive(Debug, Clone, Copy, Zeroable)]
            #[repr(transparent)]
            $vis struct $fmt_name<T> {
                colors: [T; const { const_arr_max( &[ $( $idx ),+ ] ) + 1 }]
            }
            // Safety: format is repr(transparent) over [T; N], which is guaranteed not to introduce padding (if T is Pod, T also contains no padding)
            unsafe impl<T: Pod> Pod for $fmt_name<T> {}
            impl<C, T: $(ChannelSource<$chan, Channel: ConvertChannel<C>> +)+ Pod> ConvertPixel<$fmt_name<C>> for T {

                #[inline]
                fn convert_pixel(self) -> $fmt_name<C> {
                    const __INDICES: [usize; const { [ $( $idx ),+ ].len() }] = [ $( $idx ),+ ];
                    const __ARR_LEN: usize = const { const_arr_max( &__INDICES ) + 1 };
                    const { assert_covered(&__INDICES) }

                    let mut uninit = [const { MaybeUninit::<C>::uninit() }; __ARR_LEN];
                    $(
                        uninit[$idx].write(<Self as ChannelSource<$chan>>::channel(self).convert_channel());
                    )+
                    $fmt_name {
                        // Safety: transmute from [MaybeUninit<T>] to [T] ok if all values are initialized
                        // justification TODO
                        colors: unsafe { crate::util::transmute_unchecked(uninit) },
                    }
                }
            }
            impl<C> AlignInfo for $fmt_name<C> {
                const CHANNEL_ALIGN: Option<GlAlignment> = GlAlignment::from_size(::std::mem::size_of::<C>());
            }
            $(
                impl<T: Copy> ChannelSource<$chan> for $fmt_name<T> {
                    type Channel = T;
                    #[inline]
                    fn channel(self) -> Self::Channel {
                        self.colors[$idx]
                    }
                }
            )+
            $(
                channelsource_default! {
                    default impl(T: Pod) ChannelSource<$dty> for $fmt_name<T> {}
                }
            )+
        )+
    };
}

decl_simple_formats! {
    pub(crate) struct RgColorFormat { (0: Red), (1: Green) default: [Blue, Alpha, Depth, Stencil] },
    pub(crate) struct RgbColorFormat { (0: Red), (1: Green), (2: Blue) default: [Alpha, Depth, Stencil] },
    pub(crate) struct RgbaColorFormat { (0: Red), (1: Green), (2: Blue), (3: Alpha) default: [Depth, Stencil] },
    pub(crate) struct BgrColorFormat { (0: Blue), (1: Green), (2: Red) default: [Alpha, Depth, Stencil] },
    pub(crate) struct BgraColorFormat { (0: Blue), (1: Green), (2: Red), (3: Alpha) default: [Depth, Stencil] },
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct SingleChannelFormat<T, Ctype: ChannelType> {
    value: T,
    phantom: PhantomData<Ctype>,
}
// Safety: struct is repr(transparent) over T, if T is Pod, so is SingleChannelFormat<T, _>
unsafe impl<T: Pod, U: ChannelType> Pod for SingleChannelFormat<T, U> {}
// Safety: struct is repr(transparent) over T, if T is Zeroable, so is SingleChannelFormat<T, _>
unsafe impl<T: Zeroable, U: ChannelType> Zeroable for SingleChannelFormat<T, U> {}

impl<T, U: ChannelType> SingleChannelFormat<T, U> {
    fn new(value: T) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}
macro_rules! impl_single_color {
    (
        $( $prety:ident ),* | $t:ident, $( $postty:ident, )*
    ) => {
        $(
            channelsource_default! {
                default impl(T: Copy) ChannelSource<$prety> for SingleChannelFormat<T, $t> {}
            }
        )*
        $(
            channelsource_default! {
                default impl(T: Copy) ChannelSource<$postty> for SingleChannelFormat<T, $t> {}
            }
        )*
        impl<T: Copy> ChannelSource<$t> for SingleChannelFormat<T, $t> {
            type Channel = T;
            fn channel(self) -> Self::Channel {
                self.value
            }
        }
        impl<T> AlignInfo for SingleChannelFormat<T, $t> {
            const CHANNEL_ALIGN: Option<GlAlignment> = GlAlignment::from_size(::std::mem::size_of::<T>());
        }
        impl_single_color! {
            $t $(, $prety )*  | $( $postty,)*
        }
    };
    ( $($pre:ident),+ | ) => {};
}
impl_single_color! {
    | Red, Green, Blue, Alpha, Depth, Stencil, // <---- load bearing comma, do not remove :)
}
impl<Ctype: ChannelType, Cs: ConvertChannel<C>, C, T: ChannelSource<Ctype, Channel = Cs>>
    ConvertPixel<SingleChannelFormat<C, Ctype>> for T
{
    fn convert_pixel(self) -> SingleChannelFormat<C, Ctype> {
        SingleChannelFormat {
            value: <Cs as ConvertChannel<C>>::convert_channel(self.channel()),
            phantom: PhantomData,
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct NormalizedIntChannel<Int: GlIntegerType>(Int);

impl<Int: GlIntegerType> ConvertChannel<f16> for NormalizedIntChannel<Int> {
    fn convert_channel(self) -> f16 {
        f16::from_f32(self.0.normalized_to_float())
    }
}
impl<Int: GlIntegerType> ConvertChannel<f32> for NormalizedIntChannel<Int> {
    fn convert_channel(self) -> f32 {
        self.0.normalized_to_float()
    }
}
impl<Int: GlIntegerType> ConvertChannel<f16> for Int {
    fn convert_channel(self) -> f16 {
        f16::from_f32(self.as_float())
    }
}
impl<Int: GlIntegerType> ConvertChannel<f32> for Int {
    fn convert_channel(self) -> f32 {
        self.as_float()
    }
}

// Trait that generalizes over integers of all bitwidths supported as OpenGL pixel types
trait GlIntegerType: Sized + Copy {
    /// Whether this integer is signed
    const SIGNED: bool;
    /// number of bits in this integer's representation
    const BITWIDTH: u8;
    /// power of two to divide by when normalizing this value into a float
    const NORMALIZATION_FACTOR: u8 = Self::BITWIDTH - Self::SIGNED as u8;
    const MAX_BITS: u32;
    /// Interprets this integer as a normalized fixed-point float according the the GL spec, returning the closest 32 bit floating point value to the normalized result
    fn normalized_to_float(self) -> f32;
    fn as_float(self) -> f32;
    /// Creates an instance of this number from a u32, truncating the most significant bits to size
    fn from_bits(bits: u32) -> Self;
    /// bitcasts this number into (the low bits of) a u32
    fn to_bits(self) -> u32;
}

fn convert_normalized_int<IntInitial: GlIntegerType, IntFinal: GlIntegerType>(
    val: NormalizedIntChannel<IntInitial>,
) -> NormalizedIntChannel<IntFinal> {
    let mut int: u32 = val.0.to_bits();
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    if IntInitial::SIGNED != IntFinal::SIGNED {
        int = match IntInitial::SIGNED {
            // clamp negative values to 0 for signed -> unsigned
            true => (int as i32).clamp(0, i32::MAX) as u32,
            false => int.clamp(0, i32::MAX as u32),
        }
    }
    // truncates to size
    NormalizedIntChannel(IntFinal::from_bits(int))
}

macro_rules! impl_gl_integer {
    ( $( ($t:ident, $signed:expr, $width:expr, $ftype:ty) ),+ ) => {
        $(
            impl GlIntegerType for $t {
                const SIGNED: bool = $signed;
                const BITWIDTH: u8 = $width;
                const MAX_BITS: u32 = <$t>::MAX as u32;
                #[allow(clippy::cast_possible_truncation)]
                fn normalized_to_float(self) -> f32 {
                    let f1 = <$ftype>::from(self);
                    let factor = <$ftype>::from(2u8).powi(i32::from(Self::NORMALIZATION_FACTOR) );
                    (f1 / factor) as f32
                }
                #[allow(clippy::cast_possible_truncation)]
                fn as_float(self) -> f32 {
                    <$ftype>::from(self) as f32
                }
                fn from_bits(bits: u32) -> Self {
                    const { assert!((Self::BITWIDTH / 8) as usize <= 4, "Bad GlInteger implementation") }
                    // Safety: checked by const assertion above
                    Self::from_le_bytes(unsafe { *bits.to_le_bytes().first_chunk::<{ (Self::BITWIDTH / 8) as usize }>().unwrap_unchecked() })
                }
                #[allow(clippy::cast_possible_truncation, clippy::cast_lossless, clippy::cast_sign_loss, reason = "truncation is desired here")]
                fn to_bits(self) -> u32 {
                    self as u32
                }
            }
            impl ConvertChannel<NormalizedIntChannel<$t>> for $t {
                fn convert_channel(self) -> NormalizedIntChannel<$t> {
                    NormalizedIntChannel(self)
                }
            }
            impl ConvertChannel<$t> for NormalizedIntChannel<$t> {
                fn convert_channel(self) -> $t {
                    self.0
                }
            }
        )+
        impl_gl_integer! {
            convertchannel: | $($t,)+
        }
    };
    ( convertchannel: $( $prety:ident ),* | $t:ident, $( $postty:ident, )* ) => {
        $(
            impl ConvertChannel<NormalizedIntChannel<$prety>> for NormalizedIntChannel<$t> {
                fn convert_channel(self) -> NormalizedIntChannel<$prety> {
                    convert_normalized_int::<$t, $prety>(self)
                }
            }
            impl ConvertChannel<$t> for $prety {
                fn convert_channel(self) -> $t {
                    $t::from_bits(self.to_bits())
                }
            }
            impl ConvertChannel<$t> for NormalizedIntChannel<$prety> {
                fn convert_channel(self) -> $t {
                    self.0.convert_channel()
                }
            }
            impl ConvertChannel<NormalizedIntChannel<$t>> for $prety {
                fn convert_channel(self) -> NormalizedIntChannel<$t> {
                    let val: $t = self.convert_channel();
                    NormalizedIntChannel(val)
                }
            }
        )*
        $(
            impl ConvertChannel<NormalizedIntChannel<$postty>> for NormalizedIntChannel<$t> {
                fn convert_channel(self) -> NormalizedIntChannel<$postty> {
                    convert_normalized_int::<$t, $postty>(self)
                }
            }
            impl ConvertChannel<$t> for $postty {
                fn convert_channel(self) -> $t {
                    $t::from_bits(self.to_bits())
                }
            }
            impl ConvertChannel<$t> for NormalizedIntChannel<$postty> {
                fn convert_channel(self) -> $t {
                    self.0.convert_channel()
                }
            }
            impl ConvertChannel<NormalizedIntChannel<$t>> for $postty {
                fn convert_channel(self) -> NormalizedIntChannel<$t> {
                    let val: $t = self.convert_channel();
                    NormalizedIntChannel(val)
                }
            }
        )*
        impl_gl_integer! {
            convertchannel: $t $(, $prety )*  | $( $postty,)*
        }
    };
    ( convertchannel: $($pre:ident),+ | ) => {};
}
impl_gl_integer! {
    (u8, false, 8, f32),
    (u16, false, 16, f32),
    (u32, false, 32, f64),
    (i8, true, 8, f32),
    (i16, true, 16, f32),
    (i32, true, 32, f64)
}
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct PackUnpackState {
    swap_bytes: bool,
    lsb_first: bool,

    image_height: u32,
    skip_images: u32,
    row_length: Option<u32>,
    skip_rows: u32,
    skip_pixels: u32,

    alignment: GlAlignment,

    compressed_block_width: u32,
    compressed_block_height: u32,
    compressed_block_depth: u32,
    compressed_block_size: u32,
}

struct PixelBuf<'a> {
    width: u32,
    height: u32,
    format: InternalFormat,
    buf: MaybeBorrowedBuf<'a>,
}

enum MaybeBorrowedBuf<'a> {
    Borrowed(&'a [u8]),
    Owned(BoxBytes),
}

impl<'a> From<&'a [u8]> for MaybeBorrowedBuf<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self::Borrowed(value)
    }
}
impl<T: Pod> From<Vec<T>> for MaybeBorrowedBuf<'static> {
    fn from(value: Vec<T>) -> Self {
        Self::Owned(box_bytes_of(value.into_boxed_slice()))
    }
}
#[derive(Clone, Copy)]
struct UnsizedBuf<'a> {
    ptr: *const u8,
    _boo: PhantomData<&'a ()>,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, Default)]
enum GlAlignment {
    One = 1,
    Two = 2,
    #[default]
    Four = 4,
    Eight = 8,
}
impl PackUnpackState {
    pub(crate) unsafe fn src_fmt_buf_to_internal<'buf>(
        &self,
        in_buf: UnsizedBuf<'buf>,
        src: GlPixelTypeFormat,
        dst: InternalFormat,
        width: u32,
        height: u32,
        proxy: bool,
    ) -> GlFallible<Option<PixelBuf<'buf>>> {
        use NormalizedIntChannel as NI;
        use SingleChannelFormat as SC;
        macro_rules! convert_from {
            ($with_ty:ty) => {
                // SAFETY: caller
                unsafe { self.convert_from::<$with_ty>(in_buf, dst, width, height, proxy) }
            };
        }
        match src.ty {
            PixelType::Byte => match src.fmt {
                PixelFormat::DepthComponent => convert_from!(SC<NI<i8>, Depth>),
                PixelFormat::Red => convert_from!(SC<NI<i8>, Red>),
                PixelFormat::Green => convert_from!(SC<NI<i8>, Green>),
                PixelFormat::Blue => convert_from!(SC<NI<i8>, Blue>),
                PixelFormat::Alpha => {
                    convert_from!(SC<NI<i8>, Alpha>)
                }
                PixelFormat::Rg => convert_from!(RgColorFormat<NI<i8>>),
                PixelFormat::Rgb => convert_from!(RgbColorFormat<NI<i8>>),
                PixelFormat::Rgba => convert_from!(RgbaColorFormat<NI<i8>>),
                PixelFormat::Bgr => convert_from!(BgrColorFormat<NI<i8>>),
                PixelFormat::Bgra => convert_from!(BgraColorFormat<NI<i8>>),
                PixelFormat::RedInteger => convert_from!(SC<i8, Red>),
                PixelFormat::GreenInteger => convert_from!(SC<i8, Green>),
                PixelFormat::BlueInteger => convert_from!(SC<i8, Blue>),
                PixelFormat::RgInteger => convert_from!(RgColorFormat<i8>),
                PixelFormat::RgbInteger => convert_from!(RgbColorFormat<i8>),
                PixelFormat::RgbaInteger => convert_from!(RgbaColorFormat<i8>),
                PixelFormat::BgrInteger => convert_from!(BgrColorFormat<i8>),
                PixelFormat::BgraInteger => convert_from!(BgraColorFormat<i8>),
                _ => unreachable!(),
            },
            PixelType::UnsignedByte => todo!(),
            PixelType::Short => todo!(),
            PixelType::UnsignedShort => todo!(),
            PixelType::Int => todo!(),
            PixelType::UnsignedInt => todo!(),
            PixelType::Float => todo!(),
            PixelType::HalfFloat => todo!(),
            PixelType::UnsignedByte332 => todo!(),
            PixelType::UnsignedShort4444 => todo!(),
            PixelType::UnsignedShort5551 => todo!(),
            PixelType::UnsignedInt8888 => todo!(),
            PixelType::UnsignedInt1010102 => todo!(),
            PixelType::UnsignedByte233Rev => todo!(),
            PixelType::UnsignedShort565 => todo!(),
            PixelType::UnsignedShort565Rev => todo!(),
            PixelType::UnsignedShort4444Rev => todo!(),
            PixelType::UnsignedShort1555Rev => todo!(),
            PixelType::UnsignedInt8888Rev => todo!(),
            PixelType::UnsignedInt2101010Rev => todo!(),
            PixelType::UnsignedInt5999Rev => todo!(),
            PixelType::Float32UnsignedInt248Rev => todo!(),
            PixelType::UnsignedInt248 => todo!(),
            PixelType::UnsignedInt10F11F11FRev => todo!(),
        }
    }
    unsafe fn convert_from<'buf, ReprIn: ConvertibleToInternalFormat + AlignInfo + Pod>(
        &self,
        in_buf: UnsizedBuf<'buf>,
        dst: InternalFormat,
        width: u32,
        height: u32,
        proxy: bool,
    ) -> GlFallible<Option<PixelBuf<'buf>>> {
        macro_rules! dispatch_over {
            (
                $(
                    $val:ident
                ),+
            ) => {
                match dst {
                    $(
                        // SAFETY: caller
                        InternalFormat::$val => unsafe { self.convert_inner::<ReprIn, internal_formats::$val>(in_buf, width, height, proxy) }
                    ),+
                }
            };
        }
        dispatch_over! {
            DepthComponent,
            Red,
            Rgb,
            Rgba,
            R3G3B2,
            Rgb4,
            Rgb5,
            Rgb8,
            Rgb10,
            Rgb12,
            Rgb16,
            Rgba2,
            Rgba4,
            Rgb5A1,
            Rgba8,
            Rgb10A2,
            Rgba12,
            Rgba16,
            CompressedRgb,
            CompressedRgba,
            DepthComponent16,
            DepthComponent24,
            DepthComponent32,
            Srgb,
            Srgb8,
            SrgbAlpha,
            Srgb8Alpha8,
            CompressedSrgb,
            CompressedSrgbAlpha,
            CompressedRed,
            CompressedRg,
            Rgba32f,
            Rgb32f,
            Rgba16f,
            Rgb16f,
            R11fG11fB10f,
            Rgb9E5,
            Rgba32ui,
            Rgb32ui,
            Rgba16ui,
            Rgb16ui,
            Rgba8ui,
            Rgb8ui,
            Rgba32i,
            Rgb32i,
            Rgba16i,
            Rgb16i,
            Rgba8i,
            Rgb8i,
            DepthComponent32f,
            Depth32fStencil8,
            DepthStencil,
            Depth24Stencil8,
            StencilIndex1,
            StencilIndex4,
            StencilIndex16,
            CompressedRedRgtc1,
            CompressedSignedRedRgtc1,
            CompressedRgRgtc2,
            CompressedSignedRgRgtc2,
            Rg,
            R8,
            R16,
            Rg8,
            Rg16,
            R16f,
            R32f,
            Rg16f,
            Rg32f,
            R8i,
            R8ui,
            R16i,
            R16ui,
            R32i,
            R32ui,
            Rg8i,
            Rg8ui,
            Rg16i,
            Rg16ui,
            Rg32i,
            Rg32ui,
            R8Snorm,
            Rg8Snorm,
            Rgb8Snorm,
            Rgba8Snorm,
            R16Snorm,
            Rg16Snorm,
            Rgb16Snorm,
            Rgba16Snorm,
            Rgb10A2ui,
            Rgb565,
            CompressedRgbaBptcUnorm,
            CompressedSrgbAlphaBptcUnorm,
            CompressedRgbBptcSignedFloat,
            CompressedRgbBptcUnsignedFloat,
            CompressedRgb8Etc2,
            CompressedSrgb8Etc2,
            CompressedRgb8PunchthroughAlpha1Etc2,
            CompressedSrgb8PunchthroughAlpha1Etc2,
            CompressedRgba8Etc2Eac,
            CompressedSrgb8Alpha8Etc2Eac,
            CompressedR11Eac,
            CompressedSignedR11Eac,
            CompressedRg11Eac,
            CompressedSignedRg11Eac,
            StencilIndex,
            StencilIndex8
        }
    }
    unsafe fn convert_inner<
        'buf,
        T: ConvertPixel<U> + AlignInfo + Pod,
        U: InternalFormatInfo + Pod,
    >(
        &self,
        in_buf: UnsizedBuf<'buf>,
        width: u32,
        height: u32,
        proxy: bool,
    ) -> GlFallible<Option<PixelBuf<'buf>>> {
        let () = T::CHECK_ALIGN;
        gl_assert!(U::copyable(), InvalidOperation);

        let pixels_per_row = self.row_length.unwrap_or(width);
        let pixel_size = size_of::<T>();
        // See Eqns. 8.1, 8.2, GL Core Spec v4.6

        // value corresponds to `n * l * s`
        let mut groups_per_row = pixels_per_row as usize;

        // Eq. 8.2, inherent_align = `s`, self.alignment = `a`
        if let Some(inherent_align) = T::CHANNEL_ALIGN {
            // Only Some "If the number of bits per element is [] 1, 2, 4, or 8 times the number of bits in a GL `ubyte`"
            if (inherent_align as u8) < (self.alignment as u8) {
                // bottom branch of Eqn. 8.2
                groups_per_row =
                    (groups_per_row * pixel_size).div_ceil(self.alignment as u8 as usize);
                // one of: 1/1, 2/1, 2/2, 4/1, 4/2, 4/4, 8/1, 8/2, 8/4, 8/8, result will always be exact
                groups_per_row *=
                    usize::from(self.alignment as u8) / usize::from(inherent_align as u8);
            }
        }

        let skip_pixel_offset = self.skip_pixels as usize;
        let initial_offset = groups_per_row * self.skip_rows as usize + skip_pixel_offset;
        let do_swap = self.swap_bytes && T::SHOULD_SWAP_BYTES;

        let base = in_buf.ptr.cast::<T>();

        if proxy {
            return Ok(None);
        }

        // If we don't need to perform a conversion or trimming, just reborrow the buffer with the correct length
        if skip_pixel_offset == 0
            && pixels_per_row as usize == groups_per_row
            && !do_swap
            && U::repr_typeid() == TypeId::of::<T>()
        {
            // Safety: T: Pod
            let bytes = unsafe {
                slice::from_raw_parts(
                    // Safety: caller ensures buffer actually contains all of the pixels they say it does
                    base.cast::<u8>().add(initial_offset),
                    height as usize * groups_per_row * pixel_size,
                )
            };
            return Ok(Some(PixelBuf {
                width: pixels_per_row,
                height,
                format: U::id(),
                buf: bytes.into(),
            }));
        }
        let mut v: Vec<U> = Vec::with_capacity((pixels_per_row * height) as usize);
        for row in 0..height {
            // Safety: caller ensures buffer actually contains all of the pixels they say it does
            let mut ptr = unsafe { base.add(row as usize * groups_per_row + skip_pixel_offset) };
            let needs_unaligned = base.is_aligned();
            for _ in 0..pixels_per_row {
                let mut pixel = unsafe {
                    match needs_unaligned {
                        true => ptr::read_unaligned(ptr),
                        false => ptr::read(ptr),
                    }
                };
                if do_swap {
                    // Safety: T: Pod
                    unsafe {
                        slice::from_raw_parts_mut(ptr::from_mut(&mut pixel).cast::<u8>(), T::SIZE)
                    }
                    .reverse();
                }
                // Safety: caller ensures buffer actually contains all of the pixels they say it does
                unsafe { ptr = ptr.add(1) };
                v.push(pixel.convert_pixel());
            }
        }
        Ok(Some(PixelBuf {
            width: pixels_per_row,
            height,
            format: U::id(),
            buf: v.into(),
        }))
    }
}

fn convert_dispatch_test(buf: *const u8, src_fmt: GlPixelTypeFormat, dst: InternalFormat) {
    // special type handling
    match src_fmt.ty {
        // "normal" pixel types
        PixelType::Byte => match src_fmt.fmt {
            PixelFormat::DepthComponent => todo!(),
            PixelFormat::Red => todo!(),
            PixelFormat::Green => todo!(),
            PixelFormat::Blue => todo!(),
            PixelFormat::Alpha => todo!(),
            PixelFormat::Rgb => todo!(),
            PixelFormat::Rgba => todo!(),
            PixelFormat::Bgr => todo!(),
            PixelFormat::Bgra => todo!(),
            PixelFormat::RedInteger => todo!(),
            PixelFormat::GreenInteger => todo!(),
            PixelFormat::BlueInteger => todo!(),
            PixelFormat::RgbInteger => todo!(),
            PixelFormat::RgbaInteger => todo!(),
            PixelFormat::BgrInteger => todo!(),
            PixelFormat::BgraInteger => todo!(),
            PixelFormat::DepthStencil => todo!(),
            PixelFormat::Rg => todo!(),
            PixelFormat::RgInteger => todo!(),
            PixelFormat::StencilIndex => todo!(),
            _ => unreachable!(),
        },
        PixelType::UnsignedByte => todo!(),
        PixelType::Short => todo!(),
        PixelType::UnsignedShort => todo!(),
        PixelType::Int => todo!(),
        PixelType::UnsignedInt => todo!(),
        PixelType::Float => todo!(),
        PixelType::HalfFloat => todo!(),

        PixelType::UnsignedByte332 => todo!(),
        PixelType::UnsignedShort4444 => todo!(),
        PixelType::UnsignedShort5551 => todo!(),
        PixelType::UnsignedInt8888 => todo!(),
        PixelType::UnsignedInt1010102 => todo!(),
        PixelType::UnsignedByte233Rev => todo!(),
        PixelType::UnsignedShort565 => todo!(),
        PixelType::UnsignedShort565Rev => todo!(),
        PixelType::UnsignedShort4444Rev => todo!(),
        PixelType::UnsignedShort1555Rev => todo!(),
        PixelType::UnsignedInt8888Rev => todo!(),
        PixelType::UnsignedInt2101010Rev => todo!(),
        PixelType::UnsignedInt5999Rev => todo!(),
        PixelType::Float32UnsignedInt248Rev => todo!(),
        PixelType::UnsignedInt248 => todo!(),
        PixelType::UnsignedInt10F11F11FRev => todo!(),
    }
}

#[derive(Debug, Copy, Clone)]
/// A combination of a [`PixelType`] and a [`PixelFormat`], used to specify input and output formats.
/// When user CPU code requests or supplies textures, they specify the format with this pair, as opposed to using an [`InternalFormat`]
pub struct GlPixelTypeFormat {
    ty: PixelType,
    fmt: PixelFormat,
}

impl GlPixelTypeFormat {
    pub fn new(ty: PixelType, fmt: PixelFormat) -> Self {
        Self { ty, fmt }
    }
    /// Returns the `InternalFormat` that matches this type format pair, if any. No ABI compatability guarantees are made about the format of this [`GlPixelTypeFormat`] and the returned [`InternalFormat`].
    pub(crate) fn equivalent_internal_format(self) -> Option<InternalFormat> {
        gl_trace!(
            "attempting to lower (type {:?}, fmt {:?}) tuple to a metal pixel format",
            self.ty,
            self.fmt
        );
        // 'none: {
        //     return Some(match self.ty {
        //         PixelType::Byte => match self.fmt {
        //             Red => IF::R8Snorm,
        //             Rg => IF::Rg8Snorm,
        //             Rgba => IF::Rgb8,
        //             RedInteger => IF::R8i,
        //             RgInteger => IF::Rg8i,
        //             RgbaInteger => IF::Rgba8i,
        //             StencilIndex => IF::StencilIndex8,
        //             _ => break 'none,
        //         },
        //         PixelType::UnsignedByte => match self.fmt {
        //             Red => IF::R8,
        //             Rg => IF::Rg8,
        //             Rgba => IF::Rgba8,
        //             RedInteger => IF::R8ui,
        //             RgInteger => IF::Rg8ui,
        //             RgbaInteger => IF::Rgba8ui,
        //             StencilIndex => IF::StencilIndex8,
        //             _ => break 'none,
        //         },
        //         PixelType::Short => match self.fmt {
        //             Red => IF::R16Snorm,
        //             Rg => IF::Rg16Snorm,
        //             Rgba => IF::Rgba16Snorm,
        //             RedInteger => IF::R16i,
        //             RgInteger => IF::Rg16i,
        //             RgbaInteger => IF::Rgba16i,

        //             DepthComponent => IF::DepthComponent16,
        //             StencilIndex => IF::StencilIndex16,
        //             _ => break 'none,
        //         },
        //         PixelType::UnsignedShort => match self.fmt {
        //             Red => IF::R16,
        //             Rg => IF::Rg16,
        //             Rgba => IF::Rgba16,
        //             RedInteger => IF::R16ui,
        //             RgInteger => IF::Rg16ui,
        //             RgbaInteger => IF::Rgba16ui,

        //             DepthComponent => IF::DepthComponent16,
        //             StencilIndex => IF::StencilIndex16,
        //             _ => break 'none,
        //         },

        //         // TODO: emulate GL normalization behavior for (U)Int,
        //         // those formats are not supported for now
        //         PixelType::Int => match self.fmt {
        //             // Red => IF::R32ui,
        //             // Rg => IF::RG32Snorm,
        //             // Rgba => IF::Rgba32Snorm,
        //             RedInteger => IF::R32i,
        //             RgInteger => IF::Rg32i,
        //             RgbaInteger => IF::Rgba32i,

        //             DepthComponent => IF::DepthComponent32,
        //             _ => break 'none,
        //         },
        //         PixelType::UnsignedInt => match self.fmt {
        //             // Red => IF::R32Unorm,
        //             // Rg => IF::RG32Unorm,
        //             // Rgba => IF::Rgba32Unorm,
        //             RedInteger => IF::R32ui,
        //             RgInteger => IF::Rg32ui,
        //             RgbaInteger => IF::Rgba32ui,

        //             DepthComponent => IF::DepthComponent32,
        //             _ => break 'none,
        //         },
        //         PixelType::Float => match self.fmt {
        //             Red => IF::R32f,
        //             Rg => IF::Rg32f,
        //             Rgba => IF::Rgba32f,
        //             DepthComponent => IF::DepthComponent32f,
        //             DepthStencil => IF::Depth24Stencil8,
        //             _ => break 'none,
        //         },
        //         PixelType::HalfFloat => match self.fmt {
        //             Red => IF::R16f,
        //             Rg => IF::Rg16f,
        //             Rgba => IF::Rgba16f,
        //             _ => break 'none,
        //         },
        //         // special-purpose formats
        //         PixelType::UnsignedInt10F11F11FRev => todo!(),
        //         PixelType::UnsignedShort565 => IF::Rgb565,
        //         PixelType::UnsignedInt8888 => IF::Rgba8,
        //         PixelType::UnsignedInt1010102 => IF::Rgb10A2,
        //         PixelType::UnsignedInt2101010Rev => IF::BGR10A2Unorm,
        //         PixelType::UnsignedShort4444Rev => IF::ABGR4Unorm,
        //         PixelType::UnsignedShort1555Rev => IF::A1BGR5Unorm,

        //         PixelType::UnsignedInt8888Rev => break 'none,
        //         PixelType::UnsignedByte332 => todo!(),
        //         PixelType::UnsignedShort4444 => todo!(),
        //         PixelType::UnsignedShort5551 => todo!(),
        //         PixelType::UnsignedByte233Rev => todo!(),
        //         PixelType::UnsignedInt5999Rev => todo!(),
        //         PixelType::Float32UnsignedInt248Rev => todo!(),
        //         PixelType::UnsignedInt248 => todo!(),
        //         PixelType::UnsignedShort565Rev => todo!(),
        //     });
        // }
        todo!();
        gl_warn!(
            ty: Performance,
            "failed to lower format+type tuple (type {:?}, fmt {:?}) to an internal format.
            OxideGL will attempt to perform conversions but good performance is not guaranteed",
            self.ty,
            self.fmt
        );
        None
    }
}
