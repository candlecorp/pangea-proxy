fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=pangea_engine.wick");
    println!("cargo:rerun-if-changed=pangea_api.wick");
    wick_component_codegen::configure().generate("pangea_engine.wick")?;
    Ok(())
}
