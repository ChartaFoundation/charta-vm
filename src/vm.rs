use crate::store::Store;
use crate::cycle::execute_cycle;
use crate::ir::{IR, load_ir};
use crate::error::{Result, VMError};
use std::collections::HashMap;

/// Charta Virtual Machine
pub struct VM {
    /// Current store (σ)
    store: Store,
    /// Loaded IR program
    program: IR,
    /// Signal names (for input merging)
    signal_names: Vec<String>,
    /// Coil names (for output)
    coil_names: Vec<String>,
}

impl VM {
    /// Create a new VM
    pub fn new() -> Self {
        Self {
            store: Store::new(),
            program: IR {
                version: "0.1.0".to_string(),
                module: crate::ir::Module {
                    name: String::new(),
                    context: None,
                    intent: None,
                    constraints: None,
                    signals: None,
                    coils: None,
                    rungs: None,
                    blocks: None,
                    networks: None,
                },
            },
            signal_names: Vec::new(),
            coil_names: Vec::new(),
        }
    }

    /// Load an IR program into the VM
    pub fn load_program(&mut self, ir: IR) -> Result<()> {
        self.program = ir;
        
        // Extract signal and coil names
        self.signal_names = self.program.module.signals
            .as_ref()
            .map(|signals| signals.iter().map(|s| s.name.clone()).collect())
            .unwrap_or_default();
        
        self.coil_names = self.program.module.coils
            .as_ref()
            .map(|coils| coils.iter().map(|c| c.name.clone()).collect())
            .unwrap_or_default();
        
        // Initialize store with all signals/coils set to false
        for name in &self.signal_names {
            self.store.set(name.clone(), false);
        }
        for name in &self.coil_names {
            self.store.set(name.clone(), false);
        }
        
        Ok(())
    }

    /// Execute one scan cycle
    /// 
    /// Returns a map of coil name -> value for newly energised coils
    pub fn step(&mut self, inputs: HashMap<String, bool>) -> Result<HashMap<String, bool>> {
        let rungs = self.program.module.rungs.as_ref()
            .ok_or_else(|| VMError::Execution("No rungs in program".to_string()))?;
        
        let coils = self.program.module.coils.as_ref()
            .ok_or_else(|| VMError::Execution("No coils in program".to_string()))?;
        
        execute_cycle(
            &mut self.store,
            &inputs,
            rungs,
            coils,
            &self.signal_names,
        )
    }

    /// Get current state of a coil
    pub fn get_coil_state(&self, name: &str) -> Option<bool> {
        self.store.get(name)
    }

    /// Get current state of a signal
    pub fn get_signal_state(&self, name: &str) -> Option<bool> {
        self.store.get(name)
    }

    /// Get all coil states
    pub fn get_all_coils(&self) -> HashMap<String, bool> {
        self.store.get_coils(&self.coil_names)
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_vm_load_and_step() {
        let mut vm = VM::new();
        
        // Create a simple IR program
        let ir = IR {
            version: "0.1.0".to_string(),
            module: crate::ir::Module {
                name: "test".to_string(),
                context: None,
                intent: None,
                constraints: None,
                signals: Some(vec![crate::ir::SignalDecl {
                    name: "input".to_string(),
                    parameters: None,
                    type_: None,
                }]),
                coils: Some(vec![crate::ir::CoilDecl {
                    name: "output".to_string(),
                    parameters: None,
                    latching: Some(false),
                    critical: Some(false),
                }]),
                rungs: Some(vec![crate::ir::RungDecl {
                    name: "r1".to_string(),
                    guard: crate::ir::GuardExpr::Contact {
                        name: "input".to_string(),
                        contact_type: "NO".to_string(),
                        arguments: None,
                    },
                    actions: vec![crate::ir::Action {
                        action_type: "energise".to_string(),
                        coil: "output".to_string(),
                        arguments: None,
                    }],
                }]),
                blocks: None,
                networks: None,
            },
        };
        
        vm.load_program(ir).unwrap();
        
        let mut inputs = HashMap::new();
        inputs.insert("input".to_string(), true);
        
        let outputs = vm.step(inputs).unwrap();
        assert_eq!(outputs.get("output"), Some(&true));
    }
}
