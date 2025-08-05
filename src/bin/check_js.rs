use boa_engine::{Context, Source, module::Module};
use std::fs;

fn main() -> boa_engine::JsResult<()> {
    // Read your .mjs file
    let js = fs::read_to_string("core.js").expect("failed to read core.js");
    let source = Source::from_bytes(js.as_bytes());

    // Create a minimal context (no module loader needed)
    let mut context = Context::default();

    // Try to parse the module (this is the step you're testing)
    Module::parse(source, None, &mut context)?;

    println!("✅ core.js parsed successfully.");

    Ok(())
}
