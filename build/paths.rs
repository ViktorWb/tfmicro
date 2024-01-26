use std::{env, path::PathBuf};

pub fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

pub fn manifest_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

pub fn submodules() -> PathBuf {
    manifest_dir().join("submodules")
}

pub fn flatbuffers_include_dir() -> PathBuf {
    submodules().join("tensorflow/tensorflow/lite/micro/tools/make/downloads/flatbuffers/include")
}
