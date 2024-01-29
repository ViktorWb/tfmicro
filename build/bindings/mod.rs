mod callbacks;

use std::{
    env,
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::Context;
use bindgen::*;

fn bindgen_cross_builder() -> anyhow::Result<bindgen::Builder> {
    let builder = bindgen::Builder::default().clang_arg("--verbose");

    if crate::util::is_cross_compiling()? {
        // Setup target triple
        let target = env::var("TARGET")?;
        let builder = builder.clang_arg(format!("--target={}", target));
        println!("Setting bindgen to cross compile to {}", target);

        // Find the sysroot used by the crosscompiler, and pass this to clang
        let mut gcc = cc::Build::new().get_compiler().to_command();
        let path =
            crate::util::get_gcc_command_result(gcc.arg("--print-sysroot"))?;
        let builder = builder.clang_arg(format!("--sysroot={}", path.trim()));

        // Add a path to the system headers for the target
        // compiler. Possibly we end up using a gcc header with clang
        // frontend, which is sketchy.
        let output = cc::Build::new()
            .cpp(true)
            .get_compiler()
            .to_command()
            .arg("-E")
            .arg("-Wp,-v")
            .arg("-xc++")
            .arg(".")
            .output()
            .context("Couldn't find target GCC executable.")?;

        // We have to scrape the gcc console output to find where
        // the c++ headers are. If we only needed the c headers we
        // could use `--print-file-name=include` but that's not
        // possible.
        let gcc_out = String::from_utf8(output.stderr)?;

        // Scrape the search paths
        let search_start = gcc_out.find("search starts here").unwrap();
        let search_paths: Vec<PathBuf> = gcc_out[search_start..]
            .split('\n')
            .map(|p| PathBuf::from(p.trim()))
            .filter(|path| path.exists())
            .collect();

        // Add scraped paths to builder
        let mut builder = builder.detect_include_paths(false);
        for path in search_paths {
            builder =
                builder.clang_arg(format!("-I{}", path.to_string_lossy()));
        }
        Ok(builder)
    } else {
        Ok(builder)
    }
}

pub fn bindgen_tflite_types() {
    let out_dir = crate::paths::out_dir();
    let tflite_types_name = Path::new(&out_dir).join("tflite_types.rs");

    if !tflite_types_name.exists() || cfg!(feature = "build") {
        println!("Running bindgen");
        let start = Instant::now();

        let mut bindings = bindgen_cross_builder()
            .expect("Error setting up bindgen for cross compiling")
            .allowlist_recursively(true)
            .prepend_enum_name(false)
            .impl_debug(true)
            .with_codegen_config(CodegenConfig::TYPES)
            .layout_tests(false)
            .enable_cxx_namespaces()
            .derive_default(true)
            .size_t_is_usize(true)
            .use_core()
            .ctypes_prefix("cty")
            // Types
            .allowlist_type("tflite::Model")
            .opaque_type("tflite::Model")
            .allowlist_type("tflite::MicroInterpreter")
            .opaque_type("tflite::MicroInterpreter")
            .allowlist_type("TfLiteStatus")
            .allowlist_type("TfLiteTensor")
            .allowlist_type("FrontendState")
            .allowlist_type("FrontendConfig")
            .allowlist_type("FrontendOutput")
            // Types - blacklist
            .blocklist_type("std")
            .blocklist_item("std::vector__Temporary_value")
            .blocklist_item("std::vector__Temporary_value__Storage")
            .blocklist_item("std::_Rb_tree_insert_return_type")
            .blocklist_type("tflite::Interpreter_TfLiteDelegatePtr")
            .blocklist_type("tflite::Interpreter_State")
            .default_enum_style(EnumVariation::Rust {
                non_exhaustive: false,
            })
            .derive_partialeq(true)
            .derive_eq(true)
            .header("csrc/tflite_wrapper.hpp")
            .clang_arg(format!(
                "-I{}",
                crate::paths::tflite_micro_submodule().display()
            ))
            .clang_arg("-DTF_LITE_STATIC_MEMORY")
            .clang_arg("-DTF_LITE_MCU_DEBUG_LOG")
            .clang_arg("-DGEMMLOWP_ALLOW_SLOW_SCALAR_FALLBACK")
            .clang_arg("-xc++")
            .clang_arg("-std=c++11");

        for p in crate::paths::additional_include_dirs() {
            bindings = bindings.clang_arg(format!("-I{}", p.display(),));
        }

        let bindings =
            bindings.generate().expect("Unable to generate bindings");

        // Write the bindings to $OUT_DIR/tflite_types.rs
        let out_path = PathBuf::from(out_dir).join("tflite_types.rs");
        bindings
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");

        println!("Running bindgen took {:?}", start.elapsed());
    } else {
        println!("Didn't regenerate bindings");
    }
}
