use rand::{thread_rng, Rng};
use std::collections::HashMap;

fn main() {
    let mut rng = thread_rng();
    let vector: Vec<u32> = (0..100).map(|_| rng.gen_range(0..100)).collect();

    println!(
        "Mean: {}, Median: {}, Mode: {}",
        mean(&vector),
        median(&vector),
        mode(&vector)
    )
}

fn mean(vector: &Vec<u32>) -> f32 {
    let mut sum: u32 = 0;
    for i in vector {
        sum += i
    }

    sum as f32 / vector.len() as f32
}

fn median(vector: &Vec<u32>) -> f32 {
    let mut vec = vector.clone();
    vec.sort();

    if vec.len() % 2 == 0 {
        (vec[vec.len() / 2] as f32 + vec[vec.len() / 2 - 1] as f32) / 2.0
    } else {
        vec[vec.len() / 2] as f32
    }

}

fn mode(vector: &Vec<u32>) -> &u32 {
    let mut count = HashMap::new();

    for i in vector {
        *count.entry(i).or_insert(0) += 1;
    }

    count
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")

}
