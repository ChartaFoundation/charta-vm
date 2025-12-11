use crate::store::Store;
use crate::rung::{evaluate_rung, combine_updates};
use crate::ir::{IR, RungDecl, CoilDecl};
use crate::error::{Result, VMError};
use std::collections::HashMap;

/// Execute one scan cycle
/// 
/// Scan cycle steps:
/// 1. Read environment inputs (ι)
/// 2. Merge with store: σ ⊕ ι
/// 3. Evaluate all rungs
/// 4. Update coil states
/// 5. Return new store (σ')
pub fn execute_cycle(
    store: &mut Store,
    inputs: &HashMap<String, bool>,
    rungs: &[RungDecl],
    coils: &[CoilDecl],
    signal_names: &[String],
) -> Result<HashMap<String, bool>> {
    // Step 1 & 2: Merge environment inputs into store
    store.merge_inputs(inputs, signal_names);
    
    // Step 3: Evaluate all rungs
    let mut all_updates = Vec::new();
    for rung in rungs {
        let updates = evaluate_rung(rung, store)?;
        all_updates.extend(updates);
    }
    
    // Step 4: Combine updates and apply to coils
    let combined = combine_updates(&all_updates);
    
    // Update store with coil values
    // Non-latching coils: if no rung energises, coil is false
    // Latching coils: if no rung energises, keep previous value
    let coil_names: Vec<String> = coils.iter().map(|c| c.name.clone()).collect();
    let mut coil_outputs = HashMap::new();
    
    for coil in coils {
        if let Some(&value) = combined.get(&coil.name) {
            // Rung wants to set this coil
            store.set(coil.name.clone(), value);
            coil_outputs.insert(coil.name.clone(), value);
        } else {
            // No rung wants to set this coil
            if coil.latching.unwrap_or(false) {
                // Latching: keep previous value
                let prev_value = store.get(&coil.name).unwrap_or(false);
                coil_outputs.insert(coil.name.clone(), prev_value);
            } else {
                // Non-latching: default to false
                store.set(coil.name.clone(), false);
                coil_outputs.insert(coil.name.clone(), false);
            }
        }
    }
    
    // Step 5: Return coil outputs
    Ok(coil_outputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Action, GuardExpr, RungDecl};

    #[test]
    fn test_cycle_basic() {
        let mut store = Store::new();
        store.set("input_signal".to_string(), false);
        store.set("output_coil".to_string(), false);
        
        let inputs = {
            let mut m = HashMap::new();
            m.insert("input_signal".to_string(), true);
            m
        };
        
        let rungs = vec![RungDecl {
            name: "test".to_string(),
            guard: GuardExpr::Contact {
                name: "input_signal".to_string(),
                contact_type: "NO".to_string(),
                arguments: None,
            },
            actions: vec![Action {
                action_type: "energise".to_string(),
                coil: "output_coil".to_string(),
                arguments: None,
            }],
        }];
        
        let coils = vec![CoilDecl {
            name: "output_coil".to_string(),
            parameters: None,
            latching: Some(false),
            critical: Some(false),
        }];
        
        let signal_names = vec!["input_signal".to_string()];
        
        let outputs = execute_cycle(&mut store, &inputs, &rungs, &coils, &signal_names).unwrap();
        
        assert_eq!(outputs.get("output_coil"), Some(&true));
        assert_eq!(store.get("output_coil"), Some(true));
    }
}
