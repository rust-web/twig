// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Twig library for rust

mod error_codes;
use self::error_codes::{EngineResult, EngineErrorCode, RuntimeResult, RuntimeErrorCode, MockCursor};
use error::api::ErrorCode;

fn foo_engine_error() -> EngineResult<()> {
    err!(EngineErrorCode::TemplateNotFound {
        name: "hello_world.html.twig".to_string(),
        search_paths: vec![]
    })
}

fn foo_runtime_error() -> RuntimeResult<()> {
    err!(RuntimeErrorCode::InvalidArgumentCount{
        defined: 10,
        given: 4,
        cursor: MockCursor // <-- information about current lexer position and template name
    })
}

fn foo_chain_error() -> EngineResult<()> {
    let _val: () = try_chain!(foo_runtime_error());

    unreachable!()
}

#[test]
fn test() {
    // these testcases are of course highly unstable due to the error locations(!)
    // -> just for demonstration right now:

    let my_error = EngineErrorCode::RuntimeError
        .at(location!())
        .caused_by(foo_runtime_error().unwrap_err());

    assert_eq!(my_error.to_string(),
        "Some runtime error at src/test_error/mod.rs:39:12\n \
        - caused by: Target requires 10 arguments, called with 4 for `index.twig.html` line 11 column 2 at src/test_error/mod.rs:20:4\n");

    assert_eq!(foo_engine_error().unwrap_err().to_string(),
        "Template \"hello_world.html.twig\" not found, looked in [] at src/test_error/mod.rs:13:4\n");

    assert_eq!(foo_runtime_error().unwrap_err().to_string(),
        "Target requires 10 arguments, called with 4 for `index.twig.html` line 11 column 2 at src/test_error/mod.rs:20:4\n");

    assert_eq!(foo_chain_error().unwrap_err().to_string(),
        "Some runtime error at src/test_error/mod.rs:28:19\n \
         - caused by: Target requires 10 arguments, called with 4 for `index.twig.html` line 11 column 2 at src/test_error/mod.rs:20:4\n");
}
