use tracing_subscriber::{prelude::*, registry, EnvFilter};
use tracing_tree::HierarchicalLayer;
use wasm_runtime_layer::{Engine, Imports, Instance, Module, Store, Value};

fn main() {
    registry()
        .with(EnvFilter::from_default_env())
        .with(
            HierarchicalLayer::new(4)
                .with_indent_lines(true)
                .with_span_retrace(true),
        )
        .init();

    // 1. Instantiate a runtime
    let engine = Engine::new(wasmtime::Engine::default());
    let mut store = Store::new(&engine, ());

    // 2. Create modules and instances, similar to other runtimes
    let module_bin = wabt::wat2wasm(
        r#"
(module
(type $t0 (func (param i32) (result i32)))
(func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
    get_local $p0
    i32.const 1
    i32.add))
"#,
    )
    .unwrap();

    tracing::info!("create module");

    let module = Module::new(&engine, std::io::Cursor::new(&module_bin)).unwrap();
    let instance = Instance::new(&mut store, &module, &Imports::default()).unwrap();

    let add_one = instance
        .get_export(&store, "add_one")
        .unwrap()
        .into_func()
        .unwrap();

    let mut result = [Value::I32(0)];
    add_one
        .call(&mut store, &[Value::I32(42)], &mut result)
        .unwrap();

    tracing::info!(?result, "add_one");
    assert_eq!(result[0], Value::I32(43));
}
