use cxx_qt_build::CxxQtBuilder;

fn main() {
    let mut builder = CxxQtBuilder::new();

    let rust_bridges = vec!["ki18n/klocalizedcontext"];
    
    for source in &rust_bridges {
        builder = builder.file(format!("src/{source}.rs"))
    }
    
    builder = cxx_kf6_build::setup_builder(builder);
    builder.build();
}