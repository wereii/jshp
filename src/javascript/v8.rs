// https://github.com/denoland/rusty_v8/blob/main/examples/hello_world.rs

use log::trace;
use rusty_v8;
use rusty_v8::{
    undefined, Context, ContextScope, HandleScope, Isolate, Script, ScriptOrigin, SharedRef,
    TryCatch,
};
use rusty_v8::{
    CreateParams, FunctionCallbackArguments, FunctionTemplate, ObjectTemplate, ReturnValue,
    String as V8String,
};
use std::sync::{Arc, Once};

static V8_INIT: Once = Once::new();
static V8_DESTROY: Once = Once::new();

#[derive(Debug)]
pub struct CodeError {
    message: String,
    stack: String,
}

impl std::fmt::Display for CodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.stack.len() > 0 {
            write!(f, "{}\n{}", self.message, self.stack)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for CodeError {}

pub struct CodeMetadata {
    pub file_name: String,
    pub start_line: i32,
    pub start_offset: i32,
}

#[derive(Copy, Clone)]
pub struct V8Handle {
    //isolate: OwnedIsolate,
}

impl V8Handle {
    pub fn evaluate(self, code: &str, metadata: Option<CodeMetadata>) -> Result<String, CodeError> {
        // don't @ me about this, I'm just trying to get it to work
        // here is to hoping we are not leaking memory left and right

        let mut local_isolate = Isolate::new(CreateParams::default());
        let mut handle_scope = &mut HandleScope::new(&mut local_isolate);

        let echo_buffer = Arc::new(String::new());
        handle_scope.set_slot(echo_buffer);

        let global = ObjectTemplate::new(&mut handle_scope);
        let fn_echo = FunctionTemplate::new(
            &mut handle_scope,
            |scope: &mut HandleScope, args: FunctionCallbackArguments, _retval: ReturnValue| {
                let result = args
                    .get(0)
                    .to_string(scope)
                    .unwrap()
                    .to_rust_string_lossy(scope);

                scope.get_slot::<Arc<String>>().unwrap().push_str(result.as_str());
            },
        );
        global.set(
            V8String::new(&mut handle_scope, "echo").unwrap().into(),
            fn_echo.into(),
        );
        global.set(
            V8String::new(&mut handle_scope, "buffer").unwrap().into(),
            ,
        );

        let context = Context::new(handle_scope);
        let context_scope = &mut ContextScope::new(handle_scope, context);

        let try_catch_scope = &mut TryCatch::new(context_scope);
        let code = V8String::new(try_catch_scope, code).unwrap();

        let origin = match metadata {
            Some(metadata) => Some({
                let resource_name = V8String::new(try_catch_scope, metadata.file_name.as_str())
                    .unwrap()
                    .into();
                let undef = undefined(try_catch_scope);
                ScriptOrigin::new(
                    try_catch_scope,
                    resource_name,
                    metadata.start_line,
                    metadata.start_offset,
                    false,
                    0,
                    undef.into(),
                    false,
                    false,
                    false,
                )
            }),
            None => None,
        };

        let script = match Script::compile(try_catch_scope, code, origin.as_ref()) {
            Some(script) => script,
            None => {
                return Err(CodeError {
                    message: try_catch_scope
                        .exception()
                        .expect("exception should exist")
                        .to_rust_string_lossy(try_catch_scope),
                    stack: try_catch_scope
                        .stack_trace()
                        .expect("stack trace should exist")
                        .to_rust_string_lossy(try_catch_scope),
                });
            }
        };

        match script.run(try_catch_scope) {
            Some(result) => Ok(result.to_rust_string_lossy(try_catch_scope)),
            None => Err(CodeError {
                message: try_catch_scope
                    .exception()
                    .expect("exception should exist")
                    .to_rust_string_lossy(try_catch_scope),
                stack: try_catch_scope
                    .stack_trace()
                    .expect("stack trace should exist")
                    .to_rust_string_lossy(try_catch_scope),
            }),
        }
    }
}

pub struct V8State;

impl V8State {
    /// This has to be called at maximum once per process.
    pub fn init() -> V8Handle {
        if V8_DESTROY.is_completed() {
            panic!("Can't reinitialize V8")
        };

        V8_INIT.call_once(|| {
            rusty_v8::V8::initialize_platform(SharedRef::from(rusty_v8::new_default_platform(
                0, false,
            )));
            rusty_v8::V8::initialize();
        });
        trace!("V8 initialized");

        V8Handle {}
    }

    pub fn dispose() {
        trace!("Trying to dispose V8");
        if !V8_INIT.is_completed() {
            panic!("Can't dispose V8 before initializing")
        };
        V8_DESTROY.call_once(|| {
            unsafe {
                rusty_v8::V8::dispose();
            };
            rusty_v8::V8::shutdown_platform();
        });
        trace!("V8 disposed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let v8 = V8State::init();
        let result = v8.evaluate("1 + 1", None).unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_evaluate_syntax_error() {
        let v8 = V8State::init();
        let err_result = v8.evaluate("1 + a 1", None).unwrap_err();
        assert_eq!(err_result.message, "SyntaxError: Unexpected number");
    }

    #[test]
    fn test_evaluate_thrown_error() {
        let v8 = V8State::init();
        let result = v8.evaluate("2 / 0", None).unwrap();
        assert_eq!(result, "Infinity");
    }

    #[test]
    fn test_evaluate_thrown_error_manual() {
        let v8 = V8State::init();
        let err_result = v8.evaluate("throw new Error('oops')", None).unwrap_err();
        assert_eq!(err_result.message, "Error: oops");
    }

    #[test]
    fn test_evaluate_with_metadata() {
        let v8 = V8State::init();
        let result = v8
            .evaluate(
                "1 + 1",
                Some(CodeMetadata {
                    file_name: "/test.js".to_string(),
                    start_line: 3,
                    start_offset: 5,
                }),
            )
            .unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_evaluate_with_metadata_error() {
        let v8 = V8State::init();
        let result = v8
            .evaluate(
                "1 + 1;\na + 3",
                Some(CodeMetadata {
                    file_name: "/test.js".to_string(),
                    start_line: 3,
                    start_offset: 5,
                }),
            )
            .unwrap_err();
        assert_eq!(
            result.stack,
            "ReferenceError: a is not defined\n    at /test.js:5:1"
        );
    }
}
