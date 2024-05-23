mod kf6_build;
use cxx_qt_build::CxxQtBuilder;

fn main() {
    let mut builder = CxxQtBuilder::new();
    builder = kf6_build::link_libraries(builder);

    let files = vec![
        "kcoreaddons/kaboutdata",
        "ki18n/klocalizedcontext",
        "ki18n/klocalizedstring",
    ];

    for file in &files {
        builder = builder.file(format!("src/{file}.rs"))
    }

    builder = builder.cc_builder(move |cc| {
        for file in &files {
            cc.file(format!("src/{file}.cpp"));
            println!("cargo:rerun-if-changed=src/{file}.cpp");
        }
    });

    builder
        .with_opts(cxx_qt_lib_headers::build_opts())
        .with_opts(kf6_build::header_opts())
        .build();
}
