extern crate QuantumComputer;

use QuantumComputer::{
   QuantumSystem,
   create_qubits,
   calculate_num_qubits,
   run_circuit,
   measure_all,
   apply_error_correction,
   simulate_noise,
};

l// target loss functions (Hamiltonian)
let hamiltonians = vec![
    r#"H = 2X + 3"#, // linear function
    r#"H = X^2 + 2X + 1"#, // quadratic function
    r#"H = X^3 + 2X^2 + X + 1"#, // polynomial function
    r#"H = sin(X) + cos(X)"#, // trigonometric function
    r#"H = exp(X) + 1"#, // exponential function
];


const ERROR_THRESHOLD: f64 = 0.01; // allowed result error

const MAX_RETRIES: u32 = 5; // maximum number of error correction attempts

const NOISE_LIMIT: f64 = 0.1; // maximum allowed noise level

fn check_result(results: &[f64], threshold: f64) -> bool {
   results.iter().any(|&val| val.abs() > threshold)
}

fn simulate_quantum_noise(system: &mut QuantumSystem, noise_level: f64) {
   system.simulate_noise(noise_level); // add noise at the specified level
}


fn quantum_annealing_with_errors(
   hamiltonian: &str,
   steps: u32,
   retries: u32,
   error_threshold: f64,
   noise_limit: f64,
) -> Option<Vec<f64>> {
   let mut attempts = 0;
   loop {
       
       let num_qubits = calculate_num_qubits(hamiltonian);

       let qubits = create_qubits(num_qubits);

       let mut system = QuantumSystem::new(qubits.len()); 

       apply_error_correction(&mut system); // apply preliminary error correction

       simulate_quantum_noise(&mut system, noise_limit); // simulate a small amount of noise

       let results = system.anneal(hamiltonian.to_string(), steps); // run quantum annealing

       let measurements = measure_all(&qubits); // measure the state of the qubits

       if !check_result(&measurements, error_threshold) {
           break Some(measurements.into_iter().map(|m| m.as_f64()).collect());
       } 

       attempts += 1;
       if attempts >= retries {
           eprintln!("Error: maximum number of attempts exceeded.");
           break None;
       }
   }
}

fn main() {
   println!("Running quantum annealing on the quantum computer with error correction and noise");

   const STEPS: u32 = 1000; // Number of annealing steps
    
   let hamiltonian = &hamiltonians[1]; // select the desired hamiltonian function from the list

   let result = quantum_annealing_with_errors(hamiltonian, STEPS, MAX_RETRIES, ERROR_THRESHOLD, NOISE_LIMIT);
   match result {
       Some(r) => {
           println!("Obtained quantum annealing result:");
           r.into_iter().for_each(|v| println!("{}", v) ); },
       None => {
           eprintln!("Failed to obtain a reliable result after {} attempts.", MAX_RETRIES); }
   }
}
