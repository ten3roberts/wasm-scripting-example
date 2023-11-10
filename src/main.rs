use wasm_component_layer::{Component, Linker, TypedFunc};
use wasm_runtime_layer::Engine;

const GUEST_BYTES: &[u8] = include_bytes!("../bin/guest.wasm");

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use tracing_subscriber::{prelude::*, registry, EnvFilter};
    use tracing_tree::HierarchicalLayer;
    registry()
        .with(EnvFilter::from_default_env())
        .with(
            HierarchicalLayer::new(4)
                .with_indent_lines(true)
                .with_span_retrace(true),
        )
        .init();

    run()
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use tracing_subscriber::{fmt::format::Pretty, prelude::*, registry, EnvFilter};

    use tracing_web::{performance_layer, MakeConsoleWriter};

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false) // Only partially supported across browsers
        .without_time()
        .with_writer(MakeConsoleWriter); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init(); // Install these as subscribers to tracing events

    run()
}

fn run() {
    // 1. Instantiate a runtime
    #[cfg(not(target_arch = "wasm32"))]
    let engine = Engine::new(wasmtime::Engine::default());

    #[cfg(target_arch = "wasm32")]
    let engine = Engine::new(wasm_runtime_layer::web::Engine::default());

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
