use anyhow::Result;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, Witness};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::GenericConfig;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use std::fs;

fn main() {
    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, 2>::new(config.clone());

    let a = builder.add_virtual_target();
    let b = builder.add_virtual_target();
    builder.connect(a, b);

    let data = builder.build::<C>();
    let mut pw = PartialWitness::<F>::new();

    pw.set_target(a, GoldilocksField(10));
    pw.set_target(b, GoldilocksField(10));
    let proof = data.prove(pw).unwrap();

    let proof_fs = serde_json::to_string(&proof).unwrap();
    fs::write("proof_fs.json", proof_fs);
    match data.verify(proof) {
        Ok(()) => println!("They are equal"),
        Err(x) => println!("{}", x),
    }
}
