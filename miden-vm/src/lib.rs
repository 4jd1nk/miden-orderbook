mod utils_masm_code;
mod utils_input;
mod utils_program;
use miden_vm::ProvingOptions;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize)]
pub struct Outputs {
    pub stack_output: Vec<u64>,
    pub trace_len: Option<usize>,
    pub overflow_addrs: Option<Vec<u64>>,
    pub proof: Option<Vec<u8>>,
}

/// Proves the program with the given inputs
#[wasm_bindgen]
pub fn prove_program(inputs_frontend: &str) -> Result<Outputs, JsValue> {
    let mut program = utils_program::MidenProgram::new(&utils_masm_code::get_masm_code().to_string(), utils_program::DEBUG_OFF);
    program
        .compile_program()
        .map_err(|err| format!("Failed to compile program - {:?}", err))?;

    let mut inputs = utils_input::Inputs::new();
    inputs
        .deserialize_inputs(inputs_frontend)
        .map_err(|err| format!("Failed to deserialize inputs - {:?}", err))?;

    // default (96 bits of security)
    let proving_options = ProvingOptions::default();

    let stack_input_cloned = inputs.stack_inputs.clone();
    let (output, proof) = miden_vm::prove(
        &program.program.unwrap(),
        stack_input_cloned,
        inputs.advice_provider,
        proving_options,
    )
    .map_err(|err| format!("Failed to prove execution - {:?}", err))?;

    let result = Outputs {
        stack_output: output.stack().to_vec(),
        trace_len: Some(proof.stark_proof().trace_length()),
        overflow_addrs: Some(output.overflow_addrs().to_vec()),
        proof: Some(proof.to_bytes()),
    };

    miden_vm::verify(
        program.program_info.unwrap(),
        inputs.stack_inputs,
        output,
        proof,
    )
    .map_err(|err| format!("Failed to verify proof - {:?}", err))?;

    Ok(result)
}

#[test]
fn test_prove_program() {
    let input_str: &str = r#"
    {
        "operand_stack": ["0"],
        "advice_stack": ["0"]
    }"#;

    let prove_result = prove_program(input_str).unwrap();
    assert_eq!(prove_result.stack_output, vec![3, 0, 0])
}
