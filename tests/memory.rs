//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]
#![cfg(feature = "test_accessors")]

use wasm_bindgen_test::*;
use web_sys::console;

use blueshift_gfx::Memory;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub async fn memory_mopen() {
    let r = Memory::mopen("http://localhost:8000/tests/hello.txt").await;
    assert!(r.is_ok());
    let m = r.unwrap();
    assert_eq!(m.as_str().trim(), "Hello, World!");
}

#[wasm_bindgen_test]
pub async fn memory_mopen_bad_host_url() {
    // Use a bad port number in the URL. Should generate FetchError.
    match Memory::mopen("http://localhost:1000/tests/hello.txt").await {
        Err(e) => {
            assert_eq!(e.jsvalue_as_string(), "TypeError: Failed to fetch")
        },
        Ok(_)  => panic!("mopen() should have generated an error."),
    }
}

#[wasm_bindgen_test]
pub async fn memory_mopen_file_not_exist() {
    // Use none existent file name. Should generate FetchStatusError.
    match Memory::mopen("http://localhost:8000/tests/not_there.txt").await {
        Err(e) => assert_eq!(e.status(), 404),
        Ok(_)  => panic!("mopen() should have generated an error."),
    }
}

#[wasm_bindgen_test]
pub async fn memory_mopen_file_url() {
    // Use file URL - this should give a CORS error saying only http/https are
    // supported; but the Rust wasm API only gives back TypeError with "Failed
    // to fetch". However, we can see that CORS request error in the developer
    // console (ctrl-shift-I).
    match Memory::mopen("file:///tests/hello.txt").await {
        Err(e) => {
            console::log_2(&"memory_mopen_file_url() test generated: ".into(), 
                           &e.to_string().into());
            assert_eq!(e.jsvalue_as_string(), "TypeError: Failed to fetch");
        },
        Ok(_) => panic!("mopen() should have generated an error."),
    }
}













