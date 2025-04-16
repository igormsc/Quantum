use quantum_register::QuantumRegister;

fn simons_algorithm<F>(num_bits: usize, secret_function: F) -> Result<BitVector, &'static str> where F: Fn(&BitVector) -> BitVector,
{
   let mut qr = QuantumRegister::new(num_bits * 2); // double the number of qubits for the second register

   qr.hadamard_all(); // transition to a superposition

   qr.oracle(secret_function); // processing the state with the secret function
   
   let measured_state = qr.measure()[0..num_bits].to_vec(); // measure the first num_bits qubits

   Ok(measured_state)
}

fn main() {
   
   let example_func = |input: &BitVector| -> BitVector {
       let xor_with_secret = vec![input[0] ^ 1, input[1]];
       xor_with_secret }; // example function with period s = [1, 0]

   let result = simons_algorithm(2, example_func);
   match result {
       Ok(s) => println!("Secret period found: {:?}", s),
       Err(e) => eprintln!("{}", e),
   }
}
