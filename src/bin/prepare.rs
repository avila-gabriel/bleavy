use boa_engine::{Context, JsResult, Source, module::Module};
use std::fs;

fn main() -> JsResult<()> {
    let path = "assets/scripts/systems.txt";
    let js = fs::read_to_string(path).expect("failed to read systems.txt");
    let js_cleaned = remove_final_export_block(&js);
    fs::write(path, &js_cleaned).expect("failed to write cleaned JS");

    let source = Source::from_bytes(js_cleaned.as_bytes());
    let mut context = Context::default();
    Module::parse(source, None, &mut context)?;

    println!("✅ systems.txt cleaned and parsed successfully.");
    Ok(())
}

fn remove_final_export_block(js: &str) -> String {
    if let Some(start) = js.rfind("export{") {
        return js[..start].trim_end().to_string();
    }
    js.to_string()
}
