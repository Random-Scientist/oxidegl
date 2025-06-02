use std::{cell::Cell, pin::Pin, ptr::NonNull};

use likely_stable::if_likely;
use log::trace;
use oxidegl::{
    context::Context,
    error::{GetErrorReturnValue, GlResult},
    gl_enums::ErrorCode,
};

thread_local! {
    pub static CTX: Cell<Option<NonNull<Context>>> = const { Cell::new(None) };
}
#[inline]
#[cfg_attr(debug_assertions, track_caller)]
#[expect(unused_mut, unused_variables, reason = "lint bug")]
pub fn with_ctx_mut<
    Ret,
    Err: GetErrorReturnValue<Ret> + Into<ErrorCode>,
    Res: GlResult<Ret, Err>,
    Func: for<'a> Fn(Pin<&'a mut Context>) -> Res,
>(
    f: Func,
) -> Ret {
    // optimizer hint for the Some(ptr) case
    if_likely! {
        // take the current context pointer
        // this effectively takes a single-threaded "lock" on the context which protects against
        // the user doing Weird Stuff and running multiple GL commands simultaneously
        // (i.e. by calling a GL command from the debug callback)

        let Some(ptr) = CTX.take() => {
            // need to reassign due to macro jank
            let mut ptr = ptr;
            // Safety: we are the exclusive accessor of ptr due to its thread locality and the fact that we called `take` on it previously
            // wrap the context reference in a pin to ensure it is not moved out of
            let mut p = Pin::new(unsafe { ptr.as_mut() });
            let ret = match f(p).into_result() {
                Ok(ret) => ret,
                Err(e) => {
                    trace!("GL command execution failed");
                    // Safety: f consumes p, the only other exclusive reference to this context, prior to the evaluation of this match arm,
                    // meaning we are free to create another one to write the error out with
                    unsafe { ptr.as_mut() }.set_error(e.into());
                    // Return the default value for the type
                    <Err as oxidegl::error::GetErrorReturnValue<Ret>>::get()
                }
            };
            CTX.set(Some(ptr));
            ret
        } else {
            panic!("no context set!");
        }
    }
}
