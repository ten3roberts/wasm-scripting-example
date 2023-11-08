use tracing_subscriber::{prelude::*, registry, EnvFilter};
use tracing_tree::HierarchicalLayer;
use wasm_runtime_layer::{AsContextMut, Engine, Imports, Instance, Module, Store, Value};

const GUEST_BYTES: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/debug/guest.wasm");

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

    let module = Module::new(&engine, std::io::Cursor::new(GUEST_BYTES)).unwrap();
    let imports = Imports::new();
    // let imports = Imports::new().define("root", "print", store.as_context_mut();

    let instance = Instance::new(&mut store, &module, &imports).unwrap();

    let run = instance
        .get_export(&store, "run")
        .unwrap()
        .into_func()
        .unwrap();

    let mut result = [];
    run.call(&mut store, &[], &mut result).unwrap();

    tracing::info!(?result, "result");
    // assert_eq!(result[0], Value::I32(43));
}
