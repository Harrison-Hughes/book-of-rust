use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let vector: Vec<u32> = (0..10).map(|_| rng.gen_range(0..100)).collect();

    println!("Results for vector {:?}", vector);
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

    (sum / vector.len() as u32) as f32
}

fn median(vector: &Vec<u32>) -> u32 {
    let mut vec = vector.clone();
    vec.sort();

    vec[vec.len() / 2]
}

fn mode(vector: &Vec<u32>) -> u32 {
    let mut values = HashMap::new();

    for i in vector {
        let count = values.entry(word).or_insert(0);
        *count += 1;
    }
}
