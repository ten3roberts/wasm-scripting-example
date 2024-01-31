use std::net::ToSocketAddrs;

wit_bindgen::generate!({
    world: "main",
    exports : {
        world: Host
    }
});

struct Host;

impl Guest for Host {
    fn run(mut s: Vec<String>) -> (Vec<String>, String) {
        let s = s.pop().unwrap();
        // assert_eq!(s, "Hello, World!");
        // assert_eq!(arg1, 7);
        // assert_eq!(arg2, 3);
        (vec![s.to_uppercase()], s.chars().rev().collect())
    }

    fn run_many(
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
        g: i32,
        h: i32,
        i: i32,
        j: i32,
        k: i32,
        l: i32,
        m: i32,
        n: i32,
        o: i32,
        p: i32,
        q: i32,
    ) -> Vec<i32> {
        let s = [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]
            .into_iter()
            .collect::<Vec<_>>();

        assert_eq!(
            s,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
        );

        s
    }

    fn run_list(s: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        s
    }

    fn run_variant(v: Vec<MyVariant>) -> MyVariant {
        match &v[..] {
            [MyVariant::B(v), MyVariant::A] if v == "Request" => {
                MyVariant::B("Hello, World".into())
            }
            _ => unreachable!(),
        }
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
