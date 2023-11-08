wit_bindgen::generate!( {
    world: "host",
    exports: {
        world: Host
    }
});

struct Host;

impl Guest for Host {
    fn run() {
        let mut items = Vec::new();
        for i in 0..100 {
            items.push(i);
        }

        println!("Hello from host! {items:?}");
    }
}
