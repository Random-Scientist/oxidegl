use std::{fmt::Debug, num::NonZeroU32};

use objc2::rc::Retained;
use objc2_foundation::NSRange;
use objc2_metal::{
    MTLDevice, MTLOrigin, MTLRegion, MTLResourceOptions, MTLSamplerAddressMode,
    MTLSamplerBorderColor, MTLSamplerDescriptor, MTLSamplerMinMagFilter, MTLSamplerMipFilter,
    MTLSize, MTLTexture, MTLTextureDescriptor, MTLTextureSwizzle, MTLTextureSwizzleChannels,
    MTLTextureType,
};

use crate::{
    conversions::{GLenumExt, SrcType},
    error::{GlError, error},
    gl_enums::{
        DepthFunction, GL_COMPARE_REF_TO_TEXTURE, GL_NONE, InternalFormat, SamplerParameter,
        TextureMagFilter, TextureMinFilter, TextureParameterName, TextureSwizzle, TextureTarget,
        TextureWrapMode,
    },
    gl_object::{NamedObject, NoLateInit},
    gl_types::GLenum,
    util::{CloneOptionCell, ProtoObjRef, ToU32, run_if_changed},
};

use super::{
    debug::gl_err,
    error::{GlFallible, gl_assert},
    gl_object::ObjectName,
};
#[derive(Debug, Default)]
pub struct TextureBindings {
    buffer: Option<ObjectName<Texture>>,
    one_d: Option<ObjectName<Texture>>,
    two_d: Option<ObjectName<Texture>>,
    three_d: Option<ObjectName<Texture>>,
    one_d_array: Option<ObjectName<Texture>>,
    two_d_array: Option<ObjectName<Texture>>,
    cube_map: Option<ObjectName<Texture>>,
    cube_map_array: Option<ObjectName<Texture>>,
    renderbuffer: Option<ObjectName<Texture>>,
    // more etc
}

/// * named: name is reserved, object is considered uninitialized
/// * bound: object is initialized to default state, has no storage
/// * complete: TODO
#[derive(Debug)]
pub struct Texture {
    name: ObjectName<Self>,
    swizzles: [TextureSwizzle; 4],
    target: TextureTarget,
    sampling_state: SamplerParams,
    base_level: u32,
    max_level: u32,

    realized: Option<RealizedTexture>,
}
impl NamedObject for Texture {
    type LateInitType = NoLateInit<Self>;
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct TextureDimensions {
    pub(crate) width: Option<NonZeroU32>,
    pub(crate) height: Option<NonZeroU32>,
    pub(crate) depth: Option<NonZeroU32>,
}

impl TextureDimensions {
    fn mtl_region_full(self) -> MTLRegion {
        MTLRegion {
            origin: MTLOrigin { x: 0, y: 0, z: 0 },
            size: MTLSize {
                width: self.width.to_u32() as usize,
                height: self.height.to_u32() as usize,
                depth: self.depth.to_u32() as usize,
            },
        }
    }
}
/// Represents a realized texture's storage
#[derive(Debug)]
pub struct RealizedTexture {
    root_tex: ProtoObjRef<dyn MTLTexture>,
    format: InternalFormat,
    is_array: bool,
    dims: TextureDimensions,
}
impl RealizedTexture {
    fn texture_view_for_swizzle(&self, swiz: [TextureSwizzle; 4]) -> ProtoObjRef<dyn MTLTexture> {
        unsafe {
            self.root_tex
                .newTextureViewWithPixelFormat_textureType_levels_slices_swizzle(
                    self.format.mtl_tex_format(),
                    self.root_tex.textureType(),
                    NSRange {
                        location: 0,
                        length: self.root_tex.mipmapLevelCount(),
                    },
                    NSRange {
                        location: 0,
                        length: self.root_tex.arrayLength(),
                    },
                    MTLTextureSwizzleChannels {
                        red: swiz[0].into(),
                        green: swiz[1].into(),
                        blue: swiz[2].into(),
                        alpha: swiz[3].into(),
                    },
                )
        }
        .expect("failed to create swizzled view for texture")
    }
}
// #[derive(Debug)]
// pub struct TextureLevel {}

// #[derive(Debug)]
// enum Faces {
//     /// normal texture
//     One([Vec<TextureLevel>; 1]),
//     /// cubemap texture
//     Six([Vec<TextureLevel>; 6]),
// }
// impl Faces {
//     fn as_slice(&self) -> &[Vec<TextureLevel>] {
//         match self {
//             Faces::One(r) => r,
//             Faces::Six(r) => r,
//         }
//     }
//     fn as_slice_mut(&mut self) -> &mut [Vec<TextureLevel>] {
//         match self {
//             Faces::One(r) => r,
//             Faces::Six(r) => r,
//         }
//     }
// }

impl Texture {
    pub(crate) fn new_named(name: ObjectName<Self>, target: TextureTarget) -> Self {
        Self {
            name,
            target,
            sampling_state: SamplerParams::default(),
            realized: None,
            swizzles: [
                TextureSwizzle::Red,
                TextureSwizzle::Green,
                TextureSwizzle::Blue,
                TextureSwizzle::Alpha,
            ],
            base_level: 0,
            max_level: 1000,
        }
    }
    #[allow(clippy::undocumented_unsafe_blocks)]
    pub(crate) fn make_immutable_storage(
        &mut self,
        device: &ProtoObjRef<dyn MTLDevice>,
        dims: TextureDimensions,
        fmt: InternalFormat,
        levels: u32,
    ) {
        let ty = self.target.into();
        let desc = unsafe { MTLTextureDescriptor::new() };
        desc.setTextureType(ty);
        let is_array = matches!(
            ty,
            MTLTextureType::Type1DArray
                | MTLTextureType::Type2DArray
                | MTLTextureType::Type2DMultisampleArray
                | MTLTextureType::TypeCubeArray
        );
        if is_array {
            // texture array
            unsafe { desc.setArrayLength(dims.depth.to_u32() as usize) };
        }
        unsafe { desc.setMipmapLevelCount(levels as usize) };
        desc.setPixelFormat(fmt.mtl_tex_format());
        unsafe { desc.setWidth(dims.width.to_u32() as usize) };
        unsafe { desc.setHeight(dims.height.to_u32() as usize) };
        unsafe { desc.setDepth(dims.depth.to_u32() as usize) };

        desc.setResourceOptions(MTLResourceOptions::StorageModeShared);
        self.realized = Some(RealizedTexture {
            root_tex: device
                .newTextureWithDescriptor(&desc)
                .expect("failed to create backing MTLTexture"),
            format: fmt,
            is_array,
            dims,
        });
    }
    pub(crate) unsafe fn tex_param_i<
        T: ToBorderColor + SrcType<f32> + SrcType<i32> + SrcType<u32>,
    >(
        &mut self,
        pname: TextureParameterName,
        params: *const T,
    ) -> GlFallible {
        if let TextureParameterName::TextureSwizzleRgba = pname {
            // Safety: caller
            let a = unsafe { params.cast::<[T; 4]>().read() };
            for (idx, val) in a.into_iter().enumerate() {
                self.swizzles[idx] = val.try_into_enum()?;
            }
        }
        // Safety: caller
        else if let first = unsafe { *params }
            && !self.tex_param_base(pname, first)?
        {
            // Safety: caller
            unsafe {
                self.sampling_state.sampler_param_i(
                    SamplerParameter::from_repr(pname as GLenum)
                        .expect("non SamplerParameter subset should have been handled"),
                    params,
                )?;
            }
        }
        Ok(())
    }
    fn tex_param_base(
        &mut self,
        pname: TextureParameterName,
        param: impl ToBorderColor + SrcType<f32> + SrcType<i32> + SrcType<u32>,
    ) -> GlFallible<bool> {
        match pname {
            TextureParameterName::TextureRedSize
            | TextureParameterName::TextureGreenSize
            | TextureParameterName::TextureBlueSize
            | TextureParameterName::TextureAlphaSize
            | TextureParameterName::TextureWidth
            | TextureParameterName::TextureHeight
            | TextureParameterName::TextureInternalFormat => {
                error!(
                    InvalidEnum,
                    "tried to set a read-only texture property ({pname:?})"
                )
            }

            TextureParameterName::TextureBaseLevel => self.base_level = param.convert(),
            TextureParameterName::TextureMaxLevel => self.max_level = param.convert(),
            TextureParameterName::TextureSwizzleR => self.swizzles[0] = param.try_into_enum()?,
            TextureParameterName::TextureSwizzleG => self.swizzles[1] = param.try_into_enum()?,
            TextureParameterName::TextureSwizzleB => self.swizzles[2] = param.try_into_enum()?,
            TextureParameterName::TextureSwizzleA => self.swizzles[3] = param.try_into_enum()?,
            TextureParameterName::TextureSwizzleRgba => self.swizzles = [param.try_into_enum()?; _],
            TextureParameterName::DepthStencilTextureMode => todo!(),
            v => {
                return Ok(false);
            }
        }
        Ok(true)
    }
    pub(crate) fn tex_param(
        &mut self,
        pname: TextureParameterName,
        param: impl ToBorderColor + SrcType<f32> + SrcType<i32> + SrcType<u32>,
    ) -> GlFallible {
        if !self.tex_param_base(pname, param)? {
            self.sampling_state.sampler_param(
                SamplerParameter::from_repr(pname as GLenum).expect("should contain subset"),
                param,
            )?;
        }
        Ok(())
    }
}
impl TextureTarget {
    fn is_proxy(self) -> bool {
        use TextureTarget as TT;
        matches!(
            self,
            TT::ProxyTexture1D
                | TT::ProxyTexture1DArray
                | TT::ProxyTexture2D
                | TT::ProxyTexture2DArray
                | TT::ProxyTexture2DMultisample
                | TT::ProxyTexture2DMultisampleArray
                | TT::ProxyTexture3D
                | TT::ProxyTextureCubeMap
                | TT::ProxyTextureCubeMapArray
                | TT::ProxyTextureRectangle
        )
    }
}
impl From<TextureTarget> for MTLTextureType {
    fn from(value: TextureTarget) -> Self {
        match value {
            TextureTarget::Texture1D | TextureTarget::ProxyTexture1D => Self::Type1D,
            TextureTarget::Texture1DArray | TextureTarget::ProxyTexture1DArray => Self::Type1DArray,
            TextureTarget::Renderbuffer
            | TextureTarget::TextureRectangle
            | TextureTarget::ProxyTextureRectangle
            | TextureTarget::Texture2D
            | TextureTarget::ProxyTexture2D => Self::Type2D,

            TextureTarget::Texture2DArray | TextureTarget::ProxyTexture2DArray => Self::Type2DArray,
            TextureTarget::Texture2DMultisample | TextureTarget::ProxyTexture2DMultisample => {
                Self::Type2DMultisample
            }
            TextureTarget::Texture2DMultisampleArray
            | TextureTarget::ProxyTexture2DMultisampleArray => Self::Type2DMultisampleArray,
            TextureTarget::TextureCubeMap | TextureTarget::ProxyTextureCubeMap => Self::TypeCube,
            TextureTarget::TextureCubeMapArray | TextureTarget::ProxyTextureCubeMapArray => {
                Self::TypeCubeArray
            }

            TextureTarget::TextureBuffer => Self::TypeTextureBuffer,
            TextureTarget::Texture3D | TextureTarget::ProxyTexture3D => Self::Type3D,
            _ => {
                panic!("invalid texture target")
            }
        }
    }
}

#[derive(Debug)]
pub struct Sampler {
    name: ObjectName<Self>,
    params: SamplerParams,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Anisotropy {
    NoAnisotropic,
    // INVARIANT field lies within [2, 16]
    Samples(u8),
}

impl Anisotropy {
    fn from_float(val: f32) -> GlFallible<Self> {
        // Next floating point value from 2.0
        const VAL: f32 = 2.0_f32.next_up();
        match val {
            1.0..2.0 => Ok(Self::NoAnisotropic),
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "cast will not truncate due to clamp"
            )]
            VAL.. => Ok(Self::Samples(val.floor().clamp(2.0, 16.0) as u8)),
            _ => Err(GlError::InvalidValue.e()),
        }
    }
}
#[derive(Debug, Clone)]
/// Contains the sampling parameters for a texture
/// Note: must call [`SamplerParams::mark_dirty`] after modifying values in this struct
pub struct SamplerParams {
    /// Border color for border wrap mode
    pub(crate) border_color: MTLSamplerBorderColor,
    /// Depth comparison mode if depth comparison is enabled
    pub(crate) compare_ref_to_texture: bool,
    pub(crate) compare_func: DepthFunction,
    /// Magnification filter
    pub(crate) mag_filter: TextureMagFilter,
    /// Minification filter and mipmap filter for mipmapped sampling
    pub(crate) min_filter: TextureMinFilter,
    /// Constant that is added to the shader-supplied LOD before clamping to [`min_lod`, `max_lod`] and sampling
    pub(crate) lod_bias: f32,
    pub(crate) max_lod: f32,
    pub(crate) min_lod: f32,
    pub(crate) max_anisotropy: Anisotropy,
    pub(crate) wrap_mode_s: TextureWrapMode,
    pub(crate) wrap_mode_t: TextureWrapMode,
    pub(crate) wrap_mode_r: TextureWrapMode,
    descriptor_cache: CloneOptionCell<Retained<MTLSamplerDescriptor>>,
}

pub(crate) trait ToBorderColor: Into<f64> {
    fn to_float(self) -> f64 {
        self.into()
    }
}
impl ToBorderColor for f64 {}
impl ToBorderColor for f32 {}

//TODO check if these two impls are actually compliant to the spec
impl ToBorderColor for i32 {}
impl ToBorderColor for u32 {}

impl SamplerParams {
    unsafe fn sampler_param_i<T: ToBorderColor + SrcType<f32> + SrcType<i32> + SrcType<u32>>(
        &mut self,
        pname: SamplerParameter,
        params: *const T,
    ) -> GlFallible {
        gl_assert!(
            params.is_aligned() && !params.is_null(),
            InvalidValue,
            "bad params pointer in sampler params call"
        );
        match pname {
            SamplerParameter::TextureBorderColor => {
                self.set_border_color(
                    // Safety: if `pname` is `TextureBorderColor` caller ensures this pointer points to an array of 4 4-byte values of either integer or floating point type
                    unsafe { *params.cast::<[T; 4]>() }.map(ToBorderColor::to_float),
                );
                Ok(())
            }
            // Safety: if `pname` is not `TextureBorderColor` caller ensures this pointer points to a single value of the correct type
            pname => self.sampler_param(pname, unsafe { *params }),
        }
    }
    fn sampler_param(
        &mut self,
        pname: SamplerParameter,
        param: impl SrcType<f32> + SrcType<i32> + SrcType<u32>,
    ) -> GlFallible {
        let dirty = || self.descriptor_cache.set(None);
        match pname {
            SamplerParameter::TextureMagFilter => {
                run_if_changed!(self.mag_filter;= param.try_into_enum()? => dirty());
            }
            SamplerParameter::TextureMinFilter => {
                run_if_changed!(self.min_filter;= param.try_into_enum()? => dirty());
            }
            SamplerParameter::TextureWrapS => {
                run_if_changed!(self.wrap_mode_s;= param.try_into_enum()? => dirty());
                self.wrap_mode_s = param.try_into_enum()?;
            }
            SamplerParameter::TextureWrapT => {
                run_if_changed!(self.wrap_mode_t;= param.try_into_enum()? => dirty());
                self.wrap_mode_t = param.try_into_enum()?;
            }
            SamplerParameter::TextureWrapR => {
                run_if_changed!(self.wrap_mode_r;= param.try_into_enum()? => dirty());
                self.wrap_mode_r = param.try_into_enum()?;
            }
            SamplerParameter::TextureMinLod => {
                run_if_changed!(self.min_lod;= param.convert() => dirty());
            }
            SamplerParameter::TextureMaxLod => {
                run_if_changed!(self.max_lod;= param.convert() => dirty());
            }
            SamplerParameter::TextureLodBias => {
                run_if_changed!(self.lod_bias;= param.convert() => dirty());
            }
            SamplerParameter::TextureCompareMode => {
                run_if_changed!(self.compare_ref_to_texture;= match param.convert() {
                    GL_NONE => false,
                    GL_COMPARE_REF_TO_TEXTURE => true,
                    _ => error!(
                        InvalidEnum,
                        "TextureCompareMode should have been either NONE or COMPARE_REF_TO_TEXTURE"
                    ),
                } => dirty());
            }
            SamplerParameter::TextureCompareFunc => {
                run_if_changed!(self.compare_func;= param.try_into_enum()? => dirty());
            }
            SamplerParameter::TextureMaxAnisotropy => {
                run_if_changed!(self.max_anisotropy;= Anisotropy::from_float(param.convert())? => dirty());
            }
            SamplerParameter::TextureBorderColor => error!(
                InvalidEnum,
                "TextureBorderColor can only be retrieved via samplerparami"
            ),
        }
        self.descriptor_cache.set(None);
        Ok(())
    }
    fn set_border_color(&mut self, vals: [f64; 4]) {
        self.border_color = match vals {
            [0.0, 0.0, 0.0, v] => match v {
                1.0 => MTLSamplerBorderColor::OpaqueBlack,
                _ => MTLSamplerBorderColor::TransparentBlack,
            },
            [1.0, 1.0, 1.0, 1.0] => MTLSamplerBorderColor::OpaqueWhite,
            _ => {
                gl_err!(ty: Error, "unsupported texture border color used, defaulting to opaque white");
                MTLSamplerBorderColor::OpaqueWhite
            }
        };
    }
    fn sampler_desc(&self) -> Retained<MTLSamplerDescriptor> {
        if let Some(d) = self.descriptor_cache.clone_out() {
            return d;
        }
        let desc = MTLSamplerDescriptor::new();
        if [self.wrap_mode_r, self.wrap_mode_s, self.wrap_mode_t]
            .contains(&TextureWrapMode::ClampToBorder)
        {
            desc.setBorderColor(self.border_color);
        }
        if self.compare_ref_to_texture {
            desc.setCompareFunction(self.compare_func.into());
        }
        desc.setMagFilter(self.mag_filter.into());
        if let Anisotropy::Samples(n) = self.max_anisotropy {
            desc.setMaxAnisotropy(n as usize);
        }
        let (minification, mip) = self.min_filter.into();
        desc.setMinFilter(minification);
        desc.setMipFilter(mip);
        desc.setSAddressMode(self.wrap_mode_s.into());
        desc.setTAddressMode(self.wrap_mode_t.into());
        desc.setRAddressMode(self.wrap_mode_r.into());
        self.descriptor_cache.set(Some(desc.clone()));
        desc
    }
    fn mark_dirty(&self) {
        self.descriptor_cache.set(None);
    }
}
impl Default for SamplerParams {
    fn default() -> Self {
        Self {
            border_color: MTLSamplerBorderColor::TransparentBlack,
            mag_filter: TextureMagFilter::Linear,
            min_filter: TextureMinFilter::NearestMipmapLinear,
            lod_bias: 0.0,
            max_lod: 1000.0,
            min_lod: -1000.0,
            max_anisotropy: Anisotropy::NoAnisotropic,
            wrap_mode_s: TextureWrapMode::Repeat,
            wrap_mode_t: TextureWrapMode::Repeat,
            wrap_mode_r: TextureWrapMode::Repeat,
            descriptor_cache: CloneOptionCell::new(None),
            compare_ref_to_texture: false,
            compare_func: DepthFunction::Lequal,
        }
    }
}
impl From<TextureSwizzle> for MTLTextureSwizzle {
    fn from(value: TextureSwizzle) -> Self {
        match value {
            TextureSwizzle::Zero => MTLTextureSwizzle::Zero,
            TextureSwizzle::One => MTLTextureSwizzle::One,
            TextureSwizzle::Red => MTLTextureSwizzle::Red,
            TextureSwizzle::Green => MTLTextureSwizzle::Green,
            TextureSwizzle::Blue => MTLTextureSwizzle::Blue,
            TextureSwizzle::Alpha => MTLTextureSwizzle::Alpha,
        }
    }
}
impl From<TextureWrapMode> for MTLSamplerAddressMode {
    fn from(value: TextureWrapMode) -> Self {
        match value {
            TextureWrapMode::Repeat => MTLSamplerAddressMode::Repeat,
            TextureWrapMode::MirroredRepeat => MTLSamplerAddressMode::MirrorRepeat,
            TextureWrapMode::ClampToEdge => MTLSamplerAddressMode::ClampToEdge,
            TextureWrapMode::ClampToBorder => MTLSamplerAddressMode::ClampToBorderColor,
        }
    }
}
impl From<TextureMagFilter> for MTLSamplerMinMagFilter {
    fn from(value: TextureMagFilter) -> Self {
        match value {
            TextureMagFilter::Nearest => Self::Nearest,
            TextureMagFilter::Linear => Self::Linear,
        }
    }
}
impl From<TextureMinFilter> for (MTLSamplerMinMagFilter, MTLSamplerMipFilter) {
    fn from(value: TextureMinFilter) -> Self {
        use objc2_metal::{MTLSamplerMinMagFilter as MinMag, MTLSamplerMipFilter as Mip};
        match value {
            TextureMinFilter::Nearest => (MinMag::Nearest, Mip::NotMipmapped),
            TextureMinFilter::Linear => (MinMag::Linear, Mip::NotMipmapped),
            TextureMinFilter::NearestMipmapNearest => (MinMag::Nearest, Mip::Nearest),
            TextureMinFilter::LinearMipmapNearest => (MinMag::Linear, Mip::Nearest),
            TextureMinFilter::NearestMipmapLinear => (MinMag::Nearest, Mip::Linear),
            TextureMinFilter::LinearMipmapLinear => (MinMag::Linear, Mip::Linear),
        }
    }
}
