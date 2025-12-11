// Re-export IR types from charta-core for VM use
pub use charta_core::ir::schema::{
    IR, Module, RungDecl, GuardExpr, Action, CoilDecl, SignalDecl,
};

/// Load IR from JSON string
pub fn load_ir(ir_json: &str) -> crate::error::Result<IR> {
    let ir: IR = serde_json::from_str(ir_json)
        .map_err(|e| crate::error::VMError::IrLoad(
            charta_core::error::ValidationError::JsonParse(e)
        ))?;
    Ok(ir)
}
