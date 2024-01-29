use std::time::Instant;

use super::CompilationBuilder;

pub fn build_inline_cpp() {
    println!("Building inline cpp");
    let start = Instant::now();

    let mut builder = cpp_build::Config::new();
    builder.include(crate::paths::tflite_micro_submodule());
    for p in crate::paths::additional_include_dirs() {
        builder.include(p);
    }

    #[cfg(not(feature = "cpp-std"))]
    builder.cpp_link_stdlib(None);

    builder.tensorflow_build_setup().build("src/lib.rs");

    println!("Building inline cpp took {:?}", start.elapsed());
}
