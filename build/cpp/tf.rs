use std::{
    env,
    path::{Path, PathBuf},
    time::Instant,
};

use glob::glob;

use super::CompilationBuilder;

fn move_tf_to_out_dir_if_required() -> PathBuf {
    let out_dir = crate::paths::out_dir();
    let tf_src_dir = out_dir.join("tensorflow/tensorflow");

    if !tf_src_dir.exists() || cfg!(feature = "build") {
        println!("Moving tensorflow micro source");
        let start = Instant::now();
        let submodules = crate::paths::submodules();

        let copy_dir = fs_extra::dir::CopyOptions {
            content_only: false,
            overwrite: true,
            skip_exist: false,
            buffer_size: 65536,
            copy_inside: false,
            depth: 0,
        };

        // Copy directory
        println!("Copying TF from {:?}", submodules.join("tensorflow"));
        println!("Copying TF to {:?}", out_dir);
        fs_extra::dir::copy(submodules.join("tensorflow"), &out_dir, &copy_dir)
            .expect("Unable to copy tensorflow");

        println!("Moving source took {:?}", start.elapsed());
    }

    tf_src_dir
}

/// Return a Vec of all *.cc files in `path`, excluding those that have a
/// name containing 'test.cc'
fn get_files_glob(path: PathBuf) -> Vec<String> {
    let mut paths: Vec<String> = vec![];

    for entry in glob(&path.to_string_lossy()).unwrap() {
        let p: PathBuf = entry.unwrap();
        paths.push(p.to_string_lossy().to_string());
    }

    paths
        .into_iter()
        .filter(|p| !p.contains("test.cc"))
        .filter(|p| !p.contains("debug_log.cc"))
        .filter(|p| !p.contains("frontend_memmap"))
        .filter(|p| !p.contains("frontend_main"))
        .collect()
}

pub fn build_tensorflow_library() {
    let tflite = move_tf_to_out_dir_if_required();
    let out_dir = crate::paths::out_dir();
    let tf_lib_name =
        Path::new(&out_dir).join("libtensorflow-microlite.a".to_string());

    if crate::util::is_cross_compiling().unwrap() {
        // Find include directory used by the crosscompiler for libm
        let mut gcc = cc::Build::new().get_compiler().to_command();
        let libm_location = PathBuf::from(
            crate::util::get_gcc_command_result(
                gcc.arg("--print-file-name=libm.a"),
            )
            .expect("Error querying gcc for libm location"),
        );
        let libm_path = libm_location.parent().unwrap();

        // Pass this to the linker
        println!(
            "cargo:rustc-link-search=native={}",
            libm_path.to_string_lossy()
        );
        println!("cargo:rustc-link-lib=static=m");
    }

    if !tf_lib_name.exists() || cfg!(feature = "build") {
        println!("Building tensorflow micro");
        let target = env::var("TARGET").unwrap_or_else(|_| "".to_string());
        let tfmicro_mdir = tflite.join("lite/micro/tools/make/");
        let start = Instant::now();

        let mut builder = cc::Build::new();
        let builder_ref = builder
            .cpp(true)
            .tensorflow_build_setup()
            .cpp_link_stdlib(None)
            //
            .include(tflite.parent().unwrap())
            .include(tfmicro_mdir.join("downloads"))
            .include(tfmicro_mdir.join("downloads/gemmlowp"))
            .include(tfmicro_mdir.join("downloads/flatbuffers/include"))
            .include(tfmicro_mdir.join("downloads/ruy"))
            //
            .files(get_files_glob(tflite.join("lite/micro/*.cc")))
            .files(get_files_glob(tflite.join("lite/micro/kernels/*.cc")))
            .files(get_files_glob(
                tflite.join("lite/micro/memory_planner/*.cc"),
            ))
            .files(get_files_glob(
                tflite.join("lite/experimental/microfrontend/lib/*.c"),
            ))
            .file(tflite.join("lite/c/common.c"))
            .file(tflite.join("lite/core/api/error_reporter.cc"))
            .file(tflite.join("lite/core/api/flatbuffer_conversions.cc"))
            .file(tflite.join("lite/core/api/op_resolver.cc"))
            .file(tflite.join("lite/core/api/tensor_utils.cc"))
            .file(tflite.join("lite/kernels/internal/quantization_util.cc"))
            .file(tflite.join("lite/kernels/kernel_util.cc"));

        // CMSIS-NN for ARM Cortex-M targets
        if target.starts_with("thumb")
            && target.contains("m-none-")
            && cfg!(feature = "cmsis-nn")
        {
            println!("Build includes CMSIS-NN.");
            let cmsis = tflite.join("lite/micro/tools/make/downloads/cmsis");

            builder_ref
                .files(get_files_glob(cmsis.join("CMSIS/NN/Source/*.c")))
                .include(cmsis.join("CMSIS/NN/Include"))
                .include(cmsis.join("CMSIS/DSP/Include"))
                .include(cmsis.join("CMSIS/Core/Include"));
        }

        // micro frontend
        builder_ref
            .include(tfmicro_mdir.join("downloads/kissfft"))
            .include(tfmicro_mdir.join("downloads/kissfft/tools"))
            .include(tflite.join("lite/experimental/microfrontend/lib"))
            .files(get_files_glob(
                tflite.join("lite/experimental/microfrontend/lib/*.cc"),
            ))
            .file(tfmicro_mdir.join("downloads/kissfft/kiss_fft.c"))
            .file(tfmicro_mdir.join("downloads/kissfft/tools/kiss_fftr.c"));

        // Compile
        builder_ref.compile("tensorflow-microlite");

        println!(
            "Building tensorflow micro from source took {:?}",
            start.elapsed()
        );
    } else {
        println!("Didn't rebuild tensorflow micro, using {:?}", tf_lib_name);

        println!("cargo:rustc-link-lib=static=tensorflow-microlite");
        println!("cargo:rustc-link-search=native={:?}", out_dir);
    }
}