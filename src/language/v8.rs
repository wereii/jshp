// https://github.com/denoland/rusty_v8/blob/main/examples/hello_world.rs

use lazy_static::lazy_static;

use rusty_v8 as v8;
use rusty_v8::SharedRef;

lazy_static! {
    static ref PLATFORM: SharedRef<v8::Platform> = v8::new_default_platform(0, false).make_shared();
}

pub struct V8State {
    initialized: bool,
}

impl V8State {
    pub fn new() -> Self {
        Self { initialized: false }
    }

    pub fn init(&mut self) {
        if self.initialized {
            panic!("V8Factory already initialized");
        }
        v8::V8::initialize_platform(PLATFORM.to_owned());
        v8::V8::initialize();
        self.initialized = true;
    }

    pub fn dispose(&mut self) {
        if !self.initialized {
            panic!("V8Factory not initialized");
        }

        unsafe {
            v8::V8::dispose();
        }
        v8::V8::shutdown_platform();
    }

    fn evaluate(&self, code: &str) -> Result<String, String> {
        // don't @ me about this, I'm just trying to get it to work
        let isolate = &mut v8::Isolate::new(Default::default());
        let handle_scope = &mut v8::HandleScope::new(isolate);

        let context = v8::Context::new(handle_scope);
        let context_scope = &mut v8::ContextScope::new(handle_scope, context);

        let scope = &mut v8::TryCatch::new(context_scope);
        let code = v8::String::new(scope, code).unwrap();

        let script = match v8::Script::compile(scope, code, None) {
            Some(script) => script,
            None => {
                let exception = scope.exception().expect("exception should exist");
                return Err(exception.to_rust_string_lossy(scope));
            }
        };

        match script.run(scope) {
            Some(result) => Ok(result.to_rust_string_lossy(scope)),
            None => Err(scope.exception().expect("exception should exist").to_rust_string_lossy(scope)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let mut v8 = V8State::new();
        v8.init();
        let result = v8.evaluate("1 + 1").unwrap();
        assert_eq!(result, "2");
        v8.cleanup();
    }

    #[test]
    fn test_evaluate_syntax_error() {
        let mut v8 = V8State::new();
        v8.init();
        let result = v8.evaluate("1 + a 1").unwrap_err();
        println!("{}", result);
    }

    #[test]
    fn test_evaluate_thrown_error() {
        let mut v8 = V8State::new();
        v8.init();
        let result = v8.evaluate("2 / 0").unwrap_err();
        println!("{}", result);
        v8.cleanup();
    }

    #[test]
    fn test_evaluate_thrown_error_manual() {
        let mut v8 = V8State::new();
        v8.init();
        let result = v8.evaluate("throw new Error('oops')").unwrap_err();
        println!("{}", result);
        v8.cleanup();
    }
}
