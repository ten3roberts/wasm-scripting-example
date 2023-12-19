wit_bindgen::generate!({
    world: "main",
    exports : {
        world: Host
    }
});

struct Host;

impl Guest for Host {
    fn run(mut s: Vec<String>) -> (String, String) {
        let s = s.pop().unwrap();
        // assert_eq!(s, "Hello, World!");
        // assert_eq!(arg1, 7);
        // assert_eq!(arg2, 3);
        (s.to_uppercase(), s.chars().rev().collect())
    }

    // fn run(args: Vec<String>) -> Result<i32, String> {
    //     if args == ["guest", "Hello"] {
    //         print("Hello from the other side");
    //     } else {
    //         return Err("Invalid arguments".into());
    //     }

    //     let mut items = Vec::new();
    //     for i in 0..10 {
    //         items.push(i);
    //     }

    //     let (sq, sqrt) = get_value(16);
    //     assert_eq!(sq, 256);
    //     assert_eq!(sqrt as u32, 4);

    //     let val = get_value_tuple(16);
    //     let val2 = get_value_tuple2(16);

    //     print(&format!("Hello from guest {items:?} {val:?} {val2:?}"));
    //     Ok(42)
    // }

    // fn get_guest_name() -> String {
    //     "guest-module".into()
    // }
}
