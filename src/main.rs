mod protocol {
    pub mod char_to_bytes;
    pub mod bytes_to_char;
}

use protocol::char_to_bytes::char_to_bytes;
use protocol::bytes_to_char::bytes_to_char;

fn main() {
    println!("Hello, world!");

    let input_char: char = 'A'; // Example character

    let ascii: u8 = input_char as u8;

    println!("Character: '{}'", input_char);
    println!("Hex: 0x{:02X}", ascii);
    println!("Binary: {:08b}", ascii);

    let obfuscated: [u8; 4] = char_to_bytes(input_char);

    println!("Obfuscated: ");
    for byte in obfuscated.iter() {
        println!("{:08b}", byte);
    }

    println!("REVERSE ------------------");

    let deobfuscated: char = bytes_to_char(obfuscated);
    println!("Deobfuscated: '{}'", deobfuscated);
}
