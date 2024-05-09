#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!("kf6/klocalizedcontext.h");

        #[qobject]
        type KLocalizedContext;

        include!("cxx-qt-lib/qqmlengine.h");
        type QQmlEngine = cxx_qt_lib::QQmlEngine;
    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++" {
        #[rust_name = "init_from_engine"]
        fn initFromEngine(engine: Pin<&mut QQmlEngine>);
    }
}
use core::pin::Pin;
use cxx_qt_lib::QQmlEngine;

pub use ffi::KLocalizedContext;

impl KLocalizedContext {
    pub fn init_from_engine(engine: Pin<&mut QQmlEngine>) {
        ffi::init_from_engine(engine);
    }
}
