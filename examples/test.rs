use urandom;

fn main() {
    println!("Opening urandom file... (/dev/urandom)");

    let mut random: urandom::Urandom = urandom::Urandom::new().unwrap();

    let mut output: [u8; 2] = [0u8; 2];

    println!("Reading urandom...");
    match random.read_buffer(&mut output) {
        Err(e) => println!("Error: {}", e),
        Ok(_) => {
            println!("Out: {:?}", output);
            println!("Out in u16: {:?}", u16::from_le_bytes(output));
            println!("Out for read_u16: {:?}", random.read_u16().unwrap());
        }
    };
    
}