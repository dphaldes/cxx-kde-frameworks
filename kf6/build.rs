use cxx_qt_build::CxxQtBuilder;

fn main() {
    let mut builder = CxxQtBuilder::new();
    builder = builder.with_opts(cxx_qt_lib_headers::build_opts());
    builder = cxx_kf6_build::setup_builder(builder);

    let rust_bridges = vec!["ki18n/klocalizedcontext", "ki18n/klocalizedstring"];
    for source in &rust_bridges {
        builder = builder.file(format!("src/{source}.rs"))
    }

    let cpp_files = vec!["ki18n/klocalizedcontext", "ki18n/klocalizedstring"];
    builder = builder.cc_builder(move |cc| {
        for file in &cpp_files {
            cc.file(format!("src/{file}.cpp"));
            println!("cargo:rerun-if-changed=src/{file}.cpp");
        }
    });

    builder.build();
}
