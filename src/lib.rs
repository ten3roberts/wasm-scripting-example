use anyhow::Context;
use wasm_component_layer::{Component, Linker, TypedFunc};
use wasm_runtime_layer::Engine;

const GUEST_BYTES: &[u8] = include_bytes!("../bin/guest.wasm");

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    use tracing_subscriber::{
        fmt::format::{FmtSpan, Pretty},
        prelude::*,
    };

    use tracing_web::{performance_layer, MakeConsoleWriter};

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .without_time()
        .with_span_events(FmtSpan::ACTIVE)
        .with_writer(MakeConsoleWriter)
        .compact();

    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .with(tracing_error::ErrorLayer::default())
        .init();

    console_error_panic_hook::set_once();

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

    let root = linker.root_mut();
    root.define_func(
        "print",
        TypedFunc::new(&mut store, |_, msg: String| {
            tracing::info!(target: "guest", "{msg}");
            Ok(())
        })
        .func(),
    )
    .unwrap();

    root.define_func(
        "get-value",
        TypedFunc::new(&mut store, |_, key: u32| Ok((key as u64) * (key as u64))).func(),
    )
    .unwrap();

    // Create an instance of the component using the linker.
    let instance = linker.instantiate(&mut store, &component)?;

    tracing::info!("Finished instantiating component");

    let interface = instance.exports().root();

    tracing::info!("Calling run");

    let _span = tracing::info_span!("run").entered();
    let func_run = interface
        .func("run")
        .ok_or_else(|| anyhow::anyhow!("no such function"))?
        .typed::<Vec<String>, Result<i32, String>>()?;

    let func_get_name = interface
        .func("get-name")
        .ok_or_else(|| anyhow::anyhow!("no such function"))?
        .typed::<(), String>()?;

    let result = func_run
        .call(&mut store, vec!["guest".into(), "Hello".into()])
        .context("Failed to call `run`")?;

    tracing::info!(?result, "result");

    assert_eq!(result, Ok(42));

    let result = func_get_name.call(&mut store, ())?;
    tracing::info!("received name: {result:?}");

    assert_eq!(result, "guest-module");

    Ok(())
}
