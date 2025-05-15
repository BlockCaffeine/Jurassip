/// Converts a character into an obfuscated 4-byte array.
/// 
/// # Arguments
/// * `char` - The character to be converted.
/// 
/// # Returns
/// A 4-byte array representing the obfuscated character.
pub fn char_to_bytes(char: char) -> [u8; 4] {
    let ascii: u8 = char as u8;
    let ascii_lsb: u8 = reverse_bits(ascii);
    let ascii_obf: [u8; 4] = encode_lsb_to_4_bytes(ascii_lsb);

    ascii_obf
}

/// Reverses the bits of a byte.
/// 
/// # Arguments
/// * `byte` - The byte to reverse.
/// 
/// # Returns
/// The byte with its bits reversed.
fn reverse_bits(byte: u8) -> u8 {
    let mut b: u8 = byte;
    b = (b & 0xF0) >> 4 | (b & 0x0F) << 4;
    b = (b & 0xCC) >> 2 | (b & 0x33) << 2;
    b = (b & 0xAA) >> 1 | (b & 0x55) << 1;
    b
}

/// Encodes the least significant bits of a byte into a 4-byte array.
/// 
/// # Arguments
/// * `lsb` - The least significant bits to encode.
/// 
/// # Returns
/// A 4-byte array representing the encoded bits.
fn encode_lsb_to_4_bytes(lsb: u8) -> [u8; 4] {
    let mut block: [u8; 4] = [0b11111111; 4]; // Initialize with all bits set to 1

    for i in 0..4 {
        let bit_2: u8 = (lsb >> (i * 2)) & 1; // Extract the i-th bit for Bit 2
        let bit_5: u8 = (lsb >> (i * 2 + 1)) & 1; // Extract the i-th bit for Bit 5

        // Reverse the block order: block[3 - i] instead of block[i]
        block[3 - i] = (block[3 - i] & !(1 << 2)) | (bit_5 << 2); // Set Bit 2
        block[3 - i] = (block[3 - i] & !(1 << 5)) | (bit_2 << 5); // Set Bit 5

        // Ensure Bit 7 is always 0
        block[3 - i] &= !(1 << 7);
    }

    block
}
