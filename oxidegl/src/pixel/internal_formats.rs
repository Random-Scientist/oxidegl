// FIXME remove when implemented
#![allow(unused)]
use super::{
    ConvertPixel, Depth, NormalizedIntChannel, RgColorFormat, RgbaColorFormat, SingleChannelFormat,
    Stencil,
};
use crate::{
    gl_enums::InternalFormat,
    pixel::{ChannelSource, ChannelType, ConvertChannel, channelsource_default},
};
use bytemuck::{Pod, Zeroable};
use half::f16;
use objc2_metal::MTLPixelFormat;
use std::any::TypeId;
use std::marker::PhantomData;

// meow

pub(crate) trait InternalFormatInfo: Copy {
    /// The underlying [`MTLPixelFormat`] used to represent this [`InternalFormat`].
    fn mtl_tex_format() -> MTLPixelFormat;
    fn copyable() -> bool;
    fn view_class() -> Option<TextureViewClass>;
    fn id() -> InternalFormat;
    fn repr_typeid() -> TypeId;
}

trait WrappedFormat {
    type Inner;
    fn from_inner(inner: Self::Inner) -> Self;
    fn into_inner(self) -> Self::Inner;
}
impl<T: WrappedFormat<Inner = InnerTy>, InnerTy, S: ConvertPixel<InnerTy>> ConvertPixel<T> for S {
    #[inline]
    fn convert_pixel(self) -> T {
        T::from_inner(self.convert_pixel())
    }
}
impl<T: WrappedFormat<Inner = InnerTy> + Copy, InnerTy, Ctype: ChannelType, Cfmt>
    ChannelSource<Ctype> for T
where
    InnerTy: ChannelSource<Ctype, Channel = Cfmt>,
{
    type Channel = Cfmt;
    #[inline]
    fn channel(self) -> Self::Channel {
        self.into_inner().channel()
    }
}

macro_rules! decl_internal_formats {
    (
        unsafe {
            $(
                #[mtl_format = $mtl_format:ident]
                repr( $( $body:tt )* ) {
                    $(
                        $( #[doc = $doc:expr] )*
                        $( #[view_class = $view_class:ident ] )?
                        $( #[not_copyable $_mark:vis] )?
                        format struct $format:ident;
                    )+
                }
            )+
        }
    ) => {
        $(
            ::concat_idents::concat_idents! {
                fmt_name = Mtl, $mtl_format {
                    #[allow(non_camel_case_types)]
                    #[derive(Clone, Copy, Debug, ::bytemuck::Pod, ::bytemuck::Zeroable)]
                    #[repr(transparent)]
                    pub(crate) struct fmt_name( $( $body )* );
                    impl WrappedFormat for fmt_name {
                        type Inner = $( $body )*;
                        #[inline]
                        fn from_inner(inner: Self::Inner) -> Self {
                            Self(inner)
                        }
                        #[inline]
                        fn into_inner(self) -> Self::Inner {
                            self.0
                        }
                    }
                }

            }

            $(
                ::concat_idents::concat_idents! {
                    fmt_name = Mtl, $mtl_format {
                        $( #[doc = $doc] )*
                        #[repr(transparent)]
                        #[derive(Clone, Copy, Debug, ::bytemuck::Pod, ::bytemuck::Zeroable)]
                        pub(crate) struct $format( fmt_name );
                        impl WrappedFormat for $format {
                            type Inner = fmt_name;
                            #[inline]
                            fn from_inner(inner: Self::Inner) -> Self {
                                Self(inner)
                            }
                            #[inline]
                            fn into_inner(self) -> Self::Inner {
                                self.0
                            }
                        }

                    }
                }
                impl InternalFormatInfo for $format {
                    fn mtl_tex_format() -> MTLPixelFormat {
                        MTLPixelFormat::$mtl_format
                    }
                    fn copyable() -> bool {
                        decl_internal_formats! { present: $(v $_mark)? }
                    }
                    fn view_class() -> Option<TextureViewClass> {
                        decl_internal_formats! {
                            optional: $( $view_class )?
                        }
                    }
                    fn id() -> InternalFormat {
                        InternalFormat::$format
                    }
                    #[inline]
                    fn repr_typeid() -> TypeId {
                        TypeId::of::<<<$format as WrappedFormat>::Inner as WrappedFormat>::Inner>()
                    }
                }
            )+
        )+
        impl InternalFormat {
            pub(crate) fn mtl_tex_format(self) -> MTLPixelFormat {
                match self {
                    $(
                        $(
                            Self::$format => $format::mtl_tex_format(),
                        )+
                    )+
                    _ => panic!("unsupported format {self:#?}")
                }
            }
            pub(crate) fn copyable(self) -> bool {
                match self {
                    $(
                        $(
                            Self::$format => $format::copyable(),
                        )+
                    )+
                    _ => panic!("unsupported format {self:#?}")
                }
            }
            pub(crate) fn view_class(self) -> Option<TextureViewClass> {
                match self {
                    $(
                        $(
                            Self::$format => $format::view_class(),
                        )+
                    )+
                    _ => panic!("unsupported format {self:#?}")
                }
            }
        }
        pub(crate) trait ConvertibleToInternalFormat: $( $(ConvertPixel<$format> + )+ )+ Copy + 'static {}
        impl<T: $( $(ConvertPixel<$format> + )+ )+ Copy + 'static> ConvertibleToInternalFormat for T {}
    };
    // helpers
    (
        present: v
    ) => {
        true
    };
    (
        present:
    ) => {
        false
    };
    (
        optional: $val:ident
    ) => {
        Some(TextureViewClass::$val)
    };
    (
        optional:
    ) => {
        None
    };
}
#[test]
fn test() {
    let v = Rgba8i(MtlRGBA8Sint(RgbaColorFormat {
        colors: [1, 2, 3, 4],
    }));
    let c: u8 = 1i8.convert_channel();
    let r: R8 = v.convert_pixel();
    let r: SingleChannelFormat<NormalizedIntChannel<u8>, super::Red> = RgbaColorFormat {
        colors: [1i8, 2, 3, 4],
    }
    .convert_pixel();

    let rgba8 = Rgba8(MtlRGBA8Unorm(RgbaColorFormat {
        colors: [
            NormalizedIntChannel(50u8),
            NormalizedIntChannel(100u8),
            NormalizedIntChannel(0u8),
            NormalizedIntChannel(0u8),
        ],
    }));
    let rg32: RgColorFormat<f16> = rgba8.convert_pixel();
    dbg!(rg32);
}
// TODO abi check macro helper that perform basic ABI sanity checks on the metal wrapper structs (we already have `unsafe` to signify that the layouts are user defined but a little more safety wouldn't hurt)

decl_internal_formats! {
    unsafe {
        // Red
        // 8bit
        #[mtl_format = R8Unorm]
        repr(SingleChannelFormat<NormalizedIntChannel<u8>, super::Red>) {
            #[view_class = Bits8]
            format struct R8;
            format struct CompressedRed;
            format struct Red;
        }
        #[mtl_format = R8Snorm]
        repr(SingleChannelFormat<NormalizedIntChannel<i8>, super::Red>) {
            #[view_class = Bits8]
            format struct R8Snorm;
        }
        #[mtl_format = R8Sint]
        repr(SingleChannelFormat<i8, super::Red>) {
            #[view_class = Bits8]
            format struct R8i;
        }
        #[mtl_format = R8Uint]
        repr(SingleChannelFormat<u8, super::Red>) {
            #[view_class = Bits8]
            format struct R8ui;
        }

        // 16 bit
        #[mtl_format = R16Unorm]
        repr(SingleChannelFormat<NormalizedIntChannel<u16>, super::Red>) {
            #[view_class = Bits16]
            format struct R16;
        }
        #[mtl_format = R16Snorm]
        repr(SingleChannelFormat<NormalizedIntChannel<i16>, super::Red>) {
            #[view_class = Bits16]
            format struct R16Snorm;
        }
        #[mtl_format = R16Sint]
        repr(SingleChannelFormat<i16, super::Red>) {
            #[view_class = Bits16]
            format struct R16i;
        }
        #[mtl_format = R16Uint]
        repr(SingleChannelFormat<u16, super::Red>) {
            #[view_class = Bits16]
            format struct R16ui;
        }

        // 32 bit
        // #[mtl_format = R32Unorm]
        // repr(SingleChannelFormat<NormalizedIntChannel<u32>, super::Red>) {
        //     #[view_class = Bits32]
        //     format struct R32;
        // }
        // #[mtl_format = R32Snorm]
        // repr(SingleChannelFormat<NormalizedIntChannel<i32>, super::Red>) {
        //     #[view_class = Bits32]
        //     format struct R32Snorm;
        // }
        #[mtl_format = R32Sint]
        repr(SingleChannelFormat<i32, super::Red>) {
            #[view_class = Bits32]
            format struct R32i;
        }
        #[mtl_format = R32Uint]
        repr(SingleChannelFormat<u32, super::Red>) {
            #[view_class = Bits32]
            format struct R32ui;
        }

        #[mtl_format = R32Float]
        repr(SingleChannelFormat<f32, super::Red>) {
            #[view_class = Bits32]
            format struct R32f;
        }
        #[mtl_format = R16Float]
        repr(SingleChannelFormat<f16, super::Red>) {
            #[view_class = Bits16]
            format struct R16f;
        }

        // Red Green

        // 8 bit
        #[mtl_format = RG8Unorm]
        repr(RgColorFormat<NormalizedIntChannel<u8>>) {
            #[view_class = Bits16]
            format struct Rg8;
            format struct Rg;
            format struct CompressedRg;
        }
        #[mtl_format = RG8Snorm]
        repr(RgColorFormat<NormalizedIntChannel<i8>>) {
            #[view_class = Bits16]
            format struct Rg8Snorm;
        }
        #[mtl_format = RG8Sint]
        repr(RgColorFormat<i8>) {
            #[view_class = Bits16]
            format struct Rg8i;
        }
        #[mtl_format = RG8Uint]
        repr(RgColorFormat<u8>) {
            #[view_class = Bits16]
            format struct Rg8ui;
        }

        // 16 bit
        #[mtl_format = RG16Unorm]
        repr(RgColorFormat<NormalizedIntChannel<u16>>) {
            #[view_class = Bits32]
            format struct Rg16;

        }
        #[mtl_format = RG16Snorm]
        repr(RgColorFormat<NormalizedIntChannel<i16>>) {
            #[view_class = Bits32]
            format struct Rg16Snorm;
        }
        #[mtl_format = RG16Sint]
        repr(RgColorFormat<i16>) {
            #[view_class = Bits32]
            format struct Rg16i;
        }
        #[mtl_format = RG16Uint]
        repr(RgColorFormat<u16>) {
            #[view_class = Bits32]
            format struct Rg16ui;
        }

        // 32 bit
        // #[mtl_format = RG32Unorm]
        // repr(RgColorFormat<NormalizedIntChannel<u32>>) {
        //     #[view_class = Bits64]
        //     format struct Rg32;
        // }
        // #[mtl_format = RG32Snorm]
        // repr(RgColorFormat<NormalizedIntChannel<i32>>) {
        //     #[view_class = Bits64]
        //     format struct Rg32Snorm;
        // }
        #[mtl_format = RG32Sint]
        repr(RgColorFormat<i32>) {
            #[view_class = Bits64]
            format struct Rg32i;
        }
        #[mtl_format = RG32Uint]
        repr(RgColorFormat<u32>) {
            #[view_class = Bits64]
            format struct Rg32ui;
        }

        #[mtl_format = RG16Float]
        repr(RgColorFormat<f16>) {
            #[view_class = Bits16]
            format struct Rg16f;
        }
        #[mtl_format = RG32Float]
        repr(RgColorFormat<f32>) {
            #[view_class = Bits32]
            format struct Rg32f;
        }


        // rgb and rgba
        // 4 bit
        #[mtl_format = ABGR4Unorm]
        repr(ReprABGR4) {
            format struct Rgb4;
            #[view_class = Bits16]
            format struct Rgba4;
        }
        // 5 bit
        #[mtl_format = A1BGR5Unorm]
        // repr(u16) {
            repr(Unimplemented) {
            format struct Rgb5;
            #[view_class = Bits16]
            format struct Rgb5A1;
        }

        // 8 bit
        #[mtl_format = RGBA8Unorm]
        repr(RgbaColorFormat<NormalizedIntChannel<u8>>) {
            #[view_class = Bits24]
            format struct Rgb8;
            format struct Rgb;
            #[view_class = Bits32]
            format struct Rgba8;
            format struct Rgba;
            format struct CompressedRgb;
            format struct CompressedRgba;
        }
        #[mtl_format = RGBA8Snorm]
        repr(RgbaColorFormat<NormalizedIntChannel<i8>>) {
            #[view_class = Bits24]
            format struct Rgb8Snorm;
            #[view_class = Bits32]
            format struct Rgba8Snorm;
        }
        #[mtl_format = RGBA8Sint]
        repr(RgbaColorFormat<i8>) {
            #[view_class = Bits24]
            format struct Rgb8i;
            #[view_class = Bits32]
            format struct Rgba8i;
        }
        #[mtl_format = RGBA8Uint]
        repr(RgbaColorFormat<u8>) {
            #[view_class = Bits24]
            format struct Rgb8ui;
            #[view_class = Bits32]
            format struct Rgba8ui;
        }
        // 10 bit
        #[mtl_format = RGB10A2Unorm]
        // repr(u32) {
            repr(Unimplemented) {
            format struct Rgb10;
            #[view_class = Bits32]
            format struct Rgb10A2;
        }
        // 16 bit
        #[mtl_format = RGBA16Unorm]
        repr(RgbaColorFormat<NormalizedIntChannel<u16>>) {
            #[view_class = Bits48]
            format struct Rgb16;
            format struct Rgb12;
            #[view_class = Bits64]
            format struct Rgba16;
            #[view_class = Bits48]
            format struct Rgba12;
        }
        #[mtl_format = RGBA16Snorm]
        repr(RgbaColorFormat<NormalizedIntChannel<i16>>) {
            #[view_class = Bits48]
            format struct Rgb16Snorm;
            #[view_class = Bits64]
            format struct Rgba16Snorm;
        }
        #[mtl_format = RGBA16Sint]
        repr(RgbaColorFormat<i16>) {
            #[view_class = Bits48]
            format struct Rgb16i;
            #[view_class = Bits64]
            format struct Rgba16i;
        }
        #[mtl_format = RGBA16Uint]
        repr(RgbaColorFormat<u16>) {
            #[view_class = Bits48]
            format struct Rgb16ui;
            #[view_class = Bits64]
            format struct Rgba16ui;
        }

         // 32 bit
        //  #[mtl_format = RGBA32Unorm]
        //  repr(RgbaColorFormat<NormalizedIntChannel<u32>>) {
        //      #[view_class = Bits96]
        //      format struct Rgb32;
        //      #[view_class = Bits128]
        //      format struct Rgba32;
        //  }
        //  #[mtl_format = RGBA32Snorm]
        //  repr(RgbaColorFormat<NormalizedIntChannel<i32>>) {
        //      #[view_class = Bits96]
        //      format struct Rgb32Snorm;
        //      #[view_class = Bits128]
        //      format struct Rgba32Snorm;
        //  }
        #[mtl_format = RGBA32Sint]
        repr(RgbaColorFormat<i32>) {
            #[view_class = Bits96]
            format struct Rgb32i;
            #[view_class = Bits128]
            format struct Rgba32i;
        }
        #[mtl_format = RGBA32Uint]
        repr(RgbaColorFormat<u32>) {
            #[view_class = Bits96]
            format struct Rgb32ui;
            #[view_class = Bits128]
            format struct Rgba32ui;
        }
        #[mtl_format = RGBA32Float]
        repr(RgbaColorFormat<f32>) {
            #[view_class = Bits128]
            format struct Rgba32f;
            #[view_class = Bits96]
            format struct Rgb32f;
        }
        #[mtl_format = RGBA16Float]
        repr(RgbaColorFormat<f32>) {
            #[view_class = Bits64]
            format struct Rgba16f;
            #[view_class = Bits48]
            format struct Rgb16f;
        }
        #[mtl_format = RGBA8Unorm_sRGB]
        repr(RgbaColorFormat<NormalizedIntChannel<u8>>) {
            format struct CompressedSrgbAlpha;
            format struct CompressedSrgb;
            format struct Srgb;
            format struct SrgbAlpha;
            #[view_class = Bits24]
            format struct Srgb8;
            #[view_class = Bits32]
            format struct Srgb8Alpha8;
        }

        #[mtl_format = Stencil8]
        repr(SingleChannelFormat<u8, Stencil>) {
            format struct StencilIndex1;
            format struct StencilIndex4;
            format struct StencilIndex;
            format struct StencilIndex8;
        }
        #[mtl_format = Depth32Float_Stencil8]
        // repr([u8; 5]) {
        repr(Unimplemented) {
            format struct Depth32fStencil8;
        }
        #[mtl_format = Depth24Unorm_Stencil8]
        // repr(u32) {
        repr(Unimplemented) {
            format struct DepthComponent24;
            format struct Depth24Stencil8;
            format struct DepthStencil;
        }

        #[mtl_format = Depth32Float]
        repr(SingleChannelFormat<f32, Depth>) {
            format struct DepthComponent;
            format struct DepthComponent32f;
            // should be DepthComponent32Unorm but metal doesn't have that. We just use floats instead :)
            format struct DepthComponent32;
        }
        #[mtl_format = RG11B10Float]
        // repr(u32) {
        repr(Unimplemented) {
            #[view_class = Bits32]
            format struct R11fG11fB10f;
        }
        #[mtl_format = RGB10A2Uint]
        // repr(u32) {
        repr(Unimplemented) {
            format struct Rgb10A2ui;
        }
        #[mtl_format = Invalid]
        repr(Unimplemented) {
            format struct Rgb9E5;
            format struct R3G3B2;
            format struct Rgb565;
            format struct Rgba2;
            format struct DepthComponent16;
            format struct StencilIndex16;
            format struct CompressedRedRgtc1;
            format struct CompressedSignedRedRgtc1;
            format struct CompressedRgRgtc2;
            format struct CompressedSignedRgRgtc2;

            format struct CompressedRgbaBptcUnorm;
            format struct CompressedSrgbAlphaBptcUnorm;
            format struct CompressedRgbBptcSignedFloat;
            format struct CompressedRgbBptcUnsignedFloat;
            format struct CompressedRgb8Etc2;
            format struct CompressedSrgb8Etc2;
            format struct CompressedRgb8PunchthroughAlpha1Etc2;
            format struct CompressedSrgb8PunchthroughAlpha1Etc2;
            format struct CompressedRgba8Etc2Eac;
            format struct CompressedSrgb8Alpha8Etc2Eac;
            format struct CompressedR11Eac;
            format struct CompressedSignedR11Eac;
            format struct CompressedRg11Eac;
            format struct CompressedSignedRg11Eac;
        }
    }
}
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct Unimplemented;
impl<T> ConvertPixel<Unimplemented> for T {
    fn convert_pixel(self) -> Unimplemented {
        panic!("format not implemented")
    }
}
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct ReprABGR4 {
    inner: u16,
}
impl ReprABGR4 {
    const MAX_COMPONENT: u16 = 0b1111;
    fn index(self, idx: u32) -> u8 {
        ((self.inner >> (idx * 4)) & Self::MAX_COMPONENT) as u8
    }
    fn index_float<const N: u32>(self) -> f32 {
        self.index(N) as f32 / Self::MAX_COMPONENT as f32
    }
    fn from_bytes(vals: [u8; 4]) -> Self {
        let mut inner = 0;
        for v in vals {
            inner |= (v >> 4) as u16 & Self::MAX_COMPONENT;
            inner <<= 4;
        }
        Self { inner }
    }
}
impl ChannelSource<super::Red> for ReprABGR4 {
    type Channel = f32;
    fn channel(self) -> Self::Channel {
        self.index_float::<3>()
    }
}
impl ChannelSource<super::Green> for ReprABGR4 {
    type Channel = f32;
    fn channel(self) -> Self::Channel {
        self.index_float::<2>()
    }
}
impl ChannelSource<super::Blue> for ReprABGR4 {
    type Channel = f32;
    fn channel(self) -> Self::Channel {
        self.index_float::<1>()
    }
}
impl ChannelSource<super::Alpha> for ReprABGR4 {
    type Channel = f32;
    fn channel(self) -> Self::Channel {
        self.index_float::<0>()
    }
}
impl<
    T: ChannelSource<super::Red, Channel: ConvertChannel<NormalizedIntChannel<u8>>>
        + ChannelSource<super::Green, Channel: ConvertChannel<NormalizedIntChannel<u8>>>
        + ChannelSource<super::Blue, Channel: ConvertChannel<NormalizedIntChannel<u8>>>
        + ChannelSource<super::Alpha, Channel: ConvertChannel<NormalizedIntChannel<u8>>>,
> ConvertPixel<ReprABGR4> for T
{
    fn convert_pixel(self) -> ReprABGR4 {
        ReprABGR4::from_bytes(
            [
                <Self as ChannelSource<super::Red>>::get(self),
                <Self as ChannelSource<super::Green>>::get(self),
                <Self as ChannelSource<super::Blue>>::get(self),
                <Self as ChannelSource<super::Alpha>>::get(self),
            ]
            .map(|v| v.0),
        )
    }
}
channelsource_default! {
    default impl ChannelSource<super::Stencil> for ReprABGR4 {}
    default impl ChannelSource<super::Depth> for ReprABGR4 {}
}

// #[allow(clippy::enum_glob_use)]
// impl InternalFormat {
//     #[allow(clippy::too_many_lines)]
//     pub(crate) const fn mtl_texture_format(self) -> MTLPixelFormat {
//         use InternalFormat::*;
//         use MTLPixelFormat as M;
//         // fallback to uncompressed for "default" compressed formats. People really shouldn't be using them anyways...
//         // TODO should emit a performance warning from compressedXYZ fallbacks
//         match self {
//             Rgb4 | Rgba4 => M::ABGR4Unorm,
//             Rgb5 | Rgb5A1 => M::A1BGR5Unorm,

//             CompressedRgba | CompressedRgb | Rgb8 | Rgb | Rgba | Rgba8 => M::RGBA8Unorm,
//             Rgba8Snorm | Rgb8Snorm => M::RGBA8Snorm,

//             Rgb10 | Rgb10A2 => M::RGB10A2Unorm,
//             Rgba16Snorm | Rgb16Snorm => M::RGBA16Snorm,
//             Rgb16 | Rgb12 | Rgba16 | Rgba12 => M::RGBA16Unorm,

//             DepthComponent16 => M::Depth16Unorm,
//             DepthComponent | DepthComponent32f | DepthComponent32 => M::Depth32Float,

//             Depth32fStencil8 => M::Depth32Float_Stencil8,
//             DepthComponent24 | Depth24Stencil8 | DepthStencil => M::Depth24Unorm_Stencil8,

//             StencilIndex1 | StencilIndex4 | StencilIndex | StencilIndex8 => M::Stencil8,

//             CompressedSrgbAlpha | CompressedSrgb | Srgb8 | Srgb | Srgb8Alpha8 | SrgbAlpha => {
//                 M::RGBA8Unorm_sRGB
//             }

//             // etc2 and eac
//             CompressedRgb8Etc2 => M::ETC2_RGB8,
//             CompressedSrgb8Etc2 => M::ETC2_RGB8_sRGB,
//             CompressedRgba8Etc2Eac => M::ETC2_RGB8A1,
//             CompressedSrgb8Alpha8Etc2Eac => M::ETC2_RGB8A1_sRGB,
//             CompressedR11Eac => M::EAC_R11Unorm,
//             CompressedSignedR11Eac => M::EAC_R11Snorm,
//             CompressedRg11Eac => M::EAC_RG11Unorm,
//             CompressedSignedRg11Eac => M::EAC_RG11Snorm,

//             // rgtc (aka BC4 and 5)
//             CompressedRedRgtc1 => M::BC4_RUnorm,
//             CompressedSignedRedRgtc1 => M::BC4_RSnorm,
//             CompressedRgRgtc2 => M::BC5_RGUnorm,
//             CompressedSignedRgRgtc2 => M::BC5_RGSnorm,

//             // bptc (aka BC6H and BC7)
//             CompressedRgbaBptcUnorm => M::BC7_RGBAUnorm,
//             CompressedSrgbAlphaBptcUnorm => M::BC7_RGBAUnorm_sRGB,
//             CompressedRgbBptcSignedFloat => M::BC6H_RGBFloat,
//             CompressedRgbBptcUnsignedFloat => M::BC6H_RGBUfloat,

//             Rgb32f | Rgba32f => M::RGBA32Float,
//             Rgb16f | Rgba16f => M::RGBA16Float,
//             R11fG11fB10f => M::RG11B10Float,

//             Rgb9E5 => M::RGB9E5Float,
//             Rgb32ui | Rgba32ui => M::RGBA32Uint,
//             Rgb16ui | Rgba16ui => M::RGBA16Uint,
//             Rgb8ui | Rgba8ui => M::RGBA8Uint,

//             Rgb32i | Rgba32i => M::RGBA32Sint,
//             Rgb16i | Rgba16i => M::RGBA16Sint,
//             Rgb8i | Rgba8i => M::RGBA8Sint,

//             R8 | CompressedRed | Red => M::R8Unorm,
//             R8Snorm => M::R8Snorm,
//             R8i => M::R8Sint,
//             R8ui => M::R8Uint,

//             R16 => M::R16Unorm,
//             R16Snorm => M::R16Snorm,
//             R16i => M::R16Sint,
//             R16ui => M::R16Uint,
//             R16f => M::R16Float,

//             R32i => M::R32Sint,
//             R32ui => M::R32Uint,
//             R32f => M::R32Float,

//             CompressedRg | Rg | Rg8 => M::RG8Unorm,
//             Rg8Snorm => M::RG8Snorm,
//             Rg8i => M::RG8Sint,
//             Rg8ui => M::RG8Uint,

//             Rg16 => M::RG16Unorm,
//             Rg16Snorm => M::RG16Snorm,
//             Rg16ui => M::RG16Uint,
//             Rg16i => M::RG16Sint,
//             Rg16f => M::RG16Float,

//             Rg32f => M::RG32Float,
//             Rg32i => M::RG32Sint,
//             Rg32ui => M::RG32Uint,

//             Rgb10A2ui => M::RGB10A2Uint,

//             // would require emulating stencil test for only this stencil type
//             StencilIndex16 => todo!(),
//             // oddball formats that metal (rightly?) dropped
//             Rgb565 => todo!(),
//             R3G3B2 => todo!(),
//             Rgba2 => todo!(),

//             // unsupported ETC2 compression modes
//             CompressedRgb8PunchthroughAlpha1Etc2 | CompressedSrgb8PunchthroughAlpha1Etc2 => {
//                 panic!("OxideGL does not support ETC2 punchthrough format variations")
//             }
//         }
//     }
//     pub(crate) fn is_gl_copyable(self) -> bool {
//         use InternalFormat::*;
//         !matches!(
//             self,
//             CompressedRgb8Etc2
//                 | CompressedSrgb8Etc2
//                 | CompressedRgb8PunchthroughAlpha1Etc2
//                 | CompressedSrgb8PunchthroughAlpha1Etc2
//                 | CompressedRgba8Etc2Eac
//                 | CompressedSrgb8Alpha8Etc2Eac
//                 | CompressedR11Eac
//                 | CompressedSignedR11Eac
//                 | CompressedRg11Eac
//                 | CompressedSignedRg11Eac
//         )
//     }
//     pub(crate) fn view_class(self) -> Option<TextureViewClass> {
//         // Note: Metal additionally requires that the bit length of a pixel format is one of: 8, 16, 32, 64, or 128
//         use InternalFormat::*;
//         match self {
//             Rgba32f | Rgba32ui | Rgba32i => Some(TextureViewClass::Bits128),
//             Rgb32f | Rgb32ui | Rgb32i => Some(TextureViewClass::Bits96),
//             Rgba16f | Rgba16ui | Rgba16i | Rgba16Snorm | Rgba16 | Rg32f | Rg32i | Rg32ui => {
//                 Some(TextureViewClass::Bits64)
//             }
//             Rgb16 | Rgb16Snorm | Rgb16f | Rgb16ui | Rgb16i => Some(TextureViewClass::Bits48),
//             Rg16f | R11fG11fB10f | R32f | Rgb10A2ui | Rgba8ui | Rg16ui | R32ui | Rgba8i | Rg16i
//             | R32i | Rgb10A2 | Rgba8 | Rg16 | Rgba8Snorm | Srgb8Alpha8 | Rgb9E5 => {
//                 Some(TextureViewClass::Bits32)
//             }
//             Rgb8 | Rgb8Snorm | Srgb8 | Rgb8i | Rgb8ui => Some(TextureViewClass::Bits24),
//             R16f | Rg8ui | R16ui | Rg8i | R16i | Rg8 | R16 | Rg8Snorm | R16Snorm => {
//                 Some(TextureViewClass::Bits16)
//             }
//             R8ui | R8i | R8 | R8Snorm => Some(TextureViewClass::Bits8),
//             CompressedRedRgtc1 | CompressedSignedRedRgtc1 => Some(TextureViewClass::RgtcRed),
//             CompressedRgRgtc2 | CompressedSignedRgRgtc2 => Some(TextureViewClass::RgtcRg),
//             CompressedRgbaBptcUnorm | CompressedSrgbAlphaBptcUnorm => {
//                 Some(TextureViewClass::BptcUnorm)
//             }
//             CompressedRgbBptcSignedFloat | CompressedRgbBptcUnsignedFloat => {
//                 Some(TextureViewClass::BptcFloat)
//             }
//             _ => None,
//         }
//     }
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextureViewClass {
    Bits128,
    Bits96,
    Bits64,
    Bits48,
    Bits32,
    Bits24,
    Bits16,
    Bits8,
    RgtcRed,
    RgtcRg,
    BptcUnorm,
    BptcFloat,
}
