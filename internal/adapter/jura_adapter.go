package adapter

import (
	"fmt"

	"github.com/BlockCaffeine/Jurassip/internal/uart"
)

type Command string

const (
	MachineOn      Command = "AN:01"
	MachineOff     Command = "AN:02"
	MakeProduct1   Command = "FA:01"
	MakeProduct2   Command = "FA:02"
	InkassoModeOn  Command = "*EIN"
	InkassoModeOff Command = "*AUS"
)

// JuraAdapter implements CoffeemachineAdapter using UART communication.
type JuraAdapter struct {
	uartConn *uart.UART
}

// NewJuraAdapter creates a new Jura coffee machine adapter.
func NewJuraAdapter(uart *uart.UART) *JuraAdapter {
	return &JuraAdapter{uartConn: uart}
}

// MakeCoffee sends a command to make coffee.
func (j *JuraAdapter) MakeCoffee() error {
	fmt.Println("Making coffee via JuraAdapter...")
	return j.uartConn.SendCommand(string(MakeProduct1))
}

// PowerOn turns the coffee machine on.
func (j *JuraAdapter) PowerOn() error {
	fmt.Println("Powering on Jura machine...")
	return j.uartConn.SendCommand(string(MachineOn))
}

// PowerOff turns the coffee machine off.
func (j *JuraAdapter) PowerOff() error {
	fmt.Println("Powering off Jura machine...")
	return j.uartConn.SendCommand(string(MachineOff))
}

// PowerOff turns the coffee machine off.
func (j *JuraAdapter) InkassoOn() error {
	fmt.Println("Turning Inkasso Mode on...")
	return j.uartConn.SendCommand(string(InkassoModeOn))
}

// PowerOff turns the coffee machine off.
func (j *JuraAdapter) InkassoOff() error {
	fmt.Println("Turning Inkasso Mode off...")
	return j.uartConn.SendCommand(string(InkassoModeOff))
}
