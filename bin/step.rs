//! To build the binary:
//!
//!     `cargo build --release --bin step`
//!
//! To build the circuit:
//!
//!     `./target/release/circuit_function_field build`
//!
//! To prove the circuit using evm io:
//!
//!    `./target/release/circuit_function_evm prove --input-json src/bin/circuit_function_evm_input.json`
//!
//! Note that this circuit will not work with field-based io.
//!
//!
//!
use plonky2x::backend::function::Plonky2xFunction;
use tendermintx::step::StepCircuit;

fn main() {
    const VALIDATOR_SET_SIZE_MAX: usize = 100;
    StepCircuit::<VALIDATOR_SET_SIZE_MAX>::entrypoint();
}
