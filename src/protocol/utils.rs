pub fn format_obfuscated_bytes(byte: u8) -> String {
  let binary = format!("{:08b}", byte);
  format!(
      "{} {} {} {} {}",
      &binary[0..2], // First two bits
      &binary[2..3], // Third bit
      &binary[3..5], // Fourth and fifth bits
      &binary[5..6], // Sixth bit
      &binary[6..8]  // Last two bits
  )
}
