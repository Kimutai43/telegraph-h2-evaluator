# Telegraph H2 Evaluator: Port Scan Audit

Official evaluation module and benchmark dataset for the **PORT_SCAN_AUDIT** intent, built for the Telegraph Protocol Season II Hackathon (Track 2: Evaluators).

## Project Structure
- `src/lib.rs`: Core WASM evaluation logic and scoring framework.
- `data/PORT_SCAN_AUDIT/eval_material.json`: JSONL-formatted benchmark dataset and reference ground truths.

## Build Instructions
To compile the evaluator adapter to WebAssembly (`wasm32-wasip1`):
```bash
cargo build --target wasm32-wasip1 --release