use std::{ffi::c_void, ptr::NonNull};

use log::{debug, trace};
use objc2::rc::Retained;
use objc2_app_kit::NSView;

use context::{CTX, with_ctx_mut};
use oxidegl::{context::Context, gl_types::GLenum};
pub mod context;
#[allow(
    unused_variables,
    non_snake_case,
    dead_code,
    non_upper_case_globals,
    unused_mut,
    clippy::module_name_repetitions,
    // needed to pull inferred return type down to ()
    clippy::semicolon_if_nothing_returned, clippy::unit_arg,
    clippy::wildcard_imports,
    clippy::missing_safety_doc,
)]
pub mod gl_core;

// Safety: we pray no one is using this symbol name, and prefix our function name with oxidegl
#[unsafe(no_mangle)]
/// # Safety
/// if ctx is Some(a) then a must point to a valid, initalized [`Context`]
pub unsafe extern "C" fn oxidegl_set_current_context(ctx: Option<NonNull<Context>>) {
    set_context(ctx);
}
pub fn set_context(ctx: Option<NonNull<Context>>) {
    if let Some(mut prev) = CTX.take() {
        // early return if we are setting the same context again
        if ctx == Some(prev) {
            return;
        }
        // Safety: we are the exclusive accessor of this context, as we just removed it from CTX
        unsafe { prev.as_mut() }.made_not_current();
    }
    if let Some(mut c) = ctx {
        // Safety: we are the exclusive accessor of this context, because it has not been installed to CTX yet
        unsafe { c.as_mut() }.made_current();
        CTX.set(ctx);
    }
    if ctx.is_some() {
        trace!("set context to {:?}", ctx);
    }
}
pub fn swap_buffers() {
    with_ctx_mut(|mut r| r.swap_buffers());
    trace!("swap buffers");
}
// Safety: we pray no one is using this symbol name, and prefix our function name with oxidegl
#[unsafe(no_mangle)]
/// # Safety
/// if ctx is Some(a) then a must point to a valid, initalized [`Context`]
pub unsafe extern "C" fn oxidegl_swap_buffers(_ctx: Option<NonNull<Context>>) {
    swap_buffers();
}
#[must_use]
pub fn box_ctx(ctx: Context) -> NonNull<Context> {
    let p = Box::into_raw(Box::new(ctx));
    // Safety: Box guarantees that the pointer is non-null
    unsafe { NonNull::new_unchecked(p) }
}
// Safety: we pray no one is using this symbol name, and prefix our function name with oxidegl
#[unsafe(no_mangle)]
/// # Safety
/// This needs to be run as early as possible (ideally before the program spawns a thread other than the main thread)
pub unsafe extern "C" fn oxidegl_platform_init() {
    unsafe {
        oxidegl::debug_init();
    }
}

// Safety: we pray no one is using this symbol name, and prefix our function name with oxidegl
#[unsafe(no_mangle)]
/// # Safety
/// This function must be called at most once per allocated context to avoid a double free
/// ctx, if Some must point to a valid Context and be exclusive
pub unsafe extern "C" fn oxidegl_destroy_context(ctx: Option<NonNull<Context>>) {
    if let Some(c) = ctx {
        // Safety: caller ensures c points to a valid Context struct, allocated via a Box with the same allocator
        // Caller ensures c is the exclusive pointer to ctx
        drop(unsafe { Box::from_raw(c.as_ptr()) });
    }
}
// Safety: we pray no one is using this symbol name, and prefix our function name with oxidegl
#[unsafe(no_mangle)]
/// # Safety
/// view must point to a valid instance of NSView
pub unsafe extern "C" fn oxidegl_create_context(
    view: *mut NSView,
    // TODO make these params do something idk
    _format: GLenum,
    _typ: GLenum,
    _depth_format: GLenum,
    _depth_type: GLenum,
    _stencil_format: GLenum,
    _stencil_type: GLenum,
) -> *mut c_void {
    let mut ctx = Context::new();
    // Safety: caller ensures ptr is a pointer to a valid, initialized NSView.
    let view = unsafe { Retained::retain(view).unwrap() };

    ctx.set_view(&view);
    debug!("Created context");
    box_ctx(ctx).as_ptr().cast()
}
