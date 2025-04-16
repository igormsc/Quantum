use std::f64;
use quantum_state::QuantumState;

fn grovers_algorithm(search_space_size: usize, marked_item: usize) -> usize {
    
    let iterations = ((search_space_size as f64).sqrt() / 2.0).ceil() as usize;
    let mut state = QuantumState::new(search_space_size);

    state.hadamard(); // initialize superposition applying Hadamard gates 

    for _ in 0..iterations {
        state.oracle(marked_item);
        state.diffusion_operator();
    }

    state.measure() // measure the final state
}


fn main() {
    const SEARCH_SPACE_SIZE: usize = 1 << 4; // search space of 16 items
    const MARKED_ITEM: usize = 5;           // item we are searching for

    let found_item = grovers_algorithm(SEARCH_SPACE_SIZE, MARKED_ITEM);
    println!("Searching for {} in search space {}, found {}", MARKED_ITEM, SEARCH_SPACE_SIZE, found_item);
}
