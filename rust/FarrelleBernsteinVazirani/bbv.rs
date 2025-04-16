use quantum_register::QuantumRegister;

fn farrelle_bernstein_vazirani(secret: &[u8]) -> Vec<u8> {
    let num_qubits = secret.len();
    let mut qr = QuantumRegister::new(num_qubits);
 
    qr.hadamard_all(); // create superposition of all states
 
    qr.oracle(secret,
                |x, s| -> u8 {
                                let dot_product_mod2 = x.iter().zip(s.iter())
                                                        .fold(0, |acc, (xi, si)| acc ^ (xi & si)); // dot product mod 2
                                dot_product_mod2 }, 
             );
 
    qr.measure()
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
 