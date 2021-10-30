use rand::{Rng, thread_rng};

fn main() {

    let mut rng = thread_rng();
    let input_vector: Vec<u64> = (0..100).map(|_| rng.gen_range(0..100)).collect();

    println!("Results for vector {:?}", input_vector);
    println!()
}
