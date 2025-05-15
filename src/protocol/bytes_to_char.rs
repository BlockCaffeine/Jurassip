/// Converts a slice of obfuscated bytes into a string.
/// 
/// # Arguments
/// * `input_bytes` - A slice of obfuscated bytes to decode.
/// 
/// # Returns
/// A string representing the decoded characters.
pub fn bytes_to_char(input_bytes: &[u8]) -> String {
    input_bytes
        .chunks_exact(4)
        .map(|chunk| {
            let array: [u8; 4] = chunk.try_into().expect("Invalid chunk size");
            let ascii: u8 = decode_4_bytes_to_ascii(array);
            ascii as char
        })
        .collect()
}

/// Decodes a 4-byte obfuscated array into an ASCII byte.
/// 
/// # Arguments
/// * `input_bytes` - A 4-byte array to decode.
/// 
/// # Returns
/// The decoded ASCII byte.
fn decode_4_bytes_to_ascii(input_bytes: [u8; 4]) -> u8 {
    let mut decoded_byte: u8 = 0;

    for i in 0..4 {
        let bit_2: u8 = (input_bytes[i] >> 2) & 1; // Extract Bit 2
        let bit_5: u8 = (input_bytes[i] >> 5) & 1; // Extract Bit 5

        decoded_byte |= (bit_2 << (i * 2)) | (bit_5 << (i * 2 + 1)); // Set the bits in the reconstructed byte
    }

    decoded_byte
}
