#[allow(unused_unsafe, clippy::all)]
pub fn get_host_name() -> wit_bindgen::rt::string::String {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, string::String, vec::Vec};
    unsafe {
        #[repr(align(4))]
        struct RetArea([u8; 8]);

        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "get-host-name"]
            fn wit_import(_: i32);

        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = *((ptr0 + 0) as *const i32);
        let l2 = *((ptr0 + 4) as *const i32);
        let len3 = l2 as usize;
        let bytes3 = Vec::from_raw_parts(l1 as *mut _, len3, len3);
        wit_bindgen::rt::string_lift(bytes3)
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn get_value(key: u32) -> (u64, f32) {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, string::String, vec::Vec};
    unsafe {
        #[repr(align(8))]
        struct RetArea([u8; 16]);

        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "get-value"]
            fn wit_import(_: i32, _: i32);

        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: i32) {
            unreachable!()
        }
        wit_import(wit_bindgen::rt::as_i32(key), ptr0);
        let l1 = *((ptr0 + 0) as *const i64);
        let l2 = *((ptr0 + 8) as *const f32);
        (l1 as u64, l2)
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn get_value_tuple(key: u32) -> (u64, f32) {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, string::String, vec::Vec};
    unsafe {
        #[repr(align(8))]
        struct RetArea([u8; 16]);

        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "get-value-tuple"]
            fn wit_import(_: i32, _: i32);

        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: i32) {
            unreachable!()
        }
        wit_import(wit_bindgen::rt::as_i32(key), ptr0);
        let l1 = *((ptr0 + 0) as *const i64);
        let l2 = *((ptr0 + 8) as *const f32);
        (l1 as u64, l2)
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn get_value_tuple2(
    key: u32,
) -> (
    (u64, f32),
    (
        wit_bindgen::rt::string::String,
        wit_bindgen::rt::string::String,
    ),
) {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, string::String, vec::Vec};
    unsafe {
        #[repr(align(8))]
        struct RetArea([u8; 32]);

        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "get-value-tuple2"]
            fn wit_import(_: i32, _: i32);

        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: i32) {
            unreachable!()
        }
        wit_import(wit_bindgen::rt::as_i32(key), ptr0);
        let l1 = *((ptr0 + 0) as *const i64);
        let l2 = *((ptr0 + 8) as *const f32);
        let l3 = *((ptr0 + 16) as *const i32);
        let l4 = *((ptr0 + 20) as *const i32);
        let len5 = l4 as usize;
        let bytes5 = Vec::from_raw_parts(l3 as *mut _, len5, len5);
        let l6 = *((ptr0 + 24) as *const i32);
        let l7 = *((ptr0 + 28) as *const i32);
        let len8 = l7 as usize;
        let bytes8 = Vec::from_raw_parts(l6 as *mut _, len8, len8);
        (
            (l1 as u64, l2),
            (
                wit_bindgen::rt::string_lift(bytes5),
                wit_bindgen::rt::string_lift(bytes8),
            ),
        )
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn print(msg: &str) {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, string::String, vec::Vec};
    unsafe {
        let vec0 = msg;
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "print"]
            fn wit_import(_: i32, _: i32);

        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: i32) {
            unreachable!()
        }
        wit_import(ptr0, len0);
    }
}
const _: () = {
    #[doc(hidden)]
    #[export_name = "run"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_run(arg0: i32, arg1: i32) -> i32 {
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, string::String, vec::Vec};
        #[cfg(target_arch = "wasm32")]
        wit_bindgen::rt::run_ctors_once();
        let base3 = arg0;
        let len3 = arg1;
        let mut result3 = Vec::with_capacity(len3 as usize);
        for i in 0..len3 {
            let base = base3 + i * 8;
            let e3 = {
                let l0 = *((base + 0) as *const i32);
                let l1 = *((base + 4) as *const i32);
                let len2 = l1 as usize;
                let bytes2 = Vec::from_raw_parts(l0 as *mut _, len2, len2);
                wit_bindgen::rt::string_lift(bytes2)
            };
            result3.push(e3);
        }
        wit_bindgen::rt::dealloc(base3, (len3 as usize) * 8, 4);
        let result4 = <_GuestImpl as Guest>::run(result3);
        let ptr5 = _RET_AREA.0.as_mut_ptr() as i32;
        match result4 {
            Ok(e) => {
                *((ptr5 + 0) as *mut u8) = (0i32) as u8;
                *((ptr5 + 4) as *mut i32) = wit_bindgen::rt::as_i32(e);
            }
            Err(e) => {
                *((ptr5 + 0) as *mut u8) = (1i32) as u8;
                let vec6 = (e.into_bytes()).into_boxed_slice();
                let ptr6 = vec6.as_ptr() as i32;
                let len6 = vec6.len() as i32;
                ::core::mem::forget(vec6);
                *((ptr5 + 8) as *mut i32) = len6;
                *((ptr5 + 4) as *mut i32) = ptr6;
            }
        };
        ptr5
    }
    const _: () = {
        #[doc(hidden)]
        #[export_name = "cabi_post_run"]
        #[allow(non_snake_case)]
        unsafe extern "C" fn __post_return_run(arg0: i32) {
            let l0 = i32::from(*((arg0 + 0) as *const u8));
            match l0 {
                0 => (),
                _ => {
                    let l1 = *((arg0 + 4) as *const i32);
                    let l2 = *((arg0 + 8) as *const i32);
                    wit_bindgen::rt::dealloc(l1, (l2) as usize, 1);
                }
            }
        }
    };
};
const _: () = {
    #[doc(hidden)]
    #[export_name = "get-guest-name"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_get_guest_name() -> i32 {
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, string::String, vec::Vec};
        #[cfg(target_arch = "wasm32")]
        wit_bindgen::rt::run_ctors_once();
        let result0 = <_GuestImpl as Guest>::get_guest_name();
        let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
        let vec2 = (result0.into_bytes()).into_boxed_slice();
        let ptr2 = vec2.as_ptr() as i32;
        let len2 = vec2.len() as i32;
        ::core::mem::forget(vec2);
        *((ptr1 + 4) as *mut i32) = len2;
        *((ptr1 + 0) as *mut i32) = ptr2;
        ptr1
    }
    const _: () = {
        #[doc(hidden)]
        #[export_name = "cabi_post_get-guest-name"]
        #[allow(non_snake_case)]
        unsafe extern "C" fn __post_return_get_guest_name(arg0: i32) {
            let l0 = *((arg0 + 0) as *const i32);
            let l1 = *((arg0 + 4) as *const i32);
            wit_bindgen::rt::dealloc(l0, (l1) as usize, 1);
        }
    };
};
use Host as _GuestImpl;
pub trait Guest {
    fn run(
        args: wit_bindgen::rt::vec::Vec<wit_bindgen::rt::string::String>,
    ) -> Result<i32, wit_bindgen::rt::string::String>;

    fn get_guest_name() -> wit_bindgen::rt::string::String;
}
#[allow(unused_imports)]
use wit_bindgen::rt::{alloc, string::String, vec::Vec};
#[repr(align(4))]
struct _RetArea([u8; 12]);

static mut _RET_AREA: _RetArea = _RetArea([0; 12]);
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:main"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 357] = [
    3, 0, 4, 109, 97, 105, 110, 0, 97, 115, 109, 13, 0, 1, 0, 7, 225, 1, 1, 65, 2, 1, 65, 17, 1,
    64, 0, 0, 115, 3, 0, 13, 103, 101, 116, 45, 104, 111, 115, 116, 45, 110, 97, 109, 101, 1, 0, 1,
    64, 1, 3, 107, 101, 121, 121, 1, 2, 1, 97, 119, 1, 98, 118, 3, 0, 9, 103, 101, 116, 45, 118,
    97, 108, 117, 101, 1, 1, 1, 111, 2, 119, 118, 1, 64, 1, 3, 107, 101, 121, 121, 0, 2, 3, 0, 15,
    103, 101, 116, 45, 118, 97, 108, 117, 101, 45, 116, 117, 112, 108, 101, 1, 3, 1, 111, 2, 115,
    115, 1, 64, 1, 3, 107, 101, 121, 121, 1, 2, 1, 97, 2, 1, 98, 4, 3, 0, 16, 103, 101, 116, 45,
    118, 97, 108, 117, 101, 45, 116, 117, 112, 108, 101, 50, 1, 5, 1, 64, 1, 3, 109, 115, 103, 115,
    1, 0, 3, 0, 5, 112, 114, 105, 110, 116, 1, 6, 1, 112, 115, 1, 106, 1, 122, 1, 115, 1, 64, 1, 4,
    97, 114, 103, 115, 7, 0, 8, 4, 0, 3, 114, 117, 110, 1, 9, 4, 0, 14, 103, 101, 116, 45, 103,
    117, 101, 115, 116, 45, 110, 97, 109, 101, 1, 0, 4, 1, 17, 101, 120, 97, 109, 112, 108, 101,
    58, 104, 111, 115, 116, 47, 109, 97, 105, 110, 4, 0, 11, 22, 1, 1, 16, 101, 120, 97, 109, 112,
    108, 101, 58, 104, 111, 115, 116, 47, 119, 105, 116, 3, 0, 0, 0, 16, 12, 112, 97, 99, 107, 97,
    103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114,
    115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45,
    99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 55, 46, 48, 16, 119, 105, 116, 45,
    98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 51, 46, 50,
];
#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}

const _: &str = include_str!(r#"/home/freja/dev/wasm-scripting-example/guest/wit/main.wit"#);

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

        let (sq, sqrt) = get_value(16);
        assert_eq!(sq, 256);
        assert_eq!(sqrt as u32, 4);

        let val = get_value_tuple(16);
        let val2 = get_value_tuple2(16);

        print(&format!("Hello from guest {items:?} {val:?} {val2:?}"));
        Ok(42)
    }

    fn get_guest_name() -> String {
        "guest-module".into()
    }
}
