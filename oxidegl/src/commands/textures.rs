use std::num::NonZeroU32;

use crate::{
    context::Context,
    conversions::sizei,
    error::{GlFallible, error},
    gl_enums::{
        GetTextureParameter, InternalFormat, PixelFormat, PixelType, RenderbufferParameterName,
        SamplerParameter, SizedInternalFormat, TextureParameterName, TextureTarget,
    },
    gl_types::{GLboolean, GLenum, GLfloat, GLint, GLintptr, GLsizei, GLsizeiptr, GLuint, GLvoid},
    texture::{Texture, TextureDimensions},
};
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the renderbuffer object is bound for [**glGetRenderbufferParameteriv**](crate::context::Context::oxidegl_get_renderbuffer_parameteriv).
/// > `target` must be [`GL_RENDERBUFFER`](crate::gl_enums::GL_RENDERBUFFER).
///
/// `renderbuffer`
///
/// > Specifies the name of the renderbuffer object for [**glGetNamedRenderbufferParameteriv**](crate::context::Context::oxidegl_get_named_renderbuffer_parameteriv).
///
/// `pname`
///
/// > Specifies the parameter of the renderbuffer object to query.
///
/// `params`
///
/// > Returns the value of parameter `pname` for the renderbuffer object.
///
/// ### Description
/// [**glGetRenderbufferParameteriv**](crate::context::Context::oxidegl_get_renderbuffer_parameteriv)
/// and [**glGetNamedRenderbufferParameteriv**](crate::context::Context::oxidegl_get_named_renderbuffer_parameteriv)
/// query parameters of a specified renderbuffer object.
///
/// For [**glGetRenderbufferParameteriv**](crate::context::Context::oxidegl_get_renderbuffer_parameteriv),
/// the renderbuffer object is that bound to `target`, which must be [`GL_RENDERBUFFER`](crate::gl_enums::GL_RENDERBUFFER).
///
/// For [**glGetNamedRenderbufferParameteriv**](crate::context::Context::oxidegl_get_named_renderbuffer_parameteriv),
/// `renderbuffer` is the name of the renderbuffer object.
///
/// Upon successful return, `param` will contain the value of the renderbuffer
/// parameter specified by `pname`, as described below.
///
/// [`GL_RENDERBUFFER_WIDTH`](crate::gl_enums::GL_RENDERBUFFER_WIDTH), [`GL_RENDERBUFFER_HEIGHT`](crate::gl_enums::GL_RENDERBUFFER_HEIGHT),
/// [`GL_RENDERBUFFER_INTERNAL_FORMAT`](crate::gl_enums::GL_RENDERBUFFER_INTERNAL_FORMAT)
/// or [`GL_RENDERBUFFER_SAMPLES`](crate::gl_enums::GL_RENDERBUFFER_SAMPLES)
///
/// > `params` returns the width in pixels, the height in pixels, internal format,
/// > or the number of samples, respectively, of the image of the specified renderbuffer
/// > object.
///
/// [`GL_RENDERBUFFER_RED_SIZE`](crate::gl_enums::GL_RENDERBUFFER_RED_SIZE),
/// [`GL_RENDERBUFFER_GREEN_SIZE`](crate::gl_enums::GL_RENDERBUFFER_GREEN_SIZE),
/// [`GL_RENDERBUFFER_BLUE_SIZE`](crate::gl_enums::GL_RENDERBUFFER_BLUE_SIZE),
/// [`GL_RENDERBUFFER_ALPHA_SIZE`](crate::gl_enums::GL_RENDERBUFFER_ALPHA_SIZE),
/// [`GL_RENDERBUFFER_DEPTH_SIZE`](crate::gl_enums::GL_RENDERBUFFER_DEPTH_SIZE)
/// or [`GL_RENDERBUFFER_STENCIL_SIZE`](crate::gl_enums::GL_RENDERBUFFER_STENCIL_SIZE)
///
/// > `params` returns the actual resolution in bits (not the resolution specified
/// > when the image was defined) for the red, green, blue, alpha, depth or stencil
/// > components, respectively, of the image of the renderbuffer object.
impl Context {
    pub unsafe fn oxidegl_get_renderbuffer_parameteriv(
        &mut self,
        target: GLenum,
        pname: RenderbufferParameterName,
        params: *mut GLint,
    ) -> GlFallible {
        panic!("command oxidegl_get_renderbuffer_parameteriv not yet implemented");
    }
    pub unsafe fn oxidegl_get_named_renderbuffer_parameteriv(
        &mut self,
        renderbuffer: GLuint,
        pname: RenderbufferParameterName,
        params: *mut GLint,
    ) -> GlFallible {
        panic!("command oxidegl_get_named_renderbuffer_parameteriv not yet implemented");
    }
}
/// ### Parameters
/// `sampler`
///
/// > Specifies name of the sampler object from which to retrieve parameters.
///
/// `pname`
///
/// > Specifies the symbolic name of a sampler parameter. [`GL_TEXTURE_MAG_FILTER`](crate::gl_enums::GL_TEXTURE_MAG_FILTER),
/// > [`GL_TEXTURE_MIN_FILTER`](crate::gl_enums::GL_TEXTURE_MIN_FILTER), [`GL_TEXTURE_MIN_LOD`](crate::gl_enums::GL_TEXTURE_MIN_LOD),
/// > [`GL_TEXTURE_MAX_LOD`](crate::gl_enums::GL_TEXTURE_MAX_LOD), [`GL_TEXTURE_LOD_BIAS`](crate::gl_enums::GL_TEXTURE_LOD_BIAS),
/// > [`GL_TEXTURE_WRAP_S`](crate::gl_enums::GL_TEXTURE_WRAP_S), [`GL_TEXTURE_WRAP_T`](crate::gl_enums::GL_TEXTURE_WRAP_T),
/// > [`GL_TEXTURE_WRAP_R`](crate::gl_enums::GL_TEXTURE_WRAP_R), [`GL_TEXTURE_BORDER_COLOR`](crate::gl_enums::GL_TEXTURE_BORDER_COLOR),
/// > [`GL_TEXTURE_COMPARE_MODE`](crate::gl_enums::GL_TEXTURE_COMPARE_MODE), and
/// > [`GL_TEXTURE_COMPARE_FUNC`](crate::gl_enums::GL_TEXTURE_COMPARE_FUNC) are
/// > accepted.
///
/// `params`
///
/// > Returns the sampler parameters.
///
/// ### Description
/// [**glGetSamplerParameter**](crate::context::Context::oxidegl_get_sampler_parameter)
/// returns in `params` the value or values of the sampler parameter specified
/// as `pname`. `sampler` defines the target sampler, and must be the name
/// of an existing sampler object, returned from a previous call to [**glGenSamplers**](crate::context::Context::oxidegl_gen_samplers).
/// `pname` accepts the same symbols as [**glSamplerParameter**](crate::context::Context::oxidegl_sampler_parameter),
/// with the same interpretations:
///
/// [`GL_TEXTURE_MAG_FILTER`](crate::gl_enums::GL_TEXTURE_MAG_FILTER)
///
/// > Returns the single-valued texture magnification filter, a symbolic constant.
/// > The initial value is [`GL_LINEAR`](crate::gl_enums::GL_LINEAR).
///
/// [`GL_TEXTURE_MIN_FILTER`](crate::gl_enums::GL_TEXTURE_MIN_FILTER)
///
/// > Returns the single-valued texture minification filter, a symbolic constant.
/// > The initial value is [`GL_NEAREST_MIPMAP_LINEAR`](crate::gl_enums::GL_NEAREST_MIPMAP_LINEAR).
///
/// [`GL_TEXTURE_MIN_LOD`](crate::gl_enums::GL_TEXTURE_MIN_LOD)
///
/// > Returns the single-valued texture minimum level-of-detail value. The initial
/// > value is `[inlineq]`
///
/// [`GL_TEXTURE_MAX_LOD`](crate::gl_enums::GL_TEXTURE_MAX_LOD)
///
/// > Returns the single-valued texture maximum level-of-detail value. The initial
/// > value is 1000.
///
/// [`GL_TEXTURE_WRAP_S`](crate::gl_enums::GL_TEXTURE_WRAP_S)
///
/// > Returns the single-valued wrapping function for texture coordinate `[inlineq]`
/// > [`GL_REPEAT`](crate::gl_enums::GL_REPEAT).
///
/// [`GL_TEXTURE_WRAP_T`](crate::gl_enums::GL_TEXTURE_WRAP_T)
///
/// > Returns the single-valued wrapping function for texture coordinate `[inlineq]`
/// > [`GL_REPEAT`](crate::gl_enums::GL_REPEAT).
///
/// [`GL_TEXTURE_WRAP_R`](crate::gl_enums::GL_TEXTURE_WRAP_R)
///
/// > Returns the single-valued wrapping function for texture coordinate `[inlineq]`
/// > [`GL_REPEAT`](crate::gl_enums::GL_REPEAT).
///
/// [`GL_TEXTURE_BORDER_COLOR`](crate::gl_enums::GL_TEXTURE_BORDER_COLOR)
///
/// > Returns four integer or floating-point numbers that comprise the RGBA color
/// > of the texture border. Floating-point values are returned in the range
/// > `[inlineq]` `[inlineq]`
///
/// [`GL_TEXTURE_COMPARE_MODE`](crate::gl_enums::GL_TEXTURE_COMPARE_MODE)
///
/// > Returns a single-valued texture comparison mode, a symbolic constant. The
/// > initial value is [`GL_NONE`](crate::gl_enums::GL_NONE). See [**glSamplerParameter**](crate::context::Context::oxidegl_sampler_parameter).
///
/// [`GL_TEXTURE_COMPARE_FUNC`](crate::gl_enums::GL_TEXTURE_COMPARE_FUNC)
///
/// > Returns a single-valued texture comparison function, a symbolic constant.
/// > The initial value is [`GL_LEQUAL`](crate::gl_enums::GL_LEQUAL). See [**glSamplerParameter**](crate::context::Context::oxidegl_sampler_parameter).
///
/// ### Notes
/// If an error is generated, no change is made to the contents of `params`.
///
/// [**glGetSamplerParameter**](crate::context::Context::oxidegl_get_sampler_parameter)
/// is available only if the GL version is 3.3 or higher.
impl Context {
    pub unsafe fn oxidegl_get_sampler_parameteriv(
        &mut self,
        sampler: GLuint,
        pname: SamplerParameter,
        params: *mut GLint,
    ) -> GlFallible {
        panic!("command oxidegl_get_sampler_parameteriv not yet implemented");
    }
    pub unsafe fn oxidegl_get_sampler_parameter_iiv(
        &mut self,
        sampler: GLuint,
        pname: SamplerParameter,
        params: *mut GLint,
    ) -> GlFallible {
        panic!("command oxidegl_get_sampler_parameter_iiv not yet implemented");
    }
    pub unsafe fn oxidegl_get_sampler_parameterfv(
        &mut self,
        sampler: GLuint,
        pname: SamplerParameter,
        params: *mut GLfloat,
    ) -> GlFallible {
        panic!("command oxidegl_get_sampler_parameterfv not yet implemented");
    }
    pub unsafe fn oxidegl_get_sampler_parameter_iuiv(
        &mut self,
        sampler: GLuint,
        pname: SamplerParameter,
        params: *mut GLuint,
    ) -> GlFallible {
        panic!("command oxidegl_get_sampler_parameter_iuiv not yet implemented");
    }
}
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the texture is bound for [**glGetTexImage**](crate::context::Context::oxidegl_get_tex_image)
/// > and [**glGetnTexImage**](crate::context::Context::oxidegl_getn_tex_image)
/// > functions. [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D), [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D),
/// > [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D), [`GL_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_1D_ARRAY),
/// > [`GL_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_ARRAY), [`GL_TEXTURE_RECTANGLE`](crate::gl_enums::GL_TEXTURE_RECTANGLE),
/// > [`GL_TEXTURE_CUBE_MAP_POSITIVE_X`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_POSITIVE_X),
/// > [`GL_TEXTURE_CUBE_MAP_NEGATIVE_X`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_NEGATIVE_X),
/// > [`GL_TEXTURE_CUBE_MAP_POSITIVE_Y`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_POSITIVE_Y),
/// > [`GL_TEXTURE_CUBE_MAP_NEGATIVE_Y`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_NEGATIVE_Y),
/// > [`GL_TEXTURE_CUBE_MAP_POSITIVE_Z`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_POSITIVE_Z),
/// > [`GL_TEXTURE_CUBE_MAP_NEGATIVE_Z`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_NEGATIVE_Z),
/// > and [`GL_TEXTURE_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_ARRAY)
/// > are acceptable.
///
/// `texture`
///
/// > Specifies the texture object name.
///
/// `level`
///
/// > Specifies the level-of-detail number of the desired image. Level 0 is the
/// > base image level. Level `[inlineq]` `[inlineq]`
///
/// `format`
///
/// > Specifies a pixel format for the returned data. The supported formats are
/// > [`GL_STENCIL_INDEX`](crate::gl_enums::GL_STENCIL_INDEX), [`GL_DEPTH_COMPONENT`](crate::gl_enums::GL_DEPTH_COMPONENT),
/// > [`GL_DEPTH_STENCIL`](crate::gl_enums::GL_DEPTH_STENCIL), [`GL_RED`](crate::gl_enums::GL_RED),
/// > [`GL_GREEN`](crate::gl_enums::GL_GREEN), [`GL_BLUE`](crate::gl_enums::GL_BLUE),
/// > [`GL_RG`](crate::gl_enums::GL_RG), [`GL_RGB`](crate::gl_enums::GL_RGB),
/// > [`GL_RGBA`](crate::gl_enums::GL_RGBA), [`GL_BGR`](crate::gl_enums::GL_BGR),
/// > [`GL_BGRA`](crate::gl_enums::GL_BGRA), [`GL_RED_INTEGER`](crate::gl_enums::GL_RED_INTEGER),
/// > [`GL_GREEN_INTEGER`](crate::gl_enums::GL_GREEN_INTEGER), [`GL_BLUE_INTEGER`](crate::gl_enums::GL_BLUE_INTEGER),
/// > [`GL_RG_INTEGER`](crate::gl_enums::GL_RG_INTEGER), [`GL_RGB_INTEGER`](crate::gl_enums::GL_RGB_INTEGER),
/// > [`GL_RGBA_INTEGER`](crate::gl_enums::GL_RGBA_INTEGER), [`GL_BGR_INTEGER`](crate::gl_enums::GL_BGR_INTEGER),
/// > [`GL_BGRA_INTEGER`](crate::gl_enums::GL_BGRA_INTEGER).
///
/// `type`
///
/// > Specifies a pixel type for the returned data. The supported types are [`GL_UNSIGNED_BYTE`](crate::gl_enums::GL_UNSIGNED_BYTE),
/// > [`GL_BYTE`](crate::gl_enums::GL_BYTE), [`GL_UNSIGNED_SHORT`](crate::gl_enums::GL_UNSIGNED_SHORT),
/// > [`GL_SHORT`](crate::gl_enums::GL_SHORT), [`GL_UNSIGNED_INT`](crate::gl_enums::GL_UNSIGNED_INT),
/// > [`GL_INT`](crate::gl_enums::GL_INT), [`GL_HALF_FLOAT`](crate::gl_enums::GL_HALF_FLOAT),
/// > [`GL_FLOAT`](crate::gl_enums::GL_FLOAT), [`GL_UNSIGNED_BYTE_3_3_2`](crate::gl_enums::GL_UNSIGNED_BYTE_3_3_2),
/// > [`GL_UNSIGNED_BYTE_2_3_3_REV`](crate::gl_enums::GL_UNSIGNED_BYTE_2_3_3_REV),
/// > [`GL_UNSIGNED_SHORT_5_6_5`](crate::gl_enums::GL_UNSIGNED_SHORT_5_6_5), [`GL_UNSIGNED_SHORT_5_6_5_REV`](crate::gl_enums::GL_UNSIGNED_SHORT_5_6_5_REV),
/// > [`GL_UNSIGNED_SHORT_4_4_4_4`](crate::gl_enums::GL_UNSIGNED_SHORT_4_4_4_4),
/// > [`GL_UNSIGNED_SHORT_4_4_4_4_REV`](crate::gl_enums::GL_UNSIGNED_SHORT_4_4_4_4_REV),
/// > [`GL_UNSIGNED_SHORT_5_5_5_1`](crate::gl_enums::GL_UNSIGNED_SHORT_5_5_5_1),
/// > [`GL_UNSIGNED_SHORT_1_5_5_5_REV`](crate::gl_enums::GL_UNSIGNED_SHORT_1_5_5_5_REV),
/// > [`GL_UNSIGNED_INT_8_8_8_8`](crate::gl_enums::GL_UNSIGNED_INT_8_8_8_8), [`GL_UNSIGNED_INT_8_8_8_8_REV`](crate::gl_enums::GL_UNSIGNED_INT_8_8_8_8_REV),
/// > [`GL_UNSIGNED_INT_10_10_10_2`](crate::gl_enums::GL_UNSIGNED_INT_10_10_10_2),
/// > [`GL_UNSIGNED_INT_2_10_10_10_REV`](crate::gl_enums::GL_UNSIGNED_INT_2_10_10_10_REV),
/// > [`GL_UNSIGNED_INT_24_8`](crate::gl_enums::GL_UNSIGNED_INT_24_8), [`GL_UNSIGNED_INT_10F_11F_11F_REV`](crate::gl_enums::GL_UNSIGNED_INT_10F_11F_11F_REV),
/// > [`GL_UNSIGNED_INT_5_9_9_9_REV`](crate::gl_enums::GL_UNSIGNED_INT_5_9_9_9_REV),
/// > and [`GL_FLOAT_32_UNSIGNED_INT_24_8_REV`](crate::gl_enums::GL_FLOAT_32_UNSIGNED_INT_24_8_REV).
///
/// `bufSize`
///
/// > Specifies the size of the buffer `pixels` for [**glGetnTexImage**](crate::context::Context::oxidegl_getn_tex_image)
/// > and [**glGetTextureImage**](crate::context::Context::oxidegl_get_texture_image)
/// > functions.
///
/// `pixels`
///
/// > Returns the texture image. Should be a pointer to an array of the type
/// > specified by `type`.
///
/// ### Description
/// [**glGetTexImage**](crate::context::Context::oxidegl_get_tex_image), [**glGetnTexImage**](crate::context::Context::oxidegl_getn_tex_image)
/// and [**glGetTextureImage**](crate::context::Context::oxidegl_get_texture_image)
/// functions return a texture image into `pixels`. For [**glGetTexImage**](crate::context::Context::oxidegl_get_tex_image)
/// and [**glGetnTexImage**](crate::context::Context::oxidegl_getn_tex_image),
/// `target` specifies whether the desired texture image is one specified by
/// [**glTexImage1D**](crate::context::Context::oxidegl_tex_image1_d)( [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D)
/// ), [**glTexImage2D**](crate::context::Context::oxidegl_tex_image2_d)( [`GL_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_1D_ARRAY),
/// [`GL_TEXTURE_RECTANGLE`](crate::gl_enums::GL_TEXTURE_RECTANGLE), [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D)
/// or any of [`GL_TEXTURE_CUBE_MAP_*`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_*)
/// ), or [**glTexImage3D**](crate::context::Context::oxidegl_tex_image3_d)(
/// [`GL_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_ARRAY), [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D),
/// [`GL_TEXTURE_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_ARRAY)
/// ). For [**glGetTextureImage**](crate::context::Context::oxidegl_get_texture_image),
/// `texture` specifies the texture object name. In addition to types of textures
/// accepted by [**glGetTexImage**](crate::context::Context::oxidegl_get_tex_image)
/// and [**glGetnTexImage**](crate::context::Context::oxidegl_getn_tex_image),
/// the function also accepts cube map texture objects (with effective target
/// [`GL_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_CUBE_MAP) ). `level`
/// specifies the level-of-detail number of the desired image. `format` and
/// `type` specify the format and type of the desired image array. See the
/// reference page for [**glTexImage1D**](crate::context::Context::oxidegl_tex_image1_d)
/// for a description of the acceptable values for the `format` and `type`
/// parameters, respectively. For glGetnTexImage and glGetTextureImage functions,
/// bufSize tells the size of the buffer to receive the retrieved pixel data.
/// [**glGetnTexImage**](crate::context::Context::oxidegl_getn_tex_image) and
/// [**glGetTextureImage**](crate::context::Context::oxidegl_get_texture_image)
/// do not write more than `bufSize` bytes into `pixels`.
///
/// If a non-zero named buffer object is bound to the [`GL_PIXEL_PACK_BUFFER`](crate::gl_enums::GL_PIXEL_PACK_BUFFER)
/// target (see [**glBindBuffer**](crate::context::Context::oxidegl_bind_buffer))
/// while a texture image is requested, `pixels` is treated as a byte offset
/// into the buffer object's data store.
///
/// To understand the operation of [**glGetTexImage**](crate::context::Context::oxidegl_get_tex_image),
/// consider the selected internal four-component texture image to be an RGBA
/// color buffer the size of the image. The semantics of [**glGetTexImage**](crate::context::Context::oxidegl_get_tex_image)
/// are then identical to those of [**glReadPixels**](crate::context::Context::oxidegl_read_pixels),
/// with the exception that no pixel transfer operations are performed, when
/// called with the same `format` and `type`, with *x* and *y* set to 0, *width*
/// set to the width of the texture image and *height* set to 1 for 1D images,
/// or to the height of the texture image for 2D images.
///
/// If the selected texture image does not contain four components, the following
/// mappings are applied. Single-component textures are treated as RGBA buffers
/// with red set to the single-component value, green set to 0, blue set to
/// 0, and alpha set to 1. Two-component textures are treated as RGBA buffers
/// with red set to the value of component zero, alpha set to the value of
/// component one, and green and blue set to 0. Finally, three-component textures
/// are treated as RGBA buffers with red set to component zero, green set to
/// component one, blue set to component two, and alpha set to 1.
///
/// To determine the required size of `pixels`, use [**glGetTexLevelParameter**](crate::context::Context::oxidegl_get_tex_level_parameter)
/// to determine the dimensions of the internal texture image, then scale the
/// required number of pixels by the storage required for each pixel, based
/// on `format` and `type`. Be sure to take the pixel storage parameters into
/// account, especially [`GL_PACK_ALIGNMENT`](crate::gl_enums::GL_PACK_ALIGNMENT).
///
/// If [**glGetTextureImage**](crate::context::Context::oxidegl_get_texture_image)
/// is used against a cube map texture object, the texture is treated as a
/// three-dimensional image of a depth of 6, where the cube map faces are ordered
/// as image layers, in an order presented in the table below:
///
/// | Layer number        | Cube Map Face        |
/// |---------------------|----------------------|
/// | 0                   | `GL_TEXTURE_CUBE_MAP_POSITIVE_X` |
/// | 1                   | `GL_TEXTURE_CUBE_MAP_NEGATIVE_X` |
/// | 2                   | `GL_TEXTURE_CUBE_MAP_POSITIVE_Y` |
/// | 3                   | `GL_TEXTURE_CUBE_MAP_NEGATIVE_Y` |
/// | 4                   | `GL_TEXTURE_CUBE_MAP_POSITIVE_Z` |
/// | 5                   | `GL_TEXTURE_CUBE_MAP_NEGATIVE_Z` |
///
/// ### Notes
/// If an error is generated, no change is made to the contents of `pixels`.
///
/// [**glGetTexImage**](crate::context::Context::oxidegl_get_tex_image) and
/// [**glGetnTexImage**](crate::context::Context::oxidegl_getn_tex_image) return
/// the texture image for the active texture unit.
///
/// [`GL_STENCIL_INDEX`](crate::gl_enums::GL_STENCIL_INDEX) is accepted for
/// `format` only if the GL version is 4.4 or greater.
///
/// ### Associated Gets
/// [**glGetTexLevelParameter**](crate::context::Context::oxidegl_get_tex_level_parameter)
/// with argument [`GL_TEXTURE_WIDTH`](crate::gl_enums::GL_TEXTURE_WIDTH)
///
/// [**glGetTexLevelParameter**](crate::context::Context::oxidegl_get_tex_level_parameter)
/// with argument [`GL_TEXTURE_HEIGHT`](crate::gl_enums::GL_TEXTURE_HEIGHT)
///
/// [**glGetTexLevelParameter**](crate::context::Context::oxidegl_get_tex_level_parameter)
/// with argument [`GL_TEXTURE_INTERNAL_FORMAT`](crate::gl_enums::GL_TEXTURE_INTERNAL_FORMAT)
///
/// [**glGet**](crate::context::Context::oxidegl_get) with arguments [`GL_PACK_ALIGNMENT`](crate::gl_enums::GL_PACK_ALIGNMENT)
/// and others
///
/// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_PIXEL_PACK_BUFFER_BINDING`](crate::gl_enums::GL_PIXEL_PACK_BUFFER_BINDING)
impl Context {
    pub unsafe fn oxidegl_get_tex_image(
        &mut self,
        target: TextureTarget,
        level: GLint,
        format: PixelFormat,
        r#type: PixelType,
        pixels: *mut GLvoid,
    ) -> GlFallible {
        panic!("command oxidegl_get_tex_image not yet implemented");
    }
    pub unsafe fn oxidegl_get_texture_image(
        &mut self,
        texture: GLuint,
        level: GLint,
        format: PixelFormat,
        r#type: PixelType,
        buf_size: GLsizei,
        pixels: *mut GLvoid,
    ) -> GlFallible {
        panic!("command oxidegl_get_texture_image not yet implemented");
    }
    pub unsafe fn oxidegl_getn_tex_image(
        &mut self,
        target: TextureTarget,
        level: GLint,
        format: PixelFormat,
        r#type: PixelType,
        buf_size: GLsizei,
        pixels: *mut GLvoid,
    ) -> GlFallible {
        panic!("command oxidegl_getn_tex_image not yet implemented");
    }
}
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the texture is bound for [**glGetTexLevelParameterfv**](crate::context::Context::oxidegl_get_tex_level_parameterfv)
/// > and [**glGetTexLevelParameteriv**](crate::context::Context::oxidegl_get_tex_level_parameteriv)
/// > functions. Must be one of the following values: [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D),
/// > [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D), [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D),
/// > [`GL_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_1D_ARRAY), [`GL_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_ARRAY),
/// > [`GL_TEXTURE_RECTANGLE`](crate::gl_enums::GL_TEXTURE_RECTANGLE), [`GL_TEXTURE_2D_MULTISAMPLE`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE),
/// > [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY),
/// > [`GL_TEXTURE_CUBE_MAP_POSITIVE_X`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_POSITIVE_X),
/// > [`GL_TEXTURE_CUBE_MAP_NEGATIVE_X`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_NEGATIVE_X),
/// > [`GL_TEXTURE_CUBE_MAP_POSITIVE_Y`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_POSITIVE_Y),
/// > [`GL_TEXTURE_CUBE_MAP_NEGATIVE_Y`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_NEGATIVE_Y),
/// > [`GL_TEXTURE_CUBE_MAP_POSITIVE_Z`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_POSITIVE_Z),
/// > [`GL_TEXTURE_CUBE_MAP_NEGATIVE_Z`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_NEGATIVE_Z),
/// > [`GL_PROXY_TEXTURE_1D`](crate::gl_enums::GL_PROXY_TEXTURE_1D), [`GL_PROXY_TEXTURE_2D`](crate::gl_enums::GL_PROXY_TEXTURE_2D),
/// > [`GL_PROXY_TEXTURE_3D`](crate::gl_enums::GL_PROXY_TEXTURE_3D), [`GL_PROXY_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_PROXY_TEXTURE_1D_ARRAY),
/// > [`GL_PROXY_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_PROXY_TEXTURE_2D_ARRAY),
/// > [`GL_PROXY_TEXTURE_RECTANGLE`](crate::gl_enums::GL_PROXY_TEXTURE_RECTANGLE),
/// > [`GL_PROXY_TEXTURE_2D_MULTISAMPLE`](crate::gl_enums::GL_PROXY_TEXTURE_2D_MULTISAMPLE),
/// > [`GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY),
/// > [`GL_PROXY_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_PROXY_TEXTURE_CUBE_MAP),
/// > or [`GL_TEXTURE_BUFFER`](crate::gl_enums::GL_TEXTURE_BUFFER).
///
/// `texture`
///
/// > Specifies the texture object name for [**glGetTextureLevelParameterfv**](crate::context::Context::oxidegl_get_texture_level_parameterfv)
/// > and [**glGetTextureLevelParameteriv**](crate::context::Context::oxidegl_get_texture_level_parameteriv)
/// > functions.
///
/// `level`
///
/// > Specifies the level-of-detail number of the desired image. Level 0 is the
/// > base image level. Level `[inlineq]` `[inlineq]`
///
/// `pname`
///
/// > Specifies the symbolic name of a texture parameter. [`GL_TEXTURE_WIDTH`](crate::gl_enums::GL_TEXTURE_WIDTH),
/// > [`GL_TEXTURE_HEIGHT`](crate::gl_enums::GL_TEXTURE_HEIGHT), [`GL_TEXTURE_DEPTH`](crate::gl_enums::GL_TEXTURE_DEPTH),
/// > [`GL_TEXTURE_INTERNAL_FORMAT`](crate::gl_enums::GL_TEXTURE_INTERNAL_FORMAT),
/// > [`GL_TEXTURE_RED_SIZE`](crate::gl_enums::GL_TEXTURE_RED_SIZE), [`GL_TEXTURE_GREEN_SIZE`](crate::gl_enums::GL_TEXTURE_GREEN_SIZE),
/// > [`GL_TEXTURE_BLUE_SIZE`](crate::gl_enums::GL_TEXTURE_BLUE_SIZE), [`GL_TEXTURE_ALPHA_SIZE`](crate::gl_enums::GL_TEXTURE_ALPHA_SIZE),
/// > [`GL_TEXTURE_DEPTH_SIZE`](crate::gl_enums::GL_TEXTURE_DEPTH_SIZE), [`GL_TEXTURE_COMPRESSED`](crate::gl_enums::GL_TEXTURE_COMPRESSED),
/// > [`GL_TEXTURE_COMPRESSED_IMAGE_SIZE`](crate::gl_enums::GL_TEXTURE_COMPRESSED_IMAGE_SIZE),
/// > and [`GL_TEXTURE_BUFFER_OFFSET`](crate::gl_enums::GL_TEXTURE_BUFFER_OFFSET)
/// > are accepted.
///
/// `params`
///
/// > Returns the requested data.
///
/// ### Description
/// [**glGetTexLevelParameterfv**](crate::context::Context::oxidegl_get_tex_level_parameterfv),
/// [**glGetTexLevelParameteriv**](crate::context::Context::oxidegl_get_tex_level_parameteriv),
/// [**glGetTextureLevelParameterfv**](crate::context::Context::oxidegl_get_texture_level_parameterfv)
/// and [**glGetTextureLevelParameteriv**](crate::context::Context::oxidegl_get_texture_level_parameteriv)
/// return in `params` texture parameter values for a specific level-of-detail
/// value, specified as `level`. For the first two functions, `target` defines
/// the target texture, either [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D),
/// [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D), [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D),
/// [`GL_PROXY_TEXTURE_1D`](crate::gl_enums::GL_PROXY_TEXTURE_1D), [`GL_PROXY_TEXTURE_2D`](crate::gl_enums::GL_PROXY_TEXTURE_2D),
/// [`GL_PROXY_TEXTURE_3D`](crate::gl_enums::GL_PROXY_TEXTURE_3D), [`GL_TEXTURE_CUBE_MAP_POSITIVE_X`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_POSITIVE_X),
/// [`GL_TEXTURE_CUBE_MAP_NEGATIVE_X`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_NEGATIVE_X),
/// [`GL_TEXTURE_CUBE_MAP_POSITIVE_Y`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_POSITIVE_Y),
/// [`GL_TEXTURE_CUBE_MAP_NEGATIVE_Y`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_NEGATIVE_Y),
/// [`GL_TEXTURE_CUBE_MAP_POSITIVE_Z`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_POSITIVE_Z),
/// [`GL_TEXTURE_CUBE_MAP_NEGATIVE_Z`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_NEGATIVE_Z),
/// or [`GL_PROXY_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_PROXY_TEXTURE_CUBE_MAP).
/// The remaining two take a `texture` argument which specifies the name of
/// the texture object.
///
/// [`GL_MAX_TEXTURE_SIZE`](crate::gl_enums::GL_MAX_TEXTURE_SIZE), and [`GL_MAX_3D_TEXTURE_SIZE`](crate::gl_enums::GL_MAX_3D_TEXTURE_SIZE)
/// are not really descriptive enough. It has to report the largest square
/// texture image that can be accommodated with mipmaps but a long skinny texture,
/// or a texture without mipmaps may easily fit in texture memory. The proxy
/// targets allow the user to more accurately query whether the GL can accommodate
/// a texture of a given configuration. If the texture cannot be accommodated,
/// the texture state variables, which may be queried with [**glGetTexLevelParameter**](crate::context::Context::oxidegl_get_tex_level_parameter)
/// and [**glGetTextureLevelParameter**](crate::context::Context::oxidegl_get_texture_level_parameter),
/// are set to 0. If the texture can be accommodated, the texture state values
/// will be set as they would be set for a non-proxy target.
///
/// `pname` specifies the texture parameter whose value or values will be returned.
///
/// The accepted parameter names are as follows:
///
/// [`GL_TEXTURE_WIDTH`](crate::gl_enums::GL_TEXTURE_WIDTH)
///
/// > `params` returns a single value, the width of the texture image. The initial
/// > value is 0.
///
/// [`GL_TEXTURE_HEIGHT`](crate::gl_enums::GL_TEXTURE_HEIGHT)
///
/// > `params` returns a single value, the height of the texture image. The initial
/// > value is 0.
///
/// [`GL_TEXTURE_DEPTH`](crate::gl_enums::GL_TEXTURE_DEPTH)
///
/// > `params` returns a single value, the depth of the texture image. The initial
/// > value is 0.
///
/// [`GL_TEXTURE_INTERNAL_FORMAT`](crate::gl_enums::GL_TEXTURE_INTERNAL_FORMAT)
///
/// > `params` returns a single value, the internal format of the texture image.
///
/// [`GL_TEXTURE_RED_TYPE`](crate::gl_enums::GL_TEXTURE_RED_TYPE), [`GL_TEXTURE_GREEN_TYPE`](crate::gl_enums::GL_TEXTURE_GREEN_TYPE),
/// [`GL_TEXTURE_BLUE_TYPE`](crate::gl_enums::GL_TEXTURE_BLUE_TYPE), [`GL_TEXTURE_ALPHA_TYPE`](crate::gl_enums::GL_TEXTURE_ALPHA_TYPE),
/// [`GL_TEXTURE_DEPTH_TYPE`](crate::gl_enums::GL_TEXTURE_DEPTH_TYPE)
///
/// > The data type used to store the component. The types [`GL_NONE`](crate::gl_enums::GL_NONE),
/// > [`GL_SIGNED_NORMALIZED`](crate::gl_enums::GL_SIGNED_NORMALIZED), [`GL_UNSIGNED_NORMALIZED`](crate::gl_enums::GL_UNSIGNED_NORMALIZED),
/// > [`GL_FLOAT`](crate::gl_enums::GL_FLOAT), [`GL_INT`](crate::gl_enums::GL_INT),
/// > and [`GL_UNSIGNED_INT`](crate::gl_enums::GL_UNSIGNED_INT) may be returned
/// > to indicate signed normalized fixed-point, unsigned normalized fixed-point,
/// > floating-point, integer unnormalized, and unsigned integer unnormalized
/// > components, respectively.
///
/// [`GL_TEXTURE_RED_SIZE`](crate::gl_enums::GL_TEXTURE_RED_SIZE), [`GL_TEXTURE_GREEN_SIZE`](crate::gl_enums::GL_TEXTURE_GREEN_SIZE),
/// [`GL_TEXTURE_BLUE_SIZE`](crate::gl_enums::GL_TEXTURE_BLUE_SIZE), [`GL_TEXTURE_ALPHA_SIZE`](crate::gl_enums::GL_TEXTURE_ALPHA_SIZE),
/// [`GL_TEXTURE_DEPTH_SIZE`](crate::gl_enums::GL_TEXTURE_DEPTH_SIZE)
///
///
/// > The internal storage resolution of an individual component. The resolution
/// > chosen by the GL will be a close match for the resolution requested by
/// > the user with the component argument of [**glTexImage1D**](crate::context::Context::oxidegl_tex_image1_d),
/// > [**glTexImage2D**](crate::context::Context::oxidegl_tex_image2_d), [**glTexImage3D**](crate::context::Context::oxidegl_tex_image3_d),
/// > [**glCopyTexImage1D**](crate::context::Context::oxidegl_copy_tex_image1_d),
/// > and [**glCopyTexImage2D**](crate::context::Context::oxidegl_copy_tex_image2_d).
/// > The initial value is 0.
///
/// [`GL_TEXTURE_COMPRESSED`](crate::gl_enums::GL_TEXTURE_COMPRESSED)
///
///
/// > `params` returns a single boolean value indicating if the texture image
/// > is stored in a compressed internal format. The initiali value is [`GL_FALSE`](crate::gl_enums::GL_FALSE).
///
/// [`GL_TEXTURE_COMPRESSED_IMAGE_SIZE`](crate::gl_enums::GL_TEXTURE_COMPRESSED_IMAGE_SIZE)
///
///
/// > `params` returns a single integer value, the number of unsigned bytes of
/// > the compressed texture image that would be returned from [**glGetCompressedTexImage**](crate::context::Context::oxidegl_get_compressed_tex_image).
///
/// [`GL_TEXTURE_BUFFER_OFFSET`](crate::gl_enums::GL_TEXTURE_BUFFER_OFFSET)
///
///
/// > `params` returns a single integer value, the offset into the data store
/// > of the buffer bound to a buffer texture. [**glTexBufferRange**](crate::context::Context::oxidegl_tex_buffer_range).
///
/// [`GL_TEXTURE_BUFFER_SIZE`](crate::gl_enums::GL_TEXTURE_BUFFER_SIZE)
///
///
/// > `params` returns a single integer value, the size of the range of a data
/// > store of the buffer bound to a buffer texture. [**glTexBufferRange**](crate::context::Context::oxidegl_tex_buffer_range).
///
/// ### Notes
/// If an error is generated, no change is made to the contents of `params`.
///
/// [**glGetTexLevelParameter**](crate::context::Context::oxidegl_get_tex_level_parameter)
/// returns the texture level parameters for the active texture unit.
///
/// [`GL_TEXTURE_BUFFER_OFFSET`](crate::gl_enums::GL_TEXTURE_BUFFER_OFFSET)
/// and [`GL_TEXTURE_BUFFER_SIZE`](crate::gl_enums::GL_TEXTURE_BUFFER_SIZE)
/// are available only if the GL version is 4.3 or greater.
impl Context {
    pub unsafe fn oxidegl_get_tex_level_parameterfv(
        &mut self,
        target: TextureTarget,
        level: GLint,
        pname: GetTextureParameter,
        params: *mut GLfloat,
    ) -> GlFallible {
        panic!("command oxidegl_get_tex_level_parameterfv not yet implemented");
    }
    pub unsafe fn oxidegl_get_tex_level_parameteriv(
        &mut self,
        target: TextureTarget,
        level: GLint,
        pname: GetTextureParameter,
        params: *mut GLint,
    ) -> GlFallible {
        panic!("command oxidegl_get_tex_level_parameteriv not yet implemented");
    }
    pub unsafe fn oxidegl_get_texture_level_parameterfv(
        &mut self,
        texture: GLuint,
        level: GLint,
        pname: GetTextureParameter,
        params: *mut GLfloat,
    ) -> GlFallible {
        panic!("command oxidegl_get_texture_level_parameterfv not yet implemented");
    }
    pub unsafe fn oxidegl_get_texture_level_parameteriv(
        &mut self,
        texture: GLuint,
        level: GLint,
        pname: GetTextureParameter,
        params: *mut GLint,
    ) -> GlFallible {
        panic!("command oxidegl_get_texture_level_parameteriv not yet implemented");
    }
}
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the texture is bound for [**glGetTexParameterfv**](crate::context::Context::oxidegl_get_tex_parameterfv),
/// > [**glGetTexParameteriv**](crate::context::Context::oxidegl_get_tex_parameteriv),
/// > [**glGetTexParameterIiv**](crate::context::Context::oxidegl_get_tex_parameter_iiv),
/// > and [**glGetTexParameterIuiv**](crate::context::Context::oxidegl_get_tex_parameter_iuiv)
/// > functions. [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D), [`GL_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_1D_ARRAY),
/// > [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D), [`GL_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_ARRAY),
/// > [`GL_TEXTURE_2D_MULTISAMPLE`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE),
/// > [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY),
/// > [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D), [`GL_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_CUBE_MAP),
/// > [`GL_TEXTURE_RECTANGLE`](crate::gl_enums::GL_TEXTURE_RECTANGLE), and [`GL_TEXTURE_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_ARRAY)
/// > are accepted.
///
/// `texture`
///
/// > Specifies the texture object name for [**glGetTextureParameterfv**](crate::context::Context::oxidegl_get_texture_parameterfv),
/// > [**glGetTextureParameteriv**](crate::context::Context::oxidegl_get_texture_parameteriv),
/// > [**glGetTextureParameterIiv**](crate::context::Context::oxidegl_get_texture_parameter_iiv),
/// > and [**glGetTextureParameterIuiv**](crate::context::Context::oxidegl_get_texture_parameter_iuiv)
/// > functions.
///
/// `pname`
///
/// > Specifies the symbolic name of a texture parameter. [`GL_DEPTH_STENCIL_TEXTURE_MODE`](crate::gl_enums::GL_DEPTH_STENCIL_TEXTURE_MODE),
/// > [`GL_IMAGE_FORMAT_COMPATIBILITY_TYPE`](crate::gl_enums::GL_IMAGE_FORMAT_COMPATIBILITY_TYPE),
/// > [`GL_TEXTURE_BASE_LEVEL`](crate::gl_enums::GL_TEXTURE_BASE_LEVEL), [`GL_TEXTURE_BORDER_COLOR`](crate::gl_enums::GL_TEXTURE_BORDER_COLOR),
/// > [`GL_TEXTURE_COMPARE_MODE`](crate::gl_enums::GL_TEXTURE_COMPARE_MODE), [`GL_TEXTURE_COMPARE_FUNC`](crate::gl_enums::GL_TEXTURE_COMPARE_FUNC),
/// > [`GL_TEXTURE_IMMUTABLE_FORMAT`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_FORMAT),
/// > [`GL_TEXTURE_IMMUTABLE_LEVELS`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_LEVELS),
/// > [`GL_TEXTURE_LOD_BIAS`](crate::gl_enums::GL_TEXTURE_LOD_BIAS), [`GL_TEXTURE_MAG_FILTER`](crate::gl_enums::GL_TEXTURE_MAG_FILTER),
/// > [`GL_TEXTURE_MAX_LEVEL`](crate::gl_enums::GL_TEXTURE_MAX_LEVEL), [`GL_TEXTURE_MAX_LOD`](crate::gl_enums::GL_TEXTURE_MAX_LOD),
/// > [`GL_TEXTURE_MIN_FILTER`](crate::gl_enums::GL_TEXTURE_MIN_FILTER), [`GL_TEXTURE_MIN_LOD`](crate::gl_enums::GL_TEXTURE_MIN_LOD),
/// > [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R), [`GL_TEXTURE_SWIZZLE_G`](crate::gl_enums::GL_TEXTURE_SWIZZLE_G),
/// > [`GL_TEXTURE_SWIZZLE_B`](crate::gl_enums::GL_TEXTURE_SWIZZLE_B), [`GL_TEXTURE_SWIZZLE_A`](crate::gl_enums::GL_TEXTURE_SWIZZLE_A),
/// > [`GL_TEXTURE_SWIZZLE_RGBA`](crate::gl_enums::GL_TEXTURE_SWIZZLE_RGBA), [`GL_TEXTURE_TARGET`](crate::gl_enums::GL_TEXTURE_TARGET),
/// > [`GL_TEXTURE_VIEW_MIN_LAYER`](crate::gl_enums::GL_TEXTURE_VIEW_MIN_LAYER),
/// > [`GL_TEXTURE_VIEW_MIN_LEVEL`](crate::gl_enums::GL_TEXTURE_VIEW_MIN_LEVEL),
/// > [`GL_TEXTURE_VIEW_NUM_LAYERS`](crate::gl_enums::GL_TEXTURE_VIEW_NUM_LAYERS),
/// > [`GL_TEXTURE_VIEW_NUM_LEVELS`](crate::gl_enums::GL_TEXTURE_VIEW_NUM_LEVELS),
/// > [`GL_TEXTURE_WRAP_S`](crate::gl_enums::GL_TEXTURE_WRAP_S), [`GL_TEXTURE_WRAP_T`](crate::gl_enums::GL_TEXTURE_WRAP_T),
/// > and [`GL_TEXTURE_WRAP_R`](crate::gl_enums::GL_TEXTURE_WRAP_R) are accepted.
///
/// `params`
///
/// > Returns the texture parameters.
///
/// ### Description
/// [**glGetTexParameter**](crate::context::Context::oxidegl_get_tex_parameter)
/// and [**glGetTextureParameter**](crate::context::Context::oxidegl_get_texture_parameter)
/// return in `params` the value or values of the texture parameter specified
/// as `pname`. `target` defines the target texture. [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D),
/// [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D), [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D),
/// [`GL_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_1D_ARRAY), [`GL_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_ARRAY),
/// [`GL_TEXTURE_RECTANGLE`](crate::gl_enums::GL_TEXTURE_RECTANGLE), [`GL_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_CUBE_MAP),
/// [`GL_TEXTURE_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_ARRAY),
/// [`GL_TEXTURE_2D_MULTISAMPLE`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE),
/// or [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY)
/// specify one-, two-, or three-dimensional, one-dimensional array, two-dimensional
/// array, rectangle, cube-mapped or cube-mapped array, two-dimensional multisample,
/// or two-dimensional multisample array texturing, respectively. `pname` accepts
/// the same symbols as [**glTexParameter**](crate::context::Context::oxidegl_tex_parameter),
/// with the same interpretations:
///
/// [`GL_DEPTH_STENCIL_TEXTURE_MODE`](crate::gl_enums::GL_DEPTH_STENCIL_TEXTURE_MODE)
///
/// > Returns the single-value depth stencil texture mode, a symbolic constant.
/// > The initial value is [`GL_DEPTH_COMPONENT`](crate::gl_enums::GL_DEPTH_COMPONENT).
///
/// [`GL_TEXTURE_MAG_FILTER`](crate::gl_enums::GL_TEXTURE_MAG_FILTER)
///
/// > Returns the single-valued texture magnification filter, a symbolic constant.
/// > The initial value is [`GL_LINEAR`](crate::gl_enums::GL_LINEAR).
///
/// [`GL_TEXTURE_MIN_FILTER`](crate::gl_enums::GL_TEXTURE_MIN_FILTER)
///
/// > Returns the single-valued texture minification filter, a symbolic constant.
/// > The initial value is [`GL_NEAREST_MIPMAP_LINEAR`](crate::gl_enums::GL_NEAREST_MIPMAP_LINEAR).
///
/// [`GL_TEXTURE_MIN_LOD`](crate::gl_enums::GL_TEXTURE_MIN_LOD)
///
/// > Returns the single-valued texture minimum level-of-detail value. The initial
/// > value is `[inlineq]`
///
/// [`GL_TEXTURE_MAX_LOD`](crate::gl_enums::GL_TEXTURE_MAX_LOD)
///
/// > Returns the single-valued texture maximum level-of-detail value. The initial
/// > value is 1000.
///
/// [`GL_TEXTURE_BASE_LEVEL`](crate::gl_enums::GL_TEXTURE_BASE_LEVEL)
///
/// > Returns the single-valued base texture mipmap level. The initial value
/// > is 0.
///
/// [`GL_TEXTURE_MAX_LEVEL`](crate::gl_enums::GL_TEXTURE_MAX_LEVEL)
///
/// > Returns the single-valued maximum texture mipmap array level. The initial
/// > value is 1000.
///
/// [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R)
///
/// > Returns the red component swizzle. The initial value is [`GL_RED`](crate::gl_enums::GL_RED).
///
/// [`GL_TEXTURE_SWIZZLE_G`](crate::gl_enums::GL_TEXTURE_SWIZZLE_G)
///
/// > Returns the green component swizzle. The initial value is [`GL_GREEN`](crate::gl_enums::GL_GREEN).
///
/// [`GL_TEXTURE_SWIZZLE_B`](crate::gl_enums::GL_TEXTURE_SWIZZLE_B)
///
/// > Returns the blue component swizzle. The initial value is [`GL_BLUE`](crate::gl_enums::GL_BLUE).
///
/// [`GL_TEXTURE_SWIZZLE_A`](crate::gl_enums::GL_TEXTURE_SWIZZLE_A)
///
/// > Returns the alpha component swizzle. The initial value is [`GL_ALPHA`](crate::gl_enums::GL_ALPHA).
///
/// [`GL_TEXTURE_SWIZZLE_RGBA`](crate::gl_enums::GL_TEXTURE_SWIZZLE_RGBA)
///
/// > Returns the component swizzle for all channels in a single query.
///
/// [`GL_TEXTURE_WRAP_S`](crate::gl_enums::GL_TEXTURE_WRAP_S)
///
/// > Returns the single-valued wrapping function for texture coordinate `[inlineq]`
/// > [`GL_REPEAT`](crate::gl_enums::GL_REPEAT).
///
/// [`GL_TEXTURE_WRAP_T`](crate::gl_enums::GL_TEXTURE_WRAP_T)
///
/// > Returns the single-valued wrapping function for texture coordinate `[inlineq]`
/// > [`GL_REPEAT`](crate::gl_enums::GL_REPEAT).
///
/// [`GL_TEXTURE_WRAP_R`](crate::gl_enums::GL_TEXTURE_WRAP_R)
///
/// > Returns the single-valued wrapping function for texture coordinate `[inlineq]`
/// > [`GL_REPEAT`](crate::gl_enums::GL_REPEAT).
///
/// [`GL_TEXTURE_BORDER_COLOR`](crate::gl_enums::GL_TEXTURE_BORDER_COLOR)
///
/// > Returns four integer or floating-point numbers that comprise the RGBA color
/// > of the texture border. Floating-point values are returned in the range
/// > `[inlineq]` `[inlineq]`
///
/// [`GL_TEXTURE_COMPARE_MODE`](crate::gl_enums::GL_TEXTURE_COMPARE_MODE)
///
/// > Returns a single-valued texture comparison mode, a symbolic constant. The
/// > initial value is [`GL_NONE`](crate::gl_enums::GL_NONE). See [**glTexParameter**](crate::context::Context::oxidegl_tex_parameter).
///
/// [`GL_TEXTURE_COMPARE_FUNC`](crate::gl_enums::GL_TEXTURE_COMPARE_FUNC)
///
/// > Returns a single-valued texture comparison function, a symbolic constant.
/// > The initial value is [`GL_LEQUAL`](crate::gl_enums::GL_LEQUAL). See [**glTexParameter**](crate::context::Context::oxidegl_tex_parameter).
///
/// [`GL_TEXTURE_VIEW_MIN_LEVEL`](crate::gl_enums::GL_TEXTURE_VIEW_MIN_LEVEL)
///
/// > Returns a single-valued base level of a texture view relative to its parent.
/// > The initial value is 0. See [**glTextureView**](crate::context::Context::oxidegl_texture_view).
///
/// [`GL_TEXTURE_VIEW_NUM_LEVELS`](crate::gl_enums::GL_TEXTURE_VIEW_NUM_LEVELS)
///
/// > Returns a single-valued number of levels of detail of a texture view. See
/// > [**glTextureView**](crate::context::Context::oxidegl_texture_view).
///
/// [`GL_TEXTURE_VIEW_MIN_LAYER`](crate::gl_enums::GL_TEXTURE_VIEW_MIN_LAYER)
///
/// > Returns a single-valued first level of a texture array view relative to
/// > its parent. See [**glTextureView**](crate::context::Context::oxidegl_texture_view).
///
/// [`GL_TEXTURE_VIEW_NUM_LAYERS`](crate::gl_enums::GL_TEXTURE_VIEW_NUM_LAYERS)
///
/// > Returns a single-valued number of layers in a texture array view. See [**glTextureView**](crate::context::Context::oxidegl_texture_view).
///
/// [`GL_TEXTURE_IMMUTABLE_LEVELS`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_LEVELS)
///
/// > Returns a single-valued number of immutable texture levels in a texture
/// > view. See [**glTextureView**](crate::context::Context::oxidegl_texture_view).
///
/// In addition to the parameters that may be set with [**glTexParameter**](crate::context::Context::oxidegl_tex_parameter),
/// [**glGetTexParameter**](crate::context::Context::oxidegl_get_tex_parameter)
/// and [**glGetTextureParameter**](crate::context::Context::oxidegl_get_texture_parameter)
/// accept the following read-only parameters:
///
/// [`GL_IMAGE_FORMAT_COMPATIBILITY_TYPE`](crate::gl_enums::GL_IMAGE_FORMAT_COMPATIBILITY_TYPE)
///
/// > Returns the matching criteria use for the texture when used as an image
/// > texture. Can return [`GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE`](crate::gl_enums::GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE),
/// > [`GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS`](crate::gl_enums::GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS)
/// > or [`GL_NONE`](crate::gl_enums::GL_NONE).
///
/// [`GL_TEXTURE_IMMUTABLE_FORMAT`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_FORMAT)
///
/// > Returns non-zero if the texture has an immutable format. Textures become
/// > immutable if their storage is specified with [**glTexStorage1D**](crate::context::Context::oxidegl_tex_storage1_d),
/// > [**glTexStorage2D**](crate::context::Context::oxidegl_tex_storage2_d) or
/// > [**glTexStorage3D**](crate::context::Context::oxidegl_tex_storage3_d). The
/// > initial value is [`GL_FALSE`](crate::gl_enums::GL_FALSE).
///
/// [`GL_TEXTURE_TARGET`](crate::gl_enums::GL_TEXTURE_TARGET)
///
/// > Returns the effective target of the texture object. For [**glGetTex*Parameter**](crate::context::Context::oxidegl_get_tex*_parameter)
/// > functions, this is the target parameter. For [**glGetTextureParameter***](crate::context::Context::oxidegl_get_texture_parameter*),
/// > it is the target to which the texture was initially bound when it was
/// > created, or the value of the target parameter to the call to [**glCreateTextures**](crate::context::Context::oxidegl_create_textures)
/// > which created the texture.
///
/// ### Notes
/// If an error is generated, no change is made to the contents of `params`.
///
/// [`GL_IMAGE_FORMAT_COMPATIBILITY_TYPE`](crate::gl_enums::GL_IMAGE_FORMAT_COMPATIBILITY_TYPE)
/// is available only if the GL version is 4.2 or greater.
///
/// [`GL_DEPTH_STENCIL_TEXTURE_MODE`](crate::gl_enums::GL_DEPTH_STENCIL_TEXTURE_MODE),
/// [`GL_TEXTURE_VIEW_MIN_LEVEL`](crate::gl_enums::GL_TEXTURE_VIEW_MIN_LEVEL),
/// [`GL_TEXTURE_VIEW_NUM_LEVELS`](crate::gl_enums::GL_TEXTURE_VIEW_NUM_LEVELS),
/// [`GL_TEXTURE_VIEW_MIN_LAYER`](crate::gl_enums::GL_TEXTURE_VIEW_MIN_LAYER),
/// [`GL_TEXTURE_VIEW_NUM_LAYERS`](crate::gl_enums::GL_TEXTURE_VIEW_NUM_LAYERS)
/// and [`GL_TEXTURE_IMMUTABLE_LEVELS`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_LEVELS)
/// are available only if the GL version is 4.3 or greater.
///
/// [`GL_TEXTURE_TARGET`](crate::gl_enums::GL_TEXTURE_TARGET) are available
/// only if the GL version is 4.5 or greater.
impl Context {
    pub unsafe fn oxidegl_get_tex_parameterfv(
        &mut self,
        target: TextureTarget,
        pname: GetTextureParameter,
        params: *mut GLfloat,
    ) -> GlFallible {
        panic!("command oxidegl_get_tex_parameterfv not yet implemented");
    }
    pub unsafe fn oxidegl_get_tex_parameteriv(
        &mut self,
        target: TextureTarget,
        pname: GetTextureParameter,
        params: *mut GLint,
    ) -> GlFallible {
        panic!("command oxidegl_get_tex_parameteriv not yet implemented");
    }
    pub unsafe fn oxidegl_get_tex_parameter_iiv(
        &mut self,
        target: TextureTarget,
        pname: GetTextureParameter,
        params: *mut GLint,
    ) -> GlFallible {
        panic!("command oxidegl_get_tex_parameter_iiv not yet implemented");
    }
    pub unsafe fn oxidegl_get_tex_parameter_iuiv(
        &mut self,
        target: TextureTarget,
        pname: GetTextureParameter,
        params: *mut GLuint,
    ) -> GlFallible {
        panic!("command oxidegl_get_tex_parameter_iuiv not yet implemented");
    }
    pub unsafe fn oxidegl_get_texture_parameterfv(
        &mut self,
        texture: GLuint,
        pname: GetTextureParameter,
        params: *mut GLfloat,
    ) -> GlFallible {
        panic!("command oxidegl_get_texture_parameterfv not yet implemented");
    }
    pub unsafe fn oxidegl_get_texture_parameter_iiv(
        &mut self,
        texture: GLuint,
        pname: GetTextureParameter,
        params: *mut GLint,
    ) -> GlFallible {
        panic!("command oxidegl_get_texture_parameter_iiv not yet implemented");
    }
    pub unsafe fn oxidegl_get_texture_parameter_iuiv(
        &mut self,
        texture: GLuint,
        pname: GetTextureParameter,
        params: *mut GLuint,
    ) -> GlFallible {
        panic!("command oxidegl_get_texture_parameter_iuiv not yet implemented");
    }
    pub unsafe fn oxidegl_get_texture_parameteriv(
        &mut self,
        texture: GLuint,
        pname: GetTextureParameter,
        params: *mut GLint,
    ) -> GlFallible {
        panic!("command oxidegl_get_texture_parameteriv not yet implemented");
    }
}

/// ### Parameters
/// `target`
///
/// > Specifies the target to which the texture is bound for [**glTexBuffer**](crate::context::Context::oxidegl_tex_buffer).
/// > Must be [`GL_TEXTURE_BUFFER`](crate::gl_enums::GL_TEXTURE_BUFFER).
///
/// `texture`
///
/// > Specifies the texture object name for [**glTextureBuffer**](crate::context::Context::oxidegl_texture_buffer).
///
/// `internalformat`
///
/// > Specifies the internal format of the data in the store belonging to `buffer`.
///
/// `buffer`
///
/// > Specifies the name of the buffer object whose storage to attach to the
/// > active buffer texture.
///
/// ### Description
/// [**glTexBuffer**](crate::context::Context::oxidegl_tex_buffer) and [**glTextureBuffer**](crate::context::Context::oxidegl_texture_buffer)
/// attaches the data store of a specified buffer object to a specified texture
/// object, and specify the storage format for the texture image found in the
/// buffer object. The texture object must be a buffer texture.
///
/// If `buffer` is zero, any buffer object attached to the buffer texture is
/// detached and no new buffer object is attached. If `buffer` is non-zero,
/// it must be the name of an existing buffer object.
///
/// `internalformat` specifies the storage format, and must be one of the following
/// sized internal formats:
///
/// | *Sized Internal Format*                               | *Base Type*       | *Components*        | *Norm*     | 0  | 1  | 2  | 3  |
/// |-------------------------------------------------------|-------------------|---------------------|------------|----|----|----|----|
/// | [`GL_R8`](crate::gl_enums::GL_R8)                     | ubyte             | 1                   | YES        | R  | 0  | 0  | 1  |
/// | [`GL_R16`](crate::gl_enums::GL_R16)                   | ushort            | 1                   | YES        | R  | 0  | 0  | 1  |
/// | [`GL_R16F`](crate::gl_enums::GL_R16F)                 | half              | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_R32F`](crate::gl_enums::GL_R32F)                 | float             | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_R8I`](crate::gl_enums::GL_R8I)                   | byte              | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_R16I`](crate::gl_enums::GL_R16I)                 | short             | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_R32I`](crate::gl_enums::GL_R32I)                 | int               | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_R8UI`](crate::gl_enums::GL_R8UI)                 | ubyte             | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_R16UI`](crate::gl_enums::GL_R16UI)               | ushort            | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_R32UI`](crate::gl_enums::GL_R32UI)               | uint              | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_RG8`](crate::gl_enums::GL_RG8)                   | ubyte             | 2                   | YES        | R  | G  | 0  | 1  |
/// | [`GL_RG16`](crate::gl_enums::GL_RG16)                 | ushort            | 2                   | YES        | R  | G  | 0  | 1  |
/// | [`GL_RG16F`](crate::gl_enums::GL_RG16F)               | half              | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RG32F`](crate::gl_enums::GL_RG32F)               | float             | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RG8I`](crate::gl_enums::GL_RG8I)                 | byte              | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RG16I`](crate::gl_enums::GL_RG16I)               | short             | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RG32I`](crate::gl_enums::GL_RG32I)               | int               | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RG8UI`](crate::gl_enums::GL_RG8UI)               | ubyte             | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RG16UI`](crate::gl_enums::GL_RG16UI)             | ushort            | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RG32UI`](crate::gl_enums::GL_RG32UI)             | uint              | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RGB32F`](crate::gl_enums::GL_RGB32F)             | float             | 3                   | NO         | R  | G  | B  | 1  |
/// | [`GL_RGB32I`](crate::gl_enums::GL_RGB32I)             | int               | 3                   | NO         | R  | G  | B  | 1  |
/// | [`GL_RGB32UI`](crate::gl_enums::GL_RGB32UI)           | uint              | 3                   | NO         | R  | G  | B  | 1  |
/// | [`GL_RGBA8`](crate::gl_enums::GL_RGBA8)               | uint              | 4                   | YES        | R  | G  | B  | A  |
/// | [`GL_RGBA16`](crate::gl_enums::GL_RGBA16)             | short             | 4                   | YES        | R  | G  | B  | A  |
/// | [`GL_RGBA16F`](crate::gl_enums::GL_RGBA16F)           | half              | 4                   | NO         | R  | G  | B  | A  |
/// | [`GL_RGBA32F`](crate::gl_enums::GL_RGBA32F)           | float             | 4                   | NO         | R  | G  | B  | A  |
/// | [`GL_RGBA8I`](crate::gl_enums::GL_RGBA8I)             | byte              | 4                   | NO         | R  | G  | B  | A  |
/// | [`GL_RGBA16I`](crate::gl_enums::GL_RGBA16I)           | short             | 4                   | NO         | R  | G  | B  | A  |
/// | [`GL_RGBA32I`](crate::gl_enums::GL_RGBA32I)           | int               | 4                   | NO         | R  | G  | B  | A  |
/// | [`GL_RGBA8UI`](crate::gl_enums::GL_RGBA8UI)           | ubyte             | 4                   | NO         | R  | G  | B  | A  |
/// | [`GL_RGBA16UI`](crate::gl_enums::GL_RGBA16UI)         | ushort            | 4                   | NO         | R  | G  | B  | A  |
/// | [`GL_RGBA32UI`](crate::gl_enums::GL_RGBA32UI)         | uint              | 4                   | NO         | R  | G  | B  | A  |
///
/// When a buffer object is attached to a buffer texture, the buffer object's
/// data store is taken as the texture's texel array. The number of texels
/// in the buffer texture's texel array is given by $$ \left\lfloor{ size \over
///{ components \times sizeof(base\_type)}} \right\rfloor $$ where $size$ is
/// the size of the buffer object in basic machine units (the value of [`GL_BUFFER_SIZE`](crate::gl_enums::GL_BUFFER_SIZE)
/// for `buffer` ), and $components$ and $base\_type$ are the element count
/// and base data type for elements, as specified in the table above. The number
/// of texels in the texel array is then clamped to the value of the implementation-dependent
/// limit [`GL_MAX_TEXTURE_BUFFER_SIZE`](crate::gl_enums::GL_MAX_TEXTURE_BUFFER_SIZE).
/// When a buffer texture is accessed in a shader, the results of a texel
/// fetch are undefined if the specified texel coordinate is negative, or greater
/// than or equal to the clamped number of texels in the texel array.
///
/// ### Associated Gets
/// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_MAX_TEXTURE_BUFFER_SIZE`](crate::gl_enums::GL_MAX_TEXTURE_BUFFER_SIZE)
///
/// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_TEXTURE_BINDING_BUFFER`](crate::gl_enums::GL_TEXTURE_BINDING_BUFFER)
///
/// [**glGetTexLevelParameter**](crate::context::Context::oxidegl_get_tex_level_parameter)
/// with argument [`GL_TEXTURE_BUFFER_DATA_STORE_BINDING`](crate::gl_enums::GL_TEXTURE_BUFFER_DATA_STORE_BINDING)
impl Context {
    pub fn oxidegl_tex_buffer(
        &mut self,
        target: TextureTarget,
        internalformat: SizedInternalFormat,
        buffer: GLuint,
    ) -> GlFallible {
        panic!("command oxidegl_tex_buffer not yet implemented");
    }
    pub fn oxidegl_texture_buffer(
        &mut self,
        texture: GLuint,
        internalformat: SizedInternalFormat,
        buffer: GLuint,
    ) -> GlFallible {
        panic!("command oxidegl_texture_buffer not yet implemented");
    }
}
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the texture object is bound for [**glTexBufferRange**](crate::context::Context::oxidegl_tex_buffer_range).
/// > Must be [`GL_TEXTURE_BUFFER`](crate::gl_enums::GL_TEXTURE_BUFFER).
///
/// `texture`
///
/// > Specifies the texture object name for [**glTextureBufferRange**](crate::context::Context::oxidegl_texture_buffer_range).
///
/// `internalformat`
///
/// > Specifies the internal format of the data in the store belonging to `buffer`.
///
/// `buffer`
///
/// > Specifies the name of the buffer object whose storage to attach to the
/// > active buffer texture.
///
/// `offset`
///
/// > Specifies the offset of the start of the range of the buffer's data store
/// > to attach.
///
/// `size`
///
/// > Specifies the size of the range of the buffer's data store to attach.
///
/// ### Description
/// [**glTexBufferRange**](crate::context::Context::oxidegl_tex_buffer_range)
/// and [**glTextureBufferRange**](crate::context::Context::oxidegl_texture_buffer_range)
/// attach a range of the data store of a specified buffer object to a specified
/// texture object, and specify the storage format for the texture image found
/// in the buffer object. The texture object must be a buffer texture.
///
/// If `buffer` is zero, any buffer object attached to the buffer texture is
/// detached and no new buffer object is attached. If `buffer` is non-zero,
/// it must be the name of an existing buffer object.
///
/// The start and size of the range are specified by `offset` and `size` respectively,
/// both measured in basic machine units. `offset` must be greater than or
/// equal to zero, `size` must be greater than zero, and the sum of `offset`
/// and `size` must not exceed the value of [`GL_BUFFER_SIZE`](crate::gl_enums::GL_BUFFER_SIZE)
/// for `buffer`. Furthermore, `offset` must be an integer multiple of the
/// value of [`GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT`](crate::gl_enums::GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT).
///
/// `internalformat` specifies the storage format, and must be one of the following
/// sized internal formats:
///
/// | *Sized Internal Format*                               | *Base Type*       | *Components*        | *Norm*     | 0  | 1  | 2  | 3  |
/// |-------------------------------------------------------|-------------------|---------------------|------------|----|----|----|----|
/// | [`GL_R8`](crate::gl_enums::GL_R8)                     | ubyte             | 1                   | YES        | R  | 0  | 0  | 1  |
/// | [`GL_R16`](crate::gl_enums::GL_R16)                   | ushort            | 1                   | YES        | R  | 0  | 0  | 1  |
/// | [`GL_R16F`](crate::gl_enums::GL_R16F)                 | half              | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_R32F`](crate::gl_enums::GL_R32F)                 | float             | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_R8I`](crate::gl_enums::GL_R8I)                   | byte              | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_R16I`](crate::gl_enums::GL_R16I)                 | short             | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_R32I`](crate::gl_enums::GL_R32I)                 | int               | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_R8UI`](crate::gl_enums::GL_R8UI)                 | ubyte             | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_R16UI`](crate::gl_enums::GL_R16UI)               | ushort            | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_R32UI`](crate::gl_enums::GL_R32UI)               | uint              | 1                   | NO         | R  | 0  | 0  | 1  |
/// | [`GL_RG8`](crate::gl_enums::GL_RG8)                   | ubyte             | 2                   | YES        | R  | G  | 0  | 1  |
/// | [`GL_RG16`](crate::gl_enums::GL_RG16)                 | ushort            | 2                   | YES        | R  | G  | 0  | 1  |
/// | [`GL_RG16F`](crate::gl_enums::GL_RG16F)               | half              | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RG32F`](crate::gl_enums::GL_RG32F)               | float             | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RG8I`](crate::gl_enums::GL_RG8I)                 | byte              | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RG16I`](crate::gl_enums::GL_RG16I)               | short             | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RG32I`](crate::gl_enums::GL_RG32I)               | int               | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RG8UI`](crate::gl_enums::GL_RG8UI)               | ubyte             | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RG16UI`](crate::gl_enums::GL_RG16UI)             | ushort            | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RG32UI`](crate::gl_enums::GL_RG32UI)             | uint              | 2                   | NO         | R  | G  | 0  | 1  |
/// | [`GL_RGB32F`](crate::gl_enums::GL_RGB32F)             | float             | 3                   | NO         | R  | G  | B  | 1  |
/// | [`GL_RGB32I`](crate::gl_enums::GL_RGB32I)             | int               | 3                   | NO         | R  | G  | B  | 1  |
/// | [`GL_RGB32UI`](crate::gl_enums::GL_RGB32UI)           | uint              | 3                   | NO         | R  | G  | B  | 1  |
/// | [`GL_RGBA8`](crate::gl_enums::GL_RGBA8)               | uint              | 4                   | YES        | R  | G  | B  | A  |
/// | [`GL_RGBA16`](crate::gl_enums::GL_RGBA16)             | short             | 4                   | YES        | R  | G  | B  | A  |
/// | [`GL_RGBA16F`](crate::gl_enums::GL_RGBA16F)           | half              | 4                   | NO         | R  | G  | B  | A  |
/// | [`GL_RGBA32F`](crate::gl_enums::GL_RGBA32F)           | float             | 4                   | NO         | R  | G  | B  | A  |
/// | [`GL_RGBA8I`](crate::gl_enums::GL_RGBA8I)             | byte              | 4                   | NO         | R  | G  | B  | A  |
/// | [`GL_RGBA16I`](crate::gl_enums::GL_RGBA16I)           | short             | 4                   | NO         | R  | G  | B  | A  |
/// | [`GL_RGBA32I`](crate::gl_enums::GL_RGBA32I)           | int               | 4                   | NO         | R  | G  | B  | A  |
/// | [`GL_RGBA8UI`](crate::gl_enums::GL_RGBA8UI)           | ubyte             | 4                   | NO         | R  | G  | B  | A  |
/// | [`GL_RGBA16UI`](crate::gl_enums::GL_RGBA16UI)         | ushort            | 4                   | NO         | R  | G  | B  | A  |
/// | [`GL_RGBA32UI`](crate::gl_enums::GL_RGBA32UI)         | uint              | 4                   | NO         | R  | G  | B  | A  |
///
/// When a range of a buffer object is attached to a buffer texture, the specified
/// range of the buffer object's data store is taken as the texture's texel
/// array. The number of texels in the buffer texture's texel array is given
/// by $$ \left\lfloor{ size \over{ components \times sizeof(base\_type)}} \right\rfloor
/// $$ where $components$ and $base\_type$ are the element count and base data
/// type for elements, as specified in the table above. The number of texels
/// in the texel array is then clamped to the value of the implementation-dependent
/// limit [`GL_MAX_TEXTURE_BUFFER_SIZE`](crate::gl_enums::GL_MAX_TEXTURE_BUFFER_SIZE).
/// When a buffer texture is accessed in a shader, the results of a texel
/// fetch are undefined if the specified texel coordinate is negative, or greater
/// than or equal to the clamped number of texels in the texel array.
///
/// ### Associated Gets
/// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT`](crate::gl_enums::GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT)
///
/// [**glGetTexLevelParameter**](crate::context::Context::oxidegl_get_tex_level_parameter)
/// with arguments [`GL_TEXTURE_BUFFER_OFFSET`](crate::gl_enums::GL_TEXTURE_BUFFER_OFFSET)
/// or [`GL_TEXTURE_BUFFER_SIZE`](crate::gl_enums::GL_TEXTURE_BUFFER_SIZE).
impl Context {
    pub fn oxidegl_tex_buffer_range(
        &mut self,
        target: TextureTarget,
        internalformat: SizedInternalFormat,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) -> GlFallible {
        panic!("command oxidegl_tex_buffer_range not yet implemented");
    }
    pub fn oxidegl_texture_buffer_range(
        &mut self,
        texture: GLuint,
        internalformat: SizedInternalFormat,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) -> GlFallible {
        panic!("command oxidegl_texture_buffer_range not yet implemented");
    }
}
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the texture is bound for [**glTexParameter**](crate::context::Context::oxidegl_tex_parameter)
/// > functions. Must be one of [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D),
/// > [`GL_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_1D_ARRAY), [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D),
/// > [`GL_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_ARRAY), [`GL_TEXTURE_2D_MULTISAMPLE`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE),
/// > [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY),
/// > [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D), [`GL_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_CUBE_MAP),
/// > [`GL_TEXTURE_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_ARRAY),
/// > or [`GL_TEXTURE_RECTANGLE`](crate::gl_enums::GL_TEXTURE_RECTANGLE).
///
/// `texture`
///
/// > Specifies the texture object name for [**glTextureParameter**](crate::context::Context::oxidegl_texture_parameter)
/// > functions.
///
/// `pname`
///
/// > Specifies the symbolic name of a single-valued texture parameter. `pname`
/// > can be one of the following: [`GL_DEPTH_STENCIL_TEXTURE_MODE`](crate::gl_enums::GL_DEPTH_STENCIL_TEXTURE_MODE),
/// > [`GL_TEXTURE_BASE_LEVEL`](crate::gl_enums::GL_TEXTURE_BASE_LEVEL), [`GL_TEXTURE_COMPARE_FUNC`](crate::gl_enums::GL_TEXTURE_COMPARE_FUNC),
/// > [`GL_TEXTURE_COMPARE_MODE`](crate::gl_enums::GL_TEXTURE_COMPARE_MODE), [`GL_TEXTURE_LOD_BIAS`](crate::gl_enums::GL_TEXTURE_LOD_BIAS),
/// > [`GL_TEXTURE_MIN_FILTER`](crate::gl_enums::GL_TEXTURE_MIN_FILTER), [`GL_TEXTURE_MAG_FILTER`](crate::gl_enums::GL_TEXTURE_MAG_FILTER),
/// > [`GL_TEXTURE_MIN_LOD`](crate::gl_enums::GL_TEXTURE_MIN_LOD), [`GL_TEXTURE_MAX_LOD`](crate::gl_enums::GL_TEXTURE_MAX_LOD),
/// > [`GL_TEXTURE_MAX_LEVEL`](crate::gl_enums::GL_TEXTURE_MAX_LEVEL), [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R),
/// > [`GL_TEXTURE_SWIZZLE_G`](crate::gl_enums::GL_TEXTURE_SWIZZLE_G), [`GL_TEXTURE_SWIZZLE_B`](crate::gl_enums::GL_TEXTURE_SWIZZLE_B),
/// > [`GL_TEXTURE_SWIZZLE_A`](crate::gl_enums::GL_TEXTURE_SWIZZLE_A), [`GL_TEXTURE_WRAP_S`](crate::gl_enums::GL_TEXTURE_WRAP_S),
/// > [`GL_TEXTURE_WRAP_T`](crate::gl_enums::GL_TEXTURE_WRAP_T), or [`GL_TEXTURE_WRAP_R`](crate::gl_enums::GL_TEXTURE_WRAP_R).
///
/// > For the vector commands( [**glTexParameter*v**](crate::context::Context::oxidegl_tex_parameter*v)
/// > ), `pname` can also be one of [`GL_TEXTURE_BORDER_COLOR`](crate::gl_enums::GL_TEXTURE_BORDER_COLOR)
/// > or [`GL_TEXTURE_SWIZZLE_RGBA`](crate::gl_enums::GL_TEXTURE_SWIZZLE_RGBA).
///
/// `param`
///
/// > For the scalar commands, specifies the value of `pname`.
///
/// `params`
///
/// > For the vector commands, specifies a pointer to an array where the value
/// > or values of `pname` are stored.
///
/// ### Description
/// [**glTexParameter**](crate::context::Context::oxidegl_tex_parameter) and
/// [**glTextureParameter**](crate::context::Context::oxidegl_texture_parameter)
/// assign the value or values in `params` to the texture parameter specified
/// as `pname`. For [**glTexParameter**](crate::context::Context::oxidegl_tex_parameter),
/// `target` defines the target texture, either [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D),
/// [`GL_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_1D_ARRAY), [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D),
/// [`GL_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_ARRAY), [`GL_TEXTURE_2D_MULTISAMPLE`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE),
/// [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY),
/// [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D), [`GL_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_CUBE_MAP),
/// [`GL_TEXTURE_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_ARRAY),
/// or [`GL_TEXTURE_RECTANGLE`](crate::gl_enums::GL_TEXTURE_RECTANGLE). The
/// following symbols are accepted in `pname`:
///
/// [`GL_DEPTH_STENCIL_TEXTURE_MODE`](crate::gl_enums::GL_DEPTH_STENCIL_TEXTURE_MODE)
///
/// > Specifies the mode used to read from depth-stencil format textures. `params`
/// > must be one of [`GL_DEPTH_COMPONENT`](crate::gl_enums::GL_DEPTH_COMPONENT)
/// > or [`GL_STENCIL_INDEX`](crate::gl_enums::GL_STENCIL_INDEX). If the depth
/// > stencil mode is [`GL_DEPTH_COMPONENT`](crate::gl_enums::GL_DEPTH_COMPONENT),
/// > then reads from depth-stencil format textures will return the depth component
/// > of the texel in `[inlineq]` [`GL_STENCIL_INDEX`](crate::gl_enums::GL_STENCIL_INDEX)
/// > then the stencil component is returned in `[inlineq]` [`GL_DEPTH_COMPONENT`](crate::gl_enums::GL_DEPTH_COMPONENT).
///
/// [`GL_TEXTURE_BASE_LEVEL`](crate::gl_enums::GL_TEXTURE_BASE_LEVEL)
///
/// > Specifies the index of the lowest defined mipmap level. This is an integer
/// > value. The initial value is 0.
///
///
/// [`GL_TEXTURE_BORDER_COLOR`](crate::gl_enums::GL_TEXTURE_BORDER_COLOR)
///
/// > The data in `params` specifies four values that define the border values
/// > that should be used for border texels. If a texel is sampled from the border
/// > of the texture, the values of [`GL_TEXTURE_BORDER_COLOR`](crate::gl_enums::GL_TEXTURE_BORDER_COLOR)
/// > are interpreted as an RGBA color to match the texture's internal format
/// > and substituted for the non-existent texel data. If the texture contains
/// > depth components, the first component of [`GL_TEXTURE_BORDER_COLOR`](crate::gl_enums::GL_TEXTURE_BORDER_COLOR)
/// > is interpreted as a depth value. The initial value is `[inlineq]`
///
/// > If the values for [`GL_TEXTURE_BORDER_COLOR`](crate::gl_enums::GL_TEXTURE_BORDER_COLOR)
/// > are specified with [**glTexParameterIiv**](crate::context::Context::oxidegl_tex_parameter_iiv)
/// > or [**glTexParameterIuiv**](crate::context::Context::oxidegl_tex_parameter_iuiv),
/// > the values are stored unmodified with an internal data type of integer.
/// > If specified with [**glTexParameteriv**](crate::context::Context::oxidegl_tex_parameteriv),
/// > they are converted to floating point with the following equation: `[inlineq]`
/// > [**glTexParameterfv**](crate::context::Context::oxidegl_tex_parameterfv),
/// > they are stored unmodified as floating-point values.
///
/// [`GL_TEXTURE_COMPARE_FUNC`](crate::gl_enums::GL_TEXTURE_COMPARE_FUNC)
///
/// > Specifies the comparison operator used when [`GL_TEXTURE_COMPARE_MODE`](crate::gl_enums::GL_TEXTURE_COMPARE_MODE)
/// > is set to [`GL_COMPARE_REF_TO_TEXTURE`](crate::gl_enums::GL_COMPARE_REF_TO_TEXTURE).
/// > Permissible values are:
/// > |* Texture Comparison Function*                                  |* Computed result*                            |
/// > |----------------------------------------------------------------|----------------------------------------------|
/// > | [`GL_LEQUAL`](crate::gl_enums::GL_LEQUAL)                      | `[inlineq]`                                  |
/// > | [`GL_GEQUAL`](crate::gl_enums::GL_GEQUAL)                      | `[inlineq]`                                  |
/// > | [`GL_LESS`](crate::gl_enums::GL_LESS)                          | `[inlineq]`                                  |
/// > | [`GL_GREATER`](crate::gl_enums::GL_GREATER)                    | `[inlineq]`                                  |
/// > | [`GL_EQUAL`](crate::gl_enums::GL_EQUAL)                        | `[inlineq]`                                  |
/// > | [`GL_NOTEQUAL`](crate::gl_enums::GL_NOTEQUAL)                  | `[inlineq]`                                  |
/// > | [`GL_ALWAYS`](crate::gl_enums::GL_ALWAYS)                      | `[inlineq]`                                  |
/// > | [`GL_NEVER`](crate::gl_enums::GL_NEVER)                        | `[inlineq]`                                  |
///
/// > `[inlineq]` `[inlineq]` `[inlineq]`
///
/// [`GL_TEXTURE_COMPARE_MODE`](crate::gl_enums::GL_TEXTURE_COMPARE_MODE)
///
/// > Specifies the texture comparison mode for currently bound depth textures.
/// > That is, a texture whose internal format is [`GL_DEPTH_COMPONENT_*`](crate::gl_enums::GL_DEPTH_COMPONENT_*);
/// > see [**glTexImage2D**](crate::context::Context::oxidegl_tex_image2_d))
/// > Permissible values are: [`GL_COMPARE_REF_TO_TEXTURE`](crate::gl_enums::GL_COMPARE_REF_TO_TEXTURE)
///
/// >> Specifies that the interpolated and clamped `[inlineq]` [`GL_TEXTURE_COMPARE_FUNC`](crate::gl_enums::GL_TEXTURE_COMPARE_FUNC)
/// >> for details of how the comparison is evaluated. The result of the comparison
/// >> is assigned to the red channel.
///
/// > [`GL_NONE`](crate::gl_enums::GL_NONE)
///
/// >> Specifies that the red channel should be assigned the appropriate value
/// >> from the currently bound depth texture.
///
///
/// [`GL_TEXTURE_LOD_BIAS`](crate::gl_enums::GL_TEXTURE_LOD_BIAS)
///
/// > `params` specifies a fixed bias value that is to be added to the level-of-detail
/// > parameter for the texture before texture sampling. The specified value
/// > is added to the shader-supplied bias value (if any) and subsequently clamped
/// > into the implementation-defined range `[inlineq]` `[inlineq]` [`GL_MAX_TEXTURE_LOD_BIAS`](crate::gl_enums::GL_MAX_TEXTURE_LOD_BIAS).
/// > The initial value is 0.0.
///
/// [`GL_TEXTURE_MIN_FILTER`](crate::gl_enums::GL_TEXTURE_MIN_FILTER)
///
/// > The texture minifying function is used whenever the level-of-detail function
/// > used when sampling from the texture determines that the texture should
/// > be minified. There are six defined minifying functions. Two of them use
/// > either the nearest texture elements or a weighted average of multiple texture
/// > elements to compute the texture value. The other four use mipmaps.
///
/// > A mipmap is an ordered set of arrays representing the same image at progressively
/// > lower resolutions. If the texture has dimensions `[inlineq]` `[inlineq]`
/// > `[inlineq]` `[inlineq]` `[inlineq]` `[inlineq]` `[inlineq]` `[inlineq]`
/// > `[inlineq]` `[inlineq]` [**glTexImage1D**](crate::context::Context::oxidegl_tex_image1_d),
/// > [**glTexImage2D**](crate::context::Context::oxidegl_tex_image2_d), [**glTexImage3D**](crate::context::Context::oxidegl_tex_image3_d),
/// > [**glCopyTexImage1D**](crate::context::Context::oxidegl_copy_tex_image1_d),
/// > or [**glCopyTexImage2D**](crate::context::Context::oxidegl_copy_tex_image2_d)
/// > with the *level* argument indicating the order of the mipmaps. Level 0
/// > is the original texture; level `[inlineq]` `[inlineq]`
///
/// > `params` supplies a function for minifying the texture as one of the following:
/// > [`GL_NEAREST`](crate::gl_enums::GL_NEAREST)
///
/// >> Returns the value of the texture element that is nearest (in Manhattan
/// >> distance) to the specified texture coordinates.
///
/// > [`GL_LINEAR`](crate::gl_enums::GL_LINEAR)
///
/// >> Returns the weighted average of the four texture elements that are closest
/// >> to the specified texture coordinates. These can include items wrapped or
/// >> repeated from other parts of a texture, depending on the values of [`GL_TEXTURE_WRAP_S`](crate::gl_enums::GL_TEXTURE_WRAP_S)
/// >> and [`GL_TEXTURE_WRAP_T`](crate::gl_enums::GL_TEXTURE_WRAP_T), and on the
/// >> exact mapping.
///
/// > [`GL_NEAREST_MIPMAP_NEAREST`](crate::gl_enums::GL_NEAREST_MIPMAP_NEAREST)
///
/// >> Chooses the mipmap that most closely matches the size of the pixel being
/// >> textured and uses the [`GL_NEAREST`](crate::gl_enums::GL_NEAREST) criterion
/// >> (the texture element closest to the specified texture coordinates) to produce
/// >> a texture value.
///
/// > [`GL_LINEAR_MIPMAP_NEAREST`](crate::gl_enums::GL_LINEAR_MIPMAP_NEAREST)
///
/// >> Chooses the mipmap that most closely matches the size of the pixel being
/// >> textured and uses the [`GL_LINEAR`](crate::gl_enums::GL_LINEAR) criterion
/// >> (a weighted average of the four texture elements that are closest to the
/// >> specified texture coordinates) to produce a texture value.
///
/// > [`GL_NEAREST_MIPMAP_LINEAR`](crate::gl_enums::GL_NEAREST_MIPMAP_LINEAR)
///
/// >> Chooses the two mipmaps that most closely match the size of the pixel being
/// >> textured and uses the [`GL_NEAREST`](crate::gl_enums::GL_NEAREST) criterion
/// >> (the texture element closest to the specified texture coordinates) to produce
/// >> a texture value from each mipmap. The final texture value is a weighted
/// >> average of those two values.
///
/// > [`GL_LINEAR_MIPMAP_LINEAR`](crate::gl_enums::GL_LINEAR_MIPMAP_LINEAR)
///
/// >> Chooses the two mipmaps that most closely match the size of the pixel being
/// >> textured and uses the [`GL_LINEAR`](crate::gl_enums::GL_LINEAR) criterion
/// >> (a weighted average of the texture elements that are closest to the specified
/// >> texture coordinates) to produce a texture value from each mipmap. The final
/// >> texture value is a weighted average of those two values.
///
///
/// > As more texture elements are sampled in the minification process, fewer
/// > aliasing artifacts will be apparent. While the [`GL_NEAREST`](crate::gl_enums::GL_NEAREST)
/// > and [`GL_LINEAR`](crate::gl_enums::GL_LINEAR) minification functions can
/// > be faster than the other four, they sample only one or multiple texture
/// > elements to determine the texture value of the pixel being rendered and
/// > can produce moire patterns or ragged transitions. The initial value of
/// > [`GL_TEXTURE_MIN_FILTER`](crate::gl_enums::GL_TEXTURE_MIN_FILTER) is [`GL_NEAREST_MIPMAP_LINEAR`](crate::gl_enums::GL_NEAREST_MIPMAP_LINEAR).
///
///
/// [`GL_TEXTURE_MAG_FILTER`](crate::gl_enums::GL_TEXTURE_MAG_FILTER)
///
/// > The texture magnification function is used whenever the level-of-detail
/// > function used when sampling from the texture determines that the texture
/// > should be magified. It sets the texture magnification function to either
/// > [`GL_NEAREST`](crate::gl_enums::GL_NEAREST) or [`GL_LINEAR`](crate::gl_enums::GL_LINEAR)
/// > (see below). [`GL_NEAREST`](crate::gl_enums::GL_NEAREST) is generally faster
/// > than [`GL_LINEAR`](crate::gl_enums::GL_LINEAR), but it can produce textured
/// > images with sharper edges because the transition between texture elements
/// > is not as smooth. The initial value of [`GL_TEXTURE_MAG_FILTER`](crate::gl_enums::GL_TEXTURE_MAG_FILTER)
/// > is [`GL_LINEAR`](crate::gl_enums::GL_LINEAR). [`GL_NEAREST`](crate::gl_enums::GL_NEAREST)
///
/// >> Returns the value of the texture element that is nearest (in Manhattan
/// >> distance) to the specified texture coordinates.
///
/// > [`GL_LINEAR`](crate::gl_enums::GL_LINEAR)
///
/// >> Returns the weighted average of the texture elements that are closest to
/// >> the specified texture coordinates. These can include items wrapped or repeated
/// >> from other parts of a texture, depending on the values of [`GL_TEXTURE_WRAP_S`](crate::gl_enums::GL_TEXTURE_WRAP_S)
/// >> and [`GL_TEXTURE_WRAP_T`](crate::gl_enums::GL_TEXTURE_WRAP_T), and on the
/// >> exact mapping.
///
///
///
/// [`GL_TEXTURE_MIN_LOD`](crate::gl_enums::GL_TEXTURE_MIN_LOD)
///
/// > Sets the minimum level-of-detail parameter. This floating-point value limits
/// > the selection of highest resolution mipmap (lowest mipmap level). The initial
/// > value is -1000.
///
///
/// [`GL_TEXTURE_MAX_LOD`](crate::gl_enums::GL_TEXTURE_MAX_LOD)
///
/// > Sets the maximum level-of-detail parameter. This floating-point value limits
/// > the selection of the lowest resolution mipmap (highest mipmap level). The
/// > initial value is 1000.
///
///
/// [`GL_TEXTURE_MAX_LEVEL`](crate::gl_enums::GL_TEXTURE_MAX_LEVEL)
///
/// > Sets the index of the highest defined mipmap level. This is an integer
/// > value. The initial value is 1000.
///
///
/// [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R)
///
/// > Sets the swizzle that will be applied to the `[inlineq]` `param` are [`GL_RED`](crate::gl_enums::GL_RED),
/// > [`GL_GREEN`](crate::gl_enums::GL_GREEN), [`GL_BLUE`](crate::gl_enums::GL_BLUE),
/// > [`GL_ALPHA`](crate::gl_enums::GL_ALPHA), [`GL_ZERO`](crate::gl_enums::GL_ZERO)
/// > and [`GL_ONE`](crate::gl_enums::GL_ONE). If [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R)
/// > is [`GL_RED`](crate::gl_enums::GL_RED), the value for `[inlineq]` [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R)
/// > is [`GL_GREEN`](crate::gl_enums::GL_GREEN), the value for `[inlineq]` [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R)
/// > is [`GL_BLUE`](crate::gl_enums::GL_BLUE), the value for `[inlineq]` [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R)
/// > is [`GL_ALPHA`](crate::gl_enums::GL_ALPHA), the value for `[inlineq]` [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R)
/// > is [`GL_ZERO`](crate::gl_enums::GL_ZERO), the value for `[inlineq]` `[inlineq]`
/// > [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R) is [`GL_ONE`](crate::gl_enums::GL_ONE),
/// > the value for `[inlineq]` `[inlineq]` [`GL_RED`](crate::gl_enums::GL_RED).
///
///
/// [`GL_TEXTURE_SWIZZLE_G`](crate::gl_enums::GL_TEXTURE_SWIZZLE_G)
///
/// > Sets the swizzle that will be applied to the `[inlineq]` `param` and their
/// > effects are similar to those of [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R).
/// > The initial value is [`GL_GREEN`](crate::gl_enums::GL_GREEN).
///
///
/// [`GL_TEXTURE_SWIZZLE_B`](crate::gl_enums::GL_TEXTURE_SWIZZLE_B)
///
/// > Sets the swizzle that will be applied to the `[inlineq]` `param` and their
/// > effects are similar to those of [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R).
/// > The initial value is [`GL_BLUE`](crate::gl_enums::GL_BLUE).
///
///
/// [`GL_TEXTURE_SWIZZLE_A`](crate::gl_enums::GL_TEXTURE_SWIZZLE_A)
///
/// > Sets the swizzle that will be applied to the `[inlineq]` `param` and their
/// > effects are similar to those of [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R).
/// > The initial value is [`GL_ALPHA`](crate::gl_enums::GL_ALPHA).
///
///
/// [`GL_TEXTURE_SWIZZLE_RGBA`](crate::gl_enums::GL_TEXTURE_SWIZZLE_RGBA)
///
/// > Sets the swizzles that will be applied to the `[inlineq]` `[inlineq]` `[inlineq]`
/// > `[inlineq]` `params` and their effects are similar to those of [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R),
/// > except that all channels are specified simultaneously. Setting the value
/// > of [`GL_TEXTURE_SWIZZLE_RGBA`](crate::gl_enums::GL_TEXTURE_SWIZZLE_RGBA)
/// > is equivalent (assuming no errors are generated) to setting the parameters
/// > of each of [`GL_TEXTURE_SWIZZLE_R`](crate::gl_enums::GL_TEXTURE_SWIZZLE_R),
/// > [`GL_TEXTURE_SWIZZLE_G`](crate::gl_enums::GL_TEXTURE_SWIZZLE_G), [`GL_TEXTURE_SWIZZLE_B`](crate::gl_enums::GL_TEXTURE_SWIZZLE_B),
/// > and [`GL_TEXTURE_SWIZZLE_A`](crate::gl_enums::GL_TEXTURE_SWIZZLE_A) successively.
///
///
/// [`GL_TEXTURE_WRAP_S`](crate::gl_enums::GL_TEXTURE_WRAP_S)
///
/// > Sets the wrap parameter for texture coordinate `[inlineq]` [`GL_CLAMP_TO_EDGE`](crate::gl_enums::GL_CLAMP_TO_EDGE),
/// > [`GL_CLAMP_TO_BORDER`](crate::gl_enums::GL_CLAMP_TO_BORDER), [`GL_MIRRORED_REPEAT`](crate::gl_enums::GL_MIRRORED_REPEAT),
/// > [`GL_REPEAT`](crate::gl_enums::GL_REPEAT), or [`GL_MIRROR_CLAMP_TO_EDGE`](crate::gl_enums::GL_MIRROR_CLAMP_TO_EDGE).
/// > [`GL_CLAMP_TO_EDGE`](crate::gl_enums::GL_CLAMP_TO_EDGE) causes `[inlineq]`
/// > `[inlineq]` `[inlineq]` [`GL_CLAMP_TO_BORDER`](crate::gl_enums::GL_CLAMP_TO_BORDER)
/// > evaluates `[inlineq]` [`GL_CLAMP_TO_EDGE`](crate::gl_enums::GL_CLAMP_TO_EDGE).
/// > However, in cases where clamping would have occurred in [`GL_CLAMP_TO_EDGE`](crate::gl_enums::GL_CLAMP_TO_EDGE)
/// > mode, the fetched texel data is substituted with the values specified by
/// > [`GL_TEXTURE_BORDER_COLOR`](crate::gl_enums::GL_TEXTURE_BORDER_COLOR). [`GL_REPEAT`](crate::gl_enums::GL_REPEAT)
/// > causes the integer part of the `[inlineq]` [`GL_MIRRORED_REPEAT`](crate::gl_enums::GL_MIRRORED_REPEAT)
/// > causes the `[inlineq]` `[inlineq]` `[inlineq]` `[inlineq]` `[inlineq]`
/// > `[inlineq]` `[inlineq]` [`GL_MIRROR_CLAMP_TO_EDGE`](crate::gl_enums::GL_MIRROR_CLAMP_TO_EDGE)
/// > causes the `[inlineq]` [`GL_MIRRORED_REPEAT`](crate::gl_enums::GL_MIRRORED_REPEAT)
/// > for one repetition of the texture, at which point the coordinate to be
/// > clamped as in [`GL_CLAMP_TO_EDGE`](crate::gl_enums::GL_CLAMP_TO_EDGE). Initially,
/// > [`GL_TEXTURE_WRAP_S`](crate::gl_enums::GL_TEXTURE_WRAP_S) is set to [`GL_REPEAT`](crate::gl_enums::GL_REPEAT).
///
///
/// [`GL_TEXTURE_WRAP_T`](crate::gl_enums::GL_TEXTURE_WRAP_T)
///
/// > Sets the wrap parameter for texture coordinate `[inlineq]` [`GL_CLAMP_TO_EDGE`](crate::gl_enums::GL_CLAMP_TO_EDGE),
/// > [`GL_CLAMP_TO_BORDER`](crate::gl_enums::GL_CLAMP_TO_BORDER), [`GL_MIRRORED_REPEAT`](crate::gl_enums::GL_MIRRORED_REPEAT),
/// > [`GL_REPEAT`](crate::gl_enums::GL_REPEAT), or [`GL_MIRROR_CLAMP_TO_EDGE`](crate::gl_enums::GL_MIRROR_CLAMP_TO_EDGE).
/// > See the discussion under [`GL_TEXTURE_WRAP_S`](crate::gl_enums::GL_TEXTURE_WRAP_S).
/// > Initially, [`GL_TEXTURE_WRAP_T`](crate::gl_enums::GL_TEXTURE_WRAP_T) is
/// > set to [`GL_REPEAT`](crate::gl_enums::GL_REPEAT).
///
///
/// [`GL_TEXTURE_WRAP_R`](crate::gl_enums::GL_TEXTURE_WRAP_R)
///
/// > Sets the wrap parameter for texture coordinate `[inlineq]` [`GL_CLAMP_TO_EDGE`](crate::gl_enums::GL_CLAMP_TO_EDGE),
/// > [`GL_CLAMP_TO_BORDER`](crate::gl_enums::GL_CLAMP_TO_BORDER), [`GL_MIRRORED_REPEAT`](crate::gl_enums::GL_MIRRORED_REPEAT),
/// > [`GL_REPEAT`](crate::gl_enums::GL_REPEAT), or [`GL_MIRROR_CLAMP_TO_EDGE`](crate::gl_enums::GL_MIRROR_CLAMP_TO_EDGE).
/// > See the discussion under [`GL_TEXTURE_WRAP_S`](crate::gl_enums::GL_TEXTURE_WRAP_S).
/// > Initially, [`GL_TEXTURE_WRAP_R`](crate::gl_enums::GL_TEXTURE_WRAP_R) is
/// > set to [`GL_REPEAT`](crate::gl_enums::GL_REPEAT).
///
/// ### Notes
/// Suppose that a program attempts to sample from a texture and has set [`GL_TEXTURE_MIN_FILTER`](crate::gl_enums::GL_TEXTURE_MIN_FILTER)
/// to one of the functions that requires a mipmap. If either the dimensions
/// of the texture images currently defined (with previous calls to [**glTexImage1D**](crate::context::Context::oxidegl_tex_image1_d),
/// [**glTexImage2D**](crate::context::Context::oxidegl_tex_image2_d), [**glTexImage3D**](crate::context::Context::oxidegl_tex_image3_d),
/// [**glCopyTexImage1D**](crate::context::Context::oxidegl_copy_tex_image1_d),
/// or [**glCopyTexImage2D**](crate::context::Context::oxidegl_copy_tex_image2_d))
/// do not follow the proper sequence for mipmaps (described above), or there
/// are fewer texture images defined than are needed, or the set of texture
/// images have differing numbers of texture components, then the texture is
/// considered *incomplete*.
///
/// Linear filtering accesses the four nearest texture elements only in 2D
/// textures. In 1D textures, linear filtering accesses the two nearest texture
/// elements. In 3D textures, linear filtering accesses the eight nearest texture
/// elements.
///
/// [**glTexParameter**](crate::context::Context::oxidegl_tex_parameter) specifies
/// the texture parameters for the active texture unit, specified by calling
/// [**glActiveTexture**](crate::context::Context::oxidegl_active_texture).
/// [**glTextureParameter**](crate::context::Context::oxidegl_texture_parameter)
/// specifies the texture parameters for the texture object with ID `texture`.
///
/// [`GL_DEPTH_STENCIL_TEXTURE_MODE`](crate::gl_enums::GL_DEPTH_STENCIL_TEXTURE_MODE)
/// is available only if the GL version is 4.3 or greater.
///
/// [`GL_MIRROR_CLAMP_TO_EDGE`](crate::gl_enums::GL_MIRROR_CLAMP_TO_EDGE) is
/// available only if the GL version is 4.4 or greater.
///
/// ### Associated Gets
/// [**glGetTexParameter**](crate::context::Context::oxidegl_get_tex_parameter)
///
/// [**glGetTextureParameter**](crate::context::Context::oxidegl_get_texture_parameter)
///
/// [**glGetTexLevelParameter**](crate::context::Context::oxidegl_get_tex_level_parameter)
///
/// [**glGetTextureLevelParameter**](crate::context::Context::oxidegl_get_texture_level_parameter)
impl Context {
    pub fn oxidegl_tex_parameterf(
        &mut self,
        target: TextureTarget,
        pname: TextureParameterName,
        param: GLfloat,
    ) -> GlFallible {
        panic!("command oxidegl_tex_parameterf not yet implemented");
    }
    pub unsafe fn oxidegl_tex_parameterfv(
        &mut self,
        target: TextureTarget,
        pname: TextureParameterName,
        params: *const GLfloat,
    ) -> GlFallible {
        panic!("command oxidegl_tex_parameterfv not yet implemented");
    }
    pub fn oxidegl_tex_parameteri(
        &mut self,
        target: TextureTarget,
        pname: TextureParameterName,
        param: GLint,
    ) -> GlFallible {
        panic!("command oxidegl_tex_parameteri not yet implemented");
    }
    pub unsafe fn oxidegl_tex_parameteriv(
        &mut self,
        target: TextureTarget,
        pname: TextureParameterName,
        params: *const GLint,
    ) -> GlFallible {
        panic!("command oxidegl_tex_parameteriv not yet implemented");
    }
    pub unsafe fn oxidegl_tex_parameter_iiv(
        &mut self,
        target: TextureTarget,
        pname: TextureParameterName,
        params: *const GLint,
    ) -> GlFallible {
        panic!("command oxidegl_tex_parameter_iiv not yet implemented");
    }
    pub unsafe fn oxidegl_tex_parameter_iuiv(
        &mut self,
        target: TextureTarget,
        pname: TextureParameterName,
        params: *const GLuint,
    ) -> GlFallible {
        panic!("command oxidegl_tex_parameter_iuiv not yet implemented");
    }
    pub fn oxidegl_texture_parameterf(
        &mut self,
        texture: GLuint,
        pname: TextureParameterName,
        param: GLfloat,
    ) -> GlFallible {
        self.gl_state
            .texture_list
            .get_raw_mut(texture)?
            .tex_param(pname, param)
    }
    pub unsafe fn oxidegl_texture_parameterfv(
        &mut self,
        texture: GLuint,
        pname: TextureParameterName,
        param: *const GLfloat,
    ) -> GlFallible {
        unsafe {
            self.gl_state
                .texture_list
                .get_raw_mut(texture)?
                .tex_param_i(pname, param)
        }
    }
    pub fn oxidegl_texture_parameteri(
        &mut self,
        texture: GLuint,
        pname: TextureParameterName,
        param: GLint,
    ) -> GlFallible {
        self.gl_state
            .texture_list
            .get_raw_mut(texture)?
            .tex_param(pname, param)
    }
    pub unsafe fn oxidegl_texture_parameter_iiv(
        &mut self,
        texture: GLuint,
        pname: TextureParameterName,
        params: *const GLint,
    ) -> GlFallible {
        unsafe {
            self.gl_state
                .texture_list
                .get_raw_mut(texture)?
                .tex_param_i(pname, params)
        }
    }
    pub unsafe fn oxidegl_texture_parameter_iuiv(
        &mut self,
        texture: GLuint,
        pname: TextureParameterName,
        params: *const GLuint,
    ) -> GlFallible {
        unsafe {
            self.gl_state
                .texture_list
                .get_raw_mut(texture)?
                .tex_param_i(pname, params)
        }
    }
    pub unsafe fn oxidegl_texture_parameteriv(
        &mut self,
        texture: GLuint,
        pname: TextureParameterName,
        param: *const GLint,
    ) -> GlFallible {
        unsafe {
            self.gl_state
                .texture_list
                .get_raw_mut(texture)?
                .tex_param_i(pname, param)
        }
    }
}
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the texture object is bound for [**glTexStorage1D**](crate::context::Context::oxidegl_tex_storage1_d).
/// > Must be one of [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D) or [`GL_PROXY_TEXTURE_1D`](crate::gl_enums::GL_PROXY_TEXTURE_1D).
///
/// `texture`
///
/// > Specifies the texture object name for [**glTextureStorage1D**](crate::context::Context::oxidegl_texture_storage1_d).
/// > The effective target of `texture` must be one of the valid non-proxy `target`
/// > values above.
///
/// `levels`
///
/// > Specify the number of texture levels.
///
/// `internalformat`
///
/// > Specifies the sized internal format to be used to store texture image data.
///
/// `width`
///
/// > Specifies the width of the texture, in texels.
///
/// ### Description
/// [**glTexStorage1D**](crate::context::Context::oxidegl_tex_storage1_d) and
/// [**glTextureStorage1D**](crate::context::Context::oxidegl_texture_storage1_d)
/// specify the storage requirements for all levels of a one-dimensional texture
/// simultaneously. Once a texture is specified with this command, the format
/// and dimensions of all levels become immutable unless it is a proxy texture.
/// The contents of the image may still be modified, however, its storage requirements
/// may not change. Such a texture is referred to as an *immutable-format*
/// texture.
///
/// Calling [**glTexStorage1D**](crate::context::Context::oxidegl_tex_storage1_d)
/// is equivalent, assuming no errors are generated, to executing the following
/// pseudo-code:
///
/// Calling [**glTextureStorage1D**](crate::context::Context::oxidegl_texture_storage1_d)
/// is equivalent to the above pseudo-code, where `target` is the effective
/// target of `texture` and it is as if `texture` were bound to `target` for
/// the purposes of [**glTexImage1D**](crate::context::Context::oxidegl_tex_image1_d).
///
/// Since no texture data is actually provided, the values used in the pseudo-code
/// for `format` and `type` are irrelevant and may be considered to be any
/// values that are legal for the chosen `internalformat` enumerant. `internalformat`
/// must be one of the sized internal formats given in Table 1 below, one of
/// the sized depth-component formats [`GL_DEPTH_COMPONENT32F`](crate::gl_enums::GL_DEPTH_COMPONENT32F),
/// [`GL_DEPTH_COMPONENT24`](crate::gl_enums::GL_DEPTH_COMPONENT24), or [`GL_DEPTH_COMPONENT16`](crate::gl_enums::GL_DEPTH_COMPONENT16),
/// one of the combined depth-stencil formats, [`GL_DEPTH32F_STENCIL8`](crate::gl_enums::GL_DEPTH32F_STENCIL8),
/// or [`GL_DEPTH24_STENCIL8`](crate::gl_enums::GL_DEPTH24_STENCIL8), or the
/// stencil-only format, [`GL_STENCIL_INDEX8`](crate::gl_enums::GL_STENCIL_INDEX8).
/// Upon success, the value of [`GL_TEXTURE_IMMUTABLE_FORMAT`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_FORMAT)
/// becomes [`GL_TRUE`](crate::gl_enums::GL_TRUE). The value of [`GL_TEXTURE_IMMUTABLE_FORMAT`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_FORMAT)
/// may be discovered by calling [**glGetTexParameter**](crate::context::Context::oxidegl_get_tex_parameter)
/// with `pname` set to [`GL_TEXTURE_IMMUTABLE_FORMAT`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_FORMAT).
/// No further changes to the dimensions or format of the texture object may
/// be made. Using any command that might alter the dimensions or format of
/// the texture object (such as [**glTexImage1D**](crate::context::Context::oxidegl_tex_image1_d)
/// or another call to [**glTexStorage1D**](crate::context::Context::oxidegl_tex_storage1_d))
/// will result in the generation of a [`GL_INVALID_OPERATION`](crate::gl_enums::GL_INVALID_OPERATION)
/// error, even if it would not, in fact, alter the dimensions or format of
/// the object.
///
/// | *Sized Internal Format*                               | *Base Internal Format*                               | *Red Bits*       | *Green Bits*        | *Blue Bits*       | *Alpha Bits*        | *Shared Bits*        |
/// |-------------------------------------------------------|------------------------------------------------------|------------------|---------------------|-------------------|---------------------|----------------------|
/// | [`GL_R8`](crate::gl_enums::GL_R8)                     | [`GL_RED`](crate::gl_enums::GL_RED)                  | 8                |                     |                   |                     |                      |
/// | [`GL_R8_SNORM`](crate::gl_enums::GL_R8_SNORM)         | [`GL_RED`](crate::gl_enums::GL_RED)                  | s8               |                     |                   |                     |                      |
/// | [`GL_R16`](crate::gl_enums::GL_R16)                   | [`GL_RED`](crate::gl_enums::GL_RED)                  | 16               |                     |                   |                     |                      |
/// | [`GL_R16_SNORM`](crate::gl_enums::GL_R16_SNORM)       | [`GL_RED`](crate::gl_enums::GL_RED)                  | s16              |                     |                   |                     |                      |
/// | [`GL_RG8`](crate::gl_enums::GL_RG8)                   | [`GL_RG`](crate::gl_enums::GL_RG)                    | 8                | 8                   |                   |                     |                      |
/// | [`GL_RG8_SNORM`](crate::gl_enums::GL_RG8_SNORM)       | [`GL_RG`](crate::gl_enums::GL_RG)                    | s8               | s8                  |                   |                     |                      |
/// | [`GL_RG16`](crate::gl_enums::GL_RG16)                 | [`GL_RG`](crate::gl_enums::GL_RG)                    | 16               | 16                  |                   |                     |                      |
/// | [`GL_RG16_SNORM`](crate::gl_enums::GL_RG16_SNORM)     | [`GL_RG`](crate::gl_enums::GL_RG)                    | s16              | s16                 |                   |                     |                      |
/// | [`GL_R3_G3_B2`](crate::gl_enums::GL_R3_G3_B2)         | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 3                | 3                   | 2                 |                     |                      |
/// | [`GL_RGB4`](crate::gl_enums::GL_RGB4)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 4                | 4                   | 4                 |                     |                      |
/// | [`GL_RGB5`](crate::gl_enums::GL_RGB5)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 5                | 5                   | 5                 |                     |                      |
/// | [`GL_RGB8`](crate::gl_enums::GL_RGB8)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 8                | 8                   | 8                 |                     |                      |
/// | [`GL_RGB8_SNORM`](crate::gl_enums::GL_RGB8_SNORM)     | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | s8               | s8                  | s8                |                     |                      |
/// | [`GL_RGB10`](crate::gl_enums::GL_RGB10)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 10               | 10                  | 10                |                     |                      |
/// | [`GL_RGB12`](crate::gl_enums::GL_RGB12)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 12               | 12                  | 12                |                     |                      |
/// | [`GL_RGB16_SNORM`](crate::gl_enums::GL_RGB16_SNORM)   | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 16               | 16                  | 16                |                     |                      |
/// | [`GL_RGBA2`](crate::gl_enums::GL_RGBA2)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 2                | 2                   | 2                 | 2                   |                      |
/// | [`GL_RGBA4`](crate::gl_enums::GL_RGBA4)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 4                | 4                   | 4                 | 4                   |                      |
/// | [`GL_RGB5_A1`](crate::gl_enums::GL_RGB5_A1)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 5                | 5                   | 5                 | 1                   |                      |
/// | [`GL_RGBA8`](crate::gl_enums::GL_RGBA8)               | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 8                | 8                   | 8                 | 8                   |                      |
/// | [`GL_RGBA8_SNORM`](crate::gl_enums::GL_RGBA8_SNORM)   | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | s8               | s8                  | s8                | s8                  |                      |
/// | [`GL_RGB10_A2`](crate::gl_enums::GL_RGB10_A2)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 10               | 10                  | 10                | 2                   |                      |
/// | [`GL_RGB10_A2UI`](crate::gl_enums::GL_RGB10_A2UI)     | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui10             | ui10                | ui10              | ui2                 |                      |
/// | [`GL_RGBA12`](crate::gl_enums::GL_RGBA12)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 12               | 12                  | 12                | 12                  |                      |
/// | [`GL_RGBA16`](crate::gl_enums::GL_RGBA16)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 16               | 16                  | 16                | 16                  |                      |
/// | [`GL_SRGB8`](crate::gl_enums::GL_SRGB8)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 8                | 8                   | 8                 |                     |                      |
/// | [`GL_SRGB8_ALPHA8`](crate::gl_enums::GL_SRGB8_ALPHA8) | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 8                | 8                   | 8                 | 8                   |                      |
/// | [`GL_R16F`](crate::gl_enums::GL_R16F)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | f16              |                     |                   |                     |                      |
/// | [`GL_RG16F`](crate::gl_enums::GL_RG16F)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | f16              | f16                 |                   |                     |                      |
/// | [`GL_RGB16F`](crate::gl_enums::GL_RGB16F)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | f16              | f16                 | f16               |                     |                      |
/// | [`GL_RGBA16F`](crate::gl_enums::GL_RGBA16F)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | f16              | f16                 | f16               | f16                 |                      |
/// | [`GL_R32F`](crate::gl_enums::GL_R32F)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | f32              |                     |                   |                     |                      |
/// | [`GL_RG32F`](crate::gl_enums::GL_RG32F)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | f32              | f32                 |                   |                     |                      |
/// | [`GL_RGB32F`](crate::gl_enums::GL_RGB32F)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | f32              | f32                 | f32               |                     |                      |
/// | [`GL_RGBA32F`](crate::gl_enums::GL_RGBA32F)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | f32              | f32                 | f32               | f32                 |                      |
/// | [`GL_R11F_G11F_B10F`](crate::gl_enums::GL_R11F_G11F_B10F) | [`GL_RGB`](crate::gl_enums::GL_RGB)              | f11              | f11                 | f10               |                     |                      |
/// | [`GL_RGB9_E5`](crate::gl_enums::GL_RGB9_E5)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 9                | 9                   | 9                 |                     | 5                    |
/// | [`GL_R8I`](crate::gl_enums::GL_R8I)                   | [`GL_RED`](crate::gl_enums::GL_RED)                  | i8               |                     |                   |                     |                      |
/// | [`GL_R8UI`](crate::gl_enums::GL_R8UI)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui8              |                     |                   |                     |                      |
/// | [`GL_R16I`](crate::gl_enums::GL_R16I)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | i16              |                     |                   |                     |                      |
/// | [`GL_R16UI`](crate::gl_enums::GL_R16UI)               | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui16             |                     |                   |                     |                      |
/// | [`GL_R32I`](crate::gl_enums::GL_R32I)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | i32              |                     |                   |                     |                      |
/// | [`GL_R32UI`](crate::gl_enums::GL_R32UI)               | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui32             |                     |                   |                     |                      |
/// | [`GL_RG8I`](crate::gl_enums::GL_RG8I)                 | [`GL_RG`](crate::gl_enums::GL_RG)                    | i8               | i8                  |                   |                     |                      |
/// | [`GL_RG8UI`](crate::gl_enums::GL_RG8UI)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui8              | ui8                 |                   |                     |                      |
/// | [`GL_RG16I`](crate::gl_enums::GL_RG16I)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | i16              | i16                 |                   |                     |                      |
/// | [`GL_RG16UI`](crate::gl_enums::GL_RG16UI)             | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui16             | ui16                |                   |                     |                      |
/// | [`GL_RG32I`](crate::gl_enums::GL_RG32I)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | i32              | i32                 |                   |                     |                      |
/// | [`GL_RG32UI`](crate::gl_enums::GL_RG32UI)             | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui32             | ui32                |                   |                     |                      |
/// | [`GL_RGB8I`](crate::gl_enums::GL_RGB8I)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i8               | i8                  | i8                |                     |                      |
/// | [`GL_RGB8UI`](crate::gl_enums::GL_RGB8UI)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui8              | ui8                 | ui8               |                     |                      |
/// | [`GL_RGB16I`](crate::gl_enums::GL_RGB16I)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i16              | i16                 | i16               |                     |                      |
/// | [`GL_RGB16UI`](crate::gl_enums::GL_RGB16UI)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui16             | ui16                | ui16              |                     |                      |
/// | [`GL_RGB32I`](crate::gl_enums::GL_RGB32I)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i32              | i32                 | i32               |                     |                      |
/// | [`GL_RGB32UI`](crate::gl_enums::GL_RGB32UI)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui32             | ui32                | ui32              |                     |                      |
/// | [`GL_RGBA8I`](crate::gl_enums::GL_RGBA8I)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i8               | i8                  | i8                | i8                  |                      |
/// | [`GL_RGBA8UI`](crate::gl_enums::GL_RGBA8UI)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui8              | ui8                 | ui8               | ui8                 |                      |
/// | [`GL_RGBA16I`](crate::gl_enums::GL_RGBA16I)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i16              | i16                 | i16               | i16                 |                      |
/// | [`GL_RGBA16UI`](crate::gl_enums::GL_RGBA16UI)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui16             | ui16                | ui16              | ui16                |                      |
/// | [`GL_RGBA32I`](crate::gl_enums::GL_RGBA32I)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i32              | i32                 | i32               | i32                 |                      |
/// | [`GL_RGBA32UI`](crate::gl_enums::GL_RGBA32UI)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui32             | ui32                | ui32              | ui32                |                      |
///
///
/// ### Notes
/// [`GL_STENCIL_INDEX8`](crate::gl_enums::GL_STENCIL_INDEX8) is accepted for
/// `internalformat` only if the GL version is 4.4 or higher.
impl Context {
    pub fn oxidegl_tex_storage_1_d(
        &mut self,
        target: TextureTarget,
        levels: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
    ) -> GlFallible {
        panic!("command oxidegl_tex_storage1_d not yet implemented");
    }
    pub fn oxidegl_texture_storage_1_d(
        &mut self,
        texture: GLuint,
        levels: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
    ) -> GlFallible {
        panic!("command oxidegl_texture_storage1_d not yet implemented");
    }
}
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the texture object is bound for [**glTexStorage2D**](crate::context::Context::oxidegl_tex_storage2_d).
/// > Must be one of [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D), [`GL_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_1D_ARRAY),
/// > [`GL_TEXTURE_RECTANGLE`](crate::gl_enums::GL_TEXTURE_RECTANGLE), [`GL_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_CUBE_MAP),
/// > [`GL_PROXY_TEXTURE_2D`](crate::gl_enums::GL_PROXY_TEXTURE_2D), [`GL_PROXY_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_PROXY_TEXTURE_1D_ARRAY),
/// > [`GL_PROXY_TEXTURE_RECTANGLE`](crate::gl_enums::GL_PROXY_TEXTURE_RECTANGLE),
/// > or [`GL_PROXY_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_PROXY_TEXTURE_CUBE_MAP).
///
/// `texture`
///
/// > Specifies the texture object name for [**glTextureStorage2D**](crate::context::Context::oxidegl_texture_storage2_d).
/// > The effective target of `texture` must be one of the valid non-proxy `target`
/// > values above.
///
/// `levels`
///
/// > Specify the number of texture levels.
///
/// `internalformat`
///
/// > Specifies the sized internal format to be used to store texture image data.
///
/// `width`
///
/// > Specifies the width of the texture, in texels.
///
/// `height`
///
/// > Specifies the height of the texture, in texels.
///
/// ### Description
/// [**glTexStorage2D**](crate::context::Context::oxidegl_tex_storage2_d) and
/// [**glTextureStorage2D**](crate::context::Context::oxidegl_texture_storage2_d)
/// specify the storage requirements for all levels of a two-dimensional texture
/// or one-dimensional texture array simultaneously. Once a texture is specified
/// with this command, the format and dimensions of all levels become immutable
/// unless it is a proxy texture. The contents of the image may still be modified,
/// however, its storage requirements may not change. Such a texture is referred
/// to as an *immutable-format* texture.
///
/// The behavior of [**glTexStorage2D**](crate::context::Context::oxidegl_tex_storage2_d)
/// depends on the `target` parameter. When `target` is [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D),
/// [`GL_PROXY_TEXTURE_2D`](crate::gl_enums::GL_PROXY_TEXTURE_2D), [`GL_TEXTURE_RECTANGLE`](crate::gl_enums::GL_TEXTURE_RECTANGLE),
/// [`GL_PROXY_TEXTURE_RECTANGLE`](crate::gl_enums::GL_PROXY_TEXTURE_RECTANGLE)
/// or [`GL_PROXY_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_PROXY_TEXTURE_CUBE_MAP),
/// calling [**glTexStorage2D**](crate::context::Context::oxidegl_tex_storage2_d)
/// is equivalent, assuming no errors are generated, to executing the following
/// pseudo-code:
///
/// When `target` is [`GL_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_CUBE_MAP),
/// [**glTexStorage2D**](crate::context::Context::oxidegl_tex_storage2_d) is
/// equivalent to:
///
/// When `target` is [`GL_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_1D_ARRAY)
/// or [`GL_PROXY_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_PROXY_TEXTURE_1D_ARRAY),
/// [**glTexStorage2D**](crate::context::Context::oxidegl_tex_storage2_d) is
/// equivalent to:
///
/// Calling [**glTextureStorage2D**](crate::context::Context::oxidegl_texture_storage2_d)
/// is equivalent to the above pseudo-code, where `target` is the effective
/// target of `texture` and it is as if `texture` were bound to `target` for
/// the purposes of [**glTexImage2D**](crate::context::Context::oxidegl_tex_image2_d).
///
/// Since no texture data is actually provided, the values used in the pseudo-code
/// for `format` and `type` are irrelevant and may be considered to be any
/// values that are legal for the chosen `internalformat` enumerant. `internalformat`
/// must be one of the sized internal formats given in Table 1 below, one of
/// the sized depth-component formats [`GL_DEPTH_COMPONENT32F`](crate::gl_enums::GL_DEPTH_COMPONENT32F),
/// [`GL_DEPTH_COMPONENT24`](crate::gl_enums::GL_DEPTH_COMPONENT24), or [`GL_DEPTH_COMPONENT16`](crate::gl_enums::GL_DEPTH_COMPONENT16),
/// one of the combined depth-stencil formats, [`GL_DEPTH32F_STENCIL8`](crate::gl_enums::GL_DEPTH32F_STENCIL8),
/// or [`GL_DEPTH24_STENCIL8`](crate::gl_enums::GL_DEPTH24_STENCIL8), or the
/// stencil-only format, [`GL_STENCIL_INDEX8`](crate::gl_enums::GL_STENCIL_INDEX8).
/// Upon success, the value of [`GL_TEXTURE_IMMUTABLE_FORMAT`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_FORMAT)
/// becomes [`GL_TRUE`](crate::gl_enums::GL_TRUE). The value of [`GL_TEXTURE_IMMUTABLE_FORMAT`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_FORMAT)
/// may be discovered by calling [**glGetTexParameter**](crate::context::Context::oxidegl_get_tex_parameter)
/// with `pname` set to [`GL_TEXTURE_IMMUTABLE_FORMAT`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_FORMAT).
/// No further changes to the dimensions or format of the texture object may
/// be made. Using any command that might alter the dimensions or format of
/// the texture object (such as [**glTexImage2D**](crate::context::Context::oxidegl_tex_image2_d)
/// or another call to [**glTexStorage2D**](crate::context::Context::oxidegl_tex_storage2_d))
/// will result in the generation of a [`GL_INVALID_OPERATION`](crate::gl_enums::GL_INVALID_OPERATION)
/// error, even if it would not, in fact, alter the dimensions or format of
/// the object.
///
/// | *Sized Internal Format*                               | *Base Internal Format*                               | *Red Bits*       | *Green Bits*        | *Blue Bits*       | *Alpha Bits*        | *Shared Bits*        |
/// |-------------------------------------------------------|------------------------------------------------------|------------------|---------------------|-------------------|---------------------|----------------------|
/// | [`GL_R8`](crate::gl_enums::GL_R8)                     | [`GL_RED`](crate::gl_enums::GL_RED)                  | 8                |                     |                   |                     |                      |
/// | [`GL_R8_SNORM`](crate::gl_enums::GL_R8_SNORM)         | [`GL_RED`](crate::gl_enums::GL_RED)                  | s8               |                     |                   |                     |                      |
/// | [`GL_R16`](crate::gl_enums::GL_R16)                   | [`GL_RED`](crate::gl_enums::GL_RED)                  | 16               |                     |                   |                     |                      |
/// | [`GL_R16_SNORM`](crate::gl_enums::GL_R16_SNORM)       | [`GL_RED`](crate::gl_enums::GL_RED)                  | s16              |                     |                   |                     |                      |
/// | [`GL_RG8`](crate::gl_enums::GL_RG8)                   | [`GL_RG`](crate::gl_enums::GL_RG)                    | 8                | 8                   |                   |                     |                      |
/// | [`GL_RG8_SNORM`](crate::gl_enums::GL_RG8_SNORM)       | [`GL_RG`](crate::gl_enums::GL_RG)                    | s8               | s8                  |                   |                     |                      |
/// | [`GL_RG16`](crate::gl_enums::GL_RG16)                 | [`GL_RG`](crate::gl_enums::GL_RG)                    | 16               | 16                  |                   |                     |                      |
/// | [`GL_RG16_SNORM`](crate::gl_enums::GL_RG16_SNORM)     | [`GL_RG`](crate::gl_enums::GL_RG)                    | s16              | s16                 |                   |                     |                      |
/// | [`GL_R3_G3_B2`](crate::gl_enums::GL_R3_G3_B2)         | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 3                | 3                   | 2                 |                     |                      |
/// | [`GL_RGB4`](crate::gl_enums::GL_RGB4)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 4                | 4                   | 4                 |                     |                      |
/// | [`GL_RGB5`](crate::gl_enums::GL_RGB5)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 5                | 5                   | 5                 |                     |                      |
/// | [`GL_RGB8`](crate::gl_enums::GL_RGB8)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 8                | 8                   | 8                 |                     |                      |
/// | [`GL_RGB8_SNORM`](crate::gl_enums::GL_RGB8_SNORM)     | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | s8               | s8                  | s8                |                     |                      |
/// | [`GL_RGB10`](crate::gl_enums::GL_RGB10)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 10               | 10                  | 10                |                     |                      |
/// | [`GL_RGB12`](crate::gl_enums::GL_RGB12)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 12               | 12                  | 12                |                     |                      |
/// | [`GL_RGB16_SNORM`](crate::gl_enums::GL_RGB16_SNORM)   | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 16               | 16                  | 16                |                     |                      |
/// | [`GL_RGBA2`](crate::gl_enums::GL_RGBA2)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 2                | 2                   | 2                 | 2                   |                      |
/// | [`GL_RGBA4`](crate::gl_enums::GL_RGBA4)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 4                | 4                   | 4                 | 4                   |                      |
/// | [`GL_RGB5_A1`](crate::gl_enums::GL_RGB5_A1)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 5                | 5                   | 5                 | 1                   |                      |
/// | [`GL_RGBA8`](crate::gl_enums::GL_RGBA8)               | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 8                | 8                   | 8                 | 8                   |                      |
/// | [`GL_RGBA8_SNORM`](crate::gl_enums::GL_RGBA8_SNORM)   | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | s8               | s8                  | s8                | s8                  |                      |
/// | [`GL_RGB10_A2`](crate::gl_enums::GL_RGB10_A2)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 10               | 10                  | 10                | 2                   |                      |
/// | [`GL_RGB10_A2UI`](crate::gl_enums::GL_RGB10_A2UI)     | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui10             | ui10                | ui10              | ui2                 |                      |
/// | [`GL_RGBA12`](crate::gl_enums::GL_RGBA12)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 12               | 12                  | 12                | 12                  |                      |
/// | [`GL_RGBA16`](crate::gl_enums::GL_RGBA16)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 16               | 16                  | 16                | 16                  |                      |
/// | [`GL_SRGB8`](crate::gl_enums::GL_SRGB8)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 8                | 8                   | 8                 |                     |                      |
/// | [`GL_SRGB8_ALPHA8`](crate::gl_enums::GL_SRGB8_ALPHA8) | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 8                | 8                   | 8                 | 8                   |                      |
/// | [`GL_R16F`](crate::gl_enums::GL_R16F)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | f16              |                     |                   |                     |                      |
/// | [`GL_RG16F`](crate::gl_enums::GL_RG16F)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | f16              | f16                 |                   |                     |                      |
/// | [`GL_RGB16F`](crate::gl_enums::GL_RGB16F)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | f16              | f16                 | f16               |                     |                      |
/// | [`GL_RGBA16F`](crate::gl_enums::GL_RGBA16F)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | f16              | f16                 | f16               | f16                 |                      |
/// | [`GL_R32F`](crate::gl_enums::GL_R32F)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | f32              |                     |                   |                     |                      |
/// | [`GL_RG32F`](crate::gl_enums::GL_RG32F)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | f32              | f32                 |                   |                     |                      |
/// | [`GL_RGB32F`](crate::gl_enums::GL_RGB32F)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | f32              | f32                 | f32               |                     |                      |
/// | [`GL_RGBA32F`](crate::gl_enums::GL_RGBA32F)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | f32              | f32                 | f32               | f32                 |                      |
/// | [`GL_R11F_G11F_B10F`](crate::gl_enums::GL_R11F_G11F_B10F) | [`GL_RGB`](crate::gl_enums::GL_RGB)              | f11              | f11                 | f10               |                     |                      |
/// | [`GL_RGB9_E5`](crate::gl_enums::GL_RGB9_E5)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 9                | 9                   | 9                 |                     | 5                    |
/// | [`GL_R8I`](crate::gl_enums::GL_R8I)                   | [`GL_RED`](crate::gl_enums::GL_RED)                  | i8               |                     |                   |                     |                      |
/// | [`GL_R8UI`](crate::gl_enums::GL_R8UI)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui8              |                     |                   |                     |                      |
/// | [`GL_R16I`](crate::gl_enums::GL_R16I)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | i16              |                     |                   |                     |                      |
/// | [`GL_R16UI`](crate::gl_enums::GL_R16UI)               | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui16             |                     |                   |                     |                      |
/// | [`GL_R32I`](crate::gl_enums::GL_R32I)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | i32              |                     |                   |                     |                      |
/// | [`GL_R32UI`](crate::gl_enums::GL_R32UI)               | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui32             |                     |                   |                     |                      |
/// | [`GL_RG8I`](crate::gl_enums::GL_RG8I)                 | [`GL_RG`](crate::gl_enums::GL_RG)                    | i8               | i8                  |                   |                     |                      |
/// | [`GL_RG8UI`](crate::gl_enums::GL_RG8UI)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui8              | ui8                 |                   |                     |                      |
/// | [`GL_RG16I`](crate::gl_enums::GL_RG16I)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | i16              | i16                 |                   |                     |                      |
/// | [`GL_RG16UI`](crate::gl_enums::GL_RG16UI)             | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui16             | ui16                |                   |                     |                      |
/// | [`GL_RG32I`](crate::gl_enums::GL_RG32I)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | i32              | i32                 |                   |                     |                      |
/// | [`GL_RG32UI`](crate::gl_enums::GL_RG32UI)             | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui32             | ui32                |                   |                     |                      |
/// | [`GL_RGB8I`](crate::gl_enums::GL_RGB8I)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i8               | i8                  | i8                |                     |                      |
/// | [`GL_RGB8UI`](crate::gl_enums::GL_RGB8UI)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui8              | ui8                 | ui8               |                     |                      |
/// | [`GL_RGB16I`](crate::gl_enums::GL_RGB16I)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i16              | i16                 | i16               |                     |                      |
/// | [`GL_RGB16UI`](crate::gl_enums::GL_RGB16UI)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui16             | ui16                | ui16              |                     |                      |
/// | [`GL_RGB32I`](crate::gl_enums::GL_RGB32I)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i32              | i32                 | i32               |                     |                      |
/// | [`GL_RGB32UI`](crate::gl_enums::GL_RGB32UI)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui32             | ui32                | ui32              |                     |                      |
/// | [`GL_RGBA8I`](crate::gl_enums::GL_RGBA8I)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i8               | i8                  | i8                | i8                  |                      |
/// | [`GL_RGBA8UI`](crate::gl_enums::GL_RGBA8UI)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui8              | ui8                 | ui8               | ui8                 |                      |
/// | [`GL_RGBA16I`](crate::gl_enums::GL_RGBA16I)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i16              | i16                 | i16               | i16                 |                      |
/// | [`GL_RGBA16UI`](crate::gl_enums::GL_RGBA16UI)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui16             | ui16                | ui16              | ui16                |                      |
/// | [`GL_RGBA32I`](crate::gl_enums::GL_RGBA32I)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i32              | i32                 | i32               | i32                 |                      |
/// | [`GL_RGBA32UI`](crate::gl_enums::GL_RGBA32UI)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui32             | ui32                | ui32              | ui32                |                      |
///
///
/// ### Notes
/// [`GL_STENCIL_INDEX8`](crate::gl_enums::GL_STENCIL_INDEX8) is accepted for
/// `internalformat` only if the GL version is 4.4 or higher.
impl Context {
    pub fn oxidegl_tex_storage_2_d(
        &mut self,
        target: TextureTarget,
        levels: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
        height: GLsizei,
    ) -> GlFallible {
        panic!("command oxidegl_tex_storage2_d not yet implemented");
    }
    pub fn oxidegl_texture_storage_2_d(
        &mut self,
        texture: GLuint,
        levels: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
        height: GLsizei,
    ) -> GlFallible {
        let tex = self.gl_state.texture_list.get_raw_mut(texture)?;
        sizei!(levels, width, height);
        let (Some(w), Some(h)) = (NonZeroU32::new(width), NonZeroU32::new(width)) else {
            error!(InvalidValue, "width or height was zero")
        };
        tex.make_immutable_storage(
            &self.renderer.device,
            TextureDimensions {
                width: Some(w),
                height: Some(h),
                depth: None,
            },
            InternalFormat::from_repr(internalformat as u32).unwrap(),
            levels,
        );
        Ok(())
    }
}
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the texture object is bound for [**glTexStorage2DMultisample**](crate::context::Context::oxidegl_tex_storage2_d_multisample).
/// > Must be one of [`GL_TEXTURE_2D_MULTISAMPLE`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE)
/// > or [`GL_PROXY_TEXTURE_2D_MULTISAMPLE`](crate::gl_enums::GL_PROXY_TEXTURE_2D_MULTISAMPLE).
///
/// `texture`
///
/// > Specifies the texture object name for [**glTextureStorage2DMultisample**](crate::context::Context::oxidegl_texture_storage2_d_multisample).
/// > The effective target of `texture` must be one of the valid non-proxy `target`
/// > values above.
///
/// `samples`
///
/// > Specify the number of samples in the texture.
///
/// `internalformat`
///
/// > Specifies the sized internal format to be used to store texture image data.
///
/// `width`
///
/// > Specifies the width of the texture, in texels.
///
/// `height`
///
/// > Specifies the height of the texture, in texels.
///
/// `fixedsamplelocations`
///
/// > Specifies whether the image will use identical sample locations and the
/// > same number of samples for all texels in the image, and the sample locations
/// > will not depend on the internal format or size of the image.
///
/// ### Description
/// [**glTexStorage2DMultisample**](crate::context::Context::oxidegl_tex_storage2_d_multisample)
/// and [**glTextureStorage2DMultisample**](crate::context::Context::oxidegl_texture_storage2_d_multisample)
/// specify the storage requirements for a two-dimensional multisample texture.
/// Once a texture is specified with this command, its format and dimensions
/// become immutable unless it is a proxy texture. The contents of the image
/// may still be modified, however, its storage requirements may not change.
/// Such a texture is referred to as an *immutable-format* texture.
///
/// `samples` specifies the number of samples to be used for the texture and
/// must be greater than zero and less than or equal to the value of [`GL_MAX_SAMPLES`](crate::gl_enums::GL_MAX_SAMPLES).
/// `internalformat` must be a color-renderable, depth-renderable, or stencil-renderable
/// format. `width` and `height` specify the width and height, respectively,
/// of the texture. If `fixedsamplelocations` is [`GL_TRUE`](crate::gl_enums::GL_TRUE),
/// the image will use identical sample locations and the same number of samples
/// for all texels in the image, and the sample locations will not depend on
/// the internal format or size of the image.
///
/// | *Sized Internal Format*                               | *Base Internal Format*                               | *Red Bits*       | *Green Bits*        | *Blue Bits*       | *Alpha Bits*        | *Shared Bits*        |
/// |-------------------------------------------------------|------------------------------------------------------|------------------|---------------------|-------------------|---------------------|----------------------|
/// | [`GL_R8`](crate::gl_enums::GL_R8)                     | [`GL_RED`](crate::gl_enums::GL_RED)                  | 8                |                     |                   |                     |                      |
/// | [`GL_R8_SNORM`](crate::gl_enums::GL_R8_SNORM)         | [`GL_RED`](crate::gl_enums::GL_RED)                  | s8               |                     |                   |                     |                      |
/// | [`GL_R16`](crate::gl_enums::GL_R16)                   | [`GL_RED`](crate::gl_enums::GL_RED)                  | 16               |                     |                   |                     |                      |
/// | [`GL_R16_SNORM`](crate::gl_enums::GL_R16_SNORM)       | [`GL_RED`](crate::gl_enums::GL_RED)                  | s16              |                     |                   |                     |                      |
/// | [`GL_RG8`](crate::gl_enums::GL_RG8)                   | [`GL_RG`](crate::gl_enums::GL_RG)                    | 8                | 8                   |                   |                     |                      |
/// | [`GL_RG8_SNORM`](crate::gl_enums::GL_RG8_SNORM)       | [`GL_RG`](crate::gl_enums::GL_RG)                    | s8               | s8                  |                   |                     |                      |
/// | [`GL_RG16`](crate::gl_enums::GL_RG16)                 | [`GL_RG`](crate::gl_enums::GL_RG)                    | 16               | 16                  |                   |                     |                      |
/// | [`GL_RG16_SNORM`](crate::gl_enums::GL_RG16_SNORM)     | [`GL_RG`](crate::gl_enums::GL_RG)                    | s16              | s16                 |                   |                     |                      |
/// | [`GL_R3_G3_B2`](crate::gl_enums::GL_R3_G3_B2)         | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 3                | 3                   | 2                 |                     |                      |
/// | [`GL_RGB4`](crate::gl_enums::GL_RGB4)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 4                | 4                   | 4                 |                     |                      |
/// | [`GL_RGB5`](crate::gl_enums::GL_RGB5)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 5                | 5                   | 5                 |                     |                      |
/// | [`GL_RGB8`](crate::gl_enums::GL_RGB8)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 8                | 8                   | 8                 |                     |                      |
/// | [`GL_RGB8_SNORM`](crate::gl_enums::GL_RGB8_SNORM)     | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | s8               | s8                  | s8                |                     |                      |
/// | [`GL_RGB10`](crate::gl_enums::GL_RGB10)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 10               | 10                  | 10                |                     |                      |
/// | [`GL_RGB12`](crate::gl_enums::GL_RGB12)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 12               | 12                  | 12                |                     |                      |
/// | [`GL_RGB16_SNORM`](crate::gl_enums::GL_RGB16_SNORM)   | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 16               | 16                  | 16                |                     |                      |
/// | [`GL_RGBA2`](crate::gl_enums::GL_RGBA2)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 2                | 2                   | 2                 | 2                   |                      |
/// | [`GL_RGBA4`](crate::gl_enums::GL_RGBA4)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 4                | 4                   | 4                 | 4                   |                      |
/// | [`GL_RGB5_A1`](crate::gl_enums::GL_RGB5_A1)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 5                | 5                   | 5                 | 1                   |                      |
/// | [`GL_RGBA8`](crate::gl_enums::GL_RGBA8)               | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 8                | 8                   | 8                 | 8                   |                      |
/// | [`GL_RGBA8_SNORM`](crate::gl_enums::GL_RGBA8_SNORM)   | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | s8               | s8                  | s8                | s8                  |                      |
/// | [`GL_RGB10_A2`](crate::gl_enums::GL_RGB10_A2)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 10               | 10                  | 10                | 2                   |                      |
/// | [`GL_RGB10_A2UI`](crate::gl_enums::GL_RGB10_A2UI)     | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui10             | ui10                | ui10              | ui2                 |                      |
/// | [`GL_RGBA12`](crate::gl_enums::GL_RGBA12)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 12               | 12                  | 12                | 12                  |                      |
/// | [`GL_RGBA16`](crate::gl_enums::GL_RGBA16)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 16               | 16                  | 16                | 16                  |                      |
/// | [`GL_SRGB8`](crate::gl_enums::GL_SRGB8)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 8                | 8                   | 8                 |                     |                      |
/// | [`GL_SRGB8_ALPHA8`](crate::gl_enums::GL_SRGB8_ALPHA8) | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 8                | 8                   | 8                 | 8                   |                      |
/// | [`GL_R16F`](crate::gl_enums::GL_R16F)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | f16              |                     |                   |                     |                      |
/// | [`GL_RG16F`](crate::gl_enums::GL_RG16F)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | f16              | f16                 |                   |                     |                      |
/// | [`GL_RGB16F`](crate::gl_enums::GL_RGB16F)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | f16              | f16                 | f16               |                     |                      |
/// | [`GL_RGBA16F`](crate::gl_enums::GL_RGBA16F)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | f16              | f16                 | f16               | f16                 |                      |
/// | [`GL_R32F`](crate::gl_enums::GL_R32F)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | f32              |                     |                   |                     |                      |
/// | [`GL_RG32F`](crate::gl_enums::GL_RG32F)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | f32              | f32                 |                   |                     |                      |
/// | [`GL_RGB32F`](crate::gl_enums::GL_RGB32F)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | f32              | f32                 | f32               |                     |                      |
/// | [`GL_RGBA32F`](crate::gl_enums::GL_RGBA32F)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | f32              | f32                 | f32               | f32                 |                      |
/// | [`GL_R11F_G11F_B10F`](crate::gl_enums::GL_R11F_G11F_B10F) | [`GL_RGB`](crate::gl_enums::GL_RGB)              | f11              | f11                 | f10               |                     |                      |
/// | [`GL_RGB9_E5`](crate::gl_enums::GL_RGB9_E5)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 9                | 9                   | 9                 |                     | 5                    |
/// | [`GL_R8I`](crate::gl_enums::GL_R8I)                   | [`GL_RED`](crate::gl_enums::GL_RED)                  | i8               |                     |                   |                     |                      |
/// | [`GL_R8UI`](crate::gl_enums::GL_R8UI)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui8              |                     |                   |                     |                      |
/// | [`GL_R16I`](crate::gl_enums::GL_R16I)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | i16              |                     |                   |                     |                      |
/// | [`GL_R16UI`](crate::gl_enums::GL_R16UI)               | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui16             |                     |                   |                     |                      |
/// | [`GL_R32I`](crate::gl_enums::GL_R32I)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | i32              |                     |                   |                     |                      |
/// | [`GL_R32UI`](crate::gl_enums::GL_R32UI)               | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui32             |                     |                   |                     |                      |
/// | [`GL_RG8I`](crate::gl_enums::GL_RG8I)                 | [`GL_RG`](crate::gl_enums::GL_RG)                    | i8               | i8                  |                   |                     |                      |
/// | [`GL_RG8UI`](crate::gl_enums::GL_RG8UI)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui8              | ui8                 |                   |                     |                      |
/// | [`GL_RG16I`](crate::gl_enums::GL_RG16I)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | i16              | i16                 |                   |                     |                      |
/// | [`GL_RG16UI`](crate::gl_enums::GL_RG16UI)             | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui16             | ui16                |                   |                     |                      |
/// | [`GL_RG32I`](crate::gl_enums::GL_RG32I)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | i32              | i32                 |                   |                     |                      |
/// | [`GL_RG32UI`](crate::gl_enums::GL_RG32UI)             | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui32             | ui32                |                   |                     |                      |
/// | [`GL_RGB8I`](crate::gl_enums::GL_RGB8I)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i8               | i8                  | i8                |                     |                      |
/// | [`GL_RGB8UI`](crate::gl_enums::GL_RGB8UI)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui8              | ui8                 | ui8               |                     |                      |
/// | [`GL_RGB16I`](crate::gl_enums::GL_RGB16I)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i16              | i16                 | i16               |                     |                      |
/// | [`GL_RGB16UI`](crate::gl_enums::GL_RGB16UI)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui16             | ui16                | ui16              |                     |                      |
/// | [`GL_RGB32I`](crate::gl_enums::GL_RGB32I)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i32              | i32                 | i32               |                     |                      |
/// | [`GL_RGB32UI`](crate::gl_enums::GL_RGB32UI)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui32             | ui32                | ui32              |                     |                      |
/// | [`GL_RGBA8I`](crate::gl_enums::GL_RGBA8I)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i8               | i8                  | i8                | i8                  |                      |
/// | [`GL_RGBA8UI`](crate::gl_enums::GL_RGBA8UI)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui8              | ui8                 | ui8               | ui8                 |                      |
/// | [`GL_RGBA16I`](crate::gl_enums::GL_RGBA16I)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i16              | i16                 | i16               | i16                 |                      |
/// | [`GL_RGBA16UI`](crate::gl_enums::GL_RGBA16UI)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui16             | ui16                | ui16              | ui16                |                      |
/// | [`GL_RGBA32I`](crate::gl_enums::GL_RGBA32I)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i32              | i32                 | i32               | i32                 |                      |
/// | [`GL_RGBA32UI`](crate::gl_enums::GL_RGBA32UI)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui32             | ui32                | ui32              | ui32                |                      |
///
impl Context {
    pub fn oxidegl_tex_storage_2_d_multisample(
        &mut self,
        target: TextureTarget,
        samples: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean,
    ) -> GlFallible {
        panic!("command oxidegl_tex_storage2_d_multisample not yet implemented");
    }
    pub fn oxidegl_texture_storage_2_d_multisample(
        &mut self,
        texture: GLuint,
        samples: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean,
    ) -> GlFallible {
        panic!("command oxidegl_texture_storage2_d_multisample not yet implemented");
    }
}
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the texture object is bound for [**glTexStorage3D**](crate::context::Context::oxidegl_tex_storage3_d).
/// > Must be one of [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D), [`GL_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_ARRAY),
/// > [`GL_TEXTURE_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_ARRAY),
/// > [`GL_PROXY_TEXTURE_3D`](crate::gl_enums::GL_PROXY_TEXTURE_3D), [`GL_PROXY_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_PROXY_TEXTURE_2D_ARRAY)
/// > or [`GL_PROXY_TEXTURE_CUBE_MAP_ARRAY`](crate::gl_enums::GL_PROXY_TEXTURE_CUBE_MAP_ARRAY).
///
/// `texture`
///
/// > Specifies the texture object name for [**glTextureStorage3D**](crate::context::Context::oxidegl_texture_storage3_d).
/// > The effective target of `texture` must be one of the valid non-proxy `target`
/// > values above.
///
/// `levels`
///
/// > Specify the number of texture levels.
///
/// `internalformat`
///
/// > Specifies the sized internal format to be used to store texture image data.
///
/// `width`
///
/// > Specifies the width of the texture, in texels.
///
/// `height`
///
/// > Specifies the height of the texture, in texels.
///
/// `depth`
///
/// > Specifies the depth of the texture, in texels.
///
/// ### Description
/// [**glTexStorage3D**](crate::context::Context::oxidegl_tex_storage3_d) and
/// [**glTextureStorage3D**](crate::context::Context::oxidegl_texture_storage3_d)
/// specify the storage requirements for all levels of a three-dimensional,
/// two-dimensional array or cube-map array texture simultaneously. Once a
/// texture is specified with this command, the format and dimensions of all
/// levels become immutable unless it is a proxy texture. The contents of the
/// image may still be modified, however, its storage requirements may not
/// change. Such a texture is referred to as an *immutable-format* texture.
///
/// The behavior of [**glTexStorage3D**](crate::context::Context::oxidegl_tex_storage3_d)
/// depends on the `target` parameter. When `target` is [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D),
/// or [`GL_PROXY_TEXTURE_3D`](crate::gl_enums::GL_PROXY_TEXTURE_3D), calling
/// [**glTexStorage3D**](crate::context::Context::oxidegl_tex_storage3_d) is
/// equivalent, assuming no errors are generated, to executing the following
/// pseudo-code:
///
/// When `target` is [`GL_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_ARRAY),
/// [`GL_PROXY_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_PROXY_TEXTURE_2D_ARRAY),
/// [`GL_TEXTURE_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_ARRAY),
/// or [`GL_PROXY_TEXTURE_CUBE_MAP_ARRAY`](crate::gl_enums::GL_PROXY_TEXTURE_CUBE_MAP_ARRAY),
/// [**glTexStorage3D**](crate::context::Context::oxidegl_tex_storage3_d) is
/// equivalent to:
///
/// Calling [**glTextureStorage3D**](crate::context::Context::oxidegl_texture_storage3_d)
/// is equivalent to the above pseudo-code, where `target` is the effective
/// target of `texture` and it is as if `texture` were bound to `target` for
/// the purposes of [**glTexImage3D**](crate::context::Context::oxidegl_tex_image3_d).
///
/// Since no texture data is actually provided, the values used in the pseudo-code
/// for `format` and `type` are irrelevant and may be considered to be any
/// values that are legal for the chosen `internalformat` enumerant. `internalformat`
/// must be one of the sized internal formats given in Table 1 below, one of
/// the sized depth-component formats [`GL_DEPTH_COMPONENT32F`](crate::gl_enums::GL_DEPTH_COMPONENT32F),
/// [`GL_DEPTH_COMPONENT24`](crate::gl_enums::GL_DEPTH_COMPONENT24), or [`GL_DEPTH_COMPONENT16`](crate::gl_enums::GL_DEPTH_COMPONENT16),
/// one of the combined depth-stencil formats, [`GL_DEPTH32F_STENCIL8`](crate::gl_enums::GL_DEPTH32F_STENCIL8),
/// or [`GL_DEPTH24_STENCIL8`](crate::gl_enums::GL_DEPTH24_STENCIL8), or the
/// stencil-only format, [`GL_STENCIL_INDEX8`](crate::gl_enums::GL_STENCIL_INDEX8).
/// Upon success, the value of [`GL_TEXTURE_IMMUTABLE_FORMAT`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_FORMAT)
/// becomes [`GL_TRUE`](crate::gl_enums::GL_TRUE). The value of [`GL_TEXTURE_IMMUTABLE_FORMAT`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_FORMAT)
/// may be discovered by calling [**glGetTexParameter**](crate::context::Context::oxidegl_get_tex_parameter)
/// with `pname` set to [`GL_TEXTURE_IMMUTABLE_FORMAT`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_FORMAT).
/// No further changes to the dimensions or format of the texture object may
/// be made. Using any command that might alter the dimensions or format of
/// the texture object (such as [**glTexImage3D**](crate::context::Context::oxidegl_tex_image3_d)
/// or another call to [**glTexStorage3D**](crate::context::Context::oxidegl_tex_storage3_d))
/// will result in the generation of a [`GL_INVALID_OPERATION`](crate::gl_enums::GL_INVALID_OPERATION)
/// error, even if it would not, in fact, alter the dimensions or format of
/// the object.
///
/// | *Sized Internal Format*                               | *Base Internal Format*                               | *Red Bits*       | *Green Bits*        | *Blue Bits*       | *Alpha Bits*        | *Shared Bits*        |
/// |-------------------------------------------------------|------------------------------------------------------|------------------|---------------------|-------------------|---------------------|----------------------|
/// | [`GL_R8`](crate::gl_enums::GL_R8)                     | [`GL_RED`](crate::gl_enums::GL_RED)                  | 8                |                     |                   |                     |                      |
/// | [`GL_R8_SNORM`](crate::gl_enums::GL_R8_SNORM)         | [`GL_RED`](crate::gl_enums::GL_RED)                  | s8               |                     |                   |                     |                      |
/// | [`GL_R16`](crate::gl_enums::GL_R16)                   | [`GL_RED`](crate::gl_enums::GL_RED)                  | 16               |                     |                   |                     |                      |
/// | [`GL_R16_SNORM`](crate::gl_enums::GL_R16_SNORM)       | [`GL_RED`](crate::gl_enums::GL_RED)                  | s16              |                     |                   |                     |                      |
/// | [`GL_RG8`](crate::gl_enums::GL_RG8)                   | [`GL_RG`](crate::gl_enums::GL_RG)                    | 8                | 8                   |                   |                     |                      |
/// | [`GL_RG8_SNORM`](crate::gl_enums::GL_RG8_SNORM)       | [`GL_RG`](crate::gl_enums::GL_RG)                    | s8               | s8                  |                   |                     |                      |
/// | [`GL_RG16`](crate::gl_enums::GL_RG16)                 | [`GL_RG`](crate::gl_enums::GL_RG)                    | 16               | 16                  |                   |                     |                      |
/// | [`GL_RG16_SNORM`](crate::gl_enums::GL_RG16_SNORM)     | [`GL_RG`](crate::gl_enums::GL_RG)                    | s16              | s16                 |                   |                     |                      |
/// | [`GL_R3_G3_B2`](crate::gl_enums::GL_R3_G3_B2)         | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 3                | 3                   | 2                 |                     |                      |
/// | [`GL_RGB4`](crate::gl_enums::GL_RGB4)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 4                | 4                   | 4                 |                     |                      |
/// | [`GL_RGB5`](crate::gl_enums::GL_RGB5)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 5                | 5                   | 5                 |                     |                      |
/// | [`GL_RGB8`](crate::gl_enums::GL_RGB8)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 8                | 8                   | 8                 |                     |                      |
/// | [`GL_RGB8_SNORM`](crate::gl_enums::GL_RGB8_SNORM)     | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | s8               | s8                  | s8                |                     |                      |
/// | [`GL_RGB10`](crate::gl_enums::GL_RGB10)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 10               | 10                  | 10                |                     |                      |
/// | [`GL_RGB12`](crate::gl_enums::GL_RGB12)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 12               | 12                  | 12                |                     |                      |
/// | [`GL_RGB16_SNORM`](crate::gl_enums::GL_RGB16_SNORM)   | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 16               | 16                  | 16                |                     |                      |
/// | [`GL_RGBA2`](crate::gl_enums::GL_RGBA2)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 2                | 2                   | 2                 | 2                   |                      |
/// | [`GL_RGBA4`](crate::gl_enums::GL_RGBA4)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 4                | 4                   | 4                 | 4                   |                      |
/// | [`GL_RGB5_A1`](crate::gl_enums::GL_RGB5_A1)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 5                | 5                   | 5                 | 1                   |                      |
/// | [`GL_RGBA8`](crate::gl_enums::GL_RGBA8)               | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 8                | 8                   | 8                 | 8                   |                      |
/// | [`GL_RGBA8_SNORM`](crate::gl_enums::GL_RGBA8_SNORM)   | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | s8               | s8                  | s8                | s8                  |                      |
/// | [`GL_RGB10_A2`](crate::gl_enums::GL_RGB10_A2)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 10               | 10                  | 10                | 2                   |                      |
/// | [`GL_RGB10_A2UI`](crate::gl_enums::GL_RGB10_A2UI)     | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui10             | ui10                | ui10              | ui2                 |                      |
/// | [`GL_RGBA12`](crate::gl_enums::GL_RGBA12)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 12               | 12                  | 12                | 12                  |                      |
/// | [`GL_RGBA16`](crate::gl_enums::GL_RGBA16)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 16               | 16                  | 16                | 16                  |                      |
/// | [`GL_SRGB8`](crate::gl_enums::GL_SRGB8)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 8                | 8                   | 8                 |                     |                      |
/// | [`GL_SRGB8_ALPHA8`](crate::gl_enums::GL_SRGB8_ALPHA8) | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 8                | 8                   | 8                 | 8                   |                      |
/// | [`GL_R16F`](crate::gl_enums::GL_R16F)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | f16              |                     |                   |                     |                      |
/// | [`GL_RG16F`](crate::gl_enums::GL_RG16F)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | f16              | f16                 |                   |                     |                      |
/// | [`GL_RGB16F`](crate::gl_enums::GL_RGB16F)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | f16              | f16                 | f16               |                     |                      |
/// | [`GL_RGBA16F`](crate::gl_enums::GL_RGBA16F)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | f16              | f16                 | f16               | f16                 |                      |
/// | [`GL_R32F`](crate::gl_enums::GL_R32F)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | f32              |                     |                   |                     |                      |
/// | [`GL_RG32F`](crate::gl_enums::GL_RG32F)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | f32              | f32                 |                   |                     |                      |
/// | [`GL_RGB32F`](crate::gl_enums::GL_RGB32F)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | f32              | f32                 | f32               |                     |                      |
/// | [`GL_RGBA32F`](crate::gl_enums::GL_RGBA32F)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | f32              | f32                 | f32               | f32                 |                      |
/// | [`GL_R11F_G11F_B10F`](crate::gl_enums::GL_R11F_G11F_B10F) | [`GL_RGB`](crate::gl_enums::GL_RGB)              | f11              | f11                 | f10               |                     |                      |
/// | [`GL_RGB9_E5`](crate::gl_enums::GL_RGB9_E5)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 9                | 9                   | 9                 |                     | 5                    |
/// | [`GL_R8I`](crate::gl_enums::GL_R8I)                   | [`GL_RED`](crate::gl_enums::GL_RED)                  | i8               |                     |                   |                     |                      |
/// | [`GL_R8UI`](crate::gl_enums::GL_R8UI)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui8              |                     |                   |                     |                      |
/// | [`GL_R16I`](crate::gl_enums::GL_R16I)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | i16              |                     |                   |                     |                      |
/// | [`GL_R16UI`](crate::gl_enums::GL_R16UI)               | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui16             |                     |                   |                     |                      |
/// | [`GL_R32I`](crate::gl_enums::GL_R32I)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | i32              |                     |                   |                     |                      |
/// | [`GL_R32UI`](crate::gl_enums::GL_R32UI)               | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui32             |                     |                   |                     |                      |
/// | [`GL_RG8I`](crate::gl_enums::GL_RG8I)                 | [`GL_RG`](crate::gl_enums::GL_RG)                    | i8               | i8                  |                   |                     |                      |
/// | [`GL_RG8UI`](crate::gl_enums::GL_RG8UI)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui8              | ui8                 |                   |                     |                      |
/// | [`GL_RG16I`](crate::gl_enums::GL_RG16I)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | i16              | i16                 |                   |                     |                      |
/// | [`GL_RG16UI`](crate::gl_enums::GL_RG16UI)             | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui16             | ui16                |                   |                     |                      |
/// | [`GL_RG32I`](crate::gl_enums::GL_RG32I)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | i32              | i32                 |                   |                     |                      |
/// | [`GL_RG32UI`](crate::gl_enums::GL_RG32UI)             | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui32             | ui32                |                   |                     |                      |
/// | [`GL_RGB8I`](crate::gl_enums::GL_RGB8I)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i8               | i8                  | i8                |                     |                      |
/// | [`GL_RGB8UI`](crate::gl_enums::GL_RGB8UI)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui8              | ui8                 | ui8               |                     |                      |
/// | [`GL_RGB16I`](crate::gl_enums::GL_RGB16I)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i16              | i16                 | i16               |                     |                      |
/// | [`GL_RGB16UI`](crate::gl_enums::GL_RGB16UI)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui16             | ui16                | ui16              |                     |                      |
/// | [`GL_RGB32I`](crate::gl_enums::GL_RGB32I)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i32              | i32                 | i32               |                     |                      |
/// | [`GL_RGB32UI`](crate::gl_enums::GL_RGB32UI)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui32             | ui32                | ui32              |                     |                      |
/// | [`GL_RGBA8I`](crate::gl_enums::GL_RGBA8I)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i8               | i8                  | i8                | i8                  |                      |
/// | [`GL_RGBA8UI`](crate::gl_enums::GL_RGBA8UI)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui8              | ui8                 | ui8               | ui8                 |                      |
/// | [`GL_RGBA16I`](crate::gl_enums::GL_RGBA16I)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i16              | i16                 | i16               | i16                 |                      |
/// | [`GL_RGBA16UI`](crate::gl_enums::GL_RGBA16UI)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui16             | ui16                | ui16              | ui16                |                      |
/// | [`GL_RGBA32I`](crate::gl_enums::GL_RGBA32I)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i32              | i32                 | i32               | i32                 |                      |
/// | [`GL_RGBA32UI`](crate::gl_enums::GL_RGBA32UI)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui32             | ui32                | ui32              | ui32                |                      |
///
///
/// ### Notes
/// [`GL_STENCIL_INDEX8`](crate::gl_enums::GL_STENCIL_INDEX8) is accepted for
/// `internalformat` only if the GL version is 4.4 or higher.
impl Context {
    pub fn oxidegl_tex_storage_3_d(
        &mut self,
        target: TextureTarget,
        levels: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) -> GlFallible {
        panic!("command oxidegl_tex_storage3_d not yet implemented");
    }
    pub fn oxidegl_texture_storage_3_d(
        &mut self,
        texture: GLuint,
        levels: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) -> GlFallible {
        panic!("command oxidegl_texture_storage3_d not yet implemented");
    }
}
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the texture object is bound for [**glTexStorage3DMultisample**](crate::context::Context::oxidegl_tex_storage3_d_multisample).
/// > Must be one of [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY)
/// > or [`GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY).
///
/// `texture`
///
/// > Specifies the texture object name for [**glTextureStorage3DMultisample**](crate::context::Context::oxidegl_texture_storage3_d_multisample).
/// > The effective target of `texture` must be one of the valid non-proxy `target`
/// > values above.
///
/// `samples`
///
/// > Specify the number of samples in the texture.
///
/// `internalformat`
///
/// > Specifies the sized internal format to be used to store texture image data.
///
/// `width`
///
/// > Specifies the width of the texture, in texels.
///
/// `height`
///
/// > Specifies the height of the texture, in texels.
///
/// `depth`
///
/// > Specifies the depth of the texture, in layers.
///
/// `fixedsamplelocations`
///
/// > Specifies whether the image will use identical sample locations and the
/// > same number of samples for all texels in the image, and the sample locations
/// > will not depend on the internal format or size of the image.
///
/// ### Description
/// [**glTexStorage3DMultisample**](crate::context::Context::oxidegl_tex_storage3_d_multisample)
/// and [**glTextureStorage3DMultisample**](crate::context::Context::oxidegl_texture_storage3_d_multisample)
/// specify the storage requirements for a two-dimensional multisample array
/// texture. Once a texture is specified with this command, its format and
/// dimensions become immutable unless it is a proxy texture. The contents
/// of the image may still be modified, however, its storage requirements may
/// not change. Such a texture is referred to as an *immutable-format* texture.
///
/// `samples` specifies the number of samples to be used for the texture and
/// must be greater than zero and less than or equal to the value of [`GL_MAX_SAMPLES`](crate::gl_enums::GL_MAX_SAMPLES).
/// `internalformat` must be a color-renderable, depth-renderable, or stencil-renderable
/// format. `width` and `height` specify the width and height, respectively,
/// of the texture and `depth` specifies the depth (or the number of layers)
/// of the texture. If `fixedsamplelocations` is [`GL_TRUE`](crate::gl_enums::GL_TRUE),
/// the image will use identical sample locations and the same number of samples
/// for all texels in the image, and the sample locations will not depend on
/// the internal format or size of the image.
///
/// | *Sized Internal Format*                               | *Base Internal Format*                               | *Red Bits*       | *Green Bits*        | *Blue Bits*       | *Alpha Bits*        | *Shared Bits*        |
/// |-------------------------------------------------------|------------------------------------------------------|------------------|---------------------|-------------------|---------------------|----------------------|
/// | [`GL_R8`](crate::gl_enums::GL_R8)                     | [`GL_RED`](crate::gl_enums::GL_RED)                  | 8                |                     |                   |                     |                      |
/// | [`GL_R8_SNORM`](crate::gl_enums::GL_R8_SNORM)         | [`GL_RED`](crate::gl_enums::GL_RED)                  | s8               |                     |                   |                     |                      |
/// | [`GL_R16`](crate::gl_enums::GL_R16)                   | [`GL_RED`](crate::gl_enums::GL_RED)                  | 16               |                     |                   |                     |                      |
/// | [`GL_R16_SNORM`](crate::gl_enums::GL_R16_SNORM)       | [`GL_RED`](crate::gl_enums::GL_RED)                  | s16              |                     |                   |                     |                      |
/// | [`GL_RG8`](crate::gl_enums::GL_RG8)                   | [`GL_RG`](crate::gl_enums::GL_RG)                    | 8                | 8                   |                   |                     |                      |
/// | [`GL_RG8_SNORM`](crate::gl_enums::GL_RG8_SNORM)       | [`GL_RG`](crate::gl_enums::GL_RG)                    | s8               | s8                  |                   |                     |                      |
/// | [`GL_RG16`](crate::gl_enums::GL_RG16)                 | [`GL_RG`](crate::gl_enums::GL_RG)                    | 16               | 16                  |                   |                     |                      |
/// | [`GL_RG16_SNORM`](crate::gl_enums::GL_RG16_SNORM)     | [`GL_RG`](crate::gl_enums::GL_RG)                    | s16              | s16                 |                   |                     |                      |
/// | [`GL_R3_G3_B2`](crate::gl_enums::GL_R3_G3_B2)         | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 3                | 3                   | 2                 |                     |                      |
/// | [`GL_RGB4`](crate::gl_enums::GL_RGB4)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 4                | 4                   | 4                 |                     |                      |
/// | [`GL_RGB5`](crate::gl_enums::GL_RGB5)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 5                | 5                   | 5                 |                     |                      |
/// | [`GL_RGB8`](crate::gl_enums::GL_RGB8)                 | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 8                | 8                   | 8                 |                     |                      |
/// | [`GL_RGB8_SNORM`](crate::gl_enums::GL_RGB8_SNORM)     | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | s8               | s8                  | s8                |                     |                      |
/// | [`GL_RGB10`](crate::gl_enums::GL_RGB10)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 10               | 10                  | 10                |                     |                      |
/// | [`GL_RGB12`](crate::gl_enums::GL_RGB12)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 12               | 12                  | 12                |                     |                      |
/// | [`GL_RGB16_SNORM`](crate::gl_enums::GL_RGB16_SNORM)   | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 16               | 16                  | 16                |                     |                      |
/// | [`GL_RGBA2`](crate::gl_enums::GL_RGBA2)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 2                | 2                   | 2                 | 2                   |                      |
/// | [`GL_RGBA4`](crate::gl_enums::GL_RGBA4)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 4                | 4                   | 4                 | 4                   |                      |
/// | [`GL_RGB5_A1`](crate::gl_enums::GL_RGB5_A1)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 5                | 5                   | 5                 | 1                   |                      |
/// | [`GL_RGBA8`](crate::gl_enums::GL_RGBA8)               | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 8                | 8                   | 8                 | 8                   |                      |
/// | [`GL_RGBA8_SNORM`](crate::gl_enums::GL_RGBA8_SNORM)   | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | s8               | s8                  | s8                | s8                  |                      |
/// | [`GL_RGB10_A2`](crate::gl_enums::GL_RGB10_A2)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 10               | 10                  | 10                | 2                   |                      |
/// | [`GL_RGB10_A2UI`](crate::gl_enums::GL_RGB10_A2UI)     | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui10             | ui10                | ui10              | ui2                 |                      |
/// | [`GL_RGBA12`](crate::gl_enums::GL_RGBA12)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 12               | 12                  | 12                | 12                  |                      |
/// | [`GL_RGBA16`](crate::gl_enums::GL_RGBA16)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 16               | 16                  | 16                | 16                  |                      |
/// | [`GL_SRGB8`](crate::gl_enums::GL_SRGB8)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 8                | 8                   | 8                 |                     |                      |
/// | [`GL_SRGB8_ALPHA8`](crate::gl_enums::GL_SRGB8_ALPHA8) | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | 8                | 8                   | 8                 | 8                   |                      |
/// | [`GL_R16F`](crate::gl_enums::GL_R16F)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | f16              |                     |                   |                     |                      |
/// | [`GL_RG16F`](crate::gl_enums::GL_RG16F)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | f16              | f16                 |                   |                     |                      |
/// | [`GL_RGB16F`](crate::gl_enums::GL_RGB16F)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | f16              | f16                 | f16               |                     |                      |
/// | [`GL_RGBA16F`](crate::gl_enums::GL_RGBA16F)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | f16              | f16                 | f16               | f16                 |                      |
/// | [`GL_R32F`](crate::gl_enums::GL_R32F)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | f32              |                     |                   |                     |                      |
/// | [`GL_RG32F`](crate::gl_enums::GL_RG32F)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | f32              | f32                 |                   |                     |                      |
/// | [`GL_RGB32F`](crate::gl_enums::GL_RGB32F)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | f32              | f32                 | f32               |                     |                      |
/// | [`GL_RGBA32F`](crate::gl_enums::GL_RGBA32F)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | f32              | f32                 | f32               | f32                 |                      |
/// | [`GL_R11F_G11F_B10F`](crate::gl_enums::GL_R11F_G11F_B10F) | [`GL_RGB`](crate::gl_enums::GL_RGB)              | f11              | f11                 | f10               |                     |                      |
/// | [`GL_RGB9_E5`](crate::gl_enums::GL_RGB9_E5)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | 9                | 9                   | 9                 |                     | 5                    |
/// | [`GL_R8I`](crate::gl_enums::GL_R8I)                   | [`GL_RED`](crate::gl_enums::GL_RED)                  | i8               |                     |                   |                     |                      |
/// | [`GL_R8UI`](crate::gl_enums::GL_R8UI)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui8              |                     |                   |                     |                      |
/// | [`GL_R16I`](crate::gl_enums::GL_R16I)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | i16              |                     |                   |                     |                      |
/// | [`GL_R16UI`](crate::gl_enums::GL_R16UI)               | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui16             |                     |                   |                     |                      |
/// | [`GL_R32I`](crate::gl_enums::GL_R32I)                 | [`GL_RED`](crate::gl_enums::GL_RED)                  | i32              |                     |                   |                     |                      |
/// | [`GL_R32UI`](crate::gl_enums::GL_R32UI)               | [`GL_RED`](crate::gl_enums::GL_RED)                  | ui32             |                     |                   |                     |                      |
/// | [`GL_RG8I`](crate::gl_enums::GL_RG8I)                 | [`GL_RG`](crate::gl_enums::GL_RG)                    | i8               | i8                  |                   |                     |                      |
/// | [`GL_RG8UI`](crate::gl_enums::GL_RG8UI)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui8              | ui8                 |                   |                     |                      |
/// | [`GL_RG16I`](crate::gl_enums::GL_RG16I)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | i16              | i16                 |                   |                     |                      |
/// | [`GL_RG16UI`](crate::gl_enums::GL_RG16UI)             | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui16             | ui16                |                   |                     |                      |
/// | [`GL_RG32I`](crate::gl_enums::GL_RG32I)               | [`GL_RG`](crate::gl_enums::GL_RG)                    | i32              | i32                 |                   |                     |                      |
/// | [`GL_RG32UI`](crate::gl_enums::GL_RG32UI)             | [`GL_RG`](crate::gl_enums::GL_RG)                    | ui32             | ui32                |                   |                     |                      |
/// | [`GL_RGB8I`](crate::gl_enums::GL_RGB8I)               | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i8               | i8                  | i8                |                     |                      |
/// | [`GL_RGB8UI`](crate::gl_enums::GL_RGB8UI)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui8              | ui8                 | ui8               |                     |                      |
/// | [`GL_RGB16I`](crate::gl_enums::GL_RGB16I)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i16              | i16                 | i16               |                     |                      |
/// | [`GL_RGB16UI`](crate::gl_enums::GL_RGB16UI)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui16             | ui16                | ui16              |                     |                      |
/// | [`GL_RGB32I`](crate::gl_enums::GL_RGB32I)             | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | i32              | i32                 | i32               |                     |                      |
/// | [`GL_RGB32UI`](crate::gl_enums::GL_RGB32UI)           | [`GL_RGB`](crate::gl_enums::GL_RGB)                  | ui32             | ui32                | ui32              |                     |                      |
/// | [`GL_RGBA8I`](crate::gl_enums::GL_RGBA8I)             | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i8               | i8                  | i8                | i8                  |                      |
/// | [`GL_RGBA8UI`](crate::gl_enums::GL_RGBA8UI)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui8              | ui8                 | ui8               | ui8                 |                      |
/// | [`GL_RGBA16I`](crate::gl_enums::GL_RGBA16I)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i16              | i16                 | i16               | i16                 |                      |
/// | [`GL_RGBA16UI`](crate::gl_enums::GL_RGBA16UI)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui16             | ui16                | ui16              | ui16                |                      |
/// | [`GL_RGBA32I`](crate::gl_enums::GL_RGBA32I)           | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | i32              | i32                 | i32               | i32                 |                      |
/// | [`GL_RGBA32UI`](crate::gl_enums::GL_RGBA32UI)         | [`GL_RGBA`](crate::gl_enums::GL_RGBA)                | ui32             | ui32                | ui32              | ui32                |                      |
///
///
/// ### Associated Gets
/// [**glGetInteger**](crate::context::Context::oxidegl_get_integer) with arguments
/// [`GL_MAX_TEXTURE_SIZE`](crate::gl_enums::GL_MAX_TEXTURE_SIZE), [`GL_MAX_ARRAY_TEXTURE_LEVELS`](crate::gl_enums::GL_MAX_ARRAY_TEXTURE_LEVELS),
/// [`GL_TEXTURE_VIEW_MIN_LAYER`](crate::gl_enums::GL_TEXTURE_VIEW_MIN_LAYER),
/// [`GL_TEXTURE_VIEW_NUM_LAYERS`](crate::gl_enums::GL_TEXTURE_VIEW_NUM_LAYERS),
/// or [`GL_TEXTURE_IMMUTABLE_LEVELS`](crate::gl_enums::GL_TEXTURE_IMMUTABLE_LEVELS).
impl Context {
    pub fn oxidegl_tex_storage_3_d_multisample(
        &mut self,
        target: TextureTarget,
        samples: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean,
    ) -> GlFallible {
        panic!("command oxidegl_tex_storage3_d_multisample not yet implemented");
    }
    pub fn oxidegl_texture_storage_3_d_multisample(
        &mut self,
        texture: GLuint,
        samples: GLsizei,
        internalformat: SizedInternalFormat,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean,
    ) -> GlFallible {
        panic!("command oxidegl_texture_storage3_d_multisample not yet implemented");
    }
}
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the texture is bound for [**glTexSubImage1D**](crate::context::Context::oxidegl_tex_sub_image1_d).
/// > Must be [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D).
///
/// `texture`
///
/// > Specifies the texture object name for [**glTextureSubImage1D**](crate::context::Context::oxidegl_texture_sub_image1_d).
/// > The effective target of `texture` must be one of the valid `target` values
/// > above.
///
/// `level`
///
/// > Specifies the level-of-detail number. Level 0 is the base image level.
/// > Level *n* is the *n* th mipmap reduction image.
///
/// `xoffset`
///
/// > Specifies a texel offset in the x direction within the texture array.
///
/// `width`
///
/// > Specifies the width of the texture subimage.
///
/// `format`
///
/// > Specifies the format of the pixel data. The following symbolic values are
/// > accepted: [`GL_RED`](crate::gl_enums::GL_RED), [`GL_RG`](crate::gl_enums::GL_RG),
/// > [`GL_RGB`](crate::gl_enums::GL_RGB), [`GL_BGR`](crate::gl_enums::GL_BGR),
/// > [`GL_RGBA`](crate::gl_enums::GL_RGBA), [`GL_DEPTH_COMPONENT`](crate::gl_enums::GL_DEPTH_COMPONENT),
/// > and [`GL_STENCIL_INDEX`](crate::gl_enums::GL_STENCIL_INDEX).
///
/// `type`
///
/// > Specifies the data type of the pixel data. The following symbolic values
/// > are accepted: [`GL_UNSIGNED_BYTE`](crate::gl_enums::GL_UNSIGNED_BYTE), [`GL_BYTE`](crate::gl_enums::GL_BYTE),
/// > [`GL_UNSIGNED_SHORT`](crate::gl_enums::GL_UNSIGNED_SHORT), [`GL_SHORT`](crate::gl_enums::GL_SHORT),
/// > [`GL_UNSIGNED_INT`](crate::gl_enums::GL_UNSIGNED_INT), [`GL_INT`](crate::gl_enums::GL_INT),
/// > [`GL_FLOAT`](crate::gl_enums::GL_FLOAT), [`GL_UNSIGNED_BYTE_3_3_2`](crate::gl_enums::GL_UNSIGNED_BYTE_3_3_2),
/// > [`GL_UNSIGNED_BYTE_2_3_3_REV`](crate::gl_enums::GL_UNSIGNED_BYTE_2_3_3_REV),
/// > [`GL_UNSIGNED_SHORT_5_6_5`](crate::gl_enums::GL_UNSIGNED_SHORT_5_6_5), [`GL_UNSIGNED_SHORT_5_6_5_REV`](crate::gl_enums::GL_UNSIGNED_SHORT_5_6_5_REV),
/// > [`GL_UNSIGNED_SHORT_4_4_4_4`](crate::gl_enums::GL_UNSIGNED_SHORT_4_4_4_4),
/// > [`GL_UNSIGNED_SHORT_4_4_4_4_REV`](crate::gl_enums::GL_UNSIGNED_SHORT_4_4_4_4_REV),
/// > [`GL_UNSIGNED_SHORT_5_5_5_1`](crate::gl_enums::GL_UNSIGNED_SHORT_5_5_5_1),
/// > [`GL_UNSIGNED_SHORT_1_5_5_5_REV`](crate::gl_enums::GL_UNSIGNED_SHORT_1_5_5_5_REV),
/// > [`GL_UNSIGNED_INT_8_8_8_8`](crate::gl_enums::GL_UNSIGNED_INT_8_8_8_8), [`GL_UNSIGNED_INT_8_8_8_8_REV`](crate::gl_enums::GL_UNSIGNED_INT_8_8_8_8_REV),
/// > [`GL_UNSIGNED_INT_10_10_10_2`](crate::gl_enums::GL_UNSIGNED_INT_10_10_10_2),
/// > and [`GL_UNSIGNED_INT_2_10_10_10_REV`](crate::gl_enums::GL_UNSIGNED_INT_2_10_10_10_REV).
///
/// `pixels`
///
/// > Specifies a pointer to the image data in memory.
///
/// ### Description
/// Texturing maps a portion of a specified texture image onto each graphical
/// primitive for which texturing is enabled. To enable or disable one-dimensional
/// texturing, call [**glEnable**](crate::context::Context::oxidegl_enable)
/// and [**glDisable**](crate::context::Context::oxidegl_disable) with argument
/// [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D).
///
/// [**glTexSubImage1D**](crate::context::Context::oxidegl_tex_sub_image1_d)
/// and [**glTextureSubImage1D**](crate::context::Context::oxidegl_texture_sub_image1_d)
/// redefine a contiguous subregion of an existing one-dimensional texture
/// image. The texels referenced by `pixels` replace the portion of the existing
/// texture array with x indices `xoffset` and `[inlineq]`
///
/// If a non-zero named buffer object is bound to the [`GL_PIXEL_UNPACK_BUFFER`](crate::gl_enums::GL_PIXEL_UNPACK_BUFFER)
/// target (see [**glBindBuffer**](crate::context::Context::oxidegl_bind_buffer))
/// while a texture image is specified, `pixels` is treated as a byte offset
/// into the buffer object's data store.
///
/// ### Notes
/// [**glPixelStore**](crate::context::Context::oxidegl_pixel_store) modes
/// affect texture images.
///
/// [**glTexSubImage1D**](crate::context::Context::oxidegl_tex_sub_image1_d)
/// and [**glTextureSubImage1D**](crate::context::Context::oxidegl_texture_sub_image1_d)
/// specify a one-dimensional subtexture for the current texture unit, specified
/// with [**glActiveTexture**](crate::context::Context::oxidegl_active_texture).
///
/// [`GL_STENCIL_INDEX`](crate::gl_enums::GL_STENCIL_INDEX) is accepted for
/// `format` only if the GL version is 4.4 or higher.
///
/// ### Associated Gets
/// [**glGetTexImage**](crate::context::Context::oxidegl_get_tex_image)
///
/// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_PIXEL_UNPACK_BUFFER_BINDING`](crate::gl_enums::GL_PIXEL_UNPACK_BUFFER_BINDING)
impl Context {
    pub unsafe fn oxidegl_tex_sub_image_1_d(
        &mut self,
        target: TextureTarget,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: PixelFormat,
        r#type: PixelType,
        pixels: *const GLvoid,
    ) -> GlFallible {
        panic!("command oxidegl_tex_sub_image1_d not yet implemented");
    }
    pub unsafe fn oxidegl_texture_sub_image_1_d(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: PixelFormat,
        r#type: PixelType,
        pixels: *const GLvoid,
    ) -> GlFallible {
        panic!("command oxidegl_texture_sub_image1_d not yet implemented");
    }
}
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the texture is bound for [**glTexSubImage2D**](crate::context::Context::oxidegl_tex_sub_image2_d).
/// > Must be [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D), [`GL_TEXTURE_CUBE_MAP_POSITIVE_X`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_POSITIVE_X),
/// > [`GL_TEXTURE_CUBE_MAP_NEGATIVE_X`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_NEGATIVE_X),
/// > [`GL_TEXTURE_CUBE_MAP_POSITIVE_Y`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_POSITIVE_Y),
/// > [`GL_TEXTURE_CUBE_MAP_NEGATIVE_Y`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_NEGATIVE_Y),
/// > [`GL_TEXTURE_CUBE_MAP_POSITIVE_Z`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_POSITIVE_Z),
/// > [`GL_TEXTURE_CUBE_MAP_NEGATIVE_Z`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_NEGATIVE_Z),
/// > or [`GL_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_1D_ARRAY).
///
/// `texture`
///
/// > Specifies the texture object name for [**glTextureSubImage2D**](crate::context::Context::oxidegl_texture_sub_image2_d).
/// > The effective target of `texture` must be one of the valid `target` values
/// > above.
///
/// `level`
///
/// > Specifies the level-of-detail number. Level 0 is the base image level.
/// > Level *n* is the *n* th mipmap reduction image.
///
/// `xoffset`
///
/// > Specifies a texel offset in the x direction within the texture array.
///
/// `yoffset`
///
/// > Specifies a texel offset in the y direction within the texture array.
///
/// `width`
///
/// > Specifies the width of the texture subimage.
///
/// `height`
///
/// > Specifies the height of the texture subimage.
///
/// `format`
///
/// > Specifies the format of the pixel data. The following symbolic values are
/// > accepted: [`GL_RED`](crate::gl_enums::GL_RED), [`GL_RG`](crate::gl_enums::GL_RG),
/// > [`GL_RGB`](crate::gl_enums::GL_RGB), [`GL_BGR`](crate::gl_enums::GL_BGR),
/// > [`GL_RGBA`](crate::gl_enums::GL_RGBA), [`GL_BGRA`](crate::gl_enums::GL_BGRA),
/// > [`GL_DEPTH_COMPONENT`](crate::gl_enums::GL_DEPTH_COMPONENT), and [`GL_STENCIL_INDEX`](crate::gl_enums::GL_STENCIL_INDEX).
///
/// `type`
///
/// > Specifies the data type of the pixel data. The following symbolic values
/// > are accepted: [`GL_UNSIGNED_BYTE`](crate::gl_enums::GL_UNSIGNED_BYTE), [`GL_BYTE`](crate::gl_enums::GL_BYTE),
/// > [`GL_UNSIGNED_SHORT`](crate::gl_enums::GL_UNSIGNED_SHORT), [`GL_SHORT`](crate::gl_enums::GL_SHORT),
/// > [`GL_UNSIGNED_INT`](crate::gl_enums::GL_UNSIGNED_INT), [`GL_INT`](crate::gl_enums::GL_INT),
/// > [`GL_FLOAT`](crate::gl_enums::GL_FLOAT), [`GL_UNSIGNED_BYTE_3_3_2`](crate::gl_enums::GL_UNSIGNED_BYTE_3_3_2),
/// > [`GL_UNSIGNED_BYTE_2_3_3_REV`](crate::gl_enums::GL_UNSIGNED_BYTE_2_3_3_REV),
/// > [`GL_UNSIGNED_SHORT_5_6_5`](crate::gl_enums::GL_UNSIGNED_SHORT_5_6_5), [`GL_UNSIGNED_SHORT_5_6_5_REV`](crate::gl_enums::GL_UNSIGNED_SHORT_5_6_5_REV),
/// > [`GL_UNSIGNED_SHORT_4_4_4_4`](crate::gl_enums::GL_UNSIGNED_SHORT_4_4_4_4),
/// > [`GL_UNSIGNED_SHORT_4_4_4_4_REV`](crate::gl_enums::GL_UNSIGNED_SHORT_4_4_4_4_REV),
/// > [`GL_UNSIGNED_SHORT_5_5_5_1`](crate::gl_enums::GL_UNSIGNED_SHORT_5_5_5_1),
/// > [`GL_UNSIGNED_SHORT_1_5_5_5_REV`](crate::gl_enums::GL_UNSIGNED_SHORT_1_5_5_5_REV),
/// > [`GL_UNSIGNED_INT_8_8_8_8`](crate::gl_enums::GL_UNSIGNED_INT_8_8_8_8), [`GL_UNSIGNED_INT_8_8_8_8_REV`](crate::gl_enums::GL_UNSIGNED_INT_8_8_8_8_REV),
/// > [`GL_UNSIGNED_INT_10_10_10_2`](crate::gl_enums::GL_UNSIGNED_INT_10_10_10_2),
/// > and [`GL_UNSIGNED_INT_2_10_10_10_REV`](crate::gl_enums::GL_UNSIGNED_INT_2_10_10_10_REV).
///
/// `pixels`
///
/// > Specifies a pointer to the image data in memory.
///
/// ### Description
/// Texturing maps a portion of a specified texture image onto each graphical
/// primitive for which texturing is enabled.
///
/// [**glTexSubImage2D**](crate::context::Context::oxidegl_tex_sub_image2_d)
/// and [**glTextureSubImage2D**](crate::context::Context::oxidegl_texture_sub_image2_d)
/// redefine a contiguous subregion of an existing two-dimensional or one-dimensional
/// array texture image. The texels referenced by `pixels` replace the portion
/// of the existing texture array with x indices `xoffset` and `[inlineq]`
/// `yoffset` and `[inlineq]`
///
/// If a non-zero named buffer object is bound to the [`GL_PIXEL_UNPACK_BUFFER`](crate::gl_enums::GL_PIXEL_UNPACK_BUFFER)
/// target (see [**glBindBuffer**](crate::context::Context::oxidegl_bind_buffer))
/// while a texture image is specified, `pixels` is treated as a byte offset
/// into the buffer object's data store.
///
/// ### Notes
/// [**glPixelStore**](crate::context::Context::oxidegl_pixel_store) modes
/// affect texture images.
///
/// [**glTexSubImage2D**](crate::context::Context::oxidegl_tex_sub_image2_d)
/// and [**glTextureSubImage3D**](crate::context::Context::oxidegl_texture_sub_image3_d)
/// specify a two-dimensional subtexture for the current texture unit, specified
/// with [**glActiveTexture**](crate::context::Context::oxidegl_active_texture).
///
/// [`GL_STENCIL_INDEX`](crate::gl_enums::GL_STENCIL_INDEX) is accepted for
/// `format` only if the GL version is 4.4 or higher.
///
/// ### Associated Gets
/// [**glGetTexImage**](crate::context::Context::oxidegl_get_tex_image)
///
/// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_PIXEL_UNPACK_BUFFER_BINDING`](crate::gl_enums::GL_PIXEL_UNPACK_BUFFER_BINDING)
impl Context {
    pub unsafe fn oxidegl_tex_sub_image_2_d(
        &mut self,
        target: TextureTarget,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: PixelFormat,
        r#type: PixelType,
        pixels: *const GLvoid,
    ) -> GlFallible {
        panic!("command oxidegl_tex_sub_image2_d not yet implemented");
    }
    pub unsafe fn oxidegl_texture_sub_image_2_d(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: PixelFormat,
        r#type: PixelType,
        pixels: *const GLvoid,
    ) -> GlFallible {
        panic!("command oxidegl_texture_sub_image2_d not yet implemented");
    }
}
/// ### Parameters
/// `target`
///
/// > Specifies the target to which the texture is bound for [**glTexSubImage3D**](crate::context::Context::oxidegl_tex_sub_image3_d).
/// > Must be [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D) or [`GL_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_ARRAY).
///
/// `texture`
///
/// > Specifies the texture object name for [**glTextureSubImage3D**](crate::context::Context::oxidegl_texture_sub_image3_d).
/// > The effective target of `texture` must be one of the valid `target` values
/// > above.
///
/// `level`
///
/// > Specifies the level-of-detail number. Level 0 is the base image level.
/// > Level *n* is the *n* th mipmap reduction image.
///
/// `xoffset`
///
/// > Specifies a texel offset in the x direction within the texture array.
///
/// `yoffset`
///
/// > Specifies a texel offset in the y direction within the texture array.
///
/// `zoffset`
///
/// > Specifies a texel offset in the z direction within the texture array.
///
/// `width`
///
/// > Specifies the width of the texture subimage.
///
/// `height`
///
/// > Specifies the height of the texture subimage.
///
/// `depth`
///
/// > Specifies the depth of the texture subimage.
///
/// `format`
///
/// > Specifies the format of the pixel data. The following symbolic values are
/// > accepted: [`GL_RED`](crate::gl_enums::GL_RED), [`GL_RG`](crate::gl_enums::GL_RG),
/// > [`GL_RGB`](crate::gl_enums::GL_RGB), [`GL_BGR`](crate::gl_enums::GL_BGR),
/// > [`GL_RGBA`](crate::gl_enums::GL_RGBA), [`GL_DEPTH_COMPONENT`](crate::gl_enums::GL_DEPTH_COMPONENT),
/// > and [`GL_STENCIL_INDEX`](crate::gl_enums::GL_STENCIL_INDEX).
///
/// `type`
///
/// > Specifies the data type of the pixel data. The following symbolic values
/// > are accepted: [`GL_UNSIGNED_BYTE`](crate::gl_enums::GL_UNSIGNED_BYTE), [`GL_BYTE`](crate::gl_enums::GL_BYTE),
/// > [`GL_UNSIGNED_SHORT`](crate::gl_enums::GL_UNSIGNED_SHORT), [`GL_SHORT`](crate::gl_enums::GL_SHORT),
/// > [`GL_UNSIGNED_INT`](crate::gl_enums::GL_UNSIGNED_INT), [`GL_INT`](crate::gl_enums::GL_INT),
/// > [`GL_FLOAT`](crate::gl_enums::GL_FLOAT), [`GL_UNSIGNED_BYTE_3_3_2`](crate::gl_enums::GL_UNSIGNED_BYTE_3_3_2),
/// > [`GL_UNSIGNED_BYTE_2_3_3_REV`](crate::gl_enums::GL_UNSIGNED_BYTE_2_3_3_REV),
/// > [`GL_UNSIGNED_SHORT_5_6_5`](crate::gl_enums::GL_UNSIGNED_SHORT_5_6_5), [`GL_UNSIGNED_SHORT_5_6_5_REV`](crate::gl_enums::GL_UNSIGNED_SHORT_5_6_5_REV),
/// > [`GL_UNSIGNED_SHORT_4_4_4_4`](crate::gl_enums::GL_UNSIGNED_SHORT_4_4_4_4),
/// > [`GL_UNSIGNED_SHORT_4_4_4_4_REV`](crate::gl_enums::GL_UNSIGNED_SHORT_4_4_4_4_REV),
/// > [`GL_UNSIGNED_SHORT_5_5_5_1`](crate::gl_enums::GL_UNSIGNED_SHORT_5_5_5_1),
/// > [`GL_UNSIGNED_SHORT_1_5_5_5_REV`](crate::gl_enums::GL_UNSIGNED_SHORT_1_5_5_5_REV),
/// > [`GL_UNSIGNED_INT_8_8_8_8`](crate::gl_enums::GL_UNSIGNED_INT_8_8_8_8), [`GL_UNSIGNED_INT_8_8_8_8_REV`](crate::gl_enums::GL_UNSIGNED_INT_8_8_8_8_REV),
/// > [`GL_UNSIGNED_INT_10_10_10_2`](crate::gl_enums::GL_UNSIGNED_INT_10_10_10_2),
/// > and [`GL_UNSIGNED_INT_2_10_10_10_REV`](crate::gl_enums::GL_UNSIGNED_INT_2_10_10_10_REV).
///
/// `pixels`
///
/// > Specifies a pointer to the image data in memory.
///
/// ### Description
/// Texturing maps a portion of a specified texture image onto each graphical
/// primitive for which texturing is enabled.
///
/// [**glTexSubImage3D**](crate::context::Context::oxidegl_tex_sub_image3_d)
/// and [**glTextureSubImage3D**](crate::context::Context::oxidegl_texture_sub_image3_d)
/// redefine a contiguous subregion of an existing three-dimensional or two-dimensioanl
/// array texture image. The texels referenced by `pixels` replace the portion
/// of the existing texture array with x indices `xoffset` and `[inlineq]`
/// `yoffset` and `[inlineq]` `zoffset` and `[inlineq]`
///
/// If a non-zero named buffer object is bound to the [`GL_PIXEL_UNPACK_BUFFER`](crate::gl_enums::GL_PIXEL_UNPACK_BUFFER)
/// target (see [**glBindBuffer**](crate::context::Context::oxidegl_bind_buffer))
/// while a texture image is specified, `pixels` is treated as a byte offset
/// into the buffer object's data store.
///
/// ### Notes
/// The [**glPixelStore**](crate::context::Context::oxidegl_pixel_store) modes
/// affect texture images.
///
/// [**glTexSubImage3D**](crate::context::Context::oxidegl_tex_sub_image3_d)
/// and [**glTextureSubImage3D**](crate::context::Context::oxidegl_texture_sub_image3_d)
/// specify a three-dimensional or two-dimensional array subtexture for the
/// current texture unit, specified with [**glActiveTexture**](crate::context::Context::oxidegl_active_texture).
///
/// [`GL_STENCIL_INDEX`](crate::gl_enums::GL_STENCIL_INDEX) is accepted for
/// `format` only if the GL version is 4.4 or higher.
///
/// ### Associated Gets
/// [**glGetTexImage**](crate::context::Context::oxidegl_get_tex_image)
///
/// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_PIXEL_UNPACK_BUFFER_BINDING`](crate::gl_enums::GL_PIXEL_UNPACK_BUFFER_BINDING)
impl Context {
    pub unsafe fn oxidegl_tex_sub_image_3_d(
        &mut self,
        target: TextureTarget,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: PixelFormat,
        r#type: PixelType,
        pixels: *const GLvoid,
    ) -> GlFallible {
        panic!("command oxidegl_tex_sub_image_3_d not yet implemented");
    }
    pub unsafe fn oxidegl_texture_sub_image_3_d(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: PixelFormat,
        r#type: PixelType,
        pixels: *const GLvoid,
    ) -> GlFallible {
        panic!("command oxidegl_texture_sub_image_3_d not yet implemented");
    }
}
impl Context {
    /// ### Parameters
    /// `target`
    ///
    /// > Specifies the effective texture target of each created texture.
    ///
    /// `n`
    ///
    /// > Number of texture objects to create.
    ///
    /// `textures`
    ///
    /// > Specifies an array in which names of the new texture objects are stored.
    ///
    /// ### Description
    /// [**glCreateTextures**](crate::context::Context::oxidegl_create_textures)
    /// returns `n` previously unused texture names in `textures`, each representing
    /// a new texture object of the dimensionality and type specified by `target`
    /// and initialized to the default values for that texture type.
    ///
    /// `target` must be one of [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D),
    /// [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D), [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D),
    /// [`GL_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_1D_ARRAY), [`GL_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_ARRAY),
    /// [`GL_TEXTURE_RECTANGLE`](crate::gl_enums::GL_TEXTURE_RECTANGLE), [`GL_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_CUBE_MAP),
    /// [`GL_TEXTURE_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_ARRAY),
    /// [`GL_TEXTURE_BUFFER`](crate::gl_enums::GL_TEXTURE_BUFFER), [`GL_TEXTURE_2D_MULTISAMPLE`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE)
    /// or [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY).
    pub unsafe fn oxidegl_create_textures(
        &mut self,
        target: TextureTarget,
        n: GLsizei,
        textures: *mut GLuint,
    ) {
        // Safety: caller
        unsafe {
            self.gl_state.texture_list.create_obj(
                |name| Texture::new_named(name, target),
                n,
                textures,
            );
        };
    }
    /// ### Parameters
    /// `n`
    ///
    /// > Specifies the number of texture names to be generated.
    ///
    /// `textures`
    ///
    /// > Specifies an array in which the generated texture names are stored.
    ///
    /// ### Description
    /// [**glGenTextures**](crate::context::Context::oxidegl_gen_textures) returns
    /// `n` texture names in `textures`. There is no guarantee that the names form
    /// a contiguous set of integers; however, it is guaranteed that none of the
    /// returned names was in use immediately before the call to [**glGenTextures**](crate::context::Context::oxidegl_gen_textures).
    ///
    /// The generated textures have no dimensionality; they assume the dimensionality
    /// of the texture target to which they are first bound (see [**glBindTexture**](crate::context::Context::oxidegl_bind_texture)
    /// ).
    ///
    /// Texture names returned by a call to [**glGenTextures**](crate::context::Context::oxidegl_gen_textures)
    /// are not returned by subsequent calls, unless they are first deleted with
    /// [**glDeleteTextures**](crate::context::Context::oxidegl_delete_textures).
    ///
    /// ### Associated Gets
    /// [**glIsTexture**](crate::context::Context::oxidegl_is_texture)
    pub unsafe fn oxidegl_gen_textures(&mut self, n: GLsizei, textures: *mut GLuint) {
        // Safety: caller
        unsafe { self.gl_state.texture_list.gen_obj(n, textures) };
    }
    /// ### Parameters
    /// `texture`
    ///
    /// > Specifies a value that may be the name of a texture.
    ///
    /// ### Description
    /// [**glIsTexture**](crate::context::Context::oxidegl_is_texture) returns
    /// [`GL_TRUE`](crate::gl_enums::GL_TRUE) if `texture` is currently the name
    /// of a texture. If `texture` is zero, or is a non-zero value that is not
    /// currently the name of a texture, or if an error occurs, [**glIsTexture**](crate::context::Context::oxidegl_is_texture)
    /// returns [`GL_FALSE`](crate::gl_enums::GL_FALSE).
    ///
    /// A name returned by [**glGenTextures**](crate::context::Context::oxidegl_gen_textures),
    /// but not yet associated with a texture by calling [**glBindTexture**](crate::context::Context::oxidegl_bind_texture),
    /// is not the name of a texture.
    pub fn oxidegl_is_texture(&mut self, texture: GLuint) -> GLboolean {
        self.gl_state.texture_list.is_obj(texture)
    }
    /// ### Parameters
    /// `target`
    ///
    /// > Specifies the target to which the texture is bound. Must be one of [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D),
    /// > [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D), [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D),
    /// > [`GL_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_1D_ARRAY), [`GL_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_ARRAY),
    /// > [`GL_TEXTURE_RECTANGLE`](crate::gl_enums::GL_TEXTURE_RECTANGLE), [`GL_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_CUBE_MAP),
    /// > [`GL_TEXTURE_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_ARRAY),
    /// > [`GL_TEXTURE_BUFFER`](crate::gl_enums::GL_TEXTURE_BUFFER), [`GL_TEXTURE_2D_MULTISAMPLE`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE)
    /// > or [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY).
    ///
    /// `texture`
    ///
    /// > Specifies the name of a texture.
    ///
    /// ### Description
    /// [**glBindTexture**](crate::context::Context::oxidegl_bind_texture) lets
    /// you create or use a named texture. Calling [**glBindTexture**](crate::context::Context::oxidegl_bind_texture)
    /// with `target` set to [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D),
    /// [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D), [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D),
    /// [`GL_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_1D_ARRAY), [`GL_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_ARRAY),
    /// [`GL_TEXTURE_RECTANGLE`](crate::gl_enums::GL_TEXTURE_RECTANGLE), [`GL_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_CUBE_MAP),
    /// [`GL_TEXTURE_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_ARRAY),
    /// [`GL_TEXTURE_BUFFER`](crate::gl_enums::GL_TEXTURE_BUFFER), [`GL_TEXTURE_2D_MULTISAMPLE`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE)
    /// or [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY)
    /// and `texture` set to the name of the new texture binds the texture name
    /// to the target. When a texture is bound to a target, the previous binding
    /// for that target is automatically broken.
    ///
    /// Texture names are unsigned integers. The value zero is reserved to represent
    /// the default texture for each texture target. Texture names and the corresponding
    /// texture contents are local to the shared object space of the current GL
    /// rendering context; two rendering contexts share texture names only if they
    /// explicitly enable sharing between contexts through the appropriate GL windows
    /// interfaces functions.
    ///
    /// You must use [**glGenTextures**](crate::context::Context::oxidegl_gen_textures)
    /// to generate a set of new texture names.
    ///
    /// When a texture is first bound, it assumes the specified target: A texture
    /// first bound to [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D) becomes
    /// one-dimensional texture, a texture first bound to [`GL_TEXTURE_2D`](crate::gl_enums::GL_TEXTURE_2D)
    /// becomes two-dimensional texture, a texture first bound to [`GL_TEXTURE_3D`](crate::gl_enums::GL_TEXTURE_3D)
    /// becomes three-dimensional texture, a texture first bound to [`GL_TEXTURE_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_1D_ARRAY)
    /// becomes one-dimensional array texture, a texture first bound to [`GL_TEXTURE_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_ARRAY)
    /// becomes two-dimensional array texture, a texture first bound to [`GL_TEXTURE_RECTANGLE`](crate::gl_enums::GL_TEXTURE_RECTANGLE)
    /// becomes rectangle texture, a texture first bound to [`GL_TEXTURE_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_CUBE_MAP)
    /// becomes a cube-mapped texture, a texture first bound to [`GL_TEXTURE_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_CUBE_MAP_ARRAY)
    /// becomes a cube-mapped array texture, a texture first bound to [`GL_TEXTURE_BUFFER`](crate::gl_enums::GL_TEXTURE_BUFFER)
    /// becomes a buffer texture, a texture first bound to [`GL_TEXTURE_2D_MULTISAMPLE`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE)
    /// becomes a two-dimensional multisampled texture, and a texture first bound
    /// to [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY)
    /// becomes a two-dimensional multisampled array texture. The state of a one-dimensional
    /// texture immediately after it is first bound is equivalent to the state
    /// of the default [`GL_TEXTURE_1D`](crate::gl_enums::GL_TEXTURE_1D) at GL
    /// initialization, and similarly for the other texture types.
    ///
    /// While a texture is bound, GL operations on the target to which it is bound
    /// affect the bound texture, and queries of the target to which it is bound
    /// return state from the bound texture. In effect, the texture targets become
    /// aliases for the textures currently bound to them, and the texture name
    /// zero refers to the default textures that were bound to them at initialization.
    ///
    /// A texture binding created with [**glBindTexture**](crate::context::Context::oxidegl_bind_texture)
    /// remains active until a different texture is bound to the same target, or
    /// until the bound texture is deleted with [**glDeleteTextures**](crate::context::Context::oxidegl_delete_textures).
    ///
    /// Once created, a named texture may be re-bound to its same original target
    /// as often as needed. It is usually much faster to use [**glBindTexture**](crate::context::Context::oxidegl_bind_texture)
    /// to bind an existing named texture to one of the texture targets than it
    /// is to reload the texture image using [**glTexImage1D**](crate::context::Context::oxidegl_tex_image1_d),
    /// [**glTexImage2D**](crate::context::Context::oxidegl_tex_image2_d), [**glTexImage3D**](crate::context::Context::oxidegl_tex_image3_d)
    /// or another similar function.
    ///
    /// ### Notes
    /// The [`GL_TEXTURE_2D_MULTISAMPLE`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE)
    /// and [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY)
    /// targets are available only if the GL version is 3.2 or higher.
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_TEXTURE_BINDING_1D`](crate::gl_enums::GL_TEXTURE_BINDING_1D),
    /// [`GL_TEXTURE_BINDING_2D`](crate::gl_enums::GL_TEXTURE_BINDING_2D), [`GL_TEXTURE_BINDING_3D`](crate::gl_enums::GL_TEXTURE_BINDING_3D),
    /// [`GL_TEXTURE_BINDING_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_BINDING_1D_ARRAY),
    /// [`GL_TEXTURE_BINDING_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_BINDING_2D_ARRAY),
    /// [`GL_TEXTURE_BINDING_RECTANGLE`](crate::gl_enums::GL_TEXTURE_BINDING_RECTANGLE),
    /// [`GL_TEXTURE_BINDING_BUFFER`](crate::gl_enums::GL_TEXTURE_BINDING_BUFFER),
    /// [`GL_TEXTURE_BINDING_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_BINDING_CUBE_MAP),
    /// [`GL_TEXTURE_BINDING_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_BINDING_CUBE_MAP_ARRAY),
    /// [`GL_TEXTURE_BINDING_2D_MULTISAMPLE`](crate::gl_enums::GL_TEXTURE_BINDING_2D_MULTISAMPLE),
    /// or [`GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY).
    pub fn oxidegl_bind_texture(&mut self, target: TextureTarget, texture: GLuint) -> GlFallible {
        panic!("command oxidegl_bind_texture not yet implemented");
    }
    /// ### Parameters
    /// `first`
    ///
    /// > Specifies the first texture unit to which a texture is to be bound.
    ///
    /// `count`
    ///
    /// > Specifies the number of textures to bind.
    ///
    /// `textures`
    ///
    /// > Specifies the address of an array of names of existing texture objects.
    ///
    /// ### Description
    /// [**glBindTextures**](crate::context::Context::oxidegl_bind_textures) binds
    /// an array of existing texture objects to a specified number of consecutive
    /// texture units. `count` specifies the number of texture objects whose names
    /// are stored in the array `textures`. That number of texture names are read
    /// from the array and bound to the `count` consecutive texture units starting
    /// from `first`. The target, or type of texture is deduced from the texture
    /// object and each texture is bound to the corresponding target of the texture
    /// unit. If the name zero appears in the `textures` array, any existing binding
    /// to any target of the texture unit is reset and the default texture for
    /// that target is bound in its place. Any non-zero entry in `textures` must
    /// be the name of an existing texture object. If `textures` is [`NULL`](crate::gl_enums::NULL)
    /// then it is as if an appropriately sized array containing only zeros had
    /// been specified.
    ///
    /// With the exception that the active texture selector maintains its current
    /// value, [**glBindTextures**](crate::context::Context::oxidegl_bind_textures)
    /// is equivalent to the following pseudo code:
    ///
    /// Each entry in `textures` will be checked individually and if found to be
    /// invalid, the state for that texture unit will not be changed and an error
    /// will be generated. However, the state for other texture units referenced
    /// by the command will still be updated.
    ///
    /// ### Notes
    /// [**glBindTextures**](crate::context::Context::oxidegl_bind_textures) is
    /// available only if the GL version is 4.4 or higher.
    ///
    /// Note that because [**glBindTextures**](crate::context::Context::oxidegl_bind_textures)
    /// cannot create new textures (even if a name passed has been previously generated
    /// by call to [**glGenTextures**](crate::context::Context::oxidegl_gen_textures)
    /// ), names passed to [**glBindTextures**](crate::context::Context::oxidegl_bind_textures)
    /// must have been bound at least once previously via a call to [**glBindTexture**](crate::context::Context::oxidegl_bind_texture).
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_TEXTURE_BINDING_1D`](crate::gl_enums::GL_TEXTURE_BINDING_1D),
    /// [`GL_TEXTURE_BINDING_2D`](crate::gl_enums::GL_TEXTURE_BINDING_2D), [`GL_TEXTURE_BINDING_3D`](crate::gl_enums::GL_TEXTURE_BINDING_3D),
    /// [`GL_TEXTURE_BINDING_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_BINDING_1D_ARRAY),
    /// [`GL_TEXTURE_BINDING_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_BINDING_2D_ARRAY),
    /// [`GL_TEXTURE_BINDING_RECTANGLE`](crate::gl_enums::GL_TEXTURE_BINDING_RECTANGLE),
    /// [`GL_TEXTURE_BINDING_BUFFER`](crate::gl_enums::GL_TEXTURE_BINDING_BUFFER),
    /// [`GL_TEXTURE_BINDING_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_BINDING_CUBE_MAP),
    /// [`GL_TEXTURE_BINDING_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_BINDING_CUBE_MAP),
    /// [`GL_TEXTURE_BINDING_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_BINDING_CUBE_MAP_ARRAY),
    /// [`GL_TEXTURE_BINDING_2D_MULTISAMPLE`](crate::gl_enums::GL_TEXTURE_BINDING_2D_MULTISAMPLE),
    /// or [`GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY).
    pub unsafe fn oxidegl_bind_textures(
        &mut self,
        first: GLuint,
        count: GLsizei,
        textures: *const GLuint,
    ) -> GlFallible {
        panic!("command oxidegl_bind_textures not yet implemented");
    }
    /// ### Parameters
    /// `unit`
    ///
    /// > Specifies the texture unit, to which the texture object should be bound
    /// > to.
    ///
    /// `texture`
    ///
    /// > Specifies the name of a texture.
    ///
    /// ### Description
    /// [**glBindTextureUnit**](crate::context::Context::oxidegl_bind_texture_unit)
    /// binds an existing texture object to the texture unit numbered `unit`.
    ///
    /// `texture` must be zero or the name of an existing texture object. When
    /// `texture` is the name of an existing texture object, that object is bound
    /// to the target, in the corresponding texture unit, that was specified when
    /// the object was created. When `texture` is zero, each of the targets enumerated
    /// at the beginning of this section is reset to its default texture for the
    /// corresponding texture image unit.
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_TEXTURE_BINDING_1D`](crate::gl_enums::GL_TEXTURE_BINDING_1D),
    /// [`GL_TEXTURE_BINDING_2D`](crate::gl_enums::GL_TEXTURE_BINDING_2D), [`GL_TEXTURE_BINDING_3D`](crate::gl_enums::GL_TEXTURE_BINDING_3D),
    /// [`GL_TEXTURE_BINDING_1D_ARRAY`](crate::gl_enums::GL_TEXTURE_BINDING_1D_ARRAY),
    /// [`GL_TEXTURE_BINDING_2D_ARRAY`](crate::gl_enums::GL_TEXTURE_BINDING_2D_ARRAY),
    /// [`GL_TEXTURE_BINDING_RECTANGLE`](crate::gl_enums::GL_TEXTURE_BINDING_RECTANGLE),
    /// [`GL_TEXTURE_BINDING_BUFFER`](crate::gl_enums::GL_TEXTURE_BINDING_BUFFER),
    /// [`GL_TEXTURE_BINDING_CUBE_MAP`](crate::gl_enums::GL_TEXTURE_BINDING_CUBE_MAP),
    /// [`GL_TEXTURE_BINDING_CUBE_MAP_ARRAY`](crate::gl_enums::GL_TEXTURE_BINDING_CUBE_MAP_ARRAY),
    /// [`GL_TEXTURE_BINDING_2D_MULTISAMPLE`](crate::gl_enums::GL_TEXTURE_BINDING_2D_MULTISAMPLE)
    /// or [`GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY`](crate::gl_enums::GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY).
    pub fn oxidegl_bind_texture_unit(&mut self, unit: GLuint, texture: GLuint) -> GlFallible {
        panic!("command oxidegl_bind_texture_unit not yet implemented");
    }
}
