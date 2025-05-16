package adapter

// CoffeemachineAdapter defines operations that a coffee machine should support.
// Implementations may support different features depending on the specific machine model.
type CoffeemachineAdapter interface {
	// MakeCoffee starts the process of brewing a coffee.
	// The specific type of coffee may depend on the implementation.
	MakeCoffee() error

	// InkassoOn enables "Inkasso mode," which is used for cash-based payment systems.
	// In this project this is used to remove the possibility of making a coffee without
	// going through our ethereum payment system.
	// This feature is optional and may not be supported by all coffee machines.
	InkassoOn() error

	// InkassoOff disables "Inkasso mode," returning the machine to standard operation.
	// This feature is optional and may not be supported by all coffee machines.
	InkassoOff() error

	// PowerOn turns on the coffee machine, initializing it for operation.
	PowerOn() error

	// PowerOff shuts down the coffee machine safely.
	PowerOff() error
}
