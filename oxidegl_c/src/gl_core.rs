// GL Commands
use crate::context::with_ctx_mut;
use log::trace;
use oxidegl::context::error::GlResult;
use oxidegl::conversions::GLenumExt;
use oxidegl::gl_types::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn glActiveShaderProgram(pipeline: GLuint, program: GLuint) {
    trace!(
        "glActiveShaderProgram called, parameters: pipeline: {:?}, program: {:?} ",
        pipeline, program
    );
    with_ctx_mut(|mut state| state.oxidegl_active_shader_program(pipeline, program))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glActiveTexture(texture: GLenum) {
    trace!(
        "glActiveTexture called, parameters: texture: {:?} ",
        texture
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_active_texture(texture.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glAttachShader(program: GLuint, shader: GLuint) {
    trace!(
        "glAttachShader called, parameters: program: {:?}, shader: {:?} ",
        program, shader
    );
    with_ctx_mut(|mut state| state.oxidegl_attach_shader(program, shader))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBeginConditionalRender(id: GLuint, mode: GLenum) {
    trace!(
        "glBeginConditionalRender called, parameters: id: {:?}, mode: {:?} ",
        id, mode
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_begin_conditional_render(id, mode.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glEndConditionalRender() {
    trace!("glEndConditionalRender called, parameters:  ",);
    with_ctx_mut(|mut state| state.oxidegl_end_conditional_render())
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBeginQuery(target: GLenum, id: GLuint) {
    trace!(
        "glBeginQuery called, parameters: target: {:?}, id: {:?} ",
        target, id
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_begin_query(target.try_into_enum()?, id))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glEndQuery(target: GLenum) {
    trace!("glEndQuery called, parameters: target: {:?} ", target);
    with_ctx_mut(|mut state| GlResult::normalize(state.oxidegl_end_query(target.try_into_enum()?)))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBeginQueryIndexed(target: GLenum, index: GLuint, id: GLuint) {
    trace!(
        "glBeginQueryIndexed called, parameters: target: {:?}, index: {:?}, id: {:?} ",
        target, index, id
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_begin_query_indexed(target.try_into_enum()?, index, id))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glEndQueryIndexed(target: GLenum, index: GLuint) {
    trace!(
        "glEndQueryIndexed called, parameters: target: {:?}, index: {:?} ",
        target, index
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_end_query_indexed(target.try_into_enum()?, index))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBeginTransformFeedback(primitiveMode: GLenum) {
    trace!(
        "glBeginTransformFeedback called, parameters: primitiveMode: {:?} ",
        primitiveMode
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_begin_transform_feedback(primitiveMode.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glEndTransformFeedback() {
    trace!("glEndTransformFeedback called, parameters:  ",);
    with_ctx_mut(|mut state| state.oxidegl_end_transform_feedback())
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) {
    trace!(
        "glBindAttribLocation called, parameters: program: {:?}, index: {:?}, name: {:?} ",
        program, index, name
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_bind_attrib_location(program, index, name) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindBuffer(target: GLenum, buffer: GLuint) {
    trace!(
        "glBindBuffer called, parameters: target: {:?}, buffer: {:?} ",
        target, buffer
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_bind_buffer(target.try_into_enum()?, buffer))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) {
    trace!(
        "glBindBufferBase called, parameters: target: {:?}, index: {:?}, buffer: {:?} ",
        target, index, buffer
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_bind_buffer_base(target.try_into_enum()?, index, buffer))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindBufferRange(
    target: GLenum,
    index: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
) {
    trace!(
        "glBindBufferRange called, parameters: target: {:?}, index: {:?}, buffer: {:?}, offset: {:?}, size: {:?} ",
        target, index, buffer, offset, size
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_bind_buffer_range(
            target.try_into_enum()?,
            index,
            buffer,
            offset,
            size,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindBuffersBase(
    target: GLenum,
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
) {
    trace!(
        "glBindBuffersBase called, parameters: target: {:?}, first: {:?}, count: {:?}, buffers: {:?} ",
        target, first, count, buffers
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_bind_buffers_base(target.try_into_enum()?, first, count, buffers)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindBuffersRange(
    target: GLenum,
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
    offsets: *const GLintptr,
    sizes: *const GLsizeiptr,
) {
    trace!(
        "glBindBuffersRange called, parameters: target: {:?}, first: {:?}, count: {:?}, buffers: {:?}, offsets: {:?}, sizes: {:?} ",
        target, first, count, buffers, offsets, sizes
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_bind_buffers_range(
                target.try_into_enum()?,
                first,
                count,
                buffers,
                offsets,
                sizes,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindFragDataLocation(
    program: GLuint,
    color: GLuint,
    name: *const GLchar,
) {
    trace!(
        "glBindFragDataLocation called, parameters: program: {:?}, color: {:?}, name: {:?} ",
        program, color, name
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_bind_frag_data_location(program, color, name) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindFragDataLocationIndexed(
    program: GLuint,
    colorNumber: GLuint,
    index: GLuint,
    name: *const GLchar,
) {
    trace!(
        "glBindFragDataLocationIndexed called, parameters: program: {:?}, colorNumber: {:?}, index: {:?}, name: {:?} ",
        program, colorNumber, index, name
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_bind_frag_data_location_indexed(program, colorNumber, index, name)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) {
    trace!(
        "glBindFramebuffer called, parameters: target: {:?}, framebuffer: {:?} ",
        target, framebuffer
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_bind_framebuffer(target.try_into_enum()?, framebuffer))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindImageTexture(
    unit: GLuint,
    texture: GLuint,
    level: GLint,
    layered: GLboolean,
    layer: GLint,
    access: GLenum,
    format: GLenum,
) {
    trace!(
        "glBindImageTexture called, parameters: unit: {:?}, texture: {:?}, level: {:?}, layered: {:?}, layer: {:?}, access: {:?}, format: {:?} ",
        unit, texture, level, layered, layer, access, format
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_bind_image_texture(
            unit,
            texture,
            level,
            layered,
            layer,
            access.try_into_enum()?,
            format.try_into_enum()?,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindImageTextures(
    first: GLuint,
    count: GLsizei,
    textures: *const GLuint,
) {
    trace!(
        "glBindImageTextures called, parameters: first: {:?}, count: {:?}, textures: {:?} ",
        first, count, textures
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_bind_image_textures(first, count, textures) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindProgramPipeline(pipeline: GLuint) {
    trace!(
        "glBindProgramPipeline called, parameters: pipeline: {:?} ",
        pipeline
    );
    with_ctx_mut(|mut state| state.oxidegl_bind_program_pipeline(pipeline))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint) {
    trace!(
        "glBindRenderbuffer called, parameters: target: {:?}, renderbuffer: {:?} ",
        target, renderbuffer
    );
    with_ctx_mut(|mut state| state.oxidegl_bind_renderbuffer(target, renderbuffer))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindSampler(unit: GLuint, sampler: GLuint) {
    trace!(
        "glBindSampler called, parameters: unit: {:?}, sampler: {:?} ",
        unit, sampler
    );
    with_ctx_mut(|mut state| state.oxidegl_bind_sampler(unit, sampler))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindSamplers(first: GLuint, count: GLsizei, samplers: *const GLuint) {
    trace!(
        "glBindSamplers called, parameters: first: {:?}, count: {:?}, samplers: {:?} ",
        first, count, samplers
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_bind_samplers(first, count, samplers) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindTexture(target: GLenum, texture: GLuint) {
    trace!(
        "glBindTexture called, parameters: target: {:?}, texture: {:?} ",
        target, texture
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_bind_texture(target.try_into_enum()?, texture))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindTextures(first: GLuint, count: GLsizei, textures: *const GLuint) {
    trace!(
        "glBindTextures called, parameters: first: {:?}, count: {:?}, textures: {:?} ",
        first, count, textures
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_bind_textures(first, count, textures) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindTextureUnit(unit: GLuint, texture: GLuint) {
    trace!(
        "glBindTextureUnit called, parameters: unit: {:?}, texture: {:?} ",
        unit, texture
    );
    with_ctx_mut(|mut state| state.oxidegl_bind_texture_unit(unit, texture))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindTransformFeedback(target: GLenum, id: GLuint) {
    trace!(
        "glBindTransformFeedback called, parameters: target: {:?}, id: {:?} ",
        target, id
    );
    with_ctx_mut(|mut state| state.oxidegl_bind_transform_feedback(target, id))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindVertexArray(array: GLuint) {
    trace!("glBindVertexArray called, parameters: array: {:?} ", array);
    with_ctx_mut(|mut state| state.oxidegl_bind_vertex_array(array))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindVertexBuffer(
    bindingindex: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    stride: GLsizei,
) {
    trace!(
        "glBindVertexBuffer called, parameters: bindingindex: {:?}, buffer: {:?}, offset: {:?}, stride: {:?} ",
        bindingindex, buffer, offset, stride
    );
    with_ctx_mut(|mut state| state.oxidegl_bind_vertex_buffer(bindingindex, buffer, offset, stride))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexArrayVertexBuffer(
    vaobj: GLuint,
    bindingindex: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    stride: GLsizei,
) {
    trace!(
        "glVertexArrayVertexBuffer called, parameters: vaobj: {:?}, bindingindex: {:?}, buffer: {:?}, offset: {:?}, stride: {:?} ",
        vaobj, bindingindex, buffer, offset, stride
    );
    with_ctx_mut(|mut state| {
        state.oxidegl_vertex_array_vertex_buffer(vaobj, bindingindex, buffer, offset, stride)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBindVertexBuffers(
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
    offsets: *const GLintptr,
    strides: *const GLsizei,
) {
    trace!(
        "glBindVertexBuffers called, parameters: first: {:?}, count: {:?}, buffers: {:?}, offsets: {:?}, strides: {:?} ",
        first, count, buffers, offsets, strides
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_bind_vertex_buffers(first, count, buffers, offsets, strides)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexArrayVertexBuffers(
    vaobj: GLuint,
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
    offsets: *const GLintptr,
    strides: *const GLsizei,
) {
    trace!(
        "glVertexArrayVertexBuffers called, parameters: vaobj: {:?}, first: {:?}, count: {:?}, buffers: {:?}, offsets: {:?}, strides: {:?} ",
        vaobj, first, count, buffers, offsets, strides
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_vertex_array_vertex_buffers(vaobj, first, count, buffers, offsets, strides)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    trace!(
        "glBlendColor called, parameters: red: {:?}, green: {:?}, blue: {:?}, alpha: {:?} ",
        red, green, blue, alpha
    );
    with_ctx_mut(|mut state| state.oxidegl_blend_color(red, green, blue, alpha))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBlendEquation(mode: GLenum) {
    trace!("glBlendEquation called, parameters: mode: {:?} ", mode);
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_blend_equation(mode.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBlendEquationi(buf: GLuint, mode: GLenum) {
    trace!(
        "glBlendEquationi called, parameters: buf: {:?}, mode: {:?} ",
        buf, mode
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_blend_equationi(buf, mode.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) {
    trace!(
        "glBlendEquationSeparate called, parameters: modeRGB: {:?}, modeAlpha: {:?} ",
        modeRGB, modeAlpha
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(
            state.oxidegl_blend_equation_separate(
                modeRGB.try_into_enum()?,
                modeAlpha.try_into_enum()?,
            ),
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBlendEquationSeparatei(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum) {
    trace!(
        "glBlendEquationSeparatei called, parameters: buf: {:?}, modeRGB: {:?}, modeAlpha: {:?} ",
        buf, modeRGB, modeAlpha
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_blend_equation_separatei(
            buf,
            modeRGB.try_into_enum()?,
            modeAlpha.try_into_enum()?,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBlendFunc(sfactor: GLenum, dfactor: GLenum) {
    trace!(
        "glBlendFunc called, parameters: sfactor: {:?}, dfactor: {:?} ",
        sfactor, dfactor
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(
            state.oxidegl_blend_func(sfactor.try_into_enum()?, dfactor.try_into_enum()?),
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBlendFunci(buf: GLuint, src: GLenum, dst: GLenum) {
    trace!(
        "glBlendFunci called, parameters: buf: {:?}, src: {:?}, dst: {:?} ",
        buf, src, dst
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_blend_funci(
            buf,
            src.try_into_enum()?,
            dst.try_into_enum()?,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBlendFuncSeparate(
    sfactorRGB: GLenum,
    dfactorRGB: GLenum,
    sfactorAlpha: GLenum,
    dfactorAlpha: GLenum,
) {
    trace!(
        "glBlendFuncSeparate called, parameters: sfactorRGB: {:?}, dfactorRGB: {:?}, sfactorAlpha: {:?}, dfactorAlpha: {:?} ",
        sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_blend_func_separate(
            sfactorRGB.try_into_enum()?,
            dfactorRGB.try_into_enum()?,
            sfactorAlpha.try_into_enum()?,
            dfactorAlpha.try_into_enum()?,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBlendFuncSeparatei(
    buf: GLuint,
    srcRGB: GLenum,
    dstRGB: GLenum,
    srcAlpha: GLenum,
    dstAlpha: GLenum,
) {
    trace!(
        "glBlendFuncSeparatei called, parameters: buf: {:?}, srcRGB: {:?}, dstRGB: {:?}, srcAlpha: {:?}, dstAlpha: {:?} ",
        buf, srcRGB, dstRGB, srcAlpha, dstAlpha
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_blend_func_separatei(
            buf,
            srcRGB.try_into_enum()?,
            dstRGB.try_into_enum()?,
            srcAlpha.try_into_enum()?,
            dstAlpha.try_into_enum()?,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBlitFramebuffer(
    srcX0: GLint,
    srcY0: GLint,
    srcX1: GLint,
    srcY1: GLint,
    dstX0: GLint,
    dstY0: GLint,
    dstX1: GLint,
    dstY1: GLint,
    mask: GLenum,
    filter: GLenum,
) {
    trace!(
        "glBlitFramebuffer called, parameters: srcX0: {:?}, srcY0: {:?}, srcX1: {:?}, srcY1: {:?}, dstX0: {:?}, dstY0: {:?}, dstX1: {:?}, dstY1: {:?}, mask: {:?}, filter: {:?} ",
        srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_blit_framebuffer(
            srcX0,
            srcY0,
            srcX1,
            srcY1,
            dstX0,
            dstY0,
            dstX1,
            dstY1,
            mask.try_into_enum()?,
            filter.try_into_enum()?,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBlitNamedFramebuffer(
    readFramebuffer: GLuint,
    drawFramebuffer: GLuint,
    srcX0: GLint,
    srcY0: GLint,
    srcX1: GLint,
    srcY1: GLint,
    dstX0: GLint,
    dstY0: GLint,
    dstX1: GLint,
    dstY1: GLint,
    mask: GLenum,
    filter: GLenum,
) {
    trace!(
        "glBlitNamedFramebuffer called, parameters: readFramebuffer: {:?}, drawFramebuffer: {:?}, srcX0: {:?}, srcY0: {:?}, srcX1: {:?}, srcY1: {:?}, dstX0: {:?}, dstY0: {:?}, dstX1: {:?}, dstY1: {:?}, mask: {:?}, filter: {:?} ",
        readFramebuffer,
        drawFramebuffer,
        srcX0,
        srcY0,
        srcX1,
        srcY1,
        dstX0,
        dstY0,
        dstX1,
        dstY1,
        mask,
        filter
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_blit_named_framebuffer(
            readFramebuffer,
            drawFramebuffer,
            srcX0,
            srcY0,
            srcX1,
            srcY1,
            dstX0,
            dstY0,
            dstX1,
            dstY1,
            mask.try_into_enum()?,
            filter.try_into_enum()?,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBufferData(
    target: GLenum,
    size: GLsizeiptr,
    data: *const GLvoid,
    usage: GLenum,
) {
    trace!(
        "glBufferData called, parameters: target: {:?}, size: {:?}, data: {:?}, usage: {:?} ",
        target, size, data, usage
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_buffer_data(target.try_into_enum()?, size, data, usage.try_into_enum()?)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glNamedBufferData(
    buffer: GLuint,
    size: GLsizeiptr,
    data: *const GLvoid,
    usage: GLenum,
) {
    trace!(
        "glNamedBufferData called, parameters: buffer: {:?}, size: {:?}, data: {:?}, usage: {:?} ",
        buffer, size, data, usage
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_named_buffer_data(buffer, size, data, usage.try_into_enum()?)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBufferStorage(
    target: GLenum,
    size: GLsizeiptr,
    data: *const GLvoid,
    flags: GLenum,
) {
    trace!(
        "glBufferStorage called, parameters: target: {:?}, size: {:?}, data: {:?}, flags: {:?} ",
        target, size, data, flags
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_buffer_storage(
                target.try_into_enum()?,
                size,
                data,
                flags.try_into_enum()?,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glNamedBufferStorage(
    buffer: GLuint,
    size: GLsizeiptr,
    data: *const GLvoid,
    flags: GLenum,
) {
    trace!(
        "glNamedBufferStorage called, parameters: buffer: {:?}, size: {:?}, data: {:?}, flags: {:?} ",
        buffer, size, data, flags
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_named_buffer_storage(buffer, size, data, flags.try_into_enum()?)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glBufferSubData(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const GLvoid,
) {
    trace!(
        "glBufferSubData called, parameters: target: {:?}, offset: {:?}, size: {:?}, data: {:?} ",
        target, offset, size, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_buffer_sub_data(target.try_into_enum()?, offset, size, data)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glNamedBufferSubData(
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const GLvoid,
) {
    trace!(
        "glNamedBufferSubData called, parameters: buffer: {:?}, offset: {:?}, size: {:?}, data: {:?} ",
        buffer, offset, size, data
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_named_buffer_sub_data(buffer, offset, size, data)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCheckFramebufferStatus(target: GLenum) -> GLenum {
    trace!(
        "glCheckFramebufferStatus called, parameters: target: {:?} ",
        target
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_check_framebuffer_status(target.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCheckNamedFramebufferStatus(
    framebuffer: GLuint,
    target: GLenum,
) -> GLenum {
    trace!(
        "glCheckNamedFramebufferStatus called, parameters: framebuffer: {:?}, target: {:?} ",
        framebuffer, target
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(
            state.oxidegl_check_named_framebuffer_status(framebuffer, target.try_into_enum()?),
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClampColor(target: GLenum, clamp: GLenum) {
    trace!(
        "glClampColor called, parameters: target: {:?}, clamp: {:?} ",
        target, clamp
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_clamp_color(target, clamp.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClear(mask: GLenum) {
    trace!("glClear called, parameters: mask: {:?} ", mask);
    with_ctx_mut(|mut state| GlResult::normalize(state.oxidegl_clear(mask.try_into_enum()?)))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint) {
    trace!(
        "glClearBufferiv called, parameters: buffer: {:?}, drawbuffer: {:?}, value: {:?} ",
        buffer, drawbuffer, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_clear_bufferiv(buffer.try_into_enum()?, drawbuffer, value)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint) {
    trace!(
        "glClearBufferuiv called, parameters: buffer: {:?}, drawbuffer: {:?}, value: {:?} ",
        buffer, drawbuffer, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_clear_bufferuiv(buffer.try_into_enum()?, drawbuffer, value)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) {
    trace!(
        "glClearBufferfv called, parameters: buffer: {:?}, drawbuffer: {:?}, value: {:?} ",
        buffer, drawbuffer, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_clear_bufferfv(buffer.try_into_enum()?, drawbuffer, value)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearBufferfi(
    buffer: GLenum,
    drawbuffer: GLint,
    depth: GLfloat,
    stencil: GLint,
) {
    trace!(
        "glClearBufferfi called, parameters: buffer: {:?}, drawbuffer: {:?}, depth: {:?}, stencil: {:?} ",
        buffer, drawbuffer, depth, stencil
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_clear_bufferfi(
            buffer.try_into_enum()?,
            drawbuffer,
            depth,
            stencil,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearNamedFramebufferiv(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLint,
) {
    trace!(
        "glClearNamedFramebufferiv called, parameters: framebuffer: {:?}, buffer: {:?}, drawbuffer: {:?}, value: {:?} ",
        framebuffer, buffer, drawbuffer, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_clear_named_framebufferiv(
                framebuffer,
                buffer.try_into_enum()?,
                drawbuffer,
                value,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearNamedFramebufferuiv(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLuint,
) {
    trace!(
        "glClearNamedFramebufferuiv called, parameters: framebuffer: {:?}, buffer: {:?}, drawbuffer: {:?}, value: {:?} ",
        framebuffer, buffer, drawbuffer, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_clear_named_framebufferuiv(
                framebuffer,
                buffer.try_into_enum()?,
                drawbuffer,
                value,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearNamedFramebufferfv(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLfloat,
) {
    trace!(
        "glClearNamedFramebufferfv called, parameters: framebuffer: {:?}, buffer: {:?}, drawbuffer: {:?}, value: {:?} ",
        framebuffer, buffer, drawbuffer, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_clear_named_framebufferfv(
                framebuffer,
                buffer.try_into_enum()?,
                drawbuffer,
                value,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearNamedFramebufferfi(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    depth: GLfloat,
    stencil: GLint,
) {
    trace!(
        "glClearNamedFramebufferfi called, parameters: framebuffer: {:?}, buffer: {:?}, drawbuffer: {:?}, depth: {:?}, stencil: {:?} ",
        framebuffer, buffer, drawbuffer, depth, stencil
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_clear_named_framebufferfi(
            framebuffer,
            buffer.try_into_enum()?,
            drawbuffer,
            depth,
            stencil,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearBufferData(
    target: GLenum,
    internalformat: GLenum,
    format: GLenum,
    r#type: GLenum,
    data: *const GLvoid,
) {
    trace!(
        "glClearBufferData called, parameters: target: {:?}, internalformat: {:?}, format: {:?}, r#type: {:?}, data: {:?} ",
        target, internalformat, format, r#type, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_clear_buffer_data(
                target.try_into_enum()?,
                internalformat.try_into_enum()?,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearNamedBufferData(
    buffer: GLuint,
    internalformat: GLenum,
    format: GLenum,
    r#type: GLenum,
    data: *const GLvoid,
) {
    trace!(
        "glClearNamedBufferData called, parameters: buffer: {:?}, internalformat: {:?}, format: {:?}, r#type: {:?}, data: {:?} ",
        buffer, internalformat, format, r#type, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_clear_named_buffer_data(
                buffer,
                internalformat.try_into_enum()?,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearBufferSubData(
    target: GLenum,
    internalformat: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    format: GLenum,
    r#type: GLenum,
    data: *const GLvoid,
) {
    trace!(
        "glClearBufferSubData called, parameters: target: {:?}, internalformat: {:?}, offset: {:?}, size: {:?}, format: {:?}, r#type: {:?}, data: {:?} ",
        target, internalformat, offset, size, format, r#type, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_clear_buffer_sub_data(
                target.try_into_enum()?,
                internalformat.try_into_enum()?,
                offset,
                size,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearNamedBufferSubData(
    buffer: GLuint,
    internalformat: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    format: GLenum,
    r#type: GLenum,
    data: *const GLvoid,
) {
    trace!(
        "glClearNamedBufferSubData called, parameters: buffer: {:?}, internalformat: {:?}, offset: {:?}, size: {:?}, format: {:?}, r#type: {:?}, data: {:?} ",
        buffer, internalformat, offset, size, format, r#type, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_clear_named_buffer_sub_data(
                buffer,
                internalformat.try_into_enum()?,
                offset,
                size,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    trace!(
        "glClearColor called, parameters: red: {:?}, green: {:?}, blue: {:?}, alpha: {:?} ",
        red, green, blue, alpha
    );
    with_ctx_mut(|mut state| state.oxidegl_clear_color(red, green, blue, alpha))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearDepth(depth: GLdouble) {
    trace!("glClearDepth called, parameters: depth: {:?} ", depth);
    with_ctx_mut(|mut state| state.oxidegl_clear_depth(depth))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearDepthf(d: GLfloat) {
    trace!("glClearDepthf called, parameters: d: {:?} ", d);
    with_ctx_mut(|mut state| state.oxidegl_clear_depthf(d))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearStencil(s: GLint) {
    trace!("glClearStencil called, parameters: s: {:?} ", s);
    with_ctx_mut(|mut state| state.oxidegl_clear_stencil(s))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearTexImage(
    texture: GLuint,
    level: GLint,
    format: GLenum,
    r#type: GLenum,
    data: *const GLvoid,
) {
    trace!(
        "glClearTexImage called, parameters: texture: {:?}, level: {:?}, format: {:?}, r#type: {:?}, data: {:?} ",
        texture, level, format, r#type, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_clear_tex_image(
                texture,
                level,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClearTexSubImage(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    r#type: GLenum,
    data: *const GLvoid,
) {
    trace!(
        "glClearTexSubImage called, parameters: texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}, format: {:?}, r#type: {:?}, data: {:?} ",
        texture, level, xoffset, yoffset, zoffset, width, height, depth, format, r#type, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_clear_tex_sub_image(
                texture,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClientWaitSync(
    sync: GLsync,
    flags: GLbitfield,
    timeout: GLuint64,
) -> GLenum {
    trace!(
        "glClientWaitSync called, parameters: sync: {:?}, flags: {:?}, timeout: {:?} ",
        sync, flags, timeout
    );
    with_ctx_mut(|mut state| state.oxidegl_client_wait_sync(sync, flags, timeout))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glClipControl(origin: GLenum, depth: GLenum) {
    trace!(
        "glClipControl called, parameters: origin: {:?}, depth: {:?} ",
        origin, depth
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(
            state.oxidegl_clip_control(origin.try_into_enum()?, depth.try_into_enum()?),
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glColorMask(
    red: GLboolean,
    green: GLboolean,
    blue: GLboolean,
    alpha: GLboolean,
) {
    trace!(
        "glColorMask called, parameters: red: {:?}, green: {:?}, blue: {:?}, alpha: {:?} ",
        red, green, blue, alpha
    );
    with_ctx_mut(|mut state| state.oxidegl_color_mask(red, green, blue, alpha))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glColorMaski(
    index: GLuint,
    r: GLboolean,
    g: GLboolean,
    b: GLboolean,
    a: GLboolean,
) {
    trace!(
        "glColorMaski called, parameters: index: {:?}, r: {:?}, g: {:?}, b: {:?}, a: {:?} ",
        index, r, g, b, a
    );
    with_ctx_mut(|mut state| state.oxidegl_color_maski(index, r, g, b, a))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCompileShader(shader: GLuint) {
    trace!("glCompileShader called, parameters: shader: {:?} ", shader);
    with_ctx_mut(|mut state| state.oxidegl_compile_shader(shader))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCompressedTexImage1D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    trace!(
        "glCompressedTexImage1D called, parameters: target: {:?}, level: {:?}, internalformat: {:?}, width: {:?}, border: {:?}, imageSize: {:?}, data: {:?} ",
        target, level, internalformat, width, border, imageSize, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_compressed_tex_image1_d(
                target.try_into_enum()?,
                level,
                internalformat.try_into_enum()?,
                width,
                border,
                imageSize,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCompressedTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    trace!(
        "glCompressedTexImage2D called, parameters: target: {:?}, level: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, border: {:?}, imageSize: {:?}, data: {:?} ",
        target, level, internalformat, width, height, border, imageSize, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_compressed_tex_image2_d(
                target.try_into_enum()?,
                level,
                internalformat.try_into_enum()?,
                width,
                height,
                border,
                imageSize,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCompressedTexImage3D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    trace!(
        "glCompressedTexImage3D called, parameters: target: {:?}, level: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, depth: {:?}, border: {:?}, imageSize: {:?}, data: {:?} ",
        target, level, internalformat, width, height, depth, border, imageSize, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_compressed_tex_image3_d(
                target.try_into_enum()?,
                level,
                internalformat.try_into_enum()?,
                width,
                height,
                depth,
                border,
                imageSize,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCompressedTexSubImage1D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    trace!(
        "glCompressedTexSubImage1D called, parameters: target: {:?}, level: {:?}, xoffset: {:?}, width: {:?}, format: {:?}, imageSize: {:?}, data: {:?} ",
        target, level, xoffset, width, format, imageSize, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_compressed_tex_sub_image1_d(
                target.try_into_enum()?,
                level,
                xoffset,
                width,
                format.try_into_enum()?,
                imageSize,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCompressedTextureSubImage1D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    trace!(
        "glCompressedTextureSubImage1D called, parameters: texture: {:?}, level: {:?}, xoffset: {:?}, width: {:?}, format: {:?}, imageSize: {:?}, data: {:?} ",
        texture, level, xoffset, width, format, imageSize, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_compressed_texture_sub_image1_d(
                texture,
                level,
                xoffset,
                width,
                format.try_into_enum()?,
                imageSize,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCompressedTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    trace!(
        "glCompressedTexSubImage2D called, parameters: target: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, width: {:?}, height: {:?}, format: {:?}, imageSize: {:?}, data: {:?} ",
        target, level, xoffset, yoffset, width, height, format, imageSize, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_compressed_tex_sub_image2_d(
                target.try_into_enum()?,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format.try_into_enum()?,
                imageSize,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCompressedTextureSubImage2D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    trace!(
        "glCompressedTextureSubImage2D called, parameters: texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, width: {:?}, height: {:?}, format: {:?}, imageSize: {:?}, data: {:?} ",
        texture, level, xoffset, yoffset, width, height, format, imageSize, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_compressed_texture_sub_image2_d(
                texture,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format.try_into_enum()?,
                imageSize,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCompressedTexSubImage3D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    trace!(
        "glCompressedTexSubImage3D called, parameters: target: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}, format: {:?}, imageSize: {:?}, data: {:?} ",
        target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_compressed_tex_sub_image3_d(
                target.try_into_enum()?,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format.try_into_enum()?,
                imageSize,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCompressedTextureSubImage3D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    trace!(
        "glCompressedTextureSubImage3D called, parameters: texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}, format: {:?}, imageSize: {:?}, data: {:?} ",
        texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_compressed_texture_sub_image3_d(
                texture,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format.try_into_enum()?,
                imageSize,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCopyBufferSubData(
    readTarget: GLenum,
    writeTarget: GLenum,
    readOffset: GLintptr,
    writeOffset: GLintptr,
    size: GLsizeiptr,
) {
    trace!(
        "glCopyBufferSubData called, parameters: readTarget: {:?}, writeTarget: {:?}, readOffset: {:?}, writeOffset: {:?}, size: {:?} ",
        readTarget, writeTarget, readOffset, writeOffset, size
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_copy_buffer_sub_data(
            readTarget.try_into_enum()?,
            writeTarget.try_into_enum()?,
            readOffset,
            writeOffset,
            size,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCopyNamedBufferSubData(
    readBuffer: GLuint,
    writeBuffer: GLuint,
    readOffset: GLintptr,
    writeOffset: GLintptr,
    size: GLsizeiptr,
) {
    trace!(
        "glCopyNamedBufferSubData called, parameters: readBuffer: {:?}, writeBuffer: {:?}, readOffset: {:?}, writeOffset: {:?}, size: {:?} ",
        readBuffer, writeBuffer, readOffset, writeOffset, size
    );
    with_ctx_mut(|mut state| {
        state.oxidegl_copy_named_buffer_sub_data(
            readBuffer,
            writeBuffer,
            readOffset,
            writeOffset,
            size,
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCopyImageSubData(
    srcName: GLuint,
    srcTarget: GLenum,
    srcLevel: GLint,
    srcX: GLint,
    srcY: GLint,
    srcZ: GLint,
    dstName: GLuint,
    dstTarget: GLenum,
    dstLevel: GLint,
    dstX: GLint,
    dstY: GLint,
    dstZ: GLint,
    srcWidth: GLsizei,
    srcHeight: GLsizei,
    srcDepth: GLsizei,
) {
    trace!(
        "glCopyImageSubData called, parameters: srcName: {:?}, srcTarget: {:?}, srcLevel: {:?}, srcX: {:?}, srcY: {:?}, srcZ: {:?}, dstName: {:?}, dstTarget: {:?}, dstLevel: {:?}, dstX: {:?}, dstY: {:?}, dstZ: {:?}, srcWidth: {:?}, srcHeight: {:?}, srcDepth: {:?} ",
        srcName,
        srcTarget,
        srcLevel,
        srcX,
        srcY,
        srcZ,
        dstName,
        dstTarget,
        dstLevel,
        dstX,
        dstY,
        dstZ,
        srcWidth,
        srcHeight,
        srcDepth
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_copy_image_sub_data(
            srcName,
            srcTarget.try_into_enum()?,
            srcLevel,
            srcX,
            srcY,
            srcZ,
            dstName,
            dstTarget.try_into_enum()?,
            dstLevel,
            dstX,
            dstY,
            dstZ,
            srcWidth,
            srcHeight,
            srcDepth,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCopyTexImage1D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    border: GLint,
) {
    trace!(
        "glCopyTexImage1D called, parameters: target: {:?}, level: {:?}, internalformat: {:?}, x: {:?}, y: {:?}, width: {:?}, border: {:?} ",
        target, level, internalformat, x, y, width, border
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_copy_tex_image1_d(
            target.try_into_enum()?,
            level,
            internalformat.try_into_enum()?,
            x,
            y,
            width,
            border,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCopyTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
) {
    trace!(
        "glCopyTexImage2D called, parameters: target: {:?}, level: {:?}, internalformat: {:?}, x: {:?}, y: {:?}, width: {:?}, height: {:?}, border: {:?} ",
        target, level, internalformat, x, y, width, height, border
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_copy_tex_image2_d(
            target.try_into_enum()?,
            level,
            internalformat.try_into_enum()?,
            x,
            y,
            width,
            height,
            border,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCopyTexSubImage1D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
) {
    trace!(
        "glCopyTexSubImage1D called, parameters: target: {:?}, level: {:?}, xoffset: {:?}, x: {:?}, y: {:?}, width: {:?} ",
        target, level, xoffset, x, y, width
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_copy_tex_sub_image1_d(
            target.try_into_enum()?,
            level,
            xoffset,
            x,
            y,
            width,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCopyTextureSubImage1D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
) {
    trace!(
        "glCopyTextureSubImage1D called, parameters: texture: {:?}, level: {:?}, xoffset: {:?}, x: {:?}, y: {:?}, width: {:?} ",
        texture, level, xoffset, x, y, width
    );
    with_ctx_mut(|mut state| {
        state.oxidegl_copy_texture_sub_image1_d(texture, level, xoffset, x, y, width)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCopyTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    trace!(
        "glCopyTexSubImage2D called, parameters: target: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, x: {:?}, y: {:?}, width: {:?}, height: {:?} ",
        target, level, xoffset, yoffset, x, y, width, height
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_copy_tex_sub_image2_d(
            target.try_into_enum()?,
            level,
            xoffset,
            yoffset,
            x,
            y,
            width,
            height,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCopyTextureSubImage2D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    trace!(
        "glCopyTextureSubImage2D called, parameters: texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, x: {:?}, y: {:?}, width: {:?}, height: {:?} ",
        texture, level, xoffset, yoffset, x, y, width, height
    );
    with_ctx_mut(|mut state| {
        state.oxidegl_copy_texture_sub_image2_d(
            texture, level, xoffset, yoffset, x, y, width, height,
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCopyTexSubImage3D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    trace!(
        "glCopyTexSubImage3D called, parameters: target: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, x: {:?}, y: {:?}, width: {:?}, height: {:?} ",
        target, level, xoffset, yoffset, zoffset, x, y, width, height
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_copy_tex_sub_image3_d(
            target.try_into_enum()?,
            level,
            xoffset,
            yoffset,
            zoffset,
            x,
            y,
            width,
            height,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCopyTextureSubImage3D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    trace!(
        "glCopyTextureSubImage3D called, parameters: texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, x: {:?}, y: {:?}, width: {:?}, height: {:?} ",
        texture, level, xoffset, yoffset, zoffset, x, y, width, height
    );
    with_ctx_mut(|mut state| {
        state.oxidegl_copy_texture_sub_image3_d(
            texture, level, xoffset, yoffset, zoffset, x, y, width, height,
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCreateBuffers(n: GLsizei, buffers: *mut GLuint) {
    trace!(
        "glCreateBuffers called, parameters: n: {:?}, buffers: {:?} ",
        n, buffers
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_create_buffers(n, buffers) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCreateFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
    trace!(
        "glCreateFramebuffers called, parameters: n: {:?}, framebuffers: {:?} ",
        n, framebuffers
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_create_framebuffers(n, framebuffers) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCreateProgram() -> GLuint {
    trace!("glCreateProgram called, parameters:  ",);
    with_ctx_mut(|mut state| state.oxidegl_create_program())
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCreateProgramPipelines(n: GLsizei, pipelines: *mut GLuint) {
    trace!(
        "glCreateProgramPipelines called, parameters: n: {:?}, pipelines: {:?} ",
        n, pipelines
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_create_program_pipelines(n, pipelines) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCreateQueries(target: GLenum, n: GLsizei, ids: *mut GLuint) {
    trace!(
        "glCreateQueries called, parameters: target: {:?}, n: {:?}, ids: {:?} ",
        target, n, ids
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_create_queries(target.try_into_enum()?, n, ids)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCreateRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
    trace!(
        "glCreateRenderbuffers called, parameters: n: {:?}, renderbuffers: {:?} ",
        n, renderbuffers
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_create_renderbuffers(n, renderbuffers) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCreateSamplers(n: GLsizei, samplers: *mut GLuint) {
    trace!(
        "glCreateSamplers called, parameters: n: {:?}, samplers: {:?} ",
        n, samplers
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_create_samplers(n, samplers) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCreateShader(r#type: GLenum) -> GLuint {
    trace!("glCreateShader called, parameters: r#type: {:?} ", r#type);
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_create_shader(r#type.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCreateShaderProgramv(
    r#type: GLenum,
    count: GLsizei,
    strings: *const *const GLchar,
) -> GLuint {
    trace!(
        "glCreateShaderProgramv called, parameters: r#type: {:?}, count: {:?}, strings: {:?} ",
        r#type, count, strings
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_create_shader_programv(r#type.try_into_enum()?, count, strings)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCreateTextures(target: GLenum, n: GLsizei, textures: *mut GLuint) {
    trace!(
        "glCreateTextures called, parameters: target: {:?}, n: {:?}, textures: {:?} ",
        target, n, textures
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_create_textures(target.try_into_enum()?, n, textures)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCreateTransformFeedbacks(n: GLsizei, ids: *mut GLuint) {
    trace!(
        "glCreateTransformFeedbacks called, parameters: n: {:?}, ids: {:?} ",
        n, ids
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_create_transform_feedbacks(n, ids) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCreateVertexArrays(n: GLsizei, arrays: *mut GLuint) {
    trace!(
        "glCreateVertexArrays called, parameters: n: {:?}, arrays: {:?} ",
        n, arrays
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_create_vertex_arrays(n, arrays) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glCullFace(mode: GLenum) {
    trace!("glCullFace called, parameters: mode: {:?} ", mode);
    with_ctx_mut(|mut state| GlResult::normalize(state.oxidegl_cull_face(mode.try_into_enum()?)))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDebugMessageCallback(callback: GLDEBUGPROC, userParam: *const GLvoid) {
    trace!(
        "glDebugMessageCallback called, parameters: callback: {:?}, userParam: {:?} ",
        callback, userParam
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_debug_message_callback(callback, userParam) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDebugMessageControl(
    source: GLenum,
    r#type: GLenum,
    severity: GLenum,
    count: GLsizei,
    ids: *const GLuint,
    enabled: GLboolean,
) {
    trace!(
        "glDebugMessageControl called, parameters: source: {:?}, r#type: {:?}, severity: {:?}, count: {:?}, ids: {:?}, enabled: {:?} ",
        source, r#type, severity, count, ids, enabled
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_debug_message_control(
                source.try_into_enum()?,
                r#type.try_into_enum()?,
                severity.try_into_enum()?,
                count,
                ids,
                enabled,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDebugMessageInsert(
    source: GLenum,
    r#type: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    buf: *const GLchar,
) {
    trace!(
        "glDebugMessageInsert called, parameters: source: {:?}, r#type: {:?}, id: {:?}, severity: {:?}, length: {:?}, buf: {:?} ",
        source, r#type, id, severity, length, buf
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_debug_message_insert(
                source.try_into_enum()?,
                r#type.try_into_enum()?,
                id,
                severity.try_into_enum()?,
                length,
                buf,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) {
    trace!(
        "glDeleteBuffers called, parameters: n: {:?}, buffers: {:?} ",
        n, buffers
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_delete_buffers(n, buffers) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
    trace!(
        "glDeleteFramebuffers called, parameters: n: {:?}, framebuffers: {:?} ",
        n, framebuffers
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_delete_framebuffers(n, framebuffers) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDeleteProgram(program: GLuint) {
    trace!(
        "glDeleteProgram called, parameters: program: {:?} ",
        program
    );
    with_ctx_mut(|mut state| state.oxidegl_delete_program(program))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDeleteProgramPipelines(n: GLsizei, pipelines: *const GLuint) {
    trace!(
        "glDeleteProgramPipelines called, parameters: n: {:?}, pipelines: {:?} ",
        n, pipelines
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_delete_program_pipelines(n, pipelines) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDeleteQueries(n: GLsizei, ids: *const GLuint) {
    trace!(
        "glDeleteQueries called, parameters: n: {:?}, ids: {:?} ",
        n, ids
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_delete_queries(n, ids) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
    trace!(
        "glDeleteRenderbuffers called, parameters: n: {:?}, renderbuffers: {:?} ",
        n, renderbuffers
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_delete_renderbuffers(n, renderbuffers) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDeleteSamplers(count: GLsizei, samplers: *const GLuint) {
    trace!(
        "glDeleteSamplers called, parameters: count: {:?}, samplers: {:?} ",
        count, samplers
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_delete_samplers(count, samplers) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDeleteShader(shader: GLuint) {
    trace!("glDeleteShader called, parameters: shader: {:?} ", shader);
    with_ctx_mut(|mut state| state.oxidegl_delete_shader(shader))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDeleteSync(sync: GLsync) {
    trace!("glDeleteSync called, parameters: sync: {:?} ", sync);
    with_ctx_mut(|mut state| state.oxidegl_delete_sync(sync))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDeleteTextures(n: GLsizei, textures: *const GLuint) {
    trace!(
        "glDeleteTextures called, parameters: n: {:?}, textures: {:?} ",
        n, textures
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_delete_textures(n, textures) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint) {
    trace!(
        "glDeleteTransformFeedbacks called, parameters: n: {:?}, ids: {:?} ",
        n, ids
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_delete_transform_feedbacks(n, ids) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint) {
    trace!(
        "glDeleteVertexArrays called, parameters: n: {:?}, arrays: {:?} ",
        n, arrays
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_delete_vertex_arrays(n, arrays) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDepthFunc(func: GLenum) {
    trace!("glDepthFunc called, parameters: func: {:?} ", func);
    with_ctx_mut(|mut state| GlResult::normalize(state.oxidegl_depth_func(func.try_into_enum()?)))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDepthMask(flag: GLboolean) {
    trace!("glDepthMask called, parameters: flag: {:?} ", flag);
    with_ctx_mut(|mut state| state.oxidegl_depth_mask(flag))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDepthRange(n: GLdouble, f: GLdouble) {
    trace!("glDepthRange called, parameters: n: {:?}, f: {:?} ", n, f);
    with_ctx_mut(|mut state| state.oxidegl_depth_range(n, f))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDepthRangef(n: GLfloat, f: GLfloat) {
    trace!("glDepthRangef called, parameters: n: {:?}, f: {:?} ", n, f);
    with_ctx_mut(|mut state| state.oxidegl_depth_rangef(n, f))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDepthRangeArrayv(first: GLuint, count: GLsizei, v: *const GLdouble) {
    trace!(
        "glDepthRangeArrayv called, parameters: first: {:?}, count: {:?}, v: {:?} ",
        first, count, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_depth_range_arrayv(first, count, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDepthRangeIndexed(index: GLuint, n: GLdouble, f: GLdouble) {
    trace!(
        "glDepthRangeIndexed called, parameters: index: {:?}, n: {:?}, f: {:?} ",
        index, n, f
    );
    with_ctx_mut(|mut state| state.oxidegl_depth_range_indexed(index, n, f))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDetachShader(program: GLuint, shader: GLuint) {
    trace!(
        "glDetachShader called, parameters: program: {:?}, shader: {:?} ",
        program, shader
    );
    with_ctx_mut(|mut state| state.oxidegl_detach_shader(program, shader))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDispatchCompute(
    num_groups_x: GLuint,
    num_groups_y: GLuint,
    num_groups_z: GLuint,
) {
    trace!(
        "glDispatchCompute called, parameters: num_groups_x: {:?}, num_groups_y: {:?}, num_groups_z: {:?} ",
        num_groups_x, num_groups_y, num_groups_z
    );
    with_ctx_mut(|mut state| {
        state.oxidegl_dispatch_compute(num_groups_x, num_groups_y, num_groups_z)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDispatchComputeIndirect(indirect: GLintptr) {
    trace!(
        "glDispatchComputeIndirect called, parameters: indirect: {:?} ",
        indirect
    );
    with_ctx_mut(|mut state| state.oxidegl_dispatch_compute_indirect(indirect))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei) {
    trace!(
        "glDrawArrays called, parameters: mode: {:?}, first: {:?}, count: {:?} ",
        mode, first, count
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_draw_arrays(mode.try_into_enum()?, first, count))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawArraysIndirect(mode: GLenum, indirect: *const GLvoid) {
    trace!(
        "glDrawArraysIndirect called, parameters: mode: {:?}, indirect: {:?} ",
        mode, indirect
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_draw_arrays_indirect(mode.try_into_enum()?, indirect)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawArraysInstanced(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    instancecount: GLsizei,
) {
    trace!(
        "glDrawArraysInstanced called, parameters: mode: {:?}, first: {:?}, count: {:?}, instancecount: {:?} ",
        mode, first, count, instancecount
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_draw_arrays_instanced(
            mode.try_into_enum()?,
            first,
            count,
            instancecount,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawArraysInstancedBaseInstance(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    instancecount: GLsizei,
    baseinstance: GLuint,
) {
    trace!(
        "glDrawArraysInstancedBaseInstance called, parameters: mode: {:?}, first: {:?}, count: {:?}, instancecount: {:?}, baseinstance: {:?} ",
        mode, first, count, instancecount, baseinstance
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_draw_arrays_instanced_base_instance(
            mode.try_into_enum()?,
            first,
            count,
            instancecount,
            baseinstance,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawBuffer(buf: GLenum) {
    trace!("glDrawBuffer called, parameters: buf: {:?} ", buf);
    with_ctx_mut(|mut state| GlResult::normalize(state.oxidegl_draw_buffer(buf.try_into_enum()?)))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glNamedFramebufferDrawBuffer(framebuffer: GLuint, buf: GLenum) {
    trace!(
        "glNamedFramebufferDrawBuffer called, parameters: framebuffer: {:?}, buf: {:?} ",
        framebuffer, buf
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(
            state.oxidegl_named_framebuffer_draw_buffer(framebuffer, buf.try_into_enum()?),
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawBuffers(n: GLsizei, bufs: GLenum) {
    trace!(
        "glDrawBuffers called, parameters: n: {:?}, bufs: {:?} ",
        n, bufs
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_draw_buffers(n, bufs.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glNamedFramebufferDrawBuffers(
    framebuffer: GLuint,
    n: GLsizei,
    bufs: GLenum,
) {
    trace!(
        "glNamedFramebufferDrawBuffers called, parameters: framebuffer: {:?}, n: {:?}, bufs: {:?} ",
        framebuffer, n, bufs
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_named_framebuffer_draw_buffers(
            framebuffer,
            n,
            bufs.try_into_enum()?,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawElements(
    mode: GLenum,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
) {
    trace!(
        "glDrawElements called, parameters: mode: {:?}, count: {:?}, r#type: {:?}, indices: {:?} ",
        mode, count, r#type, indices
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_draw_elements(
                mode.try_into_enum()?,
                count,
                r#type.try_into_enum()?,
                indices,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawElementsBaseVertex(
    mode: GLenum,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
    basevertex: GLint,
) {
    trace!(
        "glDrawElementsBaseVertex called, parameters: mode: {:?}, count: {:?}, r#type: {:?}, indices: {:?}, basevertex: {:?} ",
        mode, count, r#type, indices, basevertex
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_draw_elements_base_vertex(
                mode.try_into_enum()?,
                count,
                r#type.try_into_enum()?,
                indices,
                basevertex,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawElementsIndirect(
    mode: GLenum,
    r#type: GLenum,
    indirect: *const GLvoid,
) {
    trace!(
        "glDrawElementsIndirect called, parameters: mode: {:?}, r#type: {:?}, indirect: {:?} ",
        mode, r#type, indirect
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_draw_elements_indirect(
                mode.try_into_enum()?,
                r#type.try_into_enum()?,
                indirect,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawElementsInstanced(
    mode: GLenum,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
    instancecount: GLsizei,
) {
    trace!(
        "glDrawElementsInstanced called, parameters: mode: {:?}, count: {:?}, r#type: {:?}, indices: {:?}, instancecount: {:?} ",
        mode, count, r#type, indices, instancecount
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_draw_elements_instanced(
                mode.try_into_enum()?,
                count,
                r#type.try_into_enum()?,
                indices,
                instancecount,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawElementsInstancedBaseInstance(
    mode: GLenum,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
    instancecount: GLsizei,
    baseinstance: GLuint,
) {
    trace!(
        "glDrawElementsInstancedBaseInstance called, parameters: mode: {:?}, count: {:?}, r#type: {:?}, indices: {:?}, instancecount: {:?}, baseinstance: {:?} ",
        mode, count, r#type, indices, instancecount, baseinstance
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_draw_elements_instanced_base_instance(
                mode.try_into_enum()?,
                count,
                r#type.try_into_enum()?,
                indices,
                instancecount,
                baseinstance,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawElementsInstancedBaseVertex(
    mode: GLenum,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
    instancecount: GLsizei,
    basevertex: GLint,
) {
    trace!(
        "glDrawElementsInstancedBaseVertex called, parameters: mode: {:?}, count: {:?}, r#type: {:?}, indices: {:?}, instancecount: {:?}, basevertex: {:?} ",
        mode, count, r#type, indices, instancecount, basevertex
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_draw_elements_instanced_base_vertex(
                mode.try_into_enum()?,
                count,
                r#type.try_into_enum()?,
                indices,
                instancecount,
                basevertex,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawElementsInstancedBaseVertexBaseInstance(
    mode: GLenum,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
    instancecount: GLsizei,
    basevertex: GLint,
    baseinstance: GLuint,
) {
    trace!(
        "glDrawElementsInstancedBaseVertexBaseInstance called, parameters: mode: {:?}, count: {:?}, r#type: {:?}, indices: {:?}, instancecount: {:?}, basevertex: {:?}, baseinstance: {:?} ",
        mode, count, r#type, indices, instancecount, basevertex, baseinstance
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_draw_elements_instanced_base_vertex_base_instance(
                mode.try_into_enum()?,
                count,
                r#type.try_into_enum()?,
                indices,
                instancecount,
                basevertex,
                baseinstance,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawRangeElements(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
) {
    trace!(
        "glDrawRangeElements called, parameters: mode: {:?}, start: {:?}, end: {:?}, count: {:?}, r#type: {:?}, indices: {:?} ",
        mode, start, end, count, r#type, indices
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_draw_range_elements(
                mode.try_into_enum()?,
                start,
                end,
                count,
                r#type.try_into_enum()?,
                indices,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawRangeElementsBaseVertex(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
    basevertex: GLint,
) {
    trace!(
        "glDrawRangeElementsBaseVertex called, parameters: mode: {:?}, start: {:?}, end: {:?}, count: {:?}, r#type: {:?}, indices: {:?}, basevertex: {:?} ",
        mode, start, end, count, r#type, indices, basevertex
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_draw_range_elements_base_vertex(
                mode.try_into_enum()?,
                start,
                end,
                count,
                r#type.try_into_enum()?,
                indices,
                basevertex,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawTransformFeedback(mode: GLenum, id: GLuint) {
    trace!(
        "glDrawTransformFeedback called, parameters: mode: {:?}, id: {:?} ",
        mode, id
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_draw_transform_feedback(mode.try_into_enum()?, id))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawTransformFeedbackInstanced(
    mode: GLenum,
    id: GLuint,
    instancecount: GLsizei,
) {
    trace!(
        "glDrawTransformFeedbackInstanced called, parameters: mode: {:?}, id: {:?}, instancecount: {:?} ",
        mode, id, instancecount
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_draw_transform_feedback_instanced(
            mode.try_into_enum()?,
            id,
            instancecount,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawTransformFeedbackStream(mode: GLenum, id: GLuint, stream: GLuint) {
    trace!(
        "glDrawTransformFeedbackStream called, parameters: mode: {:?}, id: {:?}, stream: {:?} ",
        mode, id, stream
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_draw_transform_feedback_stream(
            mode.try_into_enum()?,
            id,
            stream,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDrawTransformFeedbackStreamInstanced(
    mode: GLenum,
    id: GLuint,
    stream: GLuint,
    instancecount: GLsizei,
) {
    trace!(
        "glDrawTransformFeedbackStreamInstanced called, parameters: mode: {:?}, id: {:?}, stream: {:?}, instancecount: {:?} ",
        mode, id, stream, instancecount
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_draw_transform_feedback_stream_instanced(
            mode.try_into_enum()?,
            id,
            stream,
            instancecount,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDisable(cap: GLenum) {
    trace!("glDisable called, parameters: cap: {:?} ", cap);
    with_ctx_mut(|mut state| GlResult::normalize(state.oxidegl_disable(cap.try_into_enum()?)))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glEnable(cap: GLenum) {
    trace!("glEnable called, parameters: cap: {:?} ", cap);
    with_ctx_mut(|mut state| GlResult::normalize(state.oxidegl_enable(cap.try_into_enum()?)))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glEnablei(target: GLenum, index: GLuint) {
    trace!(
        "glEnablei called, parameters: target: {:?}, index: {:?} ",
        target, index
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_enablei(target.try_into_enum()?, index))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDisablei(target: GLenum, index: GLuint) {
    trace!(
        "glDisablei called, parameters: target: {:?}, index: {:?} ",
        target, index
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_disablei(target.try_into_enum()?, index))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDisableVertexAttribArray(index: GLuint) {
    trace!(
        "glDisableVertexAttribArray called, parameters: index: {:?} ",
        index
    );
    with_ctx_mut(|mut state| state.oxidegl_disable_vertex_attrib_array(index))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glEnableVertexAttribArray(index: GLuint) {
    trace!(
        "glEnableVertexAttribArray called, parameters: index: {:?} ",
        index
    );
    with_ctx_mut(|mut state| state.oxidegl_enable_vertex_attrib_array(index))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glDisableVertexArrayAttrib(vaobj: GLuint, index: GLuint) {
    trace!(
        "glDisableVertexArrayAttrib called, parameters: vaobj: {:?}, index: {:?} ",
        vaobj, index
    );
    with_ctx_mut(|mut state| state.oxidegl_disable_vertex_array_attrib(vaobj, index))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glEnableVertexArrayAttrib(vaobj: GLuint, index: GLuint) {
    trace!(
        "glEnableVertexArrayAttrib called, parameters: vaobj: {:?}, index: {:?} ",
        vaobj, index
    );
    with_ctx_mut(|mut state| state.oxidegl_enable_vertex_array_attrib(vaobj, index))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glFenceSync(condition: GLenum, flags: GLbitfield) -> GLsync {
    trace!(
        "glFenceSync called, parameters: condition: {:?}, flags: {:?} ",
        condition, flags
    );
    with_ctx_mut(|mut state| state.oxidegl_fence_sync(condition, flags))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glFinish() {
    trace!("glFinish called, parameters:  ",);
    with_ctx_mut(|mut state| state.oxidegl_finish())
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glFlush() {
    trace!("glFlush called, parameters:  ",);
    with_ctx_mut(|mut state| state.oxidegl_flush())
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glFlushMappedBufferRange(
    target: GLenum,
    offset: GLintptr,
    length: GLsizeiptr,
) {
    trace!(
        "glFlushMappedBufferRange called, parameters: target: {:?}, offset: {:?}, length: {:?} ",
        target, offset, length
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_flush_mapped_buffer_range(
            target.try_into_enum()?,
            offset,
            length,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glFlushMappedNamedBufferRange(
    buffer: GLuint,
    offset: GLintptr,
    length: GLsizeiptr,
) {
    trace!(
        "glFlushMappedNamedBufferRange called, parameters: buffer: {:?}, offset: {:?}, length: {:?} ",
        buffer, offset, length
    );
    with_ctx_mut(|mut state| state.oxidegl_flush_mapped_named_buffer_range(buffer, offset, length))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glFramebufferParameteri(target: GLenum, pname: GLenum, param: GLint) {
    trace!(
        "glFramebufferParameteri called, parameters: target: {:?}, pname: {:?}, param: {:?} ",
        target, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_framebuffer_parameteri(
            target.try_into_enum()?,
            pname.try_into_enum()?,
            param,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glNamedFramebufferParameteri(
    framebuffer: GLuint,
    pname: GLenum,
    param: GLint,
) {
    trace!(
        "glNamedFramebufferParameteri called, parameters: framebuffer: {:?}, pname: {:?}, param: {:?} ",
        framebuffer, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_named_framebuffer_parameteri(
            framebuffer,
            pname.try_into_enum()?,
            param,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glFramebufferRenderbuffer(
    target: GLenum,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
) {
    trace!(
        "glFramebufferRenderbuffer called, parameters: target: {:?}, attachment: {:?}, renderbuffertarget: {:?}, renderbuffer: {:?} ",
        target, attachment, renderbuffertarget, renderbuffer
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_framebuffer_renderbuffer(
            target.try_into_enum()?,
            attachment.try_into_enum()?,
            renderbuffertarget,
            renderbuffer,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glNamedFramebufferRenderbuffer(
    framebuffer: GLuint,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
) {
    trace!(
        "glNamedFramebufferRenderbuffer called, parameters: framebuffer: {:?}, attachment: {:?}, renderbuffertarget: {:?}, renderbuffer: {:?} ",
        framebuffer, attachment, renderbuffertarget, renderbuffer
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_named_framebuffer_renderbuffer(
            framebuffer,
            attachment.try_into_enum()?,
            renderbuffertarget,
            renderbuffer,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glFramebufferTexture1D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
) {
    trace!(
        "glFramebufferTexture1D called, parameters: target: {:?}, attachment: {:?}, textarget: {:?}, texture: {:?}, level: {:?} ",
        target, attachment, textarget, texture, level
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_framebuffer_texture1_d(
            target.try_into_enum()?,
            attachment.try_into_enum()?,
            textarget.try_into_enum()?,
            texture,
            level,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glFramebufferTexture2D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
) {
    trace!(
        "glFramebufferTexture2D called, parameters: target: {:?}, attachment: {:?}, textarget: {:?}, texture: {:?}, level: {:?} ",
        target, attachment, textarget, texture, level
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_framebuffer_texture2_d(
            target.try_into_enum()?,
            attachment.try_into_enum()?,
            textarget.try_into_enum()?,
            texture,
            level,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glFramebufferTexture3D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
    zoffset: GLint,
) {
    trace!(
        "glFramebufferTexture3D called, parameters: target: {:?}, attachment: {:?}, textarget: {:?}, texture: {:?}, level: {:?}, zoffset: {:?} ",
        target, attachment, textarget, texture, level, zoffset
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_framebuffer_texture3_d(
            target.try_into_enum()?,
            attachment.try_into_enum()?,
            textarget.try_into_enum()?,
            texture,
            level,
            zoffset,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glFramebufferTexture(
    target: GLenum,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
) {
    trace!(
        "glFramebufferTexture called, parameters: target: {:?}, attachment: {:?}, texture: {:?}, level: {:?} ",
        target, attachment, texture, level
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_framebuffer_texture(
            target.try_into_enum()?,
            attachment.try_into_enum()?,
            texture,
            level,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glNamedFramebufferTexture(
    framebuffer: GLuint,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
) {
    trace!(
        "glNamedFramebufferTexture called, parameters: framebuffer: {:?}, attachment: {:?}, texture: {:?}, level: {:?} ",
        framebuffer, attachment, texture, level
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_named_framebuffer_texture(
            framebuffer,
            attachment.try_into_enum()?,
            texture,
            level,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glFramebufferTextureLayer(
    target: GLenum,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
    layer: GLint,
) {
    trace!(
        "glFramebufferTextureLayer called, parameters: target: {:?}, attachment: {:?}, texture: {:?}, level: {:?}, layer: {:?} ",
        target, attachment, texture, level, layer
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_framebuffer_texture_layer(
            target.try_into_enum()?,
            attachment.try_into_enum()?,
            texture,
            level,
            layer,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glNamedFramebufferTextureLayer(
    framebuffer: GLuint,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
    layer: GLint,
) {
    trace!(
        "glNamedFramebufferTextureLayer called, parameters: framebuffer: {:?}, attachment: {:?}, texture: {:?}, level: {:?}, layer: {:?} ",
        framebuffer, attachment, texture, level, layer
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_named_framebuffer_texture_layer(
            framebuffer,
            attachment.try_into_enum()?,
            texture,
            level,
            layer,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glFrontFace(mode: GLenum) {
    trace!("glFrontFace called, parameters: mode: {:?} ", mode);
    with_ctx_mut(|mut state| GlResult::normalize(state.oxidegl_front_face(mode.try_into_enum()?)))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) {
    trace!(
        "glGenBuffers called, parameters: n: {:?}, buffers: {:?} ",
        n, buffers
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_gen_buffers(n, buffers) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGenerateMipmap(target: GLenum) {
    trace!("glGenerateMipmap called, parameters: target: {:?} ", target);
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_generate_mipmap(target.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGenerateTextureMipmap(texture: GLuint) {
    trace!(
        "glGenerateTextureMipmap called, parameters: texture: {:?} ",
        texture
    );
    with_ctx_mut(|mut state| state.oxidegl_generate_texture_mipmap(texture))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
    trace!(
        "glGenFramebuffers called, parameters: n: {:?}, framebuffers: {:?} ",
        n, framebuffers
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_gen_framebuffers(n, framebuffers) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGenProgramPipelines(n: GLsizei, pipelines: *mut GLuint) {
    trace!(
        "glGenProgramPipelines called, parameters: n: {:?}, pipelines: {:?} ",
        n, pipelines
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_gen_program_pipelines(n, pipelines) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGenQueries(n: GLsizei, ids: *mut GLuint) {
    trace!(
        "glGenQueries called, parameters: n: {:?}, ids: {:?} ",
        n, ids
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_gen_queries(n, ids) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
    trace!(
        "glGenRenderbuffers called, parameters: n: {:?}, renderbuffers: {:?} ",
        n, renderbuffers
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_gen_renderbuffers(n, renderbuffers) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGenSamplers(count: GLsizei, samplers: *mut GLuint) {
    trace!(
        "glGenSamplers called, parameters: count: {:?}, samplers: {:?} ",
        count, samplers
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_gen_samplers(count, samplers) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGenTextures(n: GLsizei, textures: *mut GLuint) {
    trace!(
        "glGenTextures called, parameters: n: {:?}, textures: {:?} ",
        n, textures
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_gen_textures(n, textures) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGenTransformFeedbacks(n: GLsizei, ids: *mut GLuint) {
    trace!(
        "glGenTransformFeedbacks called, parameters: n: {:?}, ids: {:?} ",
        n, ids
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_gen_transform_feedbacks(n, ids) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) {
    trace!(
        "glGenVertexArrays called, parameters: n: {:?}, arrays: {:?} ",
        n, arrays
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_gen_vertex_arrays(n, arrays) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetBooleanv(pname: GLenum, data: *mut GLboolean) {
    trace!(
        "glGetBooleanv called, parameters: pname: {:?}, data: {:?} ",
        pname, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe { state.oxidegl_get_booleanv(pname.try_into_enum()?, data) })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetDoublev(pname: GLenum, data: *mut GLdouble) {
    trace!(
        "glGetDoublev called, parameters: pname: {:?}, data: {:?} ",
        pname, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe { state.oxidegl_get_doublev(pname.try_into_enum()?, data) })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetFloatv(pname: GLenum, data: *mut GLfloat) {
    trace!(
        "glGetFloatv called, parameters: pname: {:?}, data: {:?} ",
        pname, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe { state.oxidegl_get_floatv(pname.try_into_enum()?, data) })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetIntegerv(pname: GLenum, data: *mut GLint) {
    trace!(
        "glGetIntegerv called, parameters: pname: {:?}, data: {:?} ",
        pname, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe { state.oxidegl_get_integerv(pname.try_into_enum()?, data) })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean) {
    trace!(
        "glGetBooleani_v called, parameters: target: {:?}, index: {:?}, data: {:?} ",
        target, index, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_booleani_v(target.try_into_enum()?, index, data)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) {
    trace!(
        "glGetIntegeri_v called, parameters: target: {:?}, index: {:?}, data: {:?} ",
        target, index, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_integeri_v(target.try_into_enum()?, index, data)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetInteger64v(pname: GLenum, data: *mut GLint64) {
    trace!(
        "glGetInteger64v called, parameters: pname: {:?}, data: {:?} ",
        pname, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe { state.oxidegl_get_integer64v(pname.try_into_enum()?, data) })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64) {
    trace!(
        "glGetInteger64i_v called, parameters: target: {:?}, index: {:?}, data: {:?} ",
        target, index, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_integer64i_v(target.try_into_enum()?, index, data)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetFloati_v(target: GLenum, index: GLuint, data: *mut GLfloat) {
    trace!(
        "glGetFloati_v called, parameters: target: {:?}, index: {:?}, data: {:?} ",
        target, index, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_floati_v(target.try_into_enum()?, index, data)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetDoublei_v(target: GLenum, index: GLuint, data: *mut GLdouble) {
    trace!(
        "glGetDoublei_v called, parameters: target: {:?}, index: {:?}, data: {:?} ",
        target, index, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_doublei_v(target.try_into_enum()?, index, data)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetActiveAtomicCounterBufferiv(
    program: GLuint,
    bufferIndex: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetActiveAtomicCounterBufferiv called, parameters: program: {:?}, bufferIndex: {:?}, pname: {:?}, params: {:?} ",
        program, bufferIndex, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_active_atomic_counter_bufferiv(
                program,
                bufferIndex,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetActiveAttrib(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    r#type: GLenum,
    name: *mut GLchar,
) {
    trace!(
        "glGetActiveAttrib called, parameters: program: {:?}, index: {:?}, bufSize: {:?}, length: {:?}, size: {:?}, r#type: {:?}, name: {:?} ",
        program, index, bufSize, length, size, r#type, name
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_active_attrib(
                program,
                index,
                bufSize,
                length,
                size,
                r#type.try_into_enum()?,
                name,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetActiveSubroutineName(
    program: GLuint,
    shadertype: GLenum,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    name: *mut GLchar,
) {
    trace!(
        "glGetActiveSubroutineName called, parameters: program: {:?}, shadertype: {:?}, index: {:?}, bufSize: {:?}, length: {:?}, name: {:?} ",
        program, shadertype, index, bufSize, length, name
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_active_subroutine_name(
                program,
                shadertype.try_into_enum()?,
                index,
                bufSize,
                length,
                name,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetActiveSubroutineUniformiv(
    program: GLuint,
    shadertype: GLenum,
    index: GLuint,
    pname: GLenum,
    values: *mut GLint,
) {
    trace!(
        "glGetActiveSubroutineUniformiv called, parameters: program: {:?}, shadertype: {:?}, index: {:?}, pname: {:?}, values: {:?} ",
        program, shadertype, index, pname, values
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_active_subroutine_uniformiv(
                program,
                shadertype.try_into_enum()?,
                index,
                pname.try_into_enum()?,
                values,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetActiveSubroutineUniformName(
    program: GLuint,
    shadertype: GLenum,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    name: *mut GLchar,
) {
    trace!(
        "glGetActiveSubroutineUniformName called, parameters: program: {:?}, shadertype: {:?}, index: {:?}, bufSize: {:?}, length: {:?}, name: {:?} ",
        program, shadertype, index, bufSize, length, name
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_active_subroutine_uniform_name(
                program,
                shadertype.try_into_enum()?,
                index,
                bufSize,
                length,
                name,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetActiveUniform(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    r#type: GLenum,
    name: *mut GLchar,
) {
    trace!(
        "glGetActiveUniform called, parameters: program: {:?}, index: {:?}, bufSize: {:?}, length: {:?}, size: {:?}, r#type: {:?}, name: {:?} ",
        program, index, bufSize, length, size, r#type, name
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_active_uniform(
                program,
                index,
                bufSize,
                length,
                size,
                r#type.try_into_enum()?,
                name,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetActiveUniformBlockiv(
    program: GLuint,
    uniformBlockIndex: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetActiveUniformBlockiv called, parameters: program: {:?}, uniformBlockIndex: {:?}, pname: {:?}, params: {:?} ",
        program, uniformBlockIndex, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_active_uniform_blockiv(
                program,
                uniformBlockIndex,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetActiveUniformBlockName(
    program: GLuint,
    uniformBlockIndex: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    uniformBlockName: *mut GLchar,
) {
    trace!(
        "glGetActiveUniformBlockName called, parameters: program: {:?}, uniformBlockIndex: {:?}, bufSize: {:?}, length: {:?}, uniformBlockName: {:?} ",
        program, uniformBlockIndex, bufSize, length, uniformBlockName
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_active_uniform_block_name(
            program,
            uniformBlockIndex,
            bufSize,
            length,
            uniformBlockName,
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetActiveUniformName(
    program: GLuint,
    uniformIndex: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    uniformName: *mut GLchar,
) {
    trace!(
        "glGetActiveUniformName called, parameters: program: {:?}, uniformIndex: {:?}, bufSize: {:?}, length: {:?}, uniformName: {:?} ",
        program, uniformIndex, bufSize, length, uniformName
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_active_uniform_name(program, uniformIndex, bufSize, length, uniformName)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetActiveUniformsiv(
    program: GLuint,
    uniformCount: GLsizei,
    uniformIndices: *const GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetActiveUniformsiv called, parameters: program: {:?}, uniformCount: {:?}, uniformIndices: {:?}, pname: {:?}, params: {:?} ",
        program, uniformCount, uniformIndices, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_active_uniformsiv(
                program,
                uniformCount,
                uniformIndices,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetAttachedShaders(
    program: GLuint,
    maxCount: GLsizei,
    count: *mut GLsizei,
    shaders: *mut GLuint,
) {
    trace!(
        "glGetAttachedShaders called, parameters: program: {:?}, maxCount: {:?}, count: {:?}, shaders: {:?} ",
        program, maxCount, count, shaders
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_attached_shaders(program, maxCount, count, shaders)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint {
    trace!(
        "glGetAttribLocation called, parameters: program: {:?}, name: {:?} ",
        program, name
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_get_attrib_location(program, name) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    trace!(
        "glGetBufferParameteriv called, parameters: target: {:?}, pname: {:?}, params: {:?} ",
        target, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_buffer_parameteriv(
                target.try_into_enum()?,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetBufferParameteri64v(
    target: GLenum,
    pname: GLenum,
    params: *mut GLint64,
) {
    trace!(
        "glGetBufferParameteri64v called, parameters: target: {:?}, pname: {:?}, params: {:?} ",
        target, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_buffer_parameteri64v(
                target.try_into_enum()?,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetNamedBufferParameteriv(
    buffer: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetNamedBufferParameteriv called, parameters: buffer: {:?}, pname: {:?}, params: {:?} ",
        buffer, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_named_buffer_parameteriv(buffer, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetNamedBufferParameteri64v(
    buffer: GLuint,
    pname: GLenum,
    params: *mut GLint64,
) {
    trace!(
        "glGetNamedBufferParameteri64v called, parameters: buffer: {:?}, pname: {:?}, params: {:?} ",
        buffer, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_named_buffer_parameteri64v(buffer, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetBufferPointerv(
    target: GLenum,
    pname: GLenum,
    params: *mut *mut GLvoid,
) {
    trace!(
        "glGetBufferPointerv called, parameters: target: {:?}, pname: {:?}, params: {:?} ",
        target, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_buffer_pointerv(target.try_into_enum()?, pname, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetNamedBufferPointerv(
    buffer: GLuint,
    pname: GLenum,
    params: *mut *mut GLvoid,
) {
    trace!(
        "glGetNamedBufferPointerv called, parameters: buffer: {:?}, pname: {:?}, params: {:?} ",
        buffer, pname, params
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_named_buffer_pointerv(buffer, pname, params)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetBufferSubData(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *mut GLvoid,
) {
    trace!(
        "glGetBufferSubData called, parameters: target: {:?}, offset: {:?}, size: {:?}, data: {:?} ",
        target, offset, size, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_buffer_sub_data(target.try_into_enum()?, offset, size, data)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetNamedBufferSubData(
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *mut GLvoid,
) {
    trace!(
        "glGetNamedBufferSubData called, parameters: buffer: {:?}, offset: {:?}, size: {:?}, data: {:?} ",
        buffer, offset, size, data
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_named_buffer_sub_data(buffer, offset, size, data)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetCompressedTexImage(target: GLenum, level: GLint, img: *mut GLvoid) {
    trace!(
        "glGetCompressedTexImage called, parameters: target: {:?}, level: {:?}, img: {:?} ",
        target, level, img
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_compressed_tex_image(target.try_into_enum()?, level, img)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetCompressedTextureImage(
    texture: GLuint,
    level: GLint,
    bufSize: GLsizei,
    pixels: *mut GLvoid,
) {
    trace!(
        "glGetCompressedTextureImage called, parameters: texture: {:?}, level: {:?}, bufSize: {:?}, pixels: {:?} ",
        texture, level, bufSize, pixels
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_compressed_texture_image(texture, level, bufSize, pixels)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetnCompressedTexImage(
    target: GLenum,
    lod: GLint,
    bufSize: GLsizei,
    pixels: *mut GLvoid,
) {
    trace!(
        "glGetnCompressedTexImage called, parameters: target: {:?}, lod: {:?}, bufSize: {:?}, pixels: {:?} ",
        target, lod, bufSize, pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_getn_compressed_tex_image(target.try_into_enum()?, lod, bufSize, pixels)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetCompressedTextureSubImage(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    bufSize: GLsizei,
    pixels: *mut GLvoid,
) {
    trace!(
        "glGetCompressedTextureSubImage called, parameters: texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}, bufSize: {:?}, pixels: {:?} ",
        texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_compressed_texture_sub_image(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels,
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetDebugMessageLog(
    count: GLuint,
    bufSize: GLsizei,
    sources: GLenum,
    types: GLenum,
    ids: *mut GLuint,
    severities: GLenum,
    lengths: *mut GLsizei,
    messageLog: *mut GLchar,
) -> GLuint {
    trace!(
        "glGetDebugMessageLog called, parameters: count: {:?}, bufSize: {:?}, sources: {:?}, types: {:?}, ids: {:?}, severities: {:?}, lengths: {:?}, messageLog: {:?} ",
        count, bufSize, sources, types, ids, severities, lengths, messageLog
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_debug_message_log(
                count,
                bufSize,
                sources.try_into_enum()?,
                types.try_into_enum()?,
                ids,
                severities.try_into_enum()?,
                lengths,
                messageLog,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetError() -> GLenum {
    trace!("glGetError called, parameters:  ",);
    with_ctx_mut(|mut state| state.oxidegl_get_error())
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetFragDataIndex(program: GLuint, name: *const GLchar) -> GLint {
    trace!(
        "glGetFragDataIndex called, parameters: program: {:?}, name: {:?} ",
        program, name
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_get_frag_data_index(program, name) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint {
    trace!(
        "glGetFragDataLocation called, parameters: program: {:?}, name: {:?} ",
        program, name
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_get_frag_data_location(program, name) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetFramebufferAttachmentParameteriv(
    target: GLenum,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetFramebufferAttachmentParameteriv called, parameters: target: {:?}, attachment: {:?}, pname: {:?}, params: {:?} ",
        target, attachment, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_framebuffer_attachment_parameteriv(
                target.try_into_enum()?,
                attachment.try_into_enum()?,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetNamedFramebufferAttachmentParameteriv(
    framebuffer: GLuint,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetNamedFramebufferAttachmentParameteriv called, parameters: framebuffer: {:?}, attachment: {:?}, pname: {:?}, params: {:?} ",
        framebuffer, attachment, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_named_framebuffer_attachment_parameteriv(
                framebuffer,
                attachment.try_into_enum()?,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetFramebufferParameteriv(
    target: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetFramebufferParameteriv called, parameters: target: {:?}, pname: {:?}, params: {:?} ",
        target, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_framebuffer_parameteriv(
                target.try_into_enum()?,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetNamedFramebufferParameteriv(
    framebuffer: GLuint,
    pname: GLenum,
    param: *mut GLint,
) {
    trace!(
        "glGetNamedFramebufferParameteriv called, parameters: framebuffer: {:?}, pname: {:?}, param: {:?} ",
        framebuffer, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_named_framebuffer_parameteriv(
                framebuffer,
                pname.try_into_enum()?,
                param,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetGraphicsResetStatus() -> GLenum {
    trace!("glGetGraphicsResetStatus called, parameters:  ",);
    with_ctx_mut(|mut state| state.oxidegl_get_graphics_reset_status())
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetInternalformativ(
    target: GLenum,
    internalformat: GLenum,
    pname: GLenum,
    count: GLsizei,
    params: *mut GLint,
) {
    trace!(
        "glGetInternalformativ called, parameters: target: {:?}, internalformat: {:?}, pname: {:?}, count: {:?}, params: {:?} ",
        target, internalformat, pname, count, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_internalformativ(
                target.try_into_enum()?,
                internalformat.try_into_enum()?,
                pname.try_into_enum()?,
                count,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetInternalformati64v(
    target: GLenum,
    internalformat: GLenum,
    pname: GLenum,
    count: GLsizei,
    params: *mut GLint64,
) {
    trace!(
        "glGetInternalformati64v called, parameters: target: {:?}, internalformat: {:?}, pname: {:?}, count: {:?}, params: {:?} ",
        target, internalformat, pname, count, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_internalformati64v(
                target.try_into_enum()?,
                internalformat.try_into_enum()?,
                pname.try_into_enum()?,
                count,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat) {
    trace!(
        "glGetMultisamplefv called, parameters: pname: {:?}, index: {:?}, val: {:?} ",
        pname, index, val
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_get_multisamplefv(pname, index, val) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetObjectLabel(
    identifier: GLenum,
    name: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    label: *mut GLchar,
) {
    trace!(
        "glGetObjectLabel called, parameters: identifier: {:?}, name: {:?}, bufSize: {:?}, length: {:?}, label: {:?} ",
        identifier, name, bufSize, length, label
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_object_label(
                identifier.try_into_enum()?,
                name,
                bufSize,
                length,
                label,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetObjectPtrLabel(
    ptr: *const GLvoid,
    bufSize: GLsizei,
    length: *mut GLsizei,
    label: *mut GLchar,
) {
    trace!(
        "glGetObjectPtrLabel called, parameters: ptr: {:?}, bufSize: {:?}, length: {:?}, label: {:?} ",
        ptr, bufSize, length, label
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_object_ptr_label(ptr, bufSize, length, label)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetPointerv(pname: GLenum, params: *mut *mut GLvoid) {
    trace!(
        "glGetPointerv called, parameters: pname: {:?}, params: {:?} ",
        pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe { state.oxidegl_get_pointerv(pname.try_into_enum()?, params) })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) {
    trace!(
        "glGetProgramiv called, parameters: program: {:?}, pname: {:?}, params: {:?} ",
        program, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_programiv(program, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetProgramBinary(
    program: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    binaryFormat: *mut GLenum,
    binary: *mut GLvoid,
) {
    trace!(
        "glGetProgramBinary called, parameters: program: {:?}, bufSize: {:?}, length: {:?}, binaryFormat: {:?}, binary: {:?} ",
        program, bufSize, length, binaryFormat, binary
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_program_binary(program, bufSize, length, binaryFormat, binary)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetProgramInfoLog(
    program: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
) {
    trace!(
        "glGetProgramInfoLog called, parameters: program: {:?}, bufSize: {:?}, length: {:?}, infoLog: {:?} ",
        program, bufSize, length, infoLog
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_program_info_log(program, bufSize, length, infoLog)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetProgramInterfaceiv(
    program: GLuint,
    programInterface: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetProgramInterfaceiv called, parameters: program: {:?}, programInterface: {:?}, pname: {:?}, params: {:?} ",
        program, programInterface, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_program_interfaceiv(
                program,
                programInterface.try_into_enum()?,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetProgramPipelineiv(
    pipeline: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetProgramPipelineiv called, parameters: pipeline: {:?}, pname: {:?}, params: {:?} ",
        pipeline, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_program_pipelineiv(pipeline, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetProgramPipelineInfoLog(
    pipeline: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
) {
    trace!(
        "glGetProgramPipelineInfoLog called, parameters: pipeline: {:?}, bufSize: {:?}, length: {:?}, infoLog: {:?} ",
        pipeline, bufSize, length, infoLog
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_program_pipeline_info_log(pipeline, bufSize, length, infoLog)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetProgramResourceiv(
    program: GLuint,
    programInterface: GLenum,
    index: GLuint,
    propCount: GLsizei,
    props: GLenum,
    count: GLsizei,
    length: *mut GLsizei,
    params: *mut GLint,
) {
    trace!(
        "glGetProgramResourceiv called, parameters: program: {:?}, programInterface: {:?}, index: {:?}, propCount: {:?}, props: {:?}, count: {:?}, length: {:?}, params: {:?} ",
        program, programInterface, index, propCount, props, count, length, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_program_resourceiv(
                program,
                programInterface.try_into_enum()?,
                index,
                propCount,
                props.try_into_enum()?,
                count,
                length,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetProgramResourceIndex(
    program: GLuint,
    programInterface: GLenum,
    name: *const GLchar,
) -> GLuint {
    trace!(
        "glGetProgramResourceIndex called, parameters: program: {:?}, programInterface: {:?}, name: {:?} ",
        program, programInterface, name
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_program_resource_index(
                program,
                programInterface.try_into_enum()?,
                name,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetProgramResourceLocation(
    program: GLuint,
    programInterface: GLenum,
    name: *const GLchar,
) -> GLint {
    trace!(
        "glGetProgramResourceLocation called, parameters: program: {:?}, programInterface: {:?}, name: {:?} ",
        program, programInterface, name
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_program_resource_location(
                program,
                programInterface.try_into_enum()?,
                name,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetProgramResourceLocationIndex(
    program: GLuint,
    programInterface: GLenum,
    name: *const GLchar,
) -> GLint {
    trace!(
        "glGetProgramResourceLocationIndex called, parameters: program: {:?}, programInterface: {:?}, name: {:?} ",
        program, programInterface, name
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_program_resource_location_index(
                program,
                programInterface.try_into_enum()?,
                name,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetProgramResourceName(
    program: GLuint,
    programInterface: GLenum,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    name: *mut GLchar,
) {
    trace!(
        "glGetProgramResourceName called, parameters: program: {:?}, programInterface: {:?}, index: {:?}, bufSize: {:?}, length: {:?}, name: {:?} ",
        program, programInterface, index, bufSize, length, name
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_program_resource_name(
                program,
                programInterface.try_into_enum()?,
                index,
                bufSize,
                length,
                name,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetProgramStageiv(
    program: GLuint,
    shadertype: GLenum,
    pname: GLenum,
    values: *mut GLint,
) {
    trace!(
        "glGetProgramStageiv called, parameters: program: {:?}, shadertype: {:?}, pname: {:?}, values: {:?} ",
        program, shadertype, pname, values
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_program_stageiv(
                program,
                shadertype.try_into_enum()?,
                pname.try_into_enum()?,
                values,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetQueryIndexediv(
    target: GLenum,
    index: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetQueryIndexediv called, parameters: target: {:?}, index: {:?}, pname: {:?}, params: {:?} ",
        target, index, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_query_indexediv(
                target.try_into_enum()?,
                index,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) {
    trace!(
        "glGetQueryiv called, parameters: target: {:?}, pname: {:?}, params: {:?} ",
        target, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_queryiv(target.try_into_enum()?, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) {
    trace!(
        "glGetQueryObjectiv called, parameters: id: {:?}, pname: {:?}, params: {:?} ",
        id, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_query_objectiv(id, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) {
    trace!(
        "glGetQueryObjectuiv called, parameters: id: {:?}, pname: {:?}, params: {:?} ",
        id, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_query_objectuiv(id, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetQueryObjecti64v(id: GLuint, pname: GLenum, params: *mut GLint64) {
    trace!(
        "glGetQueryObjecti64v called, parameters: id: {:?}, pname: {:?}, params: {:?} ",
        id, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_query_objecti64v(id, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64) {
    trace!(
        "glGetQueryObjectui64v called, parameters: id: {:?}, pname: {:?}, params: {:?} ",
        id, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_query_objectui64v(id, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetQueryBufferObjecti64v(
    id: GLuint,
    buffer: GLuint,
    pname: GLenum,
    offset: GLintptr,
) {
    trace!(
        "glGetQueryBufferObjecti64v called, parameters: id: {:?}, buffer: {:?}, pname: {:?}, offset: {:?} ",
        id, buffer, pname, offset
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_get_query_buffer_objecti64v(
            id,
            buffer,
            pname.try_into_enum()?,
            offset,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetQueryBufferObjectiv(
    id: GLuint,
    buffer: GLuint,
    pname: GLenum,
    offset: GLintptr,
) {
    trace!(
        "glGetQueryBufferObjectiv called, parameters: id: {:?}, buffer: {:?}, pname: {:?}, offset: {:?} ",
        id, buffer, pname, offset
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_get_query_buffer_objectiv(
            id,
            buffer,
            pname.try_into_enum()?,
            offset,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetQueryBufferObjectui64v(
    id: GLuint,
    buffer: GLuint,
    pname: GLenum,
    offset: GLintptr,
) {
    trace!(
        "glGetQueryBufferObjectui64v called, parameters: id: {:?}, buffer: {:?}, pname: {:?}, offset: {:?} ",
        id, buffer, pname, offset
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_get_query_buffer_objectui64v(
            id,
            buffer,
            pname.try_into_enum()?,
            offset,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetQueryBufferObjectuiv(
    id: GLuint,
    buffer: GLuint,
    pname: GLenum,
    offset: GLintptr,
) {
    trace!(
        "glGetQueryBufferObjectuiv called, parameters: id: {:?}, buffer: {:?}, pname: {:?}, offset: {:?} ",
        id, buffer, pname, offset
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_get_query_buffer_objectuiv(
            id,
            buffer,
            pname.try_into_enum()?,
            offset,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetRenderbufferParameteriv(
    target: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetRenderbufferParameteriv called, parameters: target: {:?}, pname: {:?}, params: {:?} ",
        target, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_renderbuffer_parameteriv(target, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetNamedRenderbufferParameteriv(
    renderbuffer: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetNamedRenderbufferParameteriv called, parameters: renderbuffer: {:?}, pname: {:?}, params: {:?} ",
        renderbuffer, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_named_renderbuffer_parameteriv(
                renderbuffer,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetSamplerParameteriv(
    sampler: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetSamplerParameteriv called, parameters: sampler: {:?}, pname: {:?}, params: {:?} ",
        sampler, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_sampler_parameteriv(sampler, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetSamplerParameterIiv(
    sampler: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetSamplerParameterIiv called, parameters: sampler: {:?}, pname: {:?}, params: {:?} ",
        sampler, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_sampler_parameter_iiv(sampler, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetSamplerParameterfv(
    sampler: GLuint,
    pname: GLenum,
    params: *mut GLfloat,
) {
    trace!(
        "glGetSamplerParameterfv called, parameters: sampler: {:?}, pname: {:?}, params: {:?} ",
        sampler, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_sampler_parameterfv(sampler, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetSamplerParameterIuiv(
    sampler: GLuint,
    pname: GLenum,
    params: *mut GLuint,
) {
    trace!(
        "glGetSamplerParameterIuiv called, parameters: sampler: {:?}, pname: {:?}, params: {:?} ",
        sampler, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_sampler_parameter_iuiv(sampler, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
    trace!(
        "glGetShaderiv called, parameters: shader: {:?}, pname: {:?}, params: {:?} ",
        shader, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_shaderiv(shader, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetShaderInfoLog(
    shader: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
) {
    trace!(
        "glGetShaderInfoLog called, parameters: shader: {:?}, bufSize: {:?}, length: {:?}, infoLog: {:?} ",
        shader, bufSize, length, infoLog
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_shader_info_log(shader, bufSize, length, infoLog)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetShaderPrecisionFormat(
    shadertype: GLenum,
    precisiontype: GLenum,
    range: *mut GLint,
    precision: *mut GLint,
) {
    trace!(
        "glGetShaderPrecisionFormat called, parameters: shadertype: {:?}, precisiontype: {:?}, range: {:?}, precision: {:?} ",
        shadertype, precisiontype, range, precision
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_shader_precision_format(
                shadertype.try_into_enum()?,
                precisiontype.try_into_enum()?,
                range,
                precision,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetShaderSource(
    shader: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    source: *mut GLchar,
) {
    trace!(
        "glGetShaderSource called, parameters: shader: {:?}, bufSize: {:?}, length: {:?}, source: {:?} ",
        shader, bufSize, length, source
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_shader_source(shader, bufSize, length, source)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetString(name: GLenum) -> *const GLubyte {
    trace!("glGetString called, parameters: name: {:?} ", name);
    with_ctx_mut(|mut state| GlResult::normalize(state.oxidegl_get_string(name.try_into_enum()?)))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte {
    trace!(
        "glGetStringi called, parameters: name: {:?}, index: {:?} ",
        name, index
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_get_stringi(name.try_into_enum()?, index))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetSubroutineIndex(
    program: GLuint,
    shadertype: GLenum,
    name: *const GLchar,
) -> GLuint {
    trace!(
        "glGetSubroutineIndex called, parameters: program: {:?}, shadertype: {:?}, name: {:?} ",
        program, shadertype, name
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_subroutine_index(program, shadertype.try_into_enum()?, name)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetSubroutineUniformLocation(
    program: GLuint,
    shadertype: GLenum,
    name: *const GLchar,
) -> GLint {
    trace!(
        "glGetSubroutineUniformLocation called, parameters: program: {:?}, shadertype: {:?}, name: {:?} ",
        program, shadertype, name
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_subroutine_uniform_location(
                program,
                shadertype.try_into_enum()?,
                name,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetSynciv(
    sync: GLsync,
    pname: GLenum,
    count: GLsizei,
    length: *mut GLsizei,
    values: *mut GLint,
) {
    trace!(
        "glGetSynciv called, parameters: sync: {:?}, pname: {:?}, count: {:?}, length: {:?}, values: {:?} ",
        sync, pname, count, length, values
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_synciv(sync, pname.try_into_enum()?, count, length, values)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTexImage(
    target: GLenum,
    level: GLint,
    format: GLenum,
    r#type: GLenum,
    pixels: *mut GLvoid,
) {
    trace!(
        "glGetTexImage called, parameters: target: {:?}, level: {:?}, format: {:?}, r#type: {:?}, pixels: {:?} ",
        target, level, format, r#type, pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_tex_image(
                target.try_into_enum()?,
                level,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                pixels,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTextureImage(
    texture: GLuint,
    level: GLint,
    format: GLenum,
    r#type: GLenum,
    bufSize: GLsizei,
    pixels: *mut GLvoid,
) {
    trace!(
        "glGetTextureImage called, parameters: texture: {:?}, level: {:?}, format: {:?}, r#type: {:?}, bufSize: {:?}, pixels: {:?} ",
        texture, level, format, r#type, bufSize, pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_texture_image(
                texture,
                level,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                bufSize,
                pixels,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetnTexImage(
    target: GLenum,
    level: GLint,
    format: GLenum,
    r#type: GLenum,
    bufSize: GLsizei,
    pixels: *mut GLvoid,
) {
    trace!(
        "glGetnTexImage called, parameters: target: {:?}, level: {:?}, format: {:?}, r#type: {:?}, bufSize: {:?}, pixels: {:?} ",
        target, level, format, r#type, bufSize, pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_getn_tex_image(
                target.try_into_enum()?,
                level,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                bufSize,
                pixels,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTexLevelParameterfv(
    target: GLenum,
    level: GLint,
    pname: GLenum,
    params: *mut GLfloat,
) {
    trace!(
        "glGetTexLevelParameterfv called, parameters: target: {:?}, level: {:?}, pname: {:?}, params: {:?} ",
        target, level, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_tex_level_parameterfv(
                target.try_into_enum()?,
                level,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTexLevelParameteriv(
    target: GLenum,
    level: GLint,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetTexLevelParameteriv called, parameters: target: {:?}, level: {:?}, pname: {:?}, params: {:?} ",
        target, level, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_tex_level_parameteriv(
                target.try_into_enum()?,
                level,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTextureLevelParameterfv(
    texture: GLuint,
    level: GLint,
    pname: GLenum,
    params: *mut GLfloat,
) {
    trace!(
        "glGetTextureLevelParameterfv called, parameters: texture: {:?}, level: {:?}, pname: {:?}, params: {:?} ",
        texture, level, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_texture_level_parameterfv(
                texture,
                level,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTextureLevelParameteriv(
    texture: GLuint,
    level: GLint,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetTextureLevelParameteriv called, parameters: texture: {:?}, level: {:?}, pname: {:?}, params: {:?} ",
        texture, level, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_texture_level_parameteriv(
                texture,
                level,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) {
    trace!(
        "glGetTexParameterfv called, parameters: target: {:?}, pname: {:?}, params: {:?} ",
        target, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_tex_parameterfv(
                target.try_into_enum()?,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    trace!(
        "glGetTexParameteriv called, parameters: target: {:?}, pname: {:?}, params: {:?} ",
        target, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_tex_parameteriv(
                target.try_into_enum()?,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint) {
    trace!(
        "glGetTexParameterIiv called, parameters: target: {:?}, pname: {:?}, params: {:?} ",
        target, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_tex_parameter_iiv(
                target.try_into_enum()?,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint) {
    trace!(
        "glGetTexParameterIuiv called, parameters: target: {:?}, pname: {:?}, params: {:?} ",
        target, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_tex_parameter_iuiv(
                target.try_into_enum()?,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTextureParameterfv(
    texture: GLuint,
    pname: GLenum,
    params: *mut GLfloat,
) {
    trace!(
        "glGetTextureParameterfv called, parameters: texture: {:?}, pname: {:?}, params: {:?} ",
        texture, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_texture_parameterfv(texture, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTextureParameterIiv(
    texture: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetTextureParameterIiv called, parameters: texture: {:?}, pname: {:?}, params: {:?} ",
        texture, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_texture_parameter_iiv(texture, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTextureParameterIuiv(
    texture: GLuint,
    pname: GLenum,
    params: *mut GLuint,
) {
    trace!(
        "glGetTextureParameterIuiv called, parameters: texture: {:?}, pname: {:?}, params: {:?} ",
        texture, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_texture_parameter_iuiv(texture, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTextureParameteriv(
    texture: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    trace!(
        "glGetTextureParameteriv called, parameters: texture: {:?}, pname: {:?}, params: {:?} ",
        texture, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_texture_parameteriv(texture, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTextureSubImage(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    r#type: GLenum,
    bufSize: GLsizei,
    pixels: *mut GLvoid,
) {
    trace!(
        "glGetTextureSubImage called, parameters: texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}, format: {:?}, r#type: {:?}, bufSize: {:?}, pixels: {:?} ",
        texture,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        r#type,
        bufSize,
        pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_texture_sub_image(
                texture,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                bufSize,
                pixels,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTransformFeedbackiv(xfb: GLuint, pname: GLenum, param: *mut GLint) {
    trace!(
        "glGetTransformFeedbackiv called, parameters: xfb: {:?}, pname: {:?}, param: {:?} ",
        xfb, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_transform_feedbackiv(xfb, pname.try_into_enum()?, param)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTransformFeedbacki_v(
    xfb: GLuint,
    pname: GLenum,
    index: GLuint,
    param: *mut GLint,
) {
    trace!(
        "glGetTransformFeedbacki_v called, parameters: xfb: {:?}, pname: {:?}, index: {:?}, param: {:?} ",
        xfb, pname, index, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_transform_feedbacki_v(xfb, pname.try_into_enum()?, index, param)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTransformFeedbacki64_v(
    xfb: GLuint,
    pname: GLenum,
    index: GLuint,
    param: *mut GLint64,
) {
    trace!(
        "glGetTransformFeedbacki64_v called, parameters: xfb: {:?}, pname: {:?}, index: {:?}, param: {:?} ",
        xfb, pname, index, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_transform_feedbacki64_v(xfb, pname.try_into_enum()?, index, param)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetTransformFeedbackVarying(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLsizei,
    r#type: GLenum,
    name: *mut GLchar,
) {
    trace!(
        "glGetTransformFeedbackVarying called, parameters: program: {:?}, index: {:?}, bufSize: {:?}, length: {:?}, size: {:?}, r#type: {:?}, name: {:?} ",
        program, index, bufSize, length, size, r#type, name
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_transform_feedback_varying(
                program,
                index,
                bufSize,
                length,
                size,
                r#type.try_into_enum()?,
                name,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) {
    trace!(
        "glGetUniformfv called, parameters: program: {:?}, location: {:?}, params: {:?} ",
        program, location, params
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_get_uniformfv(program, location, params) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetUniformiv(program: GLuint, location: GLint, params: *mut GLint) {
    trace!(
        "glGetUniformiv called, parameters: program: {:?}, location: {:?}, params: {:?} ",
        program, location, params
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_get_uniformiv(program, location, params) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) {
    trace!(
        "glGetUniformuiv called, parameters: program: {:?}, location: {:?}, params: {:?} ",
        program, location, params
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_get_uniformuiv(program, location, params) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetUniformdv(program: GLuint, location: GLint, params: *mut GLdouble) {
    trace!(
        "glGetUniformdv called, parameters: program: {:?}, location: {:?}, params: {:?} ",
        program, location, params
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_get_uniformdv(program, location, params) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetnUniformdv(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLdouble,
) {
    trace!(
        "glGetnUniformdv called, parameters: program: {:?}, location: {:?}, bufSize: {:?}, params: {:?} ",
        program, location, bufSize, params
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_getn_uniformdv(program, location, bufSize, params)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetnUniformfv(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLfloat,
) {
    trace!(
        "glGetnUniformfv called, parameters: program: {:?}, location: {:?}, bufSize: {:?}, params: {:?} ",
        program, location, bufSize, params
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_getn_uniformfv(program, location, bufSize, params)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetnUniformiv(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLint,
) {
    trace!(
        "glGetnUniformiv called, parameters: program: {:?}, location: {:?}, bufSize: {:?}, params: {:?} ",
        program, location, bufSize, params
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_getn_uniformiv(program, location, bufSize, params)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetnUniformuiv(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLuint,
) {
    trace!(
        "glGetnUniformuiv called, parameters: program: {:?}, location: {:?}, bufSize: {:?}, params: {:?} ",
        program, location, bufSize, params
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_getn_uniformuiv(program, location, bufSize, params)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetUniformBlockIndex(
    program: GLuint,
    uniformBlockName: *const GLchar,
) -> GLuint {
    trace!(
        "glGetUniformBlockIndex called, parameters: program: {:?}, uniformBlockName: {:?} ",
        program, uniformBlockName
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_uniform_block_index(program, uniformBlockName)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetUniformIndices(
    program: GLuint,
    uniformCount: GLsizei,
    uniformNames: *const *const GLchar,
    uniformIndices: *mut GLuint,
) {
    trace!(
        "glGetUniformIndices called, parameters: program: {:?}, uniformCount: {:?}, uniformNames: {:?}, uniformIndices: {:?} ",
        program, uniformCount, uniformNames, uniformIndices
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_uniform_indices(program, uniformCount, uniformNames, uniformIndices)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
    trace!(
        "glGetUniformLocation called, parameters: program: {:?}, name: {:?} ",
        program, name
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_get_uniform_location(program, name) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetUniformSubroutineuiv(
    shadertype: GLenum,
    location: GLint,
    params: *mut GLuint,
) {
    trace!(
        "glGetUniformSubroutineuiv called, parameters: shadertype: {:?}, location: {:?}, params: {:?} ",
        shadertype, location, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_uniform_subroutineuiv(shadertype.try_into_enum()?, location, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetVertexArrayIndexediv(
    vaobj: GLuint,
    index: GLuint,
    pname: GLenum,
    param: *mut GLint,
) {
    trace!(
        "glGetVertexArrayIndexediv called, parameters: vaobj: {:?}, index: {:?}, pname: {:?}, param: {:?} ",
        vaobj, index, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_vertex_array_indexediv(vaobj, index, pname.try_into_enum()?, param)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetVertexArrayIndexed64iv(
    vaobj: GLuint,
    index: GLuint,
    pname: GLenum,
    param: *mut GLint64,
) {
    trace!(
        "glGetVertexArrayIndexed64iv called, parameters: vaobj: {:?}, index: {:?}, pname: {:?}, param: {:?} ",
        vaobj, index, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_vertex_array_indexed64iv(vaobj, index, pname.try_into_enum()?, param)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetVertexArrayiv(vaobj: GLuint, pname: GLenum, param: *mut GLint) {
    trace!(
        "glGetVertexArrayiv called, parameters: vaobj: {:?}, pname: {:?}, param: {:?} ",
        vaobj, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_vertex_arrayiv(vaobj, pname.try_into_enum()?, param)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble) {
    trace!(
        "glGetVertexAttribdv called, parameters: index: {:?}, pname: {:?}, params: {:?} ",
        index, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_vertex_attribdv(index, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) {
    trace!(
        "glGetVertexAttribfv called, parameters: index: {:?}, pname: {:?}, params: {:?} ",
        index, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_vertex_attribfv(index, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) {
    trace!(
        "glGetVertexAttribiv called, parameters: index: {:?}, pname: {:?}, params: {:?} ",
        index, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_vertex_attribiv(index, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint) {
    trace!(
        "glGetVertexAttribIiv called, parameters: index: {:?}, pname: {:?}, params: {:?} ",
        index, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_vertex_attrib_iiv(index, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint) {
    trace!(
        "glGetVertexAttribIuiv called, parameters: index: {:?}, pname: {:?}, params: {:?} ",
        index, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_vertex_attrib_iuiv(index, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetVertexAttribLdv(index: GLuint, pname: GLenum, params: *mut GLdouble) {
    trace!(
        "glGetVertexAttribLdv called, parameters: index: {:?}, pname: {:?}, params: {:?} ",
        index, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_get_vertex_attrib_ldv(index, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glGetVertexAttribPointerv(
    index: GLuint,
    pname: GLenum,
    pointer: *mut *mut GLvoid,
) {
    trace!(
        "glGetVertexAttribPointerv called, parameters: index: {:?}, pname: {:?}, pointer: {:?} ",
        index, pname, pointer
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_get_vertex_attrib_pointerv(index, pname, pointer)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glHint(target: GLenum, mode: GLenum) {
    trace!(
        "glHint called, parameters: target: {:?}, mode: {:?} ",
        target, mode
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_hint(target.try_into_enum()?, mode.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glInvalidateBufferData(buffer: GLuint) {
    trace!(
        "glInvalidateBufferData called, parameters: buffer: {:?} ",
        buffer
    );
    with_ctx_mut(|mut state| state.oxidegl_invalidate_buffer_data(buffer))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glInvalidateBufferSubData(
    buffer: GLuint,
    offset: GLintptr,
    length: GLsizeiptr,
) {
    trace!(
        "glInvalidateBufferSubData called, parameters: buffer: {:?}, offset: {:?}, length: {:?} ",
        buffer, offset, length
    );
    with_ctx_mut(|mut state| state.oxidegl_invalidate_buffer_sub_data(buffer, offset, length))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glInvalidateFramebuffer(
    target: GLenum,
    numAttachments: GLsizei,
    attachments: GLenum,
) {
    trace!(
        "glInvalidateFramebuffer called, parameters: target: {:?}, numAttachments: {:?}, attachments: {:?} ",
        target, numAttachments, attachments
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_invalidate_framebuffer(
            target.try_into_enum()?,
            numAttachments,
            attachments.try_into_enum()?,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glInvalidateNamedFramebufferData(
    framebuffer: GLuint,
    numAttachments: GLsizei,
    attachments: GLenum,
) {
    trace!(
        "glInvalidateNamedFramebufferData called, parameters: framebuffer: {:?}, numAttachments: {:?}, attachments: {:?} ",
        framebuffer, numAttachments, attachments
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_invalidate_named_framebuffer_data(
            framebuffer,
            numAttachments,
            attachments.try_into_enum()?,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glInvalidateSubFramebuffer(
    target: GLenum,
    numAttachments: GLsizei,
    attachments: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    trace!(
        "glInvalidateSubFramebuffer called, parameters: target: {:?}, numAttachments: {:?}, attachments: {:?}, x: {:?}, y: {:?}, width: {:?}, height: {:?} ",
        target, numAttachments, attachments, x, y, width, height
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_invalidate_sub_framebuffer(
            target.try_into_enum()?,
            numAttachments,
            attachments.try_into_enum()?,
            x,
            y,
            width,
            height,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glInvalidateNamedFramebufferSubData(
    framebuffer: GLuint,
    numAttachments: GLsizei,
    attachments: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    trace!(
        "glInvalidateNamedFramebufferSubData called, parameters: framebuffer: {:?}, numAttachments: {:?}, attachments: {:?}, x: {:?}, y: {:?}, width: {:?}, height: {:?} ",
        framebuffer, numAttachments, attachments, x, y, width, height
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_invalidate_named_framebuffer_sub_data(
            framebuffer,
            numAttachments,
            attachments.try_into_enum()?,
            x,
            y,
            width,
            height,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glInvalidateTexImage(texture: GLuint, level: GLint) {
    trace!(
        "glInvalidateTexImage called, parameters: texture: {:?}, level: {:?} ",
        texture, level
    );
    with_ctx_mut(|mut state| state.oxidegl_invalidate_tex_image(texture, level))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glInvalidateTexSubImage(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
) {
    trace!(
        "glInvalidateTexSubImage called, parameters: texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?} ",
        texture, level, xoffset, yoffset, zoffset, width, height, depth
    );
    with_ctx_mut(|mut state| {
        state.oxidegl_invalidate_tex_sub_image(
            texture, level, xoffset, yoffset, zoffset, width, height, depth,
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glIsBuffer(buffer: GLuint) -> GLboolean {
    trace!("glIsBuffer called, parameters: buffer: {:?} ", buffer);
    with_ctx_mut(|mut state| state.oxidegl_is_buffer(buffer))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glIsEnabled(cap: GLenum) -> GLboolean {
    trace!("glIsEnabled called, parameters: cap: {:?} ", cap);
    with_ctx_mut(|mut state| GlResult::normalize(state.oxidegl_is_enabled(cap.try_into_enum()?)))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glIsEnabledi(target: GLenum, index: GLuint) -> GLboolean {
    trace!(
        "glIsEnabledi called, parameters: target: {:?}, index: {:?} ",
        target, index
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_is_enabledi(target.try_into_enum()?, index))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean {
    trace!(
        "glIsFramebuffer called, parameters: framebuffer: {:?} ",
        framebuffer
    );
    with_ctx_mut(|mut state| state.oxidegl_is_framebuffer(framebuffer))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glIsProgram(program: GLuint) -> GLboolean {
    trace!("glIsProgram called, parameters: program: {:?} ", program);
    with_ctx_mut(|mut state| state.oxidegl_is_program(program))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glIsProgramPipeline(pipeline: GLuint) -> GLboolean {
    trace!(
        "glIsProgramPipeline called, parameters: pipeline: {:?} ",
        pipeline
    );
    with_ctx_mut(|mut state| state.oxidegl_is_program_pipeline(pipeline))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glIsQuery(id: GLuint) -> GLboolean {
    trace!("glIsQuery called, parameters: id: {:?} ", id);
    with_ctx_mut(|mut state| state.oxidegl_is_query(id))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean {
    trace!(
        "glIsRenderbuffer called, parameters: renderbuffer: {:?} ",
        renderbuffer
    );
    with_ctx_mut(|mut state| state.oxidegl_is_renderbuffer(renderbuffer))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glIsSampler(sampler: GLuint) -> GLboolean {
    trace!("glIsSampler called, parameters: sampler: {:?} ", sampler);
    with_ctx_mut(|mut state| state.oxidegl_is_sampler(sampler))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glIsShader(shader: GLuint) -> GLboolean {
    trace!("glIsShader called, parameters: shader: {:?} ", shader);
    with_ctx_mut(|mut state| state.oxidegl_is_shader(shader))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glIsSync(sync: GLsync) -> GLboolean {
    trace!("glIsSync called, parameters: sync: {:?} ", sync);
    with_ctx_mut(|mut state| state.oxidegl_is_sync(sync))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glIsTexture(texture: GLuint) -> GLboolean {
    trace!("glIsTexture called, parameters: texture: {:?} ", texture);
    with_ctx_mut(|mut state| state.oxidegl_is_texture(texture))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glIsTransformFeedback(id: GLuint) -> GLboolean {
    trace!("glIsTransformFeedback called, parameters: id: {:?} ", id);
    with_ctx_mut(|mut state| state.oxidegl_is_transform_feedback(id))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glIsVertexArray(array: GLuint) -> GLboolean {
    trace!("glIsVertexArray called, parameters: array: {:?} ", array);
    with_ctx_mut(|mut state| state.oxidegl_is_vertex_array(array))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glLineWidth(width: GLfloat) {
    trace!("glLineWidth called, parameters: width: {:?} ", width);
    with_ctx_mut(|mut state| state.oxidegl_line_width(width))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glLinkProgram(program: GLuint) {
    trace!("glLinkProgram called, parameters: program: {:?} ", program);
    with_ctx_mut(|mut state| state.oxidegl_link_program(program))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glLogicOp(opcode: GLenum) {
    trace!("glLogicOp called, parameters: opcode: {:?} ", opcode);
    with_ctx_mut(|mut state| GlResult::normalize(state.oxidegl_logic_op(opcode.try_into_enum()?)))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glMapBuffer(target: GLenum, access: GLenum) -> *mut GLvoid {
    trace!(
        "glMapBuffer called, parameters: target: {:?}, access: {:?} ",
        target, access
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(
            state.oxidegl_map_buffer(target.try_into_enum()?, access.try_into_enum()?),
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glMapNamedBuffer(buffer: GLuint, access: GLenum) -> *mut GLvoid {
    trace!(
        "glMapNamedBuffer called, parameters: buffer: {:?}, access: {:?} ",
        buffer, access
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_map_named_buffer(buffer, access.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glMapBufferRange(
    target: GLenum,
    offset: GLintptr,
    length: GLsizeiptr,
    access: GLenum,
) -> *mut GLvoid {
    trace!(
        "glMapBufferRange called, parameters: target: {:?}, offset: {:?}, length: {:?}, access: {:?} ",
        target, offset, length, access
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_map_buffer_range(
            target.try_into_enum()?,
            offset,
            length,
            access.try_into_enum()?,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glMapNamedBufferRange(
    buffer: GLuint,
    offset: GLintptr,
    length: GLsizeiptr,
    access: GLenum,
) -> *mut GLvoid {
    trace!(
        "glMapNamedBufferRange called, parameters: buffer: {:?}, offset: {:?}, length: {:?}, access: {:?} ",
        buffer, offset, length, access
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_map_named_buffer_range(
            buffer,
            offset,
            length,
            access.try_into_enum()?,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glMemoryBarrier(barriers: GLenum) {
    trace!(
        "glMemoryBarrier called, parameters: barriers: {:?} ",
        barriers
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_memory_barrier(barriers.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glMemoryBarrierByRegion(barriers: GLenum) {
    trace!(
        "glMemoryBarrierByRegion called, parameters: barriers: {:?} ",
        barriers
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_memory_barrier_by_region(barriers.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glMinSampleShading(value: GLfloat) {
    trace!("glMinSampleShading called, parameters: value: {:?} ", value);
    with_ctx_mut(|mut state| state.oxidegl_min_sample_shading(value))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glMultiDrawArrays(
    mode: GLenum,
    first: *const GLint,
    count: *const GLsizei,
    drawcount: GLsizei,
) {
    trace!(
        "glMultiDrawArrays called, parameters: mode: {:?}, first: {:?}, count: {:?}, drawcount: {:?} ",
        mode, first, count, drawcount
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_multi_draw_arrays(mode.try_into_enum()?, first, count, drawcount)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glMultiDrawArraysIndirect(
    mode: GLenum,
    indirect: *const GLvoid,
    drawcount: GLsizei,
    stride: GLsizei,
) {
    trace!(
        "glMultiDrawArraysIndirect called, parameters: mode: {:?}, indirect: {:?}, drawcount: {:?}, stride: {:?} ",
        mode, indirect, drawcount, stride
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_multi_draw_arrays_indirect(
                mode.try_into_enum()?,
                indirect,
                drawcount,
                stride,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glMultiDrawElements(
    mode: GLenum,
    count: *const GLsizei,
    r#type: GLenum,
    indices: *mut *const GLvoid,
    drawcount: GLsizei,
) {
    trace!(
        "glMultiDrawElements called, parameters: mode: {:?}, count: {:?}, r#type: {:?}, indices: {:?}, drawcount: {:?} ",
        mode, count, r#type, indices, drawcount
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_multi_draw_elements(
                mode.try_into_enum()?,
                count,
                r#type.try_into_enum()?,
                indices,
                drawcount,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glMultiDrawElementsBaseVertex(
    mode: GLenum,
    count: *const GLsizei,
    r#type: GLenum,
    indices: *mut *const GLvoid,
    drawcount: GLsizei,
    basevertex: *const GLint,
) {
    trace!(
        "glMultiDrawElementsBaseVertex called, parameters: mode: {:?}, count: {:?}, r#type: {:?}, indices: {:?}, drawcount: {:?}, basevertex: {:?} ",
        mode, count, r#type, indices, drawcount, basevertex
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_multi_draw_elements_base_vertex(
                mode.try_into_enum()?,
                count,
                r#type.try_into_enum()?,
                indices,
                drawcount,
                basevertex,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glMultiDrawElementsIndirect(
    mode: GLenum,
    r#type: GLenum,
    indirect: *const GLvoid,
    drawcount: GLsizei,
    stride: GLsizei,
) {
    trace!(
        "glMultiDrawElementsIndirect called, parameters: mode: {:?}, r#type: {:?}, indirect: {:?}, drawcount: {:?}, stride: {:?} ",
        mode, r#type, indirect, drawcount, stride
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_multi_draw_elements_indirect(
                mode.try_into_enum()?,
                r#type.try_into_enum()?,
                indirect,
                drawcount,
                stride,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glObjectLabel(
    identifier: GLenum,
    name: GLuint,
    length: GLsizei,
    label: *const GLchar,
) {
    trace!(
        "glObjectLabel called, parameters: identifier: {:?}, name: {:?}, length: {:?}, label: {:?} ",
        identifier, name, length, label
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_object_label(identifier.try_into_enum()?, name, length, label)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glObjectPtrLabel(
    ptr: *const GLvoid,
    length: GLsizei,
    label: *const GLchar,
) {
    trace!(
        "glObjectPtrLabel called, parameters: ptr: {:?}, length: {:?}, label: {:?} ",
        ptr, length, label
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_object_ptr_label(ptr, length, label) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPatchParameteri(pname: GLenum, value: GLint) {
    trace!(
        "glPatchParameteri called, parameters: pname: {:?}, value: {:?} ",
        pname, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_patch_parameteri(pname.try_into_enum()?, value))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPatchParameterfv(pname: GLenum, values: *const GLfloat) {
    trace!(
        "glPatchParameterfv called, parameters: pname: {:?}, values: {:?} ",
        pname, values
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_patch_parameterfv(pname.try_into_enum()?, values)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPauseTransformFeedback() {
    trace!("glPauseTransformFeedback called, parameters:  ",);
    with_ctx_mut(|mut state| state.oxidegl_pause_transform_feedback())
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPixelStoref(pname: GLenum, param: GLfloat) {
    trace!(
        "glPixelStoref called, parameters: pname: {:?}, param: {:?} ",
        pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_pixel_storef(pname.try_into_enum()?, param))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPixelStorei(pname: GLenum, param: GLint) {
    trace!(
        "glPixelStorei called, parameters: pname: {:?}, param: {:?} ",
        pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_pixel_storei(pname.try_into_enum()?, param))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPointParameterf(pname: GLenum, param: GLfloat) {
    trace!(
        "glPointParameterf called, parameters: pname: {:?}, param: {:?} ",
        pname, param
    );
    with_ctx_mut(|mut state| state.oxidegl_point_parameterf(pname, param))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPointParameterfv(pname: GLenum, params: *const GLfloat) {
    trace!(
        "glPointParameterfv called, parameters: pname: {:?}, params: {:?} ",
        pname, params
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_point_parameterfv(pname, params) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPointParameteri(pname: GLenum, param: GLint) {
    trace!(
        "glPointParameteri called, parameters: pname: {:?}, param: {:?} ",
        pname, param
    );
    with_ctx_mut(|mut state| state.oxidegl_point_parameteri(pname, param))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPointParameteriv(pname: GLenum, params: *const GLint) {
    trace!(
        "glPointParameteriv called, parameters: pname: {:?}, params: {:?} ",
        pname, params
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_point_parameteriv(pname, params) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPointSize(size: GLfloat) {
    trace!("glPointSize called, parameters: size: {:?} ", size);
    with_ctx_mut(|mut state| state.oxidegl_point_size(size))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPolygonMode(face: GLenum, mode: GLenum) {
    trace!(
        "glPolygonMode called, parameters: face: {:?}, mode: {:?} ",
        face, mode
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(
            state.oxidegl_polygon_mode(face.try_into_enum()?, mode.try_into_enum()?),
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPolygonOffset(factor: GLfloat, units: GLfloat) {
    trace!(
        "glPolygonOffset called, parameters: factor: {:?}, units: {:?} ",
        factor, units
    );
    with_ctx_mut(|mut state| state.oxidegl_polygon_offset(factor, units))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPopDebugGroup() {
    trace!("glPopDebugGroup called, parameters:  ",);
    with_ctx_mut(|mut state| state.oxidegl_pop_debug_group())
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPrimitiveRestartIndex(index: GLuint) {
    trace!(
        "glPrimitiveRestartIndex called, parameters: index: {:?} ",
        index
    );
    with_ctx_mut(|mut state| state.oxidegl_primitive_restart_index(index))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramBinary(
    program: GLuint,
    binaryFormat: GLenum,
    binary: *const GLvoid,
    length: GLsizei,
) {
    trace!(
        "glProgramBinary called, parameters: program: {:?}, binaryFormat: {:?}, binary: {:?}, length: {:?} ",
        program, binaryFormat, binary, length
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_binary(program, binaryFormat, binary, length)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramParameteri(program: GLuint, pname: GLenum, value: GLint) {
    trace!(
        "glProgramParameteri called, parameters: program: {:?}, pname: {:?}, value: {:?} ",
        program, pname, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_program_parameteri(
            program,
            pname.try_into_enum()?,
            value,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform1i(program: GLuint, location: GLint, v0: GLint) {
    trace!(
        "glProgramUniform1i called, parameters: program: {:?}, location: {:?}, v0: {:?} ",
        program, location, v0
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform1i(program, location, v0))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform1iv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
) {
    trace!(
        "glProgramUniform1iv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform1iv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform1f(program: GLuint, location: GLint, v0: GLfloat) {
    trace!(
        "glProgramUniform1f called, parameters: program: {:?}, location: {:?}, v0: {:?} ",
        program, location, v0
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform1f(program, location, v0))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform1fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
) {
    trace!(
        "glProgramUniform1fv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform1fv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform1ui(program: GLuint, location: GLint, v0: GLuint) {
    trace!(
        "glProgramUniform1ui called, parameters: program: {:?}, location: {:?}, v0: {:?} ",
        program, location, v0
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform1ui(program, location, v0))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform1uiv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
) {
    trace!(
        "glProgramUniform1uiv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform1uiv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform2i(
    program: GLuint,
    location: GLint,
    v0: GLint,
    v1: GLint,
) {
    trace!(
        "glProgramUniform2i called, parameters: program: {:?}, location: {:?}, v0: {:?}, v1: {:?} ",
        program, location, v0, v1
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform2i(program, location, v0, v1))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform2iv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
) {
    trace!(
        "glProgramUniform2iv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform2iv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform2f(
    program: GLuint,
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
) {
    trace!(
        "glProgramUniform2f called, parameters: program: {:?}, location: {:?}, v0: {:?}, v1: {:?} ",
        program, location, v0, v1
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform2f(program, location, v0, v1))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform2fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
) {
    trace!(
        "glProgramUniform2fv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform2fv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform2ui(
    program: GLuint,
    location: GLint,
    v0: GLuint,
    v1: GLuint,
) {
    trace!(
        "glProgramUniform2ui called, parameters: program: {:?}, location: {:?}, v0: {:?}, v1: {:?} ",
        program, location, v0, v1
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform2ui(program, location, v0, v1))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform2uiv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
) {
    trace!(
        "glProgramUniform2uiv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform2uiv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform3i(
    program: GLuint,
    location: GLint,
    v0: GLint,
    v1: GLint,
    v2: GLint,
) {
    trace!(
        "glProgramUniform3i called, parameters: program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?} ",
        program, location, v0, v1, v2
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform3i(program, location, v0, v1, v2))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform3iv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
) {
    trace!(
        "glProgramUniform3iv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform3iv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform3f(
    program: GLuint,
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
) {
    trace!(
        "glProgramUniform3f called, parameters: program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?} ",
        program, location, v0, v1, v2
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform3f(program, location, v0, v1, v2))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform3fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
) {
    trace!(
        "glProgramUniform3fv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform3fv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform3ui(
    program: GLuint,
    location: GLint,
    v0: GLuint,
    v1: GLuint,
    v2: GLuint,
) {
    trace!(
        "glProgramUniform3ui called, parameters: program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?} ",
        program, location, v0, v1, v2
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform3ui(program, location, v0, v1, v2))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform3uiv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
) {
    trace!(
        "glProgramUniform3uiv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform3uiv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform4i(
    program: GLuint,
    location: GLint,
    v0: GLint,
    v1: GLint,
    v2: GLint,
    v3: GLint,
) {
    trace!(
        "glProgramUniform4i called, parameters: program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?} ",
        program, location, v0, v1, v2, v3
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform4i(program, location, v0, v1, v2, v3))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform4iv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
) {
    trace!(
        "glProgramUniform4iv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform4iv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform4f(
    program: GLuint,
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
    v3: GLfloat,
) {
    trace!(
        "glProgramUniform4f called, parameters: program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?} ",
        program, location, v0, v1, v2, v3
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform4f(program, location, v0, v1, v2, v3))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform4fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
) {
    trace!(
        "glProgramUniform4fv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform4fv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform4ui(
    program: GLuint,
    location: GLint,
    v0: GLuint,
    v1: GLuint,
    v2: GLuint,
    v3: GLuint,
) {
    trace!(
        "glProgramUniform4ui called, parameters: program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?} ",
        program, location, v0, v1, v2, v3
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform4ui(program, location, v0, v1, v2, v3))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform4uiv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
) {
    trace!(
        "glProgramUniform4uiv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform4uiv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix2fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glProgramUniformMatrix2fv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix2fv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix3fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glProgramUniformMatrix3fv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix3fv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix4fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glProgramUniformMatrix4fv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix4fv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix2x3fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glProgramUniformMatrix2x3fv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix2x3fv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix3x2fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glProgramUniformMatrix3x2fv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix3x2fv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix2x4fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glProgramUniformMatrix2x4fv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix2x4fv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix4x2fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glProgramUniformMatrix4x2fv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix4x2fv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix3x4fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glProgramUniformMatrix3x4fv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix3x4fv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix4x3fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glProgramUniformMatrix4x3fv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix4x3fv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProvokingVertex(mode: GLenum) {
    trace!("glProvokingVertex called, parameters: mode: {:?} ", mode);
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_provoking_vertex(mode.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPushDebugGroup(
    source: GLenum,
    id: GLuint,
    length: GLsizei,
    message: *const GLchar,
) {
    trace!(
        "glPushDebugGroup called, parameters: source: {:?}, id: {:?}, length: {:?}, message: {:?} ",
        source, id, length, message
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_push_debug_group(source.try_into_enum()?, id, length, message)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glQueryCounter(id: GLuint, target: GLenum) {
    trace!(
        "glQueryCounter called, parameters: id: {:?}, target: {:?} ",
        id, target
    );
    with_ctx_mut(|mut state| state.oxidegl_query_counter(id, target))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glReadBuffer(src: GLenum) {
    trace!("glReadBuffer called, parameters: src: {:?} ", src);
    with_ctx_mut(|mut state| GlResult::normalize(state.oxidegl_read_buffer(src.try_into_enum()?)))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glNamedFramebufferReadBuffer(framebuffer: GLuint, src: GLenum) {
    trace!(
        "glNamedFramebufferReadBuffer called, parameters: framebuffer: {:?}, src: {:?} ",
        framebuffer, src
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(
            state.oxidegl_named_framebuffer_read_buffer(framebuffer, src.try_into_enum()?),
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glReadPixels(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    r#type: GLenum,
    pixels: *mut GLvoid,
) {
    trace!(
        "glReadPixels called, parameters: x: {:?}, y: {:?}, width: {:?}, height: {:?}, format: {:?}, r#type: {:?}, pixels: {:?} ",
        x, y, width, height, format, r#type, pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_read_pixels(
                x,
                y,
                width,
                height,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                pixels,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glReadnPixels(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    r#type: GLenum,
    bufSize: GLsizei,
    data: *mut GLvoid,
) {
    trace!(
        "glReadnPixels called, parameters: x: {:?}, y: {:?}, width: {:?}, height: {:?}, format: {:?}, r#type: {:?}, bufSize: {:?}, data: {:?} ",
        x, y, width, height, format, r#type, bufSize, data
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_readn_pixels(
                x,
                y,
                width,
                height,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                bufSize,
                data,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glReleaseShaderCompiler() {
    trace!("glReleaseShaderCompiler called, parameters:  ",);
    with_ctx_mut(|mut state| state.oxidegl_release_shader_compiler())
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glRenderbufferStorage(
    target: GLenum,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    trace!(
        "glRenderbufferStorage called, parameters: target: {:?}, internalformat: {:?}, width: {:?}, height: {:?} ",
        target, internalformat, width, height
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_renderbuffer_storage(
            target,
            internalformat.try_into_enum()?,
            width,
            height,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glNamedRenderbufferStorage(
    renderbuffer: GLuint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    trace!(
        "glNamedRenderbufferStorage called, parameters: renderbuffer: {:?}, internalformat: {:?}, width: {:?}, height: {:?} ",
        renderbuffer, internalformat, width, height
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_named_renderbuffer_storage(
            renderbuffer,
            internalformat.try_into_enum()?,
            width,
            height,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glRenderbufferStorageMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    trace!(
        "glRenderbufferStorageMultisample called, parameters: target: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?} ",
        target, samples, internalformat, width, height
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_renderbuffer_storage_multisample(
            target,
            samples,
            internalformat.try_into_enum()?,
            width,
            height,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glNamedRenderbufferStorageMultisample(
    renderbuffer: GLuint,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    trace!(
        "glNamedRenderbufferStorageMultisample called, parameters: renderbuffer: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?} ",
        renderbuffer, samples, internalformat, width, height
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_named_renderbuffer_storage_multisample(
            renderbuffer,
            samples,
            internalformat.try_into_enum()?,
            width,
            height,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glResumeTransformFeedback() {
    trace!("glResumeTransformFeedback called, parameters:  ",);
    with_ctx_mut(|mut state| state.oxidegl_resume_transform_feedback())
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glSampleCoverage(value: GLfloat, invert: GLboolean) {
    trace!(
        "glSampleCoverage called, parameters: value: {:?}, invert: {:?} ",
        value, invert
    );
    with_ctx_mut(|mut state| state.oxidegl_sample_coverage(value, invert))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glSampleMaski(maskNumber: GLuint, mask: GLbitfield) {
    trace!(
        "glSampleMaski called, parameters: maskNumber: {:?}, mask: {:?} ",
        maskNumber, mask
    );
    with_ctx_mut(|mut state| state.oxidegl_sample_maski(maskNumber, mask))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glSamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint) {
    trace!(
        "glSamplerParameteri called, parameters: sampler: {:?}, pname: {:?}, param: {:?} ",
        sampler, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_sampler_parameteri(
            sampler,
            pname.try_into_enum()?,
            param,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glSamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint) {
    trace!(
        "glSamplerParameteriv called, parameters: sampler: {:?}, pname: {:?}, param: {:?} ",
        sampler, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_sampler_parameteriv(sampler, pname.try_into_enum()?, param)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glSamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat) {
    trace!(
        "glSamplerParameterf called, parameters: sampler: {:?}, pname: {:?}, param: {:?} ",
        sampler, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_sampler_parameterf(
            sampler,
            pname.try_into_enum()?,
            param,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glSamplerParameterfv(
    sampler: GLuint,
    pname: GLenum,
    param: *const GLfloat,
) {
    trace!(
        "glSamplerParameterfv called, parameters: sampler: {:?}, pname: {:?}, param: {:?} ",
        sampler, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_sampler_parameterfv(sampler, pname.try_into_enum()?, param)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glSamplerParameterIiv(
    sampler: GLuint,
    pname: GLenum,
    param: *const GLint,
) {
    trace!(
        "glSamplerParameterIiv called, parameters: sampler: {:?}, pname: {:?}, param: {:?} ",
        sampler, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_sampler_parameter_iiv(sampler, pname.try_into_enum()?, param)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glSamplerParameterIuiv(
    sampler: GLuint,
    pname: GLenum,
    param: *const GLuint,
) {
    trace!(
        "glSamplerParameterIuiv called, parameters: sampler: {:?}, pname: {:?}, param: {:?} ",
        sampler, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_sampler_parameter_iuiv(sampler, pname.try_into_enum()?, param)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    trace!(
        "glScissor called, parameters: x: {:?}, y: {:?}, width: {:?}, height: {:?} ",
        x, y, width, height
    );
    with_ctx_mut(|mut state| state.oxidegl_scissor(x, y, width, height))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glScissorArrayv(first: GLuint, count: GLsizei, v: *const GLint) {
    trace!(
        "glScissorArrayv called, parameters: first: {:?}, count: {:?}, v: {:?} ",
        first, count, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_scissor_arrayv(first, count, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glScissorIndexed(
    index: GLuint,
    left: GLint,
    bottom: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    trace!(
        "glScissorIndexed called, parameters: index: {:?}, left: {:?}, bottom: {:?}, width: {:?}, height: {:?} ",
        index, left, bottom, width, height
    );
    with_ctx_mut(|mut state| state.oxidegl_scissor_indexed(index, left, bottom, width, height))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glScissorIndexedv(index: GLuint, v: *const GLint) {
    trace!(
        "glScissorIndexedv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_scissor_indexedv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glShaderBinary(
    count: GLsizei,
    shaders: *const GLuint,
    binaryFormat: GLenum,
    binary: *const GLvoid,
    length: GLsizei,
) {
    trace!(
        "glShaderBinary called, parameters: count: {:?}, shaders: {:?}, binaryFormat: {:?}, binary: {:?}, length: {:?} ",
        count, shaders, binaryFormat, binary, length
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_shader_binary(count, shaders, binaryFormat, binary, length)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glShaderSource(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar,
    length: *const GLint,
) {
    trace!(
        "glShaderSource called, parameters: shader: {:?}, count: {:?}, string: {:?}, length: {:?} ",
        shader, count, string, length
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_shader_source(shader, count, string, length) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glShaderStorageBlockBinding(
    program: GLuint,
    storageBlockIndex: GLuint,
    storageBlockBinding: GLuint,
) {
    trace!(
        "glShaderStorageBlockBinding called, parameters: program: {:?}, storageBlockIndex: {:?}, storageBlockBinding: {:?} ",
        program, storageBlockIndex, storageBlockBinding
    );
    with_ctx_mut(|mut state| {
        state.oxidegl_shader_storage_block_binding(program, storageBlockIndex, storageBlockBinding)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glStencilFunc(func: GLenum, r#ref: GLint, mask: GLuint) {
    trace!(
        "glStencilFunc called, parameters: func: {:?}, r#ref: {:?}, mask: {:?} ",
        func, r#ref, mask
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_stencil_func(func.try_into_enum()?, r#ref, mask))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glStencilFuncSeparate(
    face: GLenum,
    func: GLenum,
    r#ref: GLint,
    mask: GLuint,
) {
    trace!(
        "glStencilFuncSeparate called, parameters: face: {:?}, func: {:?}, r#ref: {:?}, mask: {:?} ",
        face, func, r#ref, mask
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_stencil_func_separate(
            face.try_into_enum()?,
            func.try_into_enum()?,
            r#ref,
            mask,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glStencilMask(mask: GLuint) {
    trace!("glStencilMask called, parameters: mask: {:?} ", mask);
    with_ctx_mut(|mut state| state.oxidegl_stencil_mask(mask))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glStencilMaskSeparate(face: GLenum, mask: GLuint) {
    trace!(
        "glStencilMaskSeparate called, parameters: face: {:?}, mask: {:?} ",
        face, mask
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_stencil_mask_separate(face.try_into_enum()?, mask))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) {
    trace!(
        "glStencilOp called, parameters: fail: {:?}, zfail: {:?}, zpass: {:?} ",
        fail, zfail, zpass
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_stencil_op(
            fail.try_into_enum()?,
            zfail.try_into_enum()?,
            zpass.try_into_enum()?,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glStencilOpSeparate(
    face: GLenum,
    sfail: GLenum,
    dpfail: GLenum,
    dppass: GLenum,
) {
    trace!(
        "glStencilOpSeparate called, parameters: face: {:?}, sfail: {:?}, dpfail: {:?}, dppass: {:?} ",
        face, sfail, dpfail, dppass
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_stencil_op_separate(
            face.try_into_enum()?,
            sfail.try_into_enum()?,
            dpfail.try_into_enum()?,
            dppass.try_into_enum()?,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint) {
    trace!(
        "glTexBuffer called, parameters: target: {:?}, internalformat: {:?}, buffer: {:?} ",
        target, internalformat, buffer
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_tex_buffer(
            target.try_into_enum()?,
            internalformat.try_into_enum()?,
            buffer,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureBuffer(texture: GLuint, internalformat: GLenum, buffer: GLuint) {
    trace!(
        "glTextureBuffer called, parameters: texture: {:?}, internalformat: {:?}, buffer: {:?} ",
        texture, internalformat, buffer
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_texture_buffer(
            texture,
            internalformat.try_into_enum()?,
            buffer,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexBufferRange(
    target: GLenum,
    internalformat: GLenum,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
) {
    trace!(
        "glTexBufferRange called, parameters: target: {:?}, internalformat: {:?}, buffer: {:?}, offset: {:?}, size: {:?} ",
        target, internalformat, buffer, offset, size
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_tex_buffer_range(
            target.try_into_enum()?,
            internalformat.try_into_enum()?,
            buffer,
            offset,
            size,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureBufferRange(
    texture: GLuint,
    internalformat: GLenum,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
) {
    trace!(
        "glTextureBufferRange called, parameters: texture: {:?}, internalformat: {:?}, buffer: {:?}, offset: {:?}, size: {:?} ",
        texture, internalformat, buffer, offset, size
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_texture_buffer_range(
            texture,
            internalformat.try_into_enum()?,
            buffer,
            offset,
            size,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexImage1D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    border: GLint,
    format: GLenum,
    r#type: GLenum,
    pixels: *const GLvoid,
) {
    trace!(
        "glTexImage1D called, parameters: target: {:?}, level: {:?}, internalformat: {:?}, width: {:?}, border: {:?}, format: {:?}, r#type: {:?}, pixels: {:?} ",
        target, level, internalformat, width, border, format, r#type, pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_tex_image1_d(
                target.try_into_enum()?,
                level,
                internalformat.try_into_enum()?,
                width,
                border,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                pixels,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    r#type: GLenum,
    pixels: *const GLvoid,
) {
    trace!(
        "glTexImage2D called, parameters: target: {:?}, level: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, border: {:?}, format: {:?}, r#type: {:?}, pixels: {:?} ",
        target, level, internalformat, width, height, border, format, r#type, pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_tex_image2_d(
                target.try_into_enum()?,
                level,
                internalformat.try_into_enum()?,
                width,
                height,
                border,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                pixels,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexImage2DMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    trace!(
        "glTexImage2DMultisample called, parameters: target: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, fixedsamplelocations: {:?} ",
        target, samples, internalformat, width, height, fixedsamplelocations
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_tex_image2_d_multisample(
            target.try_into_enum()?,
            samples,
            internalformat.try_into_enum()?,
            width,
            height,
            fixedsamplelocations,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexImage3D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    format: GLenum,
    r#type: GLenum,
    pixels: *const GLvoid,
) {
    trace!(
        "glTexImage3D called, parameters: target: {:?}, level: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, depth: {:?}, border: {:?}, format: {:?}, r#type: {:?}, pixels: {:?} ",
        target, level, internalformat, width, height, depth, border, format, r#type, pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_tex_image3_d(
                target.try_into_enum()?,
                level,
                internalformat.try_into_enum()?,
                width,
                height,
                depth,
                border,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                pixels,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexImage3DMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    trace!(
        "glTexImage3DMultisample called, parameters: target: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, depth: {:?}, fixedsamplelocations: {:?} ",
        target, samples, internalformat, width, height, depth, fixedsamplelocations
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_tex_image3_d_multisample(
            target.try_into_enum()?,
            samples,
            internalformat.try_into_enum()?,
            width,
            height,
            depth,
            fixedsamplelocations,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat) {
    trace!(
        "glTexParameterf called, parameters: target: {:?}, pname: {:?}, param: {:?} ",
        target, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_tex_parameterf(
            target.try_into_enum()?,
            pname.try_into_enum()?,
            param,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) {
    trace!(
        "glTexParameterfv called, parameters: target: {:?}, pname: {:?}, params: {:?} ",
        target, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_tex_parameterfv(target.try_into_enum()?, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) {
    trace!(
        "glTexParameteri called, parameters: target: {:?}, pname: {:?}, param: {:?} ",
        target, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_tex_parameteri(
            target.try_into_enum()?,
            pname.try_into_enum()?,
            param,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) {
    trace!(
        "glTexParameteriv called, parameters: target: {:?}, pname: {:?}, params: {:?} ",
        target, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_tex_parameteriv(target.try_into_enum()?, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexParameterIiv(target: GLenum, pname: GLenum, params: *const GLint) {
    trace!(
        "glTexParameterIiv called, parameters: target: {:?}, pname: {:?}, params: {:?} ",
        target, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_tex_parameter_iiv(target.try_into_enum()?, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexParameterIuiv(target: GLenum, pname: GLenum, params: *const GLuint) {
    trace!(
        "glTexParameterIuiv called, parameters: target: {:?}, pname: {:?}, params: {:?} ",
        target, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_tex_parameter_iuiv(
                target.try_into_enum()?,
                pname.try_into_enum()?,
                params,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureParameterf(texture: GLuint, pname: GLenum, param: GLfloat) {
    trace!(
        "glTextureParameterf called, parameters: texture: {:?}, pname: {:?}, param: {:?} ",
        texture, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_texture_parameterf(
            texture,
            pname.try_into_enum()?,
            param,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureParameterfv(
    texture: GLuint,
    pname: GLenum,
    param: *const GLfloat,
) {
    trace!(
        "glTextureParameterfv called, parameters: texture: {:?}, pname: {:?}, param: {:?} ",
        texture, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_texture_parameterfv(texture, pname.try_into_enum()?, param)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureParameteri(texture: GLuint, pname: GLenum, param: GLint) {
    trace!(
        "glTextureParameteri called, parameters: texture: {:?}, pname: {:?}, param: {:?} ",
        texture, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_texture_parameteri(
            texture,
            pname.try_into_enum()?,
            param,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureParameterIiv(
    texture: GLuint,
    pname: GLenum,
    params: *const GLint,
) {
    trace!(
        "glTextureParameterIiv called, parameters: texture: {:?}, pname: {:?}, params: {:?} ",
        texture, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_texture_parameter_iiv(texture, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureParameterIuiv(
    texture: GLuint,
    pname: GLenum,
    params: *const GLuint,
) {
    trace!(
        "glTextureParameterIuiv called, parameters: texture: {:?}, pname: {:?}, params: {:?} ",
        texture, pname, params
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_texture_parameter_iuiv(texture, pname.try_into_enum()?, params)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureParameteriv(texture: GLuint, pname: GLenum, param: *const GLint) {
    trace!(
        "glTextureParameteriv called, parameters: texture: {:?}, pname: {:?}, param: {:?} ",
        texture, pname, param
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_texture_parameteriv(texture, pname.try_into_enum()?, param)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexStorage1D(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
) {
    trace!(
        "glTexStorage1D called, parameters: target: {:?}, levels: {:?}, internalformat: {:?}, width: {:?} ",
        target, levels, internalformat, width
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_tex_storage1_d(
            target.try_into_enum()?,
            levels,
            internalformat.try_into_enum()?,
            width,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureStorage1D(
    texture: GLuint,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
) {
    trace!(
        "glTextureStorage1D called, parameters: texture: {:?}, levels: {:?}, internalformat: {:?}, width: {:?} ",
        texture, levels, internalformat, width
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_texture_storage1_d(
            texture,
            levels,
            internalformat.try_into_enum()?,
            width,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexStorage2D(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    trace!(
        "glTexStorage2D called, parameters: target: {:?}, levels: {:?}, internalformat: {:?}, width: {:?}, height: {:?} ",
        target, levels, internalformat, width, height
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_tex_storage2_d(
            target.try_into_enum()?,
            levels,
            internalformat.try_into_enum()?,
            width,
            height,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureStorage2D(
    texture: GLuint,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    trace!(
        "glTextureStorage2D called, parameters: texture: {:?}, levels: {:?}, internalformat: {:?}, width: {:?}, height: {:?} ",
        texture, levels, internalformat, width, height
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_texture_storage2_d(
            texture,
            levels,
            internalformat.try_into_enum()?,
            width,
            height,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexStorage2DMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    trace!(
        "glTexStorage2DMultisample called, parameters: target: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, fixedsamplelocations: {:?} ",
        target, samples, internalformat, width, height, fixedsamplelocations
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_tex_storage2_d_multisample(
            target.try_into_enum()?,
            samples,
            internalformat.try_into_enum()?,
            width,
            height,
            fixedsamplelocations,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureStorage2DMultisample(
    texture: GLuint,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    trace!(
        "glTextureStorage2DMultisample called, parameters: texture: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, fixedsamplelocations: {:?} ",
        texture, samples, internalformat, width, height, fixedsamplelocations
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_texture_storage2_d_multisample(
            texture,
            samples,
            internalformat.try_into_enum()?,
            width,
            height,
            fixedsamplelocations,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexStorage3D(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
) {
    trace!(
        "glTexStorage3D called, parameters: target: {:?}, levels: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, depth: {:?} ",
        target, levels, internalformat, width, height, depth
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_tex_storage3_d(
            target.try_into_enum()?,
            levels,
            internalformat.try_into_enum()?,
            width,
            height,
            depth,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureStorage3D(
    texture: GLuint,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
) {
    trace!(
        "glTextureStorage3D called, parameters: texture: {:?}, levels: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, depth: {:?} ",
        texture, levels, internalformat, width, height, depth
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_texture_storage3_d(
            texture,
            levels,
            internalformat.try_into_enum()?,
            width,
            height,
            depth,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexStorage3DMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    trace!(
        "glTexStorage3DMultisample called, parameters: target: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, depth: {:?}, fixedsamplelocations: {:?} ",
        target, samples, internalformat, width, height, depth, fixedsamplelocations
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_tex_storage3_d_multisample(
            target.try_into_enum()?,
            samples,
            internalformat.try_into_enum()?,
            width,
            height,
            depth,
            fixedsamplelocations,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureStorage3DMultisample(
    texture: GLuint,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    trace!(
        "glTextureStorage3DMultisample called, parameters: texture: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, depth: {:?}, fixedsamplelocations: {:?} ",
        texture, samples, internalformat, width, height, depth, fixedsamplelocations
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_texture_storage3_d_multisample(
            texture,
            samples,
            internalformat.try_into_enum()?,
            width,
            height,
            depth,
            fixedsamplelocations,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexSubImage1D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    r#type: GLenum,
    pixels: *const GLvoid,
) {
    trace!(
        "glTexSubImage1D called, parameters: target: {:?}, level: {:?}, xoffset: {:?}, width: {:?}, format: {:?}, r#type: {:?}, pixels: {:?} ",
        target, level, xoffset, width, format, r#type, pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_tex_sub_image1_d(
                target.try_into_enum()?,
                level,
                xoffset,
                width,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                pixels,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureSubImage1D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    r#type: GLenum,
    pixels: *const GLvoid,
) {
    trace!(
        "glTextureSubImage1D called, parameters: texture: {:?}, level: {:?}, xoffset: {:?}, width: {:?}, format: {:?}, r#type: {:?}, pixels: {:?} ",
        texture, level, xoffset, width, format, r#type, pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_texture_sub_image1_d(
                texture,
                level,
                xoffset,
                width,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                pixels,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    r#type: GLenum,
    pixels: *const GLvoid,
) {
    trace!(
        "glTexSubImage2D called, parameters: target: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, width: {:?}, height: {:?}, format: {:?}, r#type: {:?}, pixels: {:?} ",
        target, level, xoffset, yoffset, width, height, format, r#type, pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_tex_sub_image2_d(
                target.try_into_enum()?,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                pixels,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureSubImage2D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    r#type: GLenum,
    pixels: *const GLvoid,
) {
    trace!(
        "glTextureSubImage2D called, parameters: texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, width: {:?}, height: {:?}, format: {:?}, r#type: {:?}, pixels: {:?} ",
        texture, level, xoffset, yoffset, width, height, format, r#type, pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_texture_sub_image2_d(
                texture,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                pixels,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTexSubImage3D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    r#type: GLenum,
    pixels: *const GLvoid,
) {
    trace!(
        "glTexSubImage3D called, parameters: target: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}, format: {:?}, r#type: {:?}, pixels: {:?} ",
        target, level, xoffset, yoffset, zoffset, width, height, depth, format, r#type, pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_tex_sub_image3_d(
                target.try_into_enum()?,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                pixels,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureSubImage3D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    r#type: GLenum,
    pixels: *const GLvoid,
) {
    trace!(
        "glTextureSubImage3D called, parameters: texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}, format: {:?}, r#type: {:?}, pixels: {:?} ",
        texture, level, xoffset, yoffset, zoffset, width, height, depth, format, r#type, pixels
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_texture_sub_image3_d(
                texture,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format.try_into_enum()?,
                r#type.try_into_enum()?,
                pixels,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureBarrier() {
    trace!("glTextureBarrier called, parameters:  ",);
    with_ctx_mut(|mut state| state.oxidegl_texture_barrier())
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTextureView(
    texture: GLuint,
    target: GLenum,
    origtexture: GLuint,
    internalformat: GLenum,
    minlevel: GLuint,
    numlevels: GLuint,
    minlayer: GLuint,
    numlayers: GLuint,
) {
    trace!(
        "glTextureView called, parameters: texture: {:?}, target: {:?}, origtexture: {:?}, internalformat: {:?}, minlevel: {:?}, numlevels: {:?}, minlayer: {:?}, numlayers: {:?} ",
        texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_texture_view(
            texture,
            target.try_into_enum()?,
            origtexture,
            internalformat.try_into_enum()?,
            minlevel,
            numlevels,
            minlayer,
            numlayers,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTransformFeedbackBufferBase(xfb: GLuint, index: GLuint, buffer: GLuint) {
    trace!(
        "glTransformFeedbackBufferBase called, parameters: xfb: {:?}, index: {:?}, buffer: {:?} ",
        xfb, index, buffer
    );
    with_ctx_mut(|mut state| state.oxidegl_transform_feedback_buffer_base(xfb, index, buffer))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTransformFeedbackBufferRange(
    xfb: GLuint,
    index: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
) {
    trace!(
        "glTransformFeedbackBufferRange called, parameters: xfb: {:?}, index: {:?}, buffer: {:?}, offset: {:?}, size: {:?} ",
        xfb, index, buffer, offset, size
    );
    with_ctx_mut(|mut state| {
        state.oxidegl_transform_feedback_buffer_range(xfb, index, buffer, offset, size)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glTransformFeedbackVaryings(
    program: GLuint,
    count: GLsizei,
    varyings: *const *const GLchar,
    bufferMode: GLenum,
) {
    trace!(
        "glTransformFeedbackVaryings called, parameters: program: {:?}, count: {:?}, varyings: {:?}, bufferMode: {:?} ",
        program, count, varyings, bufferMode
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_transform_feedback_varyings(
                program,
                count,
                varyings,
                bufferMode.try_into_enum()?,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform1f(location: GLint, v0: GLfloat) {
    trace!(
        "glUniform1f called, parameters: location: {:?}, v0: {:?} ",
        location, v0
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform1f(location, v0))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat) {
    trace!(
        "glUniform2f called, parameters: location: {:?}, v0: {:?}, v1: {:?} ",
        location, v0, v1
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform2f(location, v0, v1))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
    trace!(
        "glUniform3f called, parameters: location: {:?}, v0: {:?}, v1: {:?}, v2: {:?} ",
        location, v0, v1, v2
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform3f(location, v0, v1, v2))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform4f(
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
    v3: GLfloat,
) {
    trace!(
        "glUniform4f called, parameters: location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?} ",
        location, v0, v1, v2, v3
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform4f(location, v0, v1, v2, v3))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform1i(location: GLint, v0: GLint) {
    trace!(
        "glUniform1i called, parameters: location: {:?}, v0: {:?} ",
        location, v0
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform1i(location, v0))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform2i(location: GLint, v0: GLint, v1: GLint) {
    trace!(
        "glUniform2i called, parameters: location: {:?}, v0: {:?}, v1: {:?} ",
        location, v0, v1
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform2i(location, v0, v1))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) {
    trace!(
        "glUniform3i called, parameters: location: {:?}, v0: {:?}, v1: {:?}, v2: {:?} ",
        location, v0, v1, v2
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform3i(location, v0, v1, v2))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
    trace!(
        "glUniform4i called, parameters: location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?} ",
        location, v0, v1, v2, v3
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform4i(location, v0, v1, v2, v3))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    trace!(
        "glUniform1fv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform1fv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    trace!(
        "glUniform2fv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform2fv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    trace!(
        "glUniform3fv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform3fv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    trace!(
        "glUniform4fv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform4fv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform1iv(location: GLint, count: GLsizei, value: *const GLint) {
    trace!(
        "glUniform1iv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform1iv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform2iv(location: GLint, count: GLsizei, value: *const GLint) {
    trace!(
        "glUniform2iv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform2iv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform3iv(location: GLint, count: GLsizei, value: *const GLint) {
    trace!(
        "glUniform3iv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform3iv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform4iv(location: GLint, count: GLsizei, value: *const GLint) {
    trace!(
        "glUniform4iv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform4iv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glUniformMatrix2fv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix2fv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glUniformMatrix3fv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix3fv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glUniformMatrix4fv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix4fv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix2x3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glUniformMatrix2x3fv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix2x3fv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix3x2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glUniformMatrix3x2fv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix3x2fv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix2x4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glUniformMatrix2x4fv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix2x4fv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix4x2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glUniformMatrix4x2fv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix4x2fv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix3x4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glUniformMatrix3x4fv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix3x4fv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix4x3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    trace!(
        "glUniformMatrix4x3fv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix4x3fv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform1ui(location: GLint, v0: GLuint) {
    trace!(
        "glUniform1ui called, parameters: location: {:?}, v0: {:?} ",
        location, v0
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform1ui(location, v0))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform2ui(location: GLint, v0: GLuint, v1: GLuint) {
    trace!(
        "glUniform2ui called, parameters: location: {:?}, v0: {:?}, v1: {:?} ",
        location, v0, v1
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform2ui(location, v0, v1))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
    trace!(
        "glUniform3ui called, parameters: location: {:?}, v0: {:?}, v1: {:?}, v2: {:?} ",
        location, v0, v1, v2
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform3ui(location, v0, v1, v2))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform4ui(
    location: GLint,
    v0: GLuint,
    v1: GLuint,
    v2: GLuint,
    v3: GLuint,
) {
    trace!(
        "glUniform4ui called, parameters: location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?} ",
        location, v0, v1, v2, v3
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform4ui(location, v0, v1, v2, v3))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform1uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    trace!(
        "glUniform1uiv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform1uiv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform2uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    trace!(
        "glUniform2uiv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform2uiv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform3uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    trace!(
        "glUniform3uiv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform3uiv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform4uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    trace!(
        "glUniform4uiv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform4uiv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformBlockBinding(
    program: GLuint,
    uniformBlockIndex: GLuint,
    uniformBlockBinding: GLuint,
) {
    trace!(
        "glUniformBlockBinding called, parameters: program: {:?}, uniformBlockIndex: {:?}, uniformBlockBinding: {:?} ",
        program, uniformBlockIndex, uniformBlockBinding
    );
    with_ctx_mut(|mut state| {
        state.oxidegl_uniform_block_binding(program, uniformBlockIndex, uniformBlockBinding)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformSubroutinesuiv(
    shadertype: GLenum,
    count: GLsizei,
    indices: *const GLuint,
) {
    trace!(
        "glUniformSubroutinesuiv called, parameters: shadertype: {:?}, count: {:?}, indices: {:?} ",
        shadertype, count, indices
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_uniform_subroutinesuiv(shadertype.try_into_enum()?, count, indices)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUnmapBuffer(target: GLenum) -> GLboolean {
    trace!("glUnmapBuffer called, parameters: target: {:?} ", target);
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_unmap_buffer(target.try_into_enum()?))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUnmapNamedBuffer(buffer: GLuint) -> GLboolean {
    trace!(
        "glUnmapNamedBuffer called, parameters: buffer: {:?} ",
        buffer
    );
    with_ctx_mut(|mut state| state.oxidegl_unmap_named_buffer(buffer))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUseProgram(program: GLuint) {
    trace!("glUseProgram called, parameters: program: {:?} ", program);
    with_ctx_mut(|mut state| state.oxidegl_use_program(program))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUseProgramStages(pipeline: GLuint, stages: GLenum, program: GLuint) {
    trace!(
        "glUseProgramStages called, parameters: pipeline: {:?}, stages: {:?}, program: {:?} ",
        pipeline, stages, program
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_use_program_stages(
            pipeline,
            stages.try_into_enum()?,
            program,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glValidateProgram(program: GLuint) {
    trace!(
        "glValidateProgram called, parameters: program: {:?} ",
        program
    );
    with_ctx_mut(|mut state| state.oxidegl_validate_program(program))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glValidateProgramPipeline(pipeline: GLuint) {
    trace!(
        "glValidateProgramPipeline called, parameters: pipeline: {:?} ",
        pipeline
    );
    with_ctx_mut(|mut state| state.oxidegl_validate_program_pipeline(pipeline))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexArrayElementBuffer(vaobj: GLuint, buffer: GLuint) {
    trace!(
        "glVertexArrayElementBuffer called, parameters: vaobj: {:?}, buffer: {:?} ",
        vaobj, buffer
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_array_element_buffer(vaobj, buffer))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib1d(index: GLuint, x: GLdouble) {
    trace!(
        "glVertexAttrib1d called, parameters: index: {:?}, x: {:?} ",
        index, x
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib1d(index, x))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib1dv(index: GLuint, v: *const GLdouble) {
    trace!(
        "glVertexAttrib1dv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib1dv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib1f(index: GLuint, x: GLfloat) {
    trace!(
        "glVertexAttrib1f called, parameters: index: {:?}, x: {:?} ",
        index, x
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib1f(index, x))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib1fv(index: GLuint, v: *const GLfloat) {
    trace!(
        "glVertexAttrib1fv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib1fv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib1s(index: GLuint, x: GLshort) {
    trace!(
        "glVertexAttrib1s called, parameters: index: {:?}, x: {:?} ",
        index, x
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib1s(index, x))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib1sv(index: GLuint, v: *const GLshort) {
    trace!(
        "glVertexAttrib1sv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib1sv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) {
    trace!(
        "glVertexAttrib2d called, parameters: index: {:?}, x: {:?}, y: {:?} ",
        index, x, y
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib2d(index, x, y))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib2dv(index: GLuint, v: *const GLdouble) {
    trace!(
        "glVertexAttrib2dv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib2dv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) {
    trace!(
        "glVertexAttrib2f called, parameters: index: {:?}, x: {:?}, y: {:?} ",
        index, x, y
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib2f(index, x, y))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib2fv(index: GLuint, v: *const GLfloat) {
    trace!(
        "glVertexAttrib2fv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib2fv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) {
    trace!(
        "glVertexAttrib2s called, parameters: index: {:?}, x: {:?}, y: {:?} ",
        index, x, y
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib2s(index, x, y))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib2sv(index: GLuint, v: *const GLshort) {
    trace!(
        "glVertexAttrib2sv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib2sv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
    trace!(
        "glVertexAttrib3d called, parameters: index: {:?}, x: {:?}, y: {:?}, z: {:?} ",
        index, x, y, z
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib3d(index, x, y, z))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib3dv(index: GLuint, v: *const GLdouble) {
    trace!(
        "glVertexAttrib3dv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib3dv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
    trace!(
        "glVertexAttrib3f called, parameters: index: {:?}, x: {:?}, y: {:?}, z: {:?} ",
        index, x, y, z
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib3f(index, x, y, z))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib3fv(index: GLuint, v: *const GLfloat) {
    trace!(
        "glVertexAttrib3fv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib3fv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) {
    trace!(
        "glVertexAttrib3s called, parameters: index: {:?}, x: {:?}, y: {:?}, z: {:?} ",
        index, x, y, z
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib3s(index, x, y, z))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib3sv(index: GLuint, v: *const GLshort) {
    trace!(
        "glVertexAttrib3sv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib3sv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4Nbv(index: GLuint, v: *const GLbyte) {
    trace!(
        "glVertexAttrib4Nbv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib4_nbv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4Niv(index: GLuint, v: *const GLint) {
    trace!(
        "glVertexAttrib4Niv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib4_niv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4Nsv(index: GLuint, v: *const GLshort) {
    trace!(
        "glVertexAttrib4Nsv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib4_nsv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4Nub(
    index: GLuint,
    x: GLubyte,
    y: GLubyte,
    z: GLubyte,
    w: GLubyte,
) {
    trace!(
        "glVertexAttrib4Nub called, parameters: index: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?} ",
        index, x, y, z, w
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib4_nub(index, x, y, z, w))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4Nubv(index: GLuint, v: *const GLubyte) {
    trace!(
        "glVertexAttrib4Nubv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib4_nubv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4Nuiv(index: GLuint, v: *const GLuint) {
    trace!(
        "glVertexAttrib4Nuiv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib4_nuiv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4Nusv(index: GLuint, v: *const GLushort) {
    trace!(
        "glVertexAttrib4Nusv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib4_nusv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4bv(index: GLuint, v: *const GLbyte) {
    trace!(
        "glVertexAttrib4bv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib4bv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4d(
    index: GLuint,
    x: GLdouble,
    y: GLdouble,
    z: GLdouble,
    w: GLdouble,
) {
    trace!(
        "glVertexAttrib4d called, parameters: index: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?} ",
        index, x, y, z, w
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib4d(index, x, y, z, w))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4dv(index: GLuint, v: *const GLdouble) {
    trace!(
        "glVertexAttrib4dv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib4dv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4f(
    index: GLuint,
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
    w: GLfloat,
) {
    trace!(
        "glVertexAttrib4f called, parameters: index: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?} ",
        index, x, y, z, w
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib4f(index, x, y, z, w))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4fv(index: GLuint, v: *const GLfloat) {
    trace!(
        "glVertexAttrib4fv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib4fv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4iv(index: GLuint, v: *const GLint) {
    trace!(
        "glVertexAttrib4iv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib4iv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4s(
    index: GLuint,
    x: GLshort,
    y: GLshort,
    z: GLshort,
    w: GLshort,
) {
    trace!(
        "glVertexAttrib4s called, parameters: index: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?} ",
        index, x, y, z, w
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib4s(index, x, y, z, w))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4sv(index: GLuint, v: *const GLshort) {
    trace!(
        "glVertexAttrib4sv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib4sv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4ubv(index: GLuint, v: *const GLubyte) {
    trace!(
        "glVertexAttrib4ubv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib4ubv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4uiv(index: GLuint, v: *const GLuint) {
    trace!(
        "glVertexAttrib4uiv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib4uiv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttrib4usv(index: GLuint, v: *const GLushort) {
    trace!(
        "glVertexAttrib4usv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib4usv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI1i(index: GLuint, x: GLint) {
    trace!(
        "glVertexAttribI1i called, parameters: index: {:?}, x: {:?} ",
        index, x
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib_i1i(index, x))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI2i(index: GLuint, x: GLint, y: GLint) {
    trace!(
        "glVertexAttribI2i called, parameters: index: {:?}, x: {:?}, y: {:?} ",
        index, x, y
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib_i2i(index, x, y))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint) {
    trace!(
        "glVertexAttribI3i called, parameters: index: {:?}, x: {:?}, y: {:?}, z: {:?} ",
        index, x, y, z
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib_i3i(index, x, y, z))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
    trace!(
        "glVertexAttribI4i called, parameters: index: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?} ",
        index, x, y, z, w
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib_i4i(index, x, y, z, w))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI1ui(index: GLuint, x: GLuint) {
    trace!(
        "glVertexAttribI1ui called, parameters: index: {:?}, x: {:?} ",
        index, x
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib_i1ui(index, x))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) {
    trace!(
        "glVertexAttribI2ui called, parameters: index: {:?}, x: {:?}, y: {:?} ",
        index, x, y
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib_i2ui(index, x, y))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint) {
    trace!(
        "glVertexAttribI3ui called, parameters: index: {:?}, x: {:?}, y: {:?}, z: {:?} ",
        index, x, y, z
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib_i3ui(index, x, y, z))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI4ui(
    index: GLuint,
    x: GLuint,
    y: GLuint,
    z: GLuint,
    w: GLuint,
) {
    trace!(
        "glVertexAttribI4ui called, parameters: index: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?} ",
        index, x, y, z, w
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib_i4ui(index, x, y, z, w))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI1iv(index: GLuint, v: *const GLint) {
    trace!(
        "glVertexAttribI1iv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_i1iv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI2iv(index: GLuint, v: *const GLint) {
    trace!(
        "glVertexAttribI2iv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_i2iv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI3iv(index: GLuint, v: *const GLint) {
    trace!(
        "glVertexAttribI3iv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_i3iv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI4iv(index: GLuint, v: *const GLint) {
    trace!(
        "glVertexAttribI4iv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_i4iv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI1uiv(index: GLuint, v: *const GLuint) {
    trace!(
        "glVertexAttribI1uiv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_i1uiv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI2uiv(index: GLuint, v: *const GLuint) {
    trace!(
        "glVertexAttribI2uiv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_i2uiv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI3uiv(index: GLuint, v: *const GLuint) {
    trace!(
        "glVertexAttribI3uiv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_i3uiv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI4uiv(index: GLuint, v: *const GLuint) {
    trace!(
        "glVertexAttribI4uiv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_i4uiv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI4bv(index: GLuint, v: *const GLbyte) {
    trace!(
        "glVertexAttribI4bv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_i4bv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI4sv(index: GLuint, v: *const GLshort) {
    trace!(
        "glVertexAttribI4sv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_i4sv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI4ubv(index: GLuint, v: *const GLubyte) {
    trace!(
        "glVertexAttribI4ubv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_i4ubv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribI4usv(index: GLuint, v: *const GLushort) {
    trace!(
        "glVertexAttribI4usv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_i4usv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribP1ui(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: GLuint,
) {
    trace!(
        "glVertexAttribP1ui called, parameters: index: {:?}, r#type: {:?}, normalized: {:?}, value: {:?} ",
        index, r#type, normalized, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_vertex_attrib_p1ui(
            index,
            r#type.try_into_enum()?,
            normalized,
            value,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribP2ui(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: GLuint,
) {
    trace!(
        "glVertexAttribP2ui called, parameters: index: {:?}, r#type: {:?}, normalized: {:?}, value: {:?} ",
        index, r#type, normalized, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_vertex_attrib_p2ui(
            index,
            r#type.try_into_enum()?,
            normalized,
            value,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribP3ui(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: GLuint,
) {
    trace!(
        "glVertexAttribP3ui called, parameters: index: {:?}, r#type: {:?}, normalized: {:?}, value: {:?} ",
        index, r#type, normalized, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_vertex_attrib_p3ui(
            index,
            r#type.try_into_enum()?,
            normalized,
            value,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribP4ui(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: GLuint,
) {
    trace!(
        "glVertexAttribP4ui called, parameters: index: {:?}, r#type: {:?}, normalized: {:?}, value: {:?} ",
        index, r#type, normalized, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_vertex_attrib_p4ui(
            index,
            r#type.try_into_enum()?,
            normalized,
            value,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribL1d(index: GLuint, x: GLdouble) {
    trace!(
        "glVertexAttribL1d called, parameters: index: {:?}, x: {:?} ",
        index, x
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib_l1d(index, x))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribL2d(index: GLuint, x: GLdouble, y: GLdouble) {
    trace!(
        "glVertexAttribL2d called, parameters: index: {:?}, x: {:?}, y: {:?} ",
        index, x, y
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib_l2d(index, x, y))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribL3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
    trace!(
        "glVertexAttribL3d called, parameters: index: {:?}, x: {:?}, y: {:?}, z: {:?} ",
        index, x, y, z
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib_l3d(index, x, y, z))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribL4d(
    index: GLuint,
    x: GLdouble,
    y: GLdouble,
    z: GLdouble,
    w: GLdouble,
) {
    trace!(
        "glVertexAttribL4d called, parameters: index: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?} ",
        index, x, y, z, w
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib_l4d(index, x, y, z, w))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribL1dv(index: GLuint, v: *const GLdouble) {
    trace!(
        "glVertexAttribL1dv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_l1dv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribL2dv(index: GLuint, v: *const GLdouble) {
    trace!(
        "glVertexAttribL2dv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_l2dv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribL3dv(index: GLuint, v: *const GLdouble) {
    trace!(
        "glVertexAttribL3dv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_l3dv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribL4dv(index: GLuint, v: *const GLdouble) {
    trace!(
        "glVertexAttribL4dv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_vertex_attrib_l4dv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribBinding(attribindex: GLuint, bindingindex: GLuint) {
    trace!(
        "glVertexAttribBinding called, parameters: attribindex: {:?}, bindingindex: {:?} ",
        attribindex, bindingindex
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib_binding(attribindex, bindingindex))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexArrayAttribBinding(
    vaobj: GLuint,
    attribindex: GLuint,
    bindingindex: GLuint,
) {
    trace!(
        "glVertexArrayAttribBinding called, parameters: vaobj: {:?}, attribindex: {:?}, bindingindex: {:?} ",
        vaobj, attribindex, bindingindex
    );
    with_ctx_mut(|mut state| {
        state.oxidegl_vertex_array_attrib_binding(vaobj, attribindex, bindingindex)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribDivisor(index: GLuint, divisor: GLuint) {
    trace!(
        "glVertexAttribDivisor called, parameters: index: {:?}, divisor: {:?} ",
        index, divisor
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_attrib_divisor(index, divisor))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribFormat(
    attribindex: GLuint,
    size: GLint,
    r#type: GLenum,
    normalized: GLboolean,
    relativeoffset: GLuint,
) {
    trace!(
        "glVertexAttribFormat called, parameters: attribindex: {:?}, size: {:?}, r#type: {:?}, normalized: {:?}, relativeoffset: {:?} ",
        attribindex, size, r#type, normalized, relativeoffset
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_vertex_attrib_format(
            attribindex,
            size,
            r#type.try_into_enum()?,
            normalized,
            relativeoffset,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribIFormat(
    attribindex: GLuint,
    size: GLint,
    r#type: GLenum,
    relativeoffset: GLuint,
) {
    trace!(
        "glVertexAttribIFormat called, parameters: attribindex: {:?}, size: {:?}, r#type: {:?}, relativeoffset: {:?} ",
        attribindex, size, r#type, relativeoffset
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_vertex_attrib_i_format(
            attribindex,
            size,
            r#type.try_into_enum()?,
            relativeoffset,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribLFormat(
    attribindex: GLuint,
    size: GLint,
    r#type: GLenum,
    relativeoffset: GLuint,
) {
    trace!(
        "glVertexAttribLFormat called, parameters: attribindex: {:?}, size: {:?}, r#type: {:?}, relativeoffset: {:?} ",
        attribindex, size, r#type, relativeoffset
    );
    with_ctx_mut(|mut state| {
        state.oxidegl_vertex_attrib_l_format(attribindex, size, r#type, relativeoffset)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexArrayAttribFormat(
    vaobj: GLuint,
    attribindex: GLuint,
    size: GLint,
    r#type: GLenum,
    normalized: GLboolean,
    relativeoffset: GLuint,
) {
    trace!(
        "glVertexArrayAttribFormat called, parameters: vaobj: {:?}, attribindex: {:?}, size: {:?}, r#type: {:?}, normalized: {:?}, relativeoffset: {:?} ",
        vaobj, attribindex, size, r#type, normalized, relativeoffset
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_vertex_array_attrib_format(
            vaobj,
            attribindex,
            size,
            r#type.try_into_enum()?,
            normalized,
            relativeoffset,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexArrayAttribIFormat(
    vaobj: GLuint,
    attribindex: GLuint,
    size: GLint,
    r#type: GLenum,
    relativeoffset: GLuint,
) {
    trace!(
        "glVertexArrayAttribIFormat called, parameters: vaobj: {:?}, attribindex: {:?}, size: {:?}, r#type: {:?}, relativeoffset: {:?} ",
        vaobj, attribindex, size, r#type, relativeoffset
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(state.oxidegl_vertex_array_attrib_i_format(
            vaobj,
            attribindex,
            size,
            r#type.try_into_enum()?,
            relativeoffset,
        ))
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexArrayAttribLFormat(
    vaobj: GLuint,
    attribindex: GLuint,
    size: GLint,
    r#type: GLenum,
    relativeoffset: GLuint,
) {
    trace!(
        "glVertexArrayAttribLFormat called, parameters: vaobj: {:?}, attribindex: {:?}, size: {:?}, r#type: {:?}, relativeoffset: {:?} ",
        vaobj, attribindex, size, r#type, relativeoffset
    );
    with_ctx_mut(|mut state| {
        state.oxidegl_vertex_array_attrib_l_format(vaobj, attribindex, size, r#type, relativeoffset)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribPointer(
    index: GLuint,
    size: GLint,
    r#type: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    pointer: *const GLvoid,
) {
    trace!(
        "glVertexAttribPointer called, parameters: index: {:?}, size: {:?}, r#type: {:?}, normalized: {:?}, stride: {:?}, pointer: {:?} ",
        index, size, r#type, normalized, stride, pointer
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_vertex_attrib_pointer(
                index,
                size,
                r#type.try_into_enum()?,
                normalized,
                stride,
                pointer,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribIPointer(
    index: GLuint,
    size: GLint,
    r#type: GLenum,
    stride: GLsizei,
    pointer: *const GLvoid,
) {
    trace!(
        "glVertexAttribIPointer called, parameters: index: {:?}, size: {:?}, r#type: {:?}, stride: {:?}, pointer: {:?} ",
        index, size, r#type, stride, pointer
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_vertex_attrib_i_pointer(
                index,
                size,
                r#type.try_into_enum()?,
                stride,
                pointer,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribLPointer(
    index: GLuint,
    size: GLint,
    r#type: GLenum,
    stride: GLsizei,
    pointer: *const GLvoid,
) {
    trace!(
        "glVertexAttribLPointer called, parameters: index: {:?}, size: {:?}, r#type: {:?}, stride: {:?}, pointer: {:?} ",
        index, size, r#type, stride, pointer
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_vertex_attrib_l_pointer(index, size, r#type, stride, pointer)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexBindingDivisor(bindingindex: GLuint, divisor: GLuint) {
    trace!(
        "glVertexBindingDivisor called, parameters: bindingindex: {:?}, divisor: {:?} ",
        bindingindex, divisor
    );
    with_ctx_mut(|mut state| state.oxidegl_vertex_binding_divisor(bindingindex, divisor))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexArrayBindingDivisor(
    vaobj: GLuint,
    bindingindex: GLuint,
    divisor: GLuint,
) {
    trace!(
        "glVertexArrayBindingDivisor called, parameters: vaobj: {:?}, bindingindex: {:?}, divisor: {:?} ",
        vaobj, bindingindex, divisor
    );
    with_ctx_mut(|mut state| {
        state.oxidegl_vertex_array_binding_divisor(vaobj, bindingindex, divisor)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    trace!(
        "glViewport called, parameters: x: {:?}, y: {:?}, width: {:?}, height: {:?} ",
        x, y, width, height
    );
    with_ctx_mut(|mut state| state.oxidegl_viewport(x, y, width, height))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glViewportArrayv(first: GLuint, count: GLsizei, v: *const GLfloat) {
    trace!(
        "glViewportArrayv called, parameters: first: {:?}, count: {:?}, v: {:?} ",
        first, count, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_viewport_arrayv(first, count, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glViewportIndexedf(
    index: GLuint,
    x: GLfloat,
    y: GLfloat,
    w: GLfloat,
    h: GLfloat,
) {
    trace!(
        "glViewportIndexedf called, parameters: index: {:?}, x: {:?}, y: {:?}, w: {:?}, h: {:?} ",
        index, x, y, w, h
    );
    with_ctx_mut(|mut state| state.oxidegl_viewport_indexedf(index, x, y, w, h))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glViewportIndexedfv(index: GLuint, v: *const GLfloat) {
    trace!(
        "glViewportIndexedfv called, parameters: index: {:?}, v: {:?} ",
        index, v
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_viewport_indexedfv(index, v) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
    trace!(
        "glWaitSync called, parameters: sync: {:?}, flags: {:?}, timeout: {:?} ",
        sync, flags, timeout
    );
    with_ctx_mut(|mut state| state.oxidegl_wait_sync(sync, flags, timeout))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribP1uiv(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
) {
    trace!(
        "glVertexAttribP1uiv called, parameters: index: {:?}, r#type: {:?}, normalized: {:?}, value: {:?} ",
        index, r#type, normalized, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_vertex_attrib_p1uiv(index, r#type.try_into_enum()?, normalized, value)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribP2uiv(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
) {
    trace!(
        "glVertexAttribP2uiv called, parameters: index: {:?}, r#type: {:?}, normalized: {:?}, value: {:?} ",
        index, r#type, normalized, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_vertex_attrib_p2uiv(index, r#type.try_into_enum()?, normalized, value)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribP3uiv(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
) {
    trace!(
        "glVertexAttribP3uiv called, parameters: index: {:?}, r#type: {:?}, normalized: {:?}, value: {:?} ",
        index, r#type, normalized, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_vertex_attrib_p3uiv(index, r#type.try_into_enum()?, normalized, value)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glVertexAttribP4uiv(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
) {
    trace!(
        "glVertexAttribP4uiv called, parameters: index: {:?}, r#type: {:?}, normalized: {:?}, value: {:?} ",
        index, r#type, normalized, value
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_vertex_attrib_p4uiv(index, r#type.try_into_enum()?, normalized, value)
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform1d(location: GLint, x: GLdouble) {
    trace!(
        "glUniform1d called, parameters: location: {:?}, x: {:?} ",
        location, x
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform1d(location, x))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform2d(location: GLint, x: GLdouble, y: GLdouble) {
    trace!(
        "glUniform2d called, parameters: location: {:?}, x: {:?}, y: {:?} ",
        location, x, y
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform2d(location, x, y))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform3d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) {
    trace!(
        "glUniform3d called, parameters: location: {:?}, x: {:?}, y: {:?}, z: {:?} ",
        location, x, y, z
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform3d(location, x, y, z))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform4d(
    location: GLint,
    x: GLdouble,
    y: GLdouble,
    z: GLdouble,
    w: GLdouble,
) {
    trace!(
        "glUniform4d called, parameters: location: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?} ",
        location, x, y, z, w
    );
    with_ctx_mut(|mut state| state.oxidegl_uniform4d(location, x, y, z, w))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform1dv(location: GLint, count: GLsizei, value: *const GLdouble) {
    trace!(
        "glUniform1dv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform1dv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform2dv(location: GLint, count: GLsizei, value: *const GLdouble) {
    trace!(
        "glUniform2dv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform2dv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform3dv(location: GLint, count: GLsizei, value: *const GLdouble) {
    trace!(
        "glUniform3dv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform3dv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniform4dv(location: GLint, count: GLsizei, value: *const GLdouble) {
    trace!(
        "glUniform4dv called, parameters: location: {:?}, count: {:?}, value: {:?} ",
        location, count, value
    );
    with_ctx_mut(|mut state| unsafe { state.oxidegl_uniform4dv(location, count, value) })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix2dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glUniformMatrix2dv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix2dv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix3dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glUniformMatrix3dv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix3dv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix4dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glUniformMatrix4dv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix4dv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix2x3dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glUniformMatrix2x3dv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix2x3dv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix2x4dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glUniformMatrix2x4dv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix2x4dv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix3x2dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glUniformMatrix3x2dv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix3x2dv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix3x4dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glUniformMatrix3x4dv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix3x4dv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix4x2dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glUniformMatrix4x2dv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix4x2dv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glUniformMatrix4x3dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glUniformMatrix4x3dv called, parameters: location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_uniform_matrix4x3dv(location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform1d(program: GLuint, location: GLint, v0: GLdouble) {
    trace!(
        "glProgramUniform1d called, parameters: program: {:?}, location: {:?}, v0: {:?} ",
        program, location, v0
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform1d(program, location, v0))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform1dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
) {
    trace!(
        "glProgramUniform1dv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform1dv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform2d(
    program: GLuint,
    location: GLint,
    v0: GLdouble,
    v1: GLdouble,
) {
    trace!(
        "glProgramUniform2d called, parameters: program: {:?}, location: {:?}, v0: {:?}, v1: {:?} ",
        program, location, v0, v1
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform2d(program, location, v0, v1))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform2dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
) {
    trace!(
        "glProgramUniform2dv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform2dv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform3d(
    program: GLuint,
    location: GLint,
    v0: GLdouble,
    v1: GLdouble,
    v2: GLdouble,
) {
    trace!(
        "glProgramUniform3d called, parameters: program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?} ",
        program, location, v0, v1, v2
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform3d(program, location, v0, v1, v2))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform3dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
) {
    trace!(
        "glProgramUniform3dv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform3dv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform4d(
    program: GLuint,
    location: GLint,
    v0: GLdouble,
    v1: GLdouble,
    v2: GLdouble,
    v3: GLdouble,
) {
    trace!(
        "glProgramUniform4d called, parameters: program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?} ",
        program, location, v0, v1, v2, v3
    );
    with_ctx_mut(|mut state| state.oxidegl_program_uniform4d(program, location, v0, v1, v2, v3))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniform4dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
) {
    trace!(
        "glProgramUniform4dv called, parameters: program: {:?}, location: {:?}, count: {:?}, value: {:?} ",
        program, location, count, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform4dv(program, location, count, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix2dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glProgramUniformMatrix2dv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix2dv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix3dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glProgramUniformMatrix3dv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix3dv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix4dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glProgramUniformMatrix4dv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix4dv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix2x3dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glProgramUniformMatrix2x3dv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix2x3dv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix3x2dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glProgramUniformMatrix3x2dv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix3x2dv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix2x4dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glProgramUniformMatrix2x4dv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix2x4dv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix4x2dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glProgramUniformMatrix4x2dv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix4x2dv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix3x4dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glProgramUniformMatrix3x4dv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix3x4dv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glProgramUniformMatrix4x3dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    trace!(
        "glProgramUniformMatrix4x3dv called, parameters: program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?} ",
        program, location, count, transpose, value
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_program_uniform_matrix4x3dv(program, location, count, transpose, value)
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glSpecializeShader(
    shader: GLuint,
    pEntryPoint: *const GLchar,
    numSpecializationConstants: GLuint,
    pConstantIndex: *const GLuint,
    pConstantValue: *const GLuint,
) {
    trace!(
        "glSpecializeShader called, parameters: shader: {:?}, pEntryPoint: {:?}, numSpecializationConstants: {:?}, pConstantIndex: {:?}, pConstantValue: {:?} ",
        shader, pEntryPoint, numSpecializationConstants, pConstantIndex, pConstantValue
    );
    with_ctx_mut(|mut state| unsafe {
        state.oxidegl_specialize_shader(
            shader,
            pEntryPoint,
            numSpecializationConstants,
            pConstantIndex,
            pConstantValue,
        )
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glMultiDrawArraysIndirectCount(
    mode: GLenum,
    indirect: *const GLvoid,
    drawcount: GLintptr,
    maxdrawcount: GLsizei,
    stride: GLsizei,
) {
    trace!(
        "glMultiDrawArraysIndirectCount called, parameters: mode: {:?}, indirect: {:?}, drawcount: {:?}, maxdrawcount: {:?}, stride: {:?} ",
        mode, indirect, drawcount, maxdrawcount, stride
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_multi_draw_arrays_indirect_count(
                mode.try_into_enum()?,
                indirect,
                drawcount,
                maxdrawcount,
                stride,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glMultiDrawElementsIndirectCount(
    mode: GLenum,
    r#type: GLenum,
    indirect: *const GLvoid,
    drawcount: GLintptr,
    maxdrawcount: GLsizei,
    stride: GLsizei,
) {
    trace!(
        "glMultiDrawElementsIndirectCount called, parameters: mode: {:?}, r#type: {:?}, indirect: {:?}, drawcount: {:?}, maxdrawcount: {:?}, stride: {:?} ",
        mode, r#type, indirect, drawcount, maxdrawcount, stride
    );
    with_ctx_mut(|mut state| {
        GlResult::normalize(unsafe {
            state.oxidegl_multi_draw_elements_indirect_count(
                mode.try_into_enum()?,
                r#type.try_into_enum()?,
                indirect,
                drawcount,
                maxdrawcount,
                stride,
            )
        })
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glPolygonOffsetClamp(factor: GLfloat, units: GLfloat, clamp: GLfloat) {
    trace!(
        "glPolygonOffsetClamp called, parameters: factor: {:?}, units: {:?}, clamp: {:?} ",
        factor, units, clamp
    );
    with_ctx_mut(|mut state| state.oxidegl_polygon_offset_clamp(factor, units, clamp))
}
