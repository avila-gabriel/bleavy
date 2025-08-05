#[allow(dead_code)]
pub mod utils {
    use boa_engine::{Context, JsString, JsValue, Source};
    use std::fs;

    pub struct JsRuntime {
        pub context: Context,
    }

    impl JsRuntime {
        pub fn new() -> Self {
            let code = fs::read_to_string("core.js").expect("Failed to read JS file");
            let source = Source::from_bytes(&code);

            let mut context = Context::default();
            let result = context.eval(source).expect("Evaluation failed");

            Self { context }
        }
        pub fn call_function(&mut self, name: &str, args: Vec<JsValue>) -> JsValue {

            let global = self.context.global_object().clone();
            let value = global
                .get(JsString::from(name), &mut self.context)
                .expect("Function not found");

            let func = value.as_object().expect("Not an object");

            let result = func
                .call(&JsValue::undefined(), &args, &mut self.context)
                .expect("Call failed");

            result
        }
    }

    pub fn to_js_f32(val: f32) -> JsValue {
        JsValue::from(val as f64)
    }
    pub fn to_js_f64(val: f64) -> JsValue {
        JsValue::from(val)
    }
    pub fn to_js_i32(val: i32) -> JsValue {
        JsValue::from(val)
    }
    pub fn to_js_bool(val: bool) -> JsValue {
        JsValue::from(val)
    }

    pub fn from_js_f32(val: &JsValue) -> f32 {
        val.as_number().expect("Expected number") as f32
    }
    pub fn from_js_f64(val: &JsValue) -> f64 {
        val.as_number().expect("Expected number")
    }
    pub fn from_js_i32(val: &JsValue) -> i32 {
        val.as_number().expect("Expected number") as i32
    }
    pub fn from_js_bool(val: &JsValue) -> bool {
        val.as_boolean().expect("Expected boolean")
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
        use boa_engine::JsValue;

        // Prepare arguments vector
        let args_vec: Vec<JsValue> = vec![
            $( $crate::js_fn!(@to_js $arg_value, $arg_type) ),*
        ];

        // Call JS function and get result
        let result = $ctx.call_function($fn_name, args_vec);
        let arr = result.as_object().expect("Expected array from JS");

        // Convert return values by index
        let mut idx = 0;
        (
            $(
                {
                    let val = arr.get(idx, &mut $ctx.context).expect("Missing return value");
                    idx += 1;
                    $crate::js_fn!(@from_js val, $ret_type)
                }
            ),*
        )
    }};

    // --- to_js conversions ---
    (@to_js $val:expr, f32) => { JsValue::from($val as f64) };
    (@to_js $val:expr, f64) => { JsValue::from($val) };
    (@to_js $val:expr, i32) => { JsValue::from($val) };
    (@to_js $val:expr, bool) => { JsValue::from($val) };

    // --- from_js conversions ---
    (@from_js $val:expr, f32) => { $val.as_number().expect("Expected number") as f32 };
    (@from_js $val:expr, f64) => { $val.as_number().expect("Expected number") };
    (@from_js $val:expr, i32) => { $val.as_number().expect("Expected number") as i32 };
    (@from_js $val:expr, bool) => { $val.as_boolean().expect("Expected boolean") };
}

#[cfg(test)]
mod tests {
    use super::*;
    use boa_engine::JsValue;

    #[test]
    fn test_to_js_f32() {
        let js_val = utils::to_js_f32(3.5);
        assert_eq!(js_val.as_number().unwrap(), 3.5);
    }

    #[test]
    fn test_to_js_f64() {
        let js_val = utils::to_js_f64(42.0);
        assert_eq!(js_val.as_number().unwrap(), 42.0);
    }

    #[test]
    fn test_to_js_i32() {
        let js_val = utils::to_js_i32(7);
        assert_eq!(js_val.as_number().unwrap() as i32, 7);
    }

    #[test]
    fn test_to_js_bool() {
        let js_val = utils::to_js_bool(true);
        assert_eq!(js_val.as_boolean().unwrap(), true);
    }

    #[test]
    fn test_from_js_f32() {
        let js_val = JsValue::from(1.23);
        let val = utils::from_js_f32(&js_val);
        assert!((val - 1.23).abs() < f32::EPSILON);
    }

    #[test]
    fn test_from_js_f64() {
        let js_val = JsValue::from(99.99);
        let val = utils::from_js_f64(&js_val);
        assert_eq!(val, 99.99);
    }

    #[test]
    fn test_from_js_i32() {
        let js_val = JsValue::from(10);
        let val = utils::from_js_i32(&js_val);
        assert_eq!(val, 10);
    }

    #[test]
    fn test_from_js_bool() {
        let js_val = JsValue::from(true);
        let val = utils::from_js_bool(&js_val);
        assert_eq!(val, true);
    }
}
