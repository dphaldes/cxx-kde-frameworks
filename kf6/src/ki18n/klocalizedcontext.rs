#[cxx_qt::bridge]
mod ffi {
    extern "C++Qt" {
        include!("klocalizedcontext.h");

        #[qobject]
        type KLocalizedContext;
    }
}

pub use ffi::KLocalizedContext;
