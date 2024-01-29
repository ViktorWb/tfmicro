pub fn setup_submodules_if_required() {
    if !crate::paths::tflite_micro_submodule()
        .join("LICENSE")
        .exists()
    {
        eprintln!("Setting up submodules");
        crate::util::run_command_or_fail(
            ".",
            "git",
            &["submodule", "update", "--init"],
        );
    }

    if !crate::paths::tflite_micro_submodule().join("tensorflow/lite/micro/tools/make/downloads/flatbuffers/CONTRIBUTING.md").exists() {
        eprintln!("Building tensorflow micro example to fetch Tensorflow dependencies");
        crate::util::run_command_or_fail(
            &crate::paths::tflite_micro_submodule().to_string_lossy(),
            "make",
            &["-f", "tensorflow/lite/micro/tools/make/Makefile", "test_micro_speech_test"]
        );
    }
}
