wit_bindgen::generate!( {
    world: "main",
    exports: {
        world: Host
    }
});

struct Host;

impl Guest for Host {
    fn run(arg: String) -> i32 {
        if arg == "Hello" {
            print("Hello from the other side");
        }

        let mut items = Vec::new();
        for i in 0..10 {
            items.push(i);
        }

        let value = get_value(5);

        print(&format!("Hello from guest {items:?}"));
        42
    }
}
