use std::{collections::BTreeMap, io::Cursor};

use scripting_tests::setup_core;
use wasm_bindgen_test::wasm_bindgen_test;
use wasm_runtime_layer::{Extern, Func, FuncType, Imports, Value, ValueType};

// The bytes of the component.
const WASM: &[u8] = include_bytes!("../bin/core_guest.wasm");

#[test]
#[wasm_bindgen_test]
pub fn core_module() {
    let (engine, mut store) = setup_core(WASM, &Imports::default());

    let func = Func::new(
        "get-values",
        &mut store,
        FuncType::new([], [ValueType::I32, ValueType::I32]),
        |_, _, res| {
            res[0] = Value::I32(5);
            res[1] = Value::I32(4);
            Ok(())
        },
    );

    let mut imports = Imports::new();
    imports.define("host", "get-values", Extern::Func(func));

    // Parse the component bytes and load its imports and exports.
    let module = wasm_runtime_layer::Module::new(&engine, Cursor::new(WASM)).unwrap();
    let instance = wasm_runtime_layer::Instance::new(&mut store, &module, &imports).unwrap();

    let exports = instance
        .exports(&store)
        .map(|v| (v.name, v.value))
        .collect::<BTreeMap<_, _>>();

    // Get the function for selecting a list element.
    let Extern::Func(func) = exports.get("add-sub").unwrap() else {
        unreachable!()
    };

    let mut result = [Value::I32(0), Value::I32(0)];
    func.call(&mut store, &[Value::I32(5), Value::I32(7)], &mut result)
        .unwrap();

    assert_eq!(result, [Value::I32(12), Value::I32(1)],);
}
