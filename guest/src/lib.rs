wit_bindgen::generate!( {
    world: "main",
    exports: {
        world: Host
    }
});

struct Host;

impl Guest for Host {
    fn run() {
        let mut items = Vec::new();
        for i in 0..10 {
            items.push(i);
        }

        // panic!("");
        print(5);
        // print(&format!("Hello from guest {items:?}"));
    }
}
