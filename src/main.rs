/*

1. x1 + x1 = x2
2. x2 * x3 = x6
3. x4 * x5 = x7
4. x6 * x7 = out

*/

use dusk_plonk::prelude::*;
use rand_core::OsRng;

#[derive(Debug, Default)]
pub struct TestCircuit {
    pub x1: BlsScalar,
    pub x2: BlsScalar,
    pub x3: BlsScalar,
    pub x4: BlsScalar,
    pub x5: BlsScalar,
    pub x6: BlsScalar,
    pub x7: BlsScalar,
    pub out: BlsScalar,
}

impl Circuit for TestCircuit {
    fn circuit(&self, composer: &mut Composer) -> Result<(), Error> {
        let x1 = composer.append_witness(self.x1);
        let x2 = composer.append_witness(self.x2);
        let x3 = composer.append_witness(self.x3);
        let x4 = composer.append_witness(self.x4);
        let x5 = composer.append_witness(self.x5);
        let x6 = composer.append_witness(self.x6);
        let x7 = composer.append_witness(self.x7);
        let out = composer.append_witness(self.out);

        // 1. x1 + x1 = x2
        let constraint = Constraint::new().left(1).right(1).a(x1).b(x1);
        let result = composer.gate_add(constraint);
        composer.assert_equal(result, x2);

        // 2. x2 * x3 = x6
        let constraint = Constraint::new().mult(1).a(x2).b(x3);
        let result = composer.gate_mul(constraint);
        composer.assert_equal(result, x6);

        // 3. x4 * x5 = x7
        let constraint = Constraint::new().mult(1).a(x4).b(x5);
        let result = composer.gate_mul(constraint);
        composer.assert_equal(result, x7);

        // 4. x6 * x7 = out
        let constraint = Constraint::new().mult(1).a(x6).b(x7);
        let result = composer.gate_mul(constraint);
        composer.assert_equal(result, out);


        Ok(())
    }
}

fn main() {
    let label = b"plonk-test-2";
    let pp =
        PublicParameters::setup(1 << 12, &mut OsRng).expect("failed to setup");

    // Compile the default circuit to generate prover and verifier
    let (prover, verifier) = Compiler::compile::<TestCircuit>(&pp, label)
        .expect("failed to compile circuit");


    let x1 = BlsScalar::from(1);
    let x2 = BlsScalar::from(2);
    let x3 = BlsScalar::from(3);
    let x4 = BlsScalar::from(4);
    let x5 = BlsScalar::from(5);
    let x6 = BlsScalar::from(6);
    let x7 = BlsScalar::from(20);
    let out = BlsScalar::from(120);

    let circuit = TestCircuit{
        x1,
        x2,
        x3,
        x4,
        x5,
        x6,
        x7,
        out,
    };
    let public_inputs = vec![];
    // Generate the proof and its public inputs
    let (proof, pi) =
        prover.prove(&mut OsRng, &circuit).expect("failed to prove");
    assert_eq!(public_inputs, pi);

    // Verify the generated proof
    verifier
        .verify(&proof, &public_inputs)
        .expect("failed to verify proof");
}
