package uart

import (
	"fmt"
	"log"

	"periph.io/x/conn/v3"
	"periph.io/x/conn/v3/physic"
	"periph.io/x/conn/v3/uart"
	"periph.io/x/conn/v3/uart/uartreg"
	"periph.io/x/host/v3"
	_ "periph.io/x/host/v3/rpi"
)

// UART handles UART communication with optional obfuscation.
type UART struct {
	conn        conn.Conn
	obfuscateFn func([]byte) []byte
}

// Option defines a functional option for UART.
type Option func(*UART)

// WithObfuscation sets an obfuscation function for commands.
func WithObfuscation(f func([]byte) []byte) Option {
	return func(u *UART) {
		u.obfuscateFn = f
	}
}

// NewUART initializes a new UART handler with optional settings
// passed as variadic parameters into the creator method.
func NewUART(conn conn.Conn, opts ...Option) *UART {
	u := &UART{conn: conn}

	// Apply options
	for _, opt := range opts {
		opt(u)
	}

	return u
}

// OpenUART initializes a UART connection
func OpenUART() (conn.Conn, uart.PortCloser, error) {
	log.Println("Initializing host ...")
	host.Init()

	ports := uartreg.All()
	if len(ports) == 0 {
		return nil, nil, fmt.Errorf("no UART ports registered")
	}

	log.Println("Available UART ports:")
	for _, port := range ports {
		fmt.Printf("  - %s (%d)\n", port.Name, port.Number)
	}

	// Use uartreg UART port registry to find the first available UART port.
	log.Println("Trying to connect to /dev/ttyAMA2...")
	portCloser, err := uartreg.Open("/dev/ttyAMA2")
	if err != nil {
		log.Fatal(err)
	}

	// Prints out the gpio pin used.
	if p, ok := portCloser.(uart.Pins); ok {
		log.Printf("  RX : %s", p.RX())
		log.Printf("  TX : %s", p.TX())
		// These aren't connected so there shouldn't be any output, just testing.
		log.Printf("  RTS: %s", p.RTS())
		log.Printf("  CTS: %s", p.CTS())
	}

	// Config for the Jura Machine will be passed in via an Config Object later on
	// https://protocol-jura.at.ua/index/protocol_to_coffeemaker/0-8: 9600 Baud, One Stop Bit, No Parity, 8 Bits
	log.Println("Open connection with port...")
	conn, err := portCloser.Connect(9600*physic.Hertz, uart.One, uart.NoParity, uart.NoFlow, 8)
	if err != nil {
		portCloser.Close()
		log.Fatal(err)
	}

	if err := conn.Tx([]byte("cmd"), nil); err != nil {
		portCloser.Close()
		log.Fatal(err)
	}

	log.Println("UART initialized")
	return conn, portCloser, nil
}

// SendCommand sends a command over UART, applying obfuscation if set.
func (u *UART) SendCommand(cmd string) error {
	data := []byte(cmd)

	// Apply obfuscation if configured
	if u.obfuscateFn != nil {
		data = u.obfuscateFn(data)
	}

	log.Printf("Sending UART command: %x\n", data)
	err := u.conn.Tx(data, nil)
	if err != nil {
		log.Printf("UART command failed: %s, error: %v\n", cmd, err)
	}
	return err
}
