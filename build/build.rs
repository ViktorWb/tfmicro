mod bindings;
mod cpp;
mod paths;
mod submodules;
mod util;

fn main() {
    submodules::setup_submodules_if_required();
    bindings::bindgen_tflite_types();
    cpp::build_inline_cpp();
    cpp::build_tensorflow_library();
}
