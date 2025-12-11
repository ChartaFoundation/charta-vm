# Charta VM

Runtime execution engine for Charta programs.

## Components

- **Store (σ)**: Signal/coil state management
- **Guard Evaluation**: NO/NC contacts, AND/OR/NOT operators
- **Rung Evaluation**: Guard-to-coil update logic
- **Scan Cycle**: Complete cycle execution (read inputs, evaluate rungs, update coils)
- **VM**: Main VM struct with program loading and execution

## Implementation Status

### Completed (Phase 1)

- Store implementation with parameterised signal/coil support
- Guard evaluation (NO/NC contacts, AND/OR/NOT)
- Rung evaluation with OR-combine for multiple rungs
- Scan cycle execution
- VM with `load_program` and `step` methods
- Latching vs non-latching coil support

## Usage

```rust
use charta_vm::VM;
use charta_vm::ir::load_ir;
use std::collections::HashMap;

// Load IR
let ir_json = std::fs::read_to_string("program.ir.json")?;
let ir = load_ir(&ir_json)?;

// Create VM and load program
let mut vm = VM::new();
vm.load_program(ir)?;

// Execute cycle
let mut inputs = HashMap::new();
inputs.insert("input_signal".to_string(), true);
let outputs = vm.step(inputs)?;

// Check coil state
let coil_state = vm.get_coil_state("output_coil");
```

## Testing

```bash
cd charta-vm
cargo test
```
