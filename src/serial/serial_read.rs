use serialport::SerialPort;

pub fn serial_read(port: &mut Box<dyn SerialPort>) {
    let mut buffer: Vec<u8> = vec![0; 1024];
    loop {
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    println!("Received {} bytes:", bytes_read);
                    for byte in &buffer[..bytes_read] {
                        println!("Byte: {:08b} (0x{:02X})", byte, byte);
                    }
                }
            }
            Err(_e) => {
                continue;
            }
        }
    }
}
