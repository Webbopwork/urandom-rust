use urandom;

fn main() {
    let mut random = urandom::Urandom::new().unwrap();

    let percentage: f32 = random.read_usize().unwrap() as f32 / usize::MAX as f32;

    println!("Hello, world!\nRandom u8: {}\nPercentage: {}", random.read_u8().unwrap(), percentage);
}
