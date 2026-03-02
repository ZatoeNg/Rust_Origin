use rand::Rng;

fn main() {
    println!("Hello, world!");

    let secret_number:u32 = rand::rng().random_range(1..=100);

    print!("The secret number is {}", secret_number);

    loop {
        print!("Please input your guess.");
        let mut guess = String::new();

    }

}
