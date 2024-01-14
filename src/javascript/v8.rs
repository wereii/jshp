// https://github.com/denoland/rusty_v8/blob/main/examples/hello_world.rs

use rusty_v8;
use rusty_v8::String as V8String;
use rusty_v8::{
    undefined, Context, ContextScope, HandleScope, Isolate, OwnedIsolate, Script, ScriptOrigin,
    SharedRef, TryCatch,
};
use std::sync::Once;

static V8_INIT: Once = Once::new();
static V8_DESTROY: Once = Once::new();

pub struct V8Handle {
    isolate: OwnedIsolate,
}

#[derive(Debug)]
pub struct CodeError {
    message: String,
    stack: String,
}

impl V8Handle {
    fn evaluate(&mut self, code: &str) -> Result<String, CodeError> {
        let mut local_isolate = Isolate::new(Default::default());
        // don't @ me about this, I'm just trying to get it to work
        let handle_scope = &mut HandleScope::new(&mut local_isolate);
        //handle_scope.set_capture_stack_trace_for_uncaught_exceptions(true, 100);

        let context = Context::new(handle_scope);
        let context_scope = &mut ContextScope::new(handle_scope, context);

        let mut scope = &mut TryCatch::new(context_scope);
        let code = V8String::new(scope, code).unwrap();

        // let orig = ScriptOrigin::new(
        //     &mut scope,
        //     V8String::new(scope, "main.js").unwrap().into(),
        //     0,
        //     false,
        //     0,
        //     0,
        //     undefined(&mut scope).into(),
        //     false,
        //     false,
        // )

        let script = match Script::compile(scope, code, None) {
            Some(script) => script,
            None => {
                return Err(CodeError {
                    message: scope
                        .exception()
                        .expect("exception should exist")
                        .to_rust_string_lossy(scope),
                    stack: scope
                        .stack_trace()
                        .expect("stack trace should exist")
                        .to_rust_string_lossy(scope),
                });
            }
        };

        match script.run(scope) {
            Some(result) => Ok(result.to_rust_string_lossy(scope)),
            None => Err(CodeError {
                message: scope
                    .exception()
                    .expect("exception should exist")
                    .to_rust_string_lossy(scope),
                stack: scope
                    .stack_trace()
                    .expect("stack trace should exist")
                    .to_rust_string_lossy(scope),
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

        V8Handle {
            isolate: Isolate::new(Default::default()),
        }
    }

    pub fn dispose() {
        if !V8_INIT.is_completed() {
            panic!("Can't dispose V8 before initializing")
        };
        V8_DESTROY.call_once(|| {
            unsafe {
                rusty_v8::V8::dispose();
            };
            rusty_v8::V8::shutdown_platform();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let mut v8 = V8State::init();
        let result = v8.evaluate("1 + 1").unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_evaluate_syntax_error() {
        let mut v8 = V8State::init();
        let err_result = v8.evaluate("1 + a 1").unwrap_err();
        assert_eq!(err_result.message, "SyntaxError: Unexpected number");
        println!("{}", err_result.stack);
    }

    #[test]
    fn test_evaluate_thrown_error() {
        let mut v8 = V8State::init();
        let result = v8.evaluate("2 / 0").unwrap();
        assert_eq!(result, "Infinity");
    }

    #[test]
    fn test_evaluate_thrown_error_manual() {
        let mut v8 = V8State::init();
        let err_result = v8.evaluate("throw new Error('oops')").unwrap_err();
        assert_eq!(err_result.message, "Error: oops");
        println!("{}", err_result.stack);
    }
}
