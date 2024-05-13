#[cxx_qt::bridge(cxx_file_stem = "klocalizedstring")]
mod ffi {
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;

        include!("kf6/klocalizedstring.h");

        #[qobject]
        type KLocalizedString;
    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++" {
        #[rust_name = "set_application_domain"]
        fn setApplicationDomain(domain: &QByteArray);

        #[rust_name = "application_domain"]
        fn applicationDomain() -> QByteArray;
    }
}

use cxx_qt_lib::QByteArray;

pub use ffi::KLocalizedString;

impl KLocalizedString {
    pub fn set_application_domain(domain: &QByteArray) {
        ffi::set_application_domain(domain);
    }

    pub fn application_domain() -> QByteArray {
        ffi::application_domain()
    }
}
