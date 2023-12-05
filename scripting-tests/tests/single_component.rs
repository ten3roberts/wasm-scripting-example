use scripting_tests::setup;
use wasm_bindgen_test::wasm_bindgen_test;

// The bytes of the component.
const WASM: &[u8] = include_bytes!("../single_component/component.wasm");

#[test]
#[wasm_bindgen_test]
pub fn single_component() {
    let (mut store, instance) = setup(WASM);

    // Get the interface that the interface exports.
    let interface = instance
        .exports()
        .instance(&"test:guest/foo".try_into().unwrap())
        .unwrap();
    // Get the function for selecting a list element.
    let select_nth = interface
        .func("select-nth")
        .unwrap()
        .typed::<(Vec<String>, u32), String>()
        .unwrap();

    // Create an example list to test upon.
    let example = ["a", "b", "c"]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    assert_eq!(
        select_nth.call(&mut store, (example.clone(), 1)).unwrap(),
        "b"
    );

    println!(
        "Calling select-nth({example:?}, 1) == {}",
        select_nth.call(&mut store, (example.clone(), 1)).unwrap()
    );
    // Prints 'Calling select-nth(["a", "b", "c"], 1) == b'
}
