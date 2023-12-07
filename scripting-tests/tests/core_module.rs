use std::collections::BTreeMap;

use scripting_tests::setup_core;
use wasm_bindgen_test::wasm_bindgen_test;
use wasm_runtime_layer::{Extern, Imports, Value};

// The bytes of the component.
const WASM: &[u8] = include_bytes!("../bin/core_guest.wasm");

#[test]
#[wasm_bindgen_test]
pub fn core_module() {
    let (mut store, instance) = setup_core(WASM, &Imports::default());

    let exports = instance
        .exports(&store)
        .map(|v| (v.name, v.value))
        .collect::<BTreeMap<_, _>>();

    // Get the function for selecting a list element.
    let Extern::Func(func) = exports.get("add").unwrap() else {
        unreachable!()
    };

    let mut result = [Value::I32(0)];
    func.call(&mut store, &[Value::I32(5), Value::I32(4)], &mut result)
        .unwrap();

    assert_eq!(result, [Value::I32(9)],);
}
