# Charta VM WASM

WebAssembly bindings for the Charta VM.

## Building

First, install `wasm-pack`:

```bash
cargo install wasm-pack
```

Then build the WASM package:

```bash
cd charta-vm/wasm
wasm-pack build --target web --out-dir ../../charta-playground/public/wasm-vm
```

This will create a WASM package in `charta-playground/public/wasm-vm/` that can be imported in the playground.

## Usage in JavaScript/TypeScript

```typescript
import init, { ChartaVMWasm } from './wasm-vm/charta_vm_wasm.js'

// Initialize WASM module
await init()

// Create VM instance
const vm = new ChartaVMWasm()

// Load program from IR JSON
vm.load_program(irJson)

// Set input signals
vm.set_signal("input_signal", true)
vm.set_signal("system_ok", true)

// Execute cycle
const inputs = { input_signal: true, system_ok: true }
const outputs = vm.execute_cycle(inputs)

// Check coil states
const coilState = vm.get_coil("output_coil")
const allCoils = vm.get_all_coils()
```

## API

### `ChartaVMWasm`

Main VM class for executing Charta programs.

#### Methods

- `new()` - Create a new VM instance
- `load_program(ir_json: string)` - Load program from IR JSON
- `execute_cycle(inputs: object)` - Execute one scan cycle with input signals
- `get_coil(name: string) -> boolean | null` - Get coil state
- `get_signal(name: string) -> boolean | null` - Get signal state
- `get_all_coils() -> object` - Get all coil states
- `get_all_signals() -> object` - Get all signal states
- `set_signal(name: string, value: boolean)` - Set signal value
- `signal_names() -> string[]` - Get signal names
- `coil_names() -> string[]` - Get coil names
