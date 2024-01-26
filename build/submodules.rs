use std::path::Path;

pub fn setup_submodules_if_required() {
    if !Path::new("submodules/tensorflow/LICENSE").exists() {
        eprintln!("Setting up submodules");
        crate::util::run_command_or_fail(
            ".",
            "git",
            &["submodule", "update", "--init"],
        );
    }

    if !Path::new("submodules/tensorflow/tensorflow/lite/micro/tools/make/downloads/flatbuffers/CONTRIBUTING.md").exists() {
        eprintln!("Building tensorflow micro example to fetch Tensorflow dependencies");
        crate::util::run_command_or_fail("submodules/tensorflow", "make", &["-f", "tensorflow/lite/micro/tools/make/Makefile", "test_micro_speech_test"]);
    }
}
