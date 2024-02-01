use wasm_component_layer::{
    Component, Linker, List, ListType, Record, RecordType, Tuple, TupleType, TypedFunc, Value,
    ValueType, Variant, VariantCase, VariantType,
};
use wasm_runtime_layer::Engine;

const GUEST_BYTES: &[u8] = include_bytes!("../bin/guest.wasm");

// wasm_component_layer::bindgen!({ path: "./guest/wit/main.wit" });

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
        .pretty();

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
    #[cfg(not(target_arch = "wasm32"))]
    let engine = Engine::new(wasmi::Engine::default());

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
        "get-host-name",
        TypedFunc::new(&mut store, |_, _: ()| -> anyhow::Result<String> {
            Ok("Host".into())
        })
        .func(),
    )
    .unwrap();

    root.define_func(
        "get-value",
        TypedFunc::new(&mut store, |_, key: u32| -> anyhow::Result<(u64, f32)> {
            Ok(((key as u64) * (key as u64), (key as f32).sqrt()))
        })
        .func(),
    )
    .unwrap();

    root.define_func(
        "get-value-tuple",
        TypedFunc::new(&mut store, |_, key: u32| -> anyhow::Result<((u64, f32),)> {
            Ok((((key as u64) * (key as u64), (key as f32).sqrt()),))
        })
        .func(),
    )
    .unwrap();

    root.define_func(
        "get-value-tuple2",
        TypedFunc::new(
            &mut store,
            |_, key: u32| -> anyhow::Result<((u64, f32), (String, String))> {
                Ok((
                    ((key as u64) * (key as u64), (key as f32).sqrt()),
                    ("Hello".into(), "World".into()),
                ))
            },
        )
        .func(),
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
        .ok_or_else(|| anyhow::anyhow!("no such function"))?;

    let func_run_many = interface
        .func("run-many")
        .ok_or_else(|| anyhow::anyhow!("no such function"))?;

    let func_run_list = interface
        .func("run-list")
        .ok_or_else(|| anyhow::anyhow!("no such function"))?;

    let func_run_variant = interface
        .func("run-variant")
        .ok_or_else(|| anyhow::anyhow!("no such function"))?;

    let func_run_record = interface
        .func("run-record")
        .ok_or_else(|| anyhow::anyhow!("no such function"))?;

    let v: Value = func_run
        .call_typed(&mut store, vec!["Hello, World!".to_string()])
        .unwrap();

    tracing::info!("received value: {v:?}");

    assert_eq!(
        v,
        (Value::Tuple(
            Tuple::new(
                TupleType::new(
                    None,
                    [
                        ValueType::List(ListType::new(ValueType::String)),
                        ValueType::String
                    ]
                ),
                vec![
                    Value::List(
                        List::new(
                            ListType::new(ValueType::String),
                            vec![Value::String("HELLO, WORLD!".into()),]
                        )
                        .unwrap()
                    ),
                    Value::String("!dlroW ,olleH".into()),
                ]
            )
            .unwrap()
        ))
    );

    let v: (Vec<String>, String) = func_run
        .call_typed(
            &mut store,
            Value::List(
                List::new(
                    ListType::new(ValueType::String),
                    [Value::String("It works!".into())],
                )
                .unwrap(),
            ),
        )
        .unwrap();

    tracing::info!("received value: {v:?}");
    assert_eq!(v, (vec!["IT WORKS!".to_string()], "!skrow tI".to_string()));

    let v: Vec<i32> = func_run_many
        .call_typed(
            &mut store,
            (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17),
        )
        .unwrap();

    assert_eq!(
        v,
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,]
    );

    let v: Vec<Vec<i32>> = func_run_list
        .call_typed(
            &mut store,
            &[List::new(
                ListType::new(ValueType::S32),
                [Value::S32(4), Value::S32(2)],
            )
            .unwrap()][..],
        )
        .unwrap();

    assert_eq!(v, [[4, 2]]);
    tracing::debug!(?v);

    tracing::info_span!("run-variant").in_scope(|| {
        let variant_ty = VariantType::new(
            None,
            [
                VariantCase::new("A", None),
                VariantCase::new("B", Some(ValueType::String)),
                VariantCase::new("C", Some(ValueType::String)),
            ],
        )
        .unwrap();

        let v: Value = func_run_variant
            .call_typed(
                &mut store,
                Value::List(
                    List::new(
                        ListType::new(ValueType::List(ListType::new(ValueType::Variant(
                            variant_ty.clone(),
                        )))),
                        [Value::List(
                            List::new(
                                ListType::new(ValueType::Variant(variant_ty.clone())),
                                [
                                    Value::Variant(
                                        Variant::new(
                                            variant_ty.clone(),
                                            1,
                                            Some(Value::String("Request".into())),
                                        )
                                        .unwrap(),
                                    ),
                                    Value::Variant(Variant::new(variant_ty, 0, None).unwrap()),
                                ],
                            )
                            .unwrap(),
                        )],
                    )
                    .unwrap(),
                ),
            )
            .unwrap();

        tracing::debug!(?v, "result");
    });

    tracing::info_span!("run-record").in_scope(|| {
        let ty = RecordType::new(None, [("a", ValueType::S32), ("b", ValueType::String)]).unwrap();

        let v: Value = func_run_record
            .call_typed(
                &mut store,
                Value::List(
                    List::new(
                        ListType::new(ValueType::List(ListType::new(ValueType::Record(
                            ty.clone(),
                        )))),
                        [Value::List(
                            List::new(
                                ListType::new(ValueType::Record(ty.clone())),
                                [
                                    Value::Record(
                                        Record::new(
                                            ty.clone(),
                                            vec![
                                                ("a", Value::S32(42)),
                                                ("b", Value::String("Hello, ".into())),
                                            ],
                                        )
                                        .unwrap(),
                                    ),
                                    Value::Record(
                                        Record::new(
                                            ty.clone(),
                                            vec![
                                                ("a", Value::S32(24)),
                                                ("b", Value::String("World".into())),
                                            ],
                                        )
                                        .unwrap(),
                                    ),
                                ],
                            )
                            .unwrap(),
                        )],
                    )
                    .unwrap(),
                ),
            )
            .unwrap();

        tracing::info!(?v, "result");

        assert_eq!(
            v,
            Value::Record(
                Record::new(
                    ty,
                    vec![
                        ("a", Value::S32(66)),
                        ("b", Value::String("Hello, World".into())),
                    ]
                )
                .unwrap()
            )
        );
    });
    // let func_run = interface
    //     .func("run")
    //     .ok_or_else(|| anyhow::anyhow!("no such function"))?
    //     .typed::<Vec<String>, Result<i32, String>>()?;

    // let func_get_name = interface
    //     .func("get-guest-name")
    //     .ok_or_else(|| anyhow::anyhow!("no such function"))?
    //     .typed::<(), String>()?;

    // let result = func_run
    //     .call(&mut store, vec!["guest".into(), "Hello".into()])
    //     .context("Failed to call `run`")?;

    // tracing::info!(?result, "result");

    // assert_eq!(result, Ok(42));

    // let result = func_get_name.call(&mut store, ())?;
    // tracing::info!("received name: {result:?}");

    // assert_eq!(result, "guest-module");

    Ok(())
}
