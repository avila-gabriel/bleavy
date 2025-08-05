use boa_engine::{Context, Source, module::Module};
use std::fs;

fn main() -> boa_engine::JsResult<()> {
    let js = fs::read_to_string("core.js").expect("failed to read core.js");
    let source = Source::from_bytes(js.as_bytes());
    let mut context = Context::default();
    Module::parse(source, None, &mut context)?;
    println!("✅ core.js parsed successfully.");

    Ok(())
}
