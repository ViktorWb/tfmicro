//! person_detection example
//!
use tfmicro::{MicroInterpreter, Model, MutableOpResolver};

use itertools::Itertools;
use log::info;

#[test]
fn person_detection() {
    env_logger::init();

    info!("---- Starting tensorflow micro example: person_detection");

    // Include trained model and test datasets
    let model =
        include_bytes!("../submodules/tflite-micro/tensorflow/lite/micro/models/person_detect.tflite");
    let no_person = include_bytes!(
        "../submodules/tflite-micro/tensorflow/lite/micro/examples/person_detection/testdata/no_person.bmp"
    );
    let person =
        include_bytes!("../submodules/tflite-micro/tensorflow/lite/micro/examples/person_detection/testdata/person.bmp");

    // Map the model into a usable data structure. This doesn't involve
    // any copying or parsing, it's a very lightweight operation.
    let model = Model::from_buffer(&model[..]).unwrap();

    // Create memory area for input, output and intermediate arrays
    const TENSOR_ARENA_SIZE: usize = 93 * 1024;
    let mut tensor_arena: [u8; TENSOR_ARENA_SIZE] = [0; TENSOR_ARENA_SIZE];

    let micro_op_resolver = MutableOpResolver::empty()
        .add_depthwise_conv_2d()
        .add_conv_2d()
        .add_average_pool_2d()
        .add_reshape()
        .add_softmax();

    // Build an interpreter to run the model with
    let mut interpreter =
        MicroInterpreter::new(&model, micro_op_resolver, &mut tensor_arena[..])
            .unwrap();

    // Check properties of the input sensor
    interpreter.input(0, person).unwrap();
    assert_eq!([1, 96, 96, 1], interpreter.input_info(0).dims);

    info!("Created setup");

    // -------- 'person' example ------------
    interpreter.invoke().unwrap();

    // get output for 'person'
    let output_tensor = interpreter.output(0);
    assert_eq!(
        [1, 2],
        output_tensor.info().dims,
        "Dimensions of output tensor"
    );

    assert_eq!(
        1,
        output_tensor.as_data::<u8>().iter().position_max().unwrap()
    );
    info!("---- Person output correct!");

    // ------- 'no person' example ----------
    interpreter.input(0, no_person).unwrap();

    interpreter.invoke().unwrap();

    // get output for 'no person'
    let output_tensor = interpreter.output(0);
    assert_eq!(
        [1, 1, 1, 3],
        output_tensor.info().dims,
        "Dimensions of output tensor"
    );

    assert_eq!(
        2,
        output_tensor.as_data::<u8>().iter().position_max().unwrap()
    );
    info!("---- No-person output correct!");

    info!("---- Done");
}
