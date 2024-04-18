use std::path::PathBuf;

use cxx_qt_build::CxxQtBuilder;

pub fn setup_builder(builder: CxxQtBuilder) -> CxxQtBuilder {
    let mut include_dir = String::from("/usr/include/");
    let mut lib_dir = String::from("/usr/lib/");

    if let Ok(dir) = std::env::var("KDE_INCLUDEDIR") {
        include_dir = dir;
    } else {
        println!(
            "cargo:warning=KDE_INCLUDEDIR is not defined, used default value: {}",
            include_dir
        );
    }
    if let Ok(dir) = std::env::var("KDE_LIBDIR") {
        lib_dir = dir;
    } else {
        println!(
            "cargo:warning=KDE_LIBDIR is not defined, used default value: {}",
            lib_dir
        );
    }

    println!(
        "cargo:rustc-link-search={}",
        std::fs::canonicalize(lib_dir)
            .expect("Cannot get canonical path of KDE_LIBDIR")
            .display()
    );
    println!("cargo:rustc-link-lib=KF6I18n");

    let ki18n_include_path = PathBuf::from(include_dir)
        .canonicalize()
        .expect("Cannot get canonical path of KDE_INCLUDEDIR")
        .join("KF6")
        .join("KI18n");

    builder.cc_builder(|cc| {
        cc.include(format!("{}", ki18n_include_path.display()));
    })
}
