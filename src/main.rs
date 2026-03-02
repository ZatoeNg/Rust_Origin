use rand::Rng;

fn main() {
    println!("Hello, world!");

    let secret_number:32u = rand::rng().random_range(1..=100);
    
}
