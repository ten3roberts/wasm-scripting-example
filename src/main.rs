use tracing_subscriber::{prelude::*, registry, EnvFilter};
use tracing_tree::HierarchicalLayer;
use wasm_runtime_layer::{
    Engine, Extern, Func, FuncType, Imports, Instance, Module, Store, ValueType,
};

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
    let mut imports = Imports::new();
    imports.define(
        "$root",
        "print",
        Extern::Func(Func::new(
            &mut store,
            FuncType::new([ValueType::I32, ValueType::I32], []),
            |_store, args, _ret| {
                tracing::info!(?args, "print");
                // Ok(vec![Value::I32(43)])
                Ok(())
            },
        )),
    );

    tracing::info!("instantiating");
    let instance = Instance::new(&mut store, &module, &imports).unwrap();

    let run = instance
        .get_export(&store, "run")
        .unwrap()
        .into_func()
        .unwrap();

    tracing::info!("call");
    let mut result = [];
    run.call(&mut store, &[], &mut result).unwrap();

    tracing::info!(?result, "result");
    // assert_eq!(result[0], Value::I32(43));
}
