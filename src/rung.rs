use crate::store::Store;
use crate::ir::{RungDecl, GuardExpr};
use crate::error::{Result, VMError};
use std::collections::HashMap;

/// Evaluate a guard expression against the store
pub fn evaluate_guard(guard: &GuardExpr, store: &Store) -> Result<bool> {
    match guard {
        GuardExpr::Contact { name, contact_type, .. } => {
            let value = store.get(name)
                .ok_or_else(|| VMError::NotFound(format!("Signal/coil not found: {}", name)))?;
            
            match contact_type.as_str() {
                "NO" => Ok(value), // Normally Open: passes when true
                "NC" => Ok(!value), // Normally Closed: passes when false
                _ => Err(VMError::InvalidGuard(format!("Invalid contact type: {}", contact_type))),
            }
        }
        GuardExpr::And { left, right } => {
            let left_val = evaluate_guard(left, store)?;
            let right_val = evaluate_guard(right, store)?;
            Ok(left_val && right_val)
        }
        GuardExpr::Or { left, right } => {
            let left_val = evaluate_guard(left, store)?;
            let right_val = evaluate_guard(right, store)?;
            Ok(left_val || right_val)
        }
        GuardExpr::Not { expr } => {
            let val = evaluate_guard(expr, store)?;
            Ok(!val)
        }
    }
}

/// Evaluate a rung and return proposed coil updates
pub fn evaluate_rung(rung: &RungDecl, store: &Store) -> Result<Vec<(String, bool)>> {
    let guard_result = evaluate_guard(&rung.guard, store)?;
    
    if guard_result {
        // Guard is true, energise coils
        let mut updates = Vec::new();
        for action in &rung.actions {
            match action.action_type.as_str() {
                "energise" => updates.push((action.coil.clone(), true)),
                "de_energise" => updates.push((action.coil.clone(), false)),
                _ => return Err(VMError::InvalidGuard(
                    format!("Unknown action type: {}", action.action_type)
                )),
            }
        }
        Ok(updates)
    } else {
        // Guard is false, no updates
        Ok(Vec::new())
    }
}

/// Combine multiple rung updates for the same coil (OR-combine)
pub fn combine_updates(updates: &[(String, bool)]) -> HashMap<String, bool> {
    let mut result = HashMap::new();
    for (coil, value) in updates {
        // OR-combine: if any rung wants to energise, coil is energised
        result.entry(coil.clone())
            .and_modify(|v: &mut bool| *v = *v || *value)
            .or_insert(*value);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_no() {
        let mut store = Store::new();
        store.set("signal1".to_string(), true);
        
        let guard = GuardExpr::Contact {
            name: "signal1".to_string(),
            contact_type: "NO".to_string(),
            arguments: None,
        };
        
        assert_eq!(evaluate_guard(&guard, &store).unwrap(), true);
    }

    #[test]
    fn test_contact_nc() {
        let mut store = Store::new();
        store.set("signal1".to_string(), true);
        
        let guard = GuardExpr::Contact {
            name: "signal1".to_string(),
            contact_type: "NC".to_string(),
            arguments: None,
        };
        
        assert_eq!(evaluate_guard(&guard, &store).unwrap(), false);
    }

    #[test]
    fn test_and() {
        let mut store = Store::new();
        store.set("s1".to_string(), true);
        store.set("s2".to_string(), true);
        
        let guard = GuardExpr::And {
            left: Box::new(GuardExpr::Contact {
                name: "s1".to_string(),
                contact_type: "NO".to_string(),
                arguments: None,
            }),
            right: Box::new(GuardExpr::Contact {
                name: "s2".to_string(),
                contact_type: "NO".to_string(),
                arguments: None,
            }),
        };
        
        assert_eq!(evaluate_guard(&guard, &store).unwrap(), true);
    }
}
