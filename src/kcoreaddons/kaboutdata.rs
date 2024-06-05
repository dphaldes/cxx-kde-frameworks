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
        license: License,
    ) -> UniquePtr<KAboutData> {
        ffi::from(
            component_name,
            display_name,
            version,
            short_description,
            license as i32,
        )
    }

    pub fn set_application_data(about_data: UniquePtr<KAboutData>) {
        if let Some(data) = about_data.as_ref() {
            ffi::set_application_data(data);
        } else {
            eprintln!("KAboutData couldn't be unwrapped");
        }
    }
}

#[allow(non_camel_case_types)]
pub enum License {
    Custom = -2,
    File = -1,
    Unknown = 0,
    GPL = 1,
    LGPL = 2,
    BSDL = 3,
    Artistic = 4,
    GPL_V3 = 5,
    LGPL_V3 = 6,
    LGPL_V2_1 = 7,
    MIT = 8,
}

impl License {
    pub const GPL_V2: Self = Self::GPL;
    pub const LGPL_V2: Self = Self::LGPL;
}
