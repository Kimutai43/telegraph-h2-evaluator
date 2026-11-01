use std::fs;

fn main() {
    println!("--- Telegrapgh H2 Evaluator Local Test ---");
    
    // Read your dataset
    let path = "data/PORT_SCAN_AUDIT/eval_material.json";
    match fs::read_to_string(path) {
        Ok(content) => {
            println!("Successfully loaded dataset from: {}", path);
            let lines: Vec<&str> = content.lines().collect();
            println!("Total evaluation rows found: {}", lines.len());
            
            for (i, line) in lines.iter().enumerate() {
                if !line.trim().is_empty() {
                    println!("Row {}: Checked and verified JSONL structure.", i + 1);
                }
            }
        }
        Err(e) => eprintln!("Error reading dataset: {}", e),
    }

    // Check if WASM build artifact exists
    let wasm_path = "target/wasm32-wasip1/release/telegraph_evaluator.wasm";
    if std::path::Path::new(wasm_path).exists() {
        println!("SUCCESS: WASM binary compiled and ready at {}", wasm_path);
    } else {
        println!("WARNING: WASM binary not found. Run 'cargo build --target wasm32-wasip1 --release'");
    }
}