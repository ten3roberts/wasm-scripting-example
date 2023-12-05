wit_bindgen::generate!( {
    world: "main",
    exports: {
        world: Host
    }
});

struct Host;

impl Guest for Host {
    fn run(args: Vec<String>) -> Result<i32, String> {
        if args == ["guest", "Hello"] {
            print("Hello from the other side");
        } else {
            return Err("Invalid arguments".into());
        }

        let mut items = Vec::new();
        for i in 0..10 {
            items.push(i);
        }

        let value = get_value(5);
        assert_eq!(value, 5 * 5);

        print(&format!("Hello from guest {items:?}"));
        Ok(42)
    }

    fn get_name() -> String {
        "guest-module".into()
    }
}
