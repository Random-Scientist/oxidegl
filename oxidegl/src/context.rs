use crate::{gl_enums::ErrorCode, render::Renderer};
use objc2::rc::Retained;
use objc2_app_kit::NSView;
use objc2_metal::MTLPixelFormat;
use state::GLState;

pub(crate) mod state;
#[derive(Debug)]
#[repr(C)]
pub struct Context {
    pub(crate) gl_state: GLState,
    pub(crate) platform_state: Renderer,
}

impl Context {
    #[must_use]
    pub fn new() -> Self {
        Self {
            gl_state: GLState::default(),
            platform_state: Renderer::new(MTLPixelFormat::BGRA8Unorm_sRGB, None, None),
        }
    }
    pub fn set_error(&mut self, error: ErrorCode) {
        self.gl_state.error = error;
    }
    pub fn set_view(&mut self, view: &Retained<NSView>) {
        let backing_scale_factor = view.window().map_or(1.0, |w| w.backingScaleFactor());
        self.platform_state.set_view(view, backing_scale_factor);
        // init scissor box/viewport now that we have an actual view
        let dims = self.platform_state.target_defaultfb_dims();
        self.gl_state.viewport.width = dims.0;
        self.gl_state.viewport.height = dims.1;

        self.gl_state.scissor_box.width = dims.0;
        self.gl_state.scissor_box.height = dims.1;
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
