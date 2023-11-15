use wasm_bindgen::prelude::*;
use wasm_component_layer::{Component, Linker, TypedFunc};
use wasm_runtime_layer::Engine;

const GUEST_BYTES: &[u8] = include_bytes!("../bin/guest.wasm");

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main() {
    use tracing_subscriber::{
        fmt::format::{FmtSpan, Pretty},
        prelude::*,
    };

    use tracing_web::{performance_layer, MakeConsoleWriter};

    console_error_panic_hook::set_once();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .without_time()
        .with_span_events(FmtSpan::ACTIVE)
        .with_writer(MakeConsoleWriter)
        .compact();

    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();

    if let Err(err) = run() {
        tracing::error!("{err:?}");
    }
}

pub fn run() -> anyhow::Result<()> {
    // 1. Instantiate a runtime
    #[cfg(not(target_arch = "wasm32"))]
    let engine = Engine::new(wasmi::Engine::default());

    #[cfg(target_arch = "wasm32")]
    let engine = Engine::new(wasm_runtime_layer::web::Engine::default());

    tracing::info!("create store");
    let mut store = wasm_component_layer::Store::new(&engine, ());

    // let module = Module::new(&engine, std::io::Cursor::new(GUEST_BYTES)).unwrap();
    // let mut imports = Imports::new();
    // imports.define(
    //     "$root",
    //     "print",
    //     Extern::Func(TypedFunc::new(&mut store, || {}).func()),
    // );

    tracing::info!("create component");
    let component = Component::new(&engine, GUEST_BYTES)?;
    // Create a linker that will be used to resolve the component's imports, if any.
    let mut linker = Linker::default();

    tracing::info!("Defining imports");
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
    let instance = linker.instantiate(&mut store, &component)?;

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
    Ok(())
}
