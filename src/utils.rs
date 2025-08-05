use boa_engine::{Context, JsString, JsValue, Source};
use std::fs;

pub struct JsRuntime {
    pub context: Context,
}

impl JsRuntime {
    pub fn new() -> Self {
        let code = fs::read_to_string("assets/scripts/systems.txt").expect("Is systems.txt there? at assets/scripts/systems.txt?");
        let source = Source::from_bytes(&code);

        let mut context = Context::default();
        let _ = context.eval(source).expect("Evaluation failed");

        Self { context }
    }

    pub fn call_function(&mut self, name: &str, args: Vec<JsValue>) -> JsValue {
        let global = self.context.global_object().clone();
        let value = global
            .get(JsString::from(name), &mut self.context)
            .expect("Function not found");
        let func = value.as_object().expect("Not an object");
        func.call(&JsValue::undefined(), &args, &mut self.context)
            .expect("Call failed")
    }
}

#[macro_export]
macro_rules! js_fn {
    (
        $ctx:expr,
        $fn_name:literal,
        [$( $arg_name:ident : $arg_type:tt = $arg_value:expr ),* $(,)?],
        [$( $ret_type:tt ),* $(,)?]
    ) => {{
    let args_vec: Vec<boa_engine::JsValue> = vec![
        $( js_fn!(@to_js $arg_value, $arg_type) ),*
    ];

    let result = $ctx.call_function($fn_name, args_vec);
    let arr = result.as_object().expect("Expected array from JS");

    let mut idx = 0;
    let tuple_out = (
        $(
            {
                let val = arr.get(idx, &mut $ctx.context).expect("Missing return value");
                idx += 1;
                js_fn!(@from_js val, $ret_type)
            }
        ),*
    );
    let _ = idx;
    tuple_out
    }};

    // to js
    (@to_js $val:expr, f32) => { boa_engine::JsValue::from($val as f64) };
    (@to_js $val:expr, f64) => { boa_engine::JsValue::from($val) };
    (@to_js $val:expr, i32) => { boa_engine::JsValue::from($val) };
    (@to_js $val:expr, bool) => { boa_engine::JsValue::from($val) };

    // from js
    (@from_js $val:expr, f32) => { $val.as_number().expect("Expected number") as f32 };
    (@from_js $val:expr, f64) => { $val.as_number().expect("Expected number") };
    (@from_js $val:expr, i32) => { $val.as_number().expect("Expected number") as i32 };
    (@from_js $val:expr, bool) => { $val.as_boolean().expect("Expected boolean") };
}
