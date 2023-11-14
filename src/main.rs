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

    scripting_example::run();
}

#[cfg(target_arch = "wasm32")]
fn main() {}
