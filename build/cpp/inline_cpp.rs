use std::time::Instant;

use super::CompilationBuilder;

pub fn build_inline_cpp() {
    let submodules = crate::paths::submodules();

    println!("Building inline cpp");
    let start = Instant::now();

    cpp_build::Config::new()
        .include(submodules.join("tensorflow"))
        .include(crate::paths::flatbuffers_include_dir())
        .tensorflow_build_setup()
        .cpp_link_stdlib(None)
        //.flag("-std=c++14")
        .build("src/lib.rs");

    println!("Building inline cpp took {:?}", start.elapsed());
}
