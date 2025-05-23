#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic, clippy::undocumented_unsafe_blocks)]
#![allow(
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::float_cmp,
    clippy::too_many_lines,
    clippy::missing_errors_doc,
    clippy::match_bool
)]

use std::sync::Once;

use context::Context;
use log::info;

pub mod context;

#[allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap
)]
pub mod conversions;

mod device_properties;

#[allow(non_upper_case_globals, unused)]
pub mod gl_enums;
#[allow(non_snake_case, non_upper_case_globals, clippy::upper_case_acronyms)]
pub mod gl_types;

pub mod util;

/// Prints the oxidegl version to stdout and enables some env vars in configurations with debug assertions
/// # Safety
/// Should be called as early in the program execution as possible, as it mutates env vars in a potentially racy way
pub unsafe fn debug_init() {
    // wrap in a Once to make sure we don't init twice
    static INIT_ONCE: Once = Once::new();
    INIT_ONCE.call_once(|| {
        info!("OxideGL {}", Context::VERSION_INFO);
        #[cfg(debug_assertions)]
        // Safety: We pray that we aren't racing with anyone else's code writing env vars.
        // This isn't *too* bad because we're running on the main thread, which is where
        // a majority of the writes occur in practice.
        unsafe {
            use std::env::set_var;

            set_var("MTL_DEBUG_LAYER", "1");
            set_var("MTL_SHADER_VALIDATION", "1");
            set_var("MTL_DEBUG_LAYER_VALIDATE_UNRETAINED_RESOURCES", "0x4");
            set_var("RUST_BACKTRACE", "1");
        }
    });
}
