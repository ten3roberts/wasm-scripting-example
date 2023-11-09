use tracing_subscriber::{prelude::*, registry, EnvFilter};
use tracing_tree::HierarchicalLayer;
use wasm_component_layer::{Component, Linker, TypedFunc};
use wasm_runtime_layer::Engine;

const GUEST_BYTES: &[u8] = include_bytes!("../bin/guest.wasm");

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
    let mut store = wasm_component_layer::Store::new(&engine, ());

    // let module = Module::new(&engine, std::io::Cursor::new(GUEST_BYTES)).unwrap();
    // let mut imports = Imports::new();
    // imports.define(
    //     "$root",
    //     "print",
    //     Extern::Func(TypedFunc::new(&mut store, || {}).func()),
    // );

    let component = Component::new(&engine, GUEST_BYTES).unwrap();
    // Create a linker that will be used to resolve the component's imports, if any.
    let mut linker = Linker::default();

    linker
        .root_mut()
        .define_func(
            "print",
            TypedFunc::new(&mut store, |_, s: String| {
                tracing::info!("guest: {s}");
                Ok(())
            })
            .func(),
        )
        .unwrap();

    // Create an instance of the component using the linker.
    let instance = linker.instantiate(&mut store, &component).unwrap();

    tracing::info!("instantiating");

    let interface = instance.exports().root();

    let mut result = [];
    interface
        .func("run")
        .unwrap()
        .call(&mut store, &[], &mut result)
        .unwrap();

    // tracing::info!("interface: {interface:#?}");
    // run.call(&mut store, &[], &mut result).unwrap();

    tracing::info!(?result, "result");
    // assert_eq!(result[0], Value::I32(43));
}
