
#[allow(unused)]
pub fn print_binary_as_rust_code(bytes: &[u8]) {
    print!("let data: [u8; {}] = [", bytes.len());
    for (i, byte) in bytes.iter().enumerate() {
        if i % 16 == 0 {
            print!("\n    ");
        }
        print!("0x{:02x}, ", byte);
    }
    println!("\n];");
}

pub fn print_binary(bytes: &[u8]) {
    println!("Hex Dump:");
    print!("        ");
    for i in 0..16 {
        print!("{:02x} ", i);
    }
    println!{"\n       ------------------------------------------------"};
    for (i, byte) in bytes.iter().enumerate() {
        let row = i / 16;
        match i%16 {
            0 => print!("{:04x}0 | {:02x} ", row, byte),
            15 => println!("{:02x}", byte),
            _ => print!("{:02x} ", byte),
        }
    }
    println!{"\n       ------------------------------------------------"};
}

