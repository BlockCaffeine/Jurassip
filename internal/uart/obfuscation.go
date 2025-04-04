package uart

// JuraObfuscation obfuscates input data by mapping each 2 bits of input
// into a byte of form 01x11x11, where 'x' bits are data bits.
func JuraObfuscation(data []byte) (obfuscatedData []byte) {
	for _, b := range data {
		// Process 8 bits in chunks of 2 bits (4 chunks per byte)
		for i := 6; i >= 0; i -= 2 {
			twoBits := (b >> i) & 0x03 // Extract 2 bits

			// Set the fixed pattern: 01x11x11
			// Start with 0b01011011 (binary: 0x5B)
			outByte := byte(0b01011011)

			// Insert bit 1 of the twoBits at position 5
			if (twoBits & 0x02) != 0 {
				outByte |= 1 << 5
			} else {
				outByte &^= 1 << 5
			}

			// Insert bit 0 of the twoBits at position 2
			if (twoBits & 0x01) != 0 {
				outByte |= 1 << 2
			} else {
				outByte &^= 1 << 2
			}

			obfuscatedData = append(obfuscatedData, outByte)
		}
	}
	return
}
