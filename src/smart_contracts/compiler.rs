pub fn compile_contract(source_code: &str) -> Result<String, String> {
    if source_code.is_empty() {
        Err("Source code is empty".to_string())
    } else {
        Ok("Contract compiled successfully".to_string())
    }
}
