//! WASM bindings for Charta VM

use wasm_bindgen::prelude::*;
use charta_vm::{VM, ir::load_ir};
use std::collections::HashMap;
use serde_wasm_bindgen::to_value;
use js_sys::Object;

/// Charta VM instance for browser execution
#[wasm_bindgen]
pub struct ChartaVMWasm {
    vm: VM,
}

#[wasm_bindgen]
impl ChartaVMWasm {
    /// Create a new VM instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            vm: VM::new(),
        }
    }

    /// Load a program from IR JSON string
    #[wasm_bindgen]
    pub fn load_program(&mut self, ir_json: &str) -> Result<(), JsValue> {
        let ir = load_ir(ir_json)
            .map_err(|e| JsValue::from_str(&format!("IR load error: {}", e)))?;
        
        self.vm.load_program(ir)
            .map_err(|e| JsValue::from_str(&format!("VM load error: {}", e)))?;
        
        Ok(())
    }

    /// Execute one scan cycle with input signals
    ///
    /// Inputs should be a JavaScript object mapping signal names to boolean values.
    /// Returns a JavaScript object mapping coil names to their new states.
    #[wasm_bindgen]
    pub fn execute_cycle(&mut self, inputs: &JsValue) -> Result<JsValue, JsValue> {
        // Convert JS object to HashMap
        let inputs_map: HashMap<String, bool> = serde_wasm_bindgen::from_value(inputs.clone())
            .map_err(|e| JsValue::from_str(&format!("Invalid inputs: {}", e)))?;
        
        // Execute cycle
        let outputs = self.vm.step(inputs_map)
            .map_err(|e| JsValue::from_str(&format!("VM execution error: {}", e)))?;
        
        // Convert HashMap to JS object
        to_value(&outputs)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get the current state of a coil
    #[wasm_bindgen]
    pub fn get_coil(&self, name: &str) -> Option<bool> {
        self.vm.get_coil_state(name)
    }

    /// Get the current state of a signal
    #[wasm_bindgen]
    pub fn get_signal(&self, name: &str) -> Option<bool> {
        self.vm.get_signal_state(name)
    }

    /// Get all coil states
    #[wasm_bindgen]
    pub fn get_all_coils(&self) -> JsValue {
        to_value(&self.vm.get_all_coils()).unwrap_or(JsValue::NULL)
    }

    /// Get all signal states
    #[wasm_bindgen]
    pub fn get_all_signals(&self) -> JsValue {
        to_value(&self.vm.get_all_signals()).unwrap_or(JsValue::NULL)
    }

    /// Set a signal value
    #[wasm_bindgen]
    pub fn set_signal(&mut self, name: &str, value: bool) {
        self.vm.set_signal(name.to_string(), value);
    }

    /// Get signal names
    #[wasm_bindgen]
    pub fn signal_names(&self) -> Vec<String> {
        self.vm.signal_names().to_vec()
    }

    /// Get coil names
    #[wasm_bindgen]
    pub fn coil_names(&self) -> Vec<String> {
        self.vm.coil_names().to_vec()
    }
}
