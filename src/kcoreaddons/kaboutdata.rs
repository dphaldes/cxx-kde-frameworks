#[cxx_qt::bridge(cxx_file_stem = "kaboutdata")]
mod ffi {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("kf6/kaboutdata.h");
        type KAboutData;

        #[rust_name = "add_author"]
        fn addAuthor(
            self: Pin<&mut KAboutData>,
            name: &QString,
            task: &QString,
            email_address: &QString,
            web_address: &QString,
            avatar_url: &QUrl,
        ) -> Pin<&mut KAboutData>;

        #[rust_name = "add_credit"]
        fn addCredit(
            self: Pin<&mut KAboutData>,
            name: &QString,
            task: &QString,
            email_address: &QString,
            web_address: &QString,
            avatar_url: &QUrl,
        ) -> Pin<&mut KAboutData>;

        #[rust_name = "set_translator"]
        fn setTranslator(
            self: Pin<&mut KAboutData>,
            name: &QString,
            email_address: &QString,
        ) -> Pin<&mut KAboutData>;

    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++" {
        fn from(
            component_name: QString,
            display_name: QString,
            version: QString,
            short_description: QString,
            license: i32,
        ) -> UniquePtr<KAboutData>;

        #[rust_name = "set_application_data"]
        fn setApplicationData(about_data: &KAboutData);
    }
}

use cxx::UniquePtr;
use cxx_qt_lib::QString;
pub use ffi::KAboutData;

impl KAboutData {
    pub fn from(
        component_name: QString,
        display_name: QString,
        version: QString,
        short_description: QString,
        license: i32,
    ) -> UniquePtr<KAboutData> {
        ffi::from(
            component_name,
            display_name,
            version,
            short_description,
            license,
        )
    }

    pub fn set_application_data(about_data: &KAboutData) {
        ffi::set_application_data(about_data);
    }
}
