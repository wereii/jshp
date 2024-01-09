use lazy_static::lazy_static;
// https://github.com/denoland/rusty_v8/blob/main/examples/hello_world.rs
use rusty_v8 as v8;
use rusty_v8::SharedRef;

lazy_static! {
    static ref PLATFORM: SharedRef<v8::Platform> =
        v8::new_default_platform(0, false).make_shared();
}

#[non_exhaustive] // avoid direct construction
pub struct V8 {}

impl V8 {
    pub fn init() {
        v8::V8::initialize_platform(PLATFORM.to_owned());
        v8::V8::initialize();
    }

    pub fn dispose() {
        unsafe {
            v8::V8::dispose();
        }
        v8::V8::shutdown_platform();
    }

    fn evaluate(code: &str) -> Option<String> {
        // don't @ me about this, I'm just trying to get it to work
        let isolate = &mut v8::Isolate::new(Default::default());
        let handle_scope = &mut v8::HandleScope::new(isolate);

        let context = v8::Context::new(handle_scope);
        let context_scope = &mut v8::ContextScope::new(handle_scope, context);

        let scope = &mut v8::TryCatch::new(context_scope);
        let code = v8::String::new(scope, code).unwrap();
        // let origin = v8::ScriptOrigin::new(
        //     &mut scope,
        //     "".into(),
        //     0,
        //     0,
        //     false,
        //     0,
        //     v8::undefined(&mut scope).into(),
        //     false,
        //     false,
        //     false,
        // );

        let script = v8::Script::compile(scope, code, None)?;
        let result = script.run(scope)?;
        let result = result.to_string(scope).unwrap();
        Some(result.to_rust_string_lossy(scope))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        V8::init();
        let a = V8::evaluate("'Hello' + ' World!'");
        println!("{:?}", a); // holy shit it /just/ works

        V8::dispose();
    }
}
