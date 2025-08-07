use boa_engine::{
    Context, JsString, JsValue, Source, object::JsObject,
    js_string, prelude::js_str,
};
use std::fs;

pub struct JsRuntime {
    pub context: Context,
}

impl JsRuntime {
    pub fn new() -> Self {
        let code = fs::read_to_string("assets/scripts/systems.txt")
            .expect("Missing assets/scripts/systems.txt");
        let source = Source::from_bytes(&code);

        let mut context = Context::default();
        let _ = context.eval(source).expect("Evaluation failed");

        Self { context }
    }

    pub fn call_function(&mut self, name: &str, args: Vec<JsValue>) -> JsValue {
        let global = self.context.global_object().clone();
        let func_val = global
            .get(JsString::from(name), &mut self.context)
            .expect("Function not found");
        let func = func_val.as_object().expect("Not an object");
        func.call(&JsValue::undefined(), &args, &mut self.context)
            .expect("Call failed")
    }
}

pub trait IntoBoa {
    fn into_boa(self, ctx: &mut Context) -> JsValue;
}

pub trait FromBoa: Sized {
    fn from_boa(val: JsValue, ctx: &mut Context) -> Self;
}

impl IntoBoa for String {
    fn into_boa(self, _ctx: &mut Context) -> JsValue {
        JsValue::String(js_string!(self))
    }
}

impl FromBoa for String {
    fn from_boa(val: JsValue, _ctx: &mut Context) -> Self {
        val.as_string().unwrap().to_std_string().unwrap()
    }
}

macro_rules! prim {
    ($t:ty, $to_js:expr, $from_js:expr) => {
        impl IntoBoa for $t {
            fn into_boa(self, _ctx: &mut Context) -> JsValue {
                $to_js(self)
            }
        }
        impl FromBoa for $t {
            fn from_boa(val: JsValue, _ctx: &mut Context) -> Self {
                $from_js(val)
            }
        }
    };
}

prim!(
    f32,
    |x| JsValue::from(x as f64),
    |v: JsValue| v.as_number().unwrap() as f32
);
prim!(f64, |x| JsValue::from(x), |v: JsValue| v
    .as_number()
    .unwrap());
prim!(
    i32,
    |x| JsValue::from(x),
    |v: JsValue| v.as_number().unwrap() as i32
);
prim!(bool, |x| JsValue::from(x), |v: JsValue| v
    .as_boolean()
    .unwrap());

/// Create a JS Array (real, with Array.prototype) from a Rust Vec<JsValue>
fn vec_to_js(list: Vec<JsValue>, ctx: &mut Context) -> JsValue {
    // Get global Array constructor/prototype
    let array_ctor = ctx.global_object()
        .get(js_str!("Array"), ctx).unwrap()
        .as_object().unwrap().clone();
    let array_proto = array_ctor
        .get(js_str!("prototype"), ctx).unwrap()
        .as_object().unwrap().clone();

    let obj = JsObject::from_proto_and_data(array_proto, ());

    for (i, value) in list.clone().into_iter().enumerate() {
        obj.set(i, value, false, ctx).unwrap();
    }

    obj.set(
        js_str!("length"),
        JsValue::from(list.len() as f64),
        false,
        ctx,
    )
    .unwrap();

    JsValue::Object(obj)
}

macro_rules! vec_impl {
    ($t:ty, $to_js:expr, $from_js:expr) => {
        impl IntoBoa for Vec<$t> {
            fn into_boa(self, ctx: &mut Context) -> JsValue {
                let arr: Vec<JsValue> = self.into_iter().map($to_js).collect();
                vec_to_js(arr, ctx)
            }
        }
        impl FromBoa for Vec<$t> {
            fn from_boa(val: JsValue, ctx: &mut Context) -> Self {
                let obj = val.as_object().expect("Expected array").clone();
                let len = obj
                    .get(js_str!("length"), ctx)
                    .unwrap()
                    .as_number()
                    .unwrap() as usize;
                (0..len)
                    .map(|i| {
                        let el = obj.get(i, ctx).unwrap();
                        $from_js(el)
                    })
                    .collect()
            }
        }
    };
}

vec_impl!(
    f32,
    |x: f32| JsValue::from(x as f64),
    |v: JsValue| v.as_number().unwrap() as f32
);
vec_impl!(f64, |x: f64| JsValue::from(x), |v: JsValue| v
    .as_number()
    .unwrap());
vec_impl!(
    i32,
    |x: i32| JsValue::from(x),
    |v: JsValue| v.as_number().unwrap() as i32
);
vec_impl!(bool, |x: bool| JsValue::from(x), |v: JsValue| v
    .as_boolean()
    .unwrap());

#[macro_export]
macro_rules! js_fn {
    (
        $ctx:expr,
        $fn_name:literal,
        [$( $arg_name:ident : $arg_ty:ty = $arg_val:expr ),* $(,)?],
        [$( $ret_ty:ty ),* $(,)?]
    ) => {{
        use crate::utils::script::{IntoBoa, FromBoa};
        use boa_engine::JsValue;

        let args: Vec<JsValue> = vec![
            $( <$arg_ty as IntoBoa>::into_boa($arg_val, &mut $ctx.context) ),*
        ];

        let result = $ctx.call_function($fn_name, args);
        let arr = result.as_object().expect("JS must return an array");

        let mut idx = 0usize;
        (
            $(
                {
                    let v = arr.get(idx, &mut $ctx.context).expect("Missing return");
                    idx += 1;
                    let _ = idx;
                    <$ret_ty as FromBoa>::from_boa(v, &mut $ctx.context)
                }
            ),*
        )
    }};
}
