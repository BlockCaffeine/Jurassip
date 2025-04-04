package main

import (
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/BlockCaffeine/Jurassip/internal/adapter"
	"github.com/BlockCaffeine/Jurassip/internal/uart"
)

func main() {
	fmt.Println("Starting the Jurassip service ...")

	// Setup UART Connection
	fmt.Println("Setting up UART Connection ...")
	conn, portCloser, err := uart.OpenUART()
	if err != nil {
		log.Fatal("Failed to initialize UART:", err)
	}
	defer portCloser.Close()

	// Setup Coffemachine Adapter
	fmt.Println("Setting up Machine Adapters ...")
	uartHandler := uart.NewUART(conn, uart.WithObfuscation(uart.JuraObfuscation))
	juraAdapter := adapter.NewJuraAdapter(uartHandler)

	fmt.Println("Shutting machine off")
	juraAdapter.PowerOff()
	fmt.Println("Did it work?")

	// Setup Blockchain Listener
	rpcURL := "http://134.155.50.136:8506"
	contractAddress := "0xSomeContractAddress"

	// Create a channel to listen for interrupts
	interruptChannel := make(chan os.Signal, 1)
	signal.Notify(interruptChannel, syscall.SIGINT, syscall.SIGTERM)

	/*
		fmt.Println("Setting up Blockchain Listener ...")
		listener, err := blockchain.NewListener(rpcURL, contractAddress, juraAdapter)
		if err != nil {
			log.Fatal("Failed to initialize blockchain listener:", err)
		}

		// Start the blockchain listener in a goroutine, passing the interrupt channel
		// to close the listener for incoming interrupts.
		go listener.WatchPayments(interruptChannel)
	*/

	// Block until a signal is received
	<-interruptChannel
	fmt.Println("Interrupt received. Service shut down gracefully.")
}
