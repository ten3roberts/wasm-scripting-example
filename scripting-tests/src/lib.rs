use wasm_component_layer::{Component, Instance, Linker, Store};
use wasm_runtime_layer::Engine;

pub use wasm_component_layer;
pub use wasm_runtime_layer;

#[cfg(target_arch = "wasm32")]
pub type EngineImpl = wasm_runtime_layer::web::Engine;
#[cfg(not(target_arch = "wasm32"))]
pub type EngineImpl = wasmi::Engine;

#[cfg(target_arch = "wasm32")]
fn setup_tracing() {
    use tracing_subscriber::{
        fmt::format::{FmtSpan, Pretty},
        prelude::*,
    };

    use tracing_web::{performance_layer, MakeConsoleWriter};
    use wasm_bindgen::prelude::wasm_bindgen;

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

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn error(msg: String);

        type Error;

        #[wasm_bindgen(constructor)]
        fn new() -> Error;

        #[wasm_bindgen(structural, method, getter)]
        fn stack(error: &Error) -> String;
    }

    std::panic::set_hook(Box::new(|info| {
        let mut msg = info.to_string();

        if let Some(location) = info.location() {
            msg.push_str("\n\nLocation: ");
            msg.push_str(&location.to_string());
        }

        let stack = Error::new().stack();

        msg.push_str("\n\nStack:\n\n");
        msg.push_str(&stack);

        tracing::error!("{msg}")
    }));

    let (_panic_hook, error_hook) = color_eyre::config::HookBuilder::default().into_hooks();
    // panic_hook.install();
    error_hook.install().unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn setup_tracing() {
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

    color_eyre::install().unwrap();
}

pub fn setup(bytes: &[u8]) -> (Store<(), EngineImpl>, Instance) {
    setup_tracing();

    // Create a new engine for instantiating a component.
    let engine = Engine::new(EngineImpl::default());

    // Create a store for managing WASM data and any custom user-defined state.
    let mut store = Store::new(&engine, ());

    // Parse the component bytes and load its imports and exports.
    let component = Component::new(&engine, bytes).unwrap();
    // Create a linker that will be used to resolve the component's imports, if any.
    let linker = Linker::default();
    // Create an instance of the component using the linker.
    let instance = linker.instantiate(&mut store, &component).unwrap();

    (store, instance)
}
