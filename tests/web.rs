//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use web_sys::console;

use blueshift_gfx::Memory;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);


#[wasm_bindgen_test]
async fn memory_mopen() {
    let r = Memory::mopen("http://localhost:8000/tests/hello.txt").await;
    
    assert!(r.is_ok());
    
    let m = r.unwrap();
    assert_eq!(m.as_str().trim(), "Hello, World!");
}

#[wasm_bindgen_test]
async fn memory_mopen_bad_host_url() {
    // Use a bad port number in the URL.
    let r = Memory::mopen("http://localhost:1000/tests/hello.txt").await;
    
    if let Err(e) = &r {
        console::log_2(&"Error: ".into(), &e.to_string().into());
        assert_eq!(e.to_string(), "Fetch from \
                                   (http://localhost:1000/tests/hello.txt) \
                                   failed with error \
                                   (TypeError: Failed to fetch).");
    } else {
        panic!("Test case should have generated an error.");
    }
}


