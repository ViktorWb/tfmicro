use std::{env, path::PathBuf};

pub fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

pub fn manifest_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

fn submodules() -> PathBuf {
    manifest_dir().join("submodules")
}

pub fn tflite_micro_submodule() -> PathBuf {
    submodules().join("tflite-micro")
}

pub fn additional_include_dirs() -> [PathBuf; 2] {
    [
        tflite_micro_submodule().join(
            "tensorflow/lite/micro/tools/make/downloads/flatbuffers/include",
        ),
        tflite_micro_submodule()
            .join("tensorflow/lite/micro/tools/make/downloads/gemmlowp"),
    ]
}
