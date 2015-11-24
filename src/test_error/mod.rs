// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Twig library for rust

mod error_codes;
use self::error_codes::{EngineResult, EngineErrorCode, RuntimeResult, RuntimeErrorCode};


fn foo_engine_error() -> EngineResult<()> {
    err!(EngineErrorCode::TemplateNotFound {
        name: "hello_world.html.twig".to_string(),
        search_paths: vec![]
    }).into()
}

fn foo_runtime_error() -> RuntimeResult<()> {
    err!(RuntimeErrorCode::InvalidArgumentCount{
        defined: 10,
        given: 4
    }).into()
}

fn foo_chain_error() -> EngineResult<()> {
    let val: () = try_chain!(foo_runtime_error());

    unreachable!()
}

#[test]
fn test() {
    println!("engine error {}", foo_engine_error().unwrap_err());
    println!("runtime error {}", foo_runtime_error().unwrap_err());

    println!("chained error {}", foo_chain_error().unwrap_err());

    assert!(false);

    // ---- test_error::test stdout ----
    // 	engine error
    //  - Template "hello_world.html.twig" was not found at src/test_error.rs:80:4
    //
    // runtime error
    //  - Target requires 10 arguments, called with 4 at src/test_error.rs:87:4
    //
    // chained error
    //  - Some Runtime Error at src/test_error.rs:94:18 caused by
    //  - Target requires 10 arguments, called with 4 at src/test_error.rs:87:4
    //
    // thread 'test_error::test' panicked at 'assertion failed: false', src/test_error.rs:108
}
