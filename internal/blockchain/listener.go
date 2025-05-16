package blockchain

import (
	"context"
	"log"
	"math/big"
	"os"

	"github.com/BlockCaffeine/Jurassip/internal/adapter"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
)

type Listener struct {
	client          *ethclient.Client
	contractAddress common.Address
	adapter         adapter.CoffeemachineAdapter
}

// NewListener initializes the Ethereum client and stores the UART connection
func NewListener(rpcURL string, contractAddr string, adapter adapter.CoffeemachineAdapter) (*Listener, error) {
	client, err := ethclient.Dial(rpcURL)
	if err != nil {
		return nil, err
	}

	return &Listener{
		client:          client,
		contractAddress: common.HexToAddress(contractAddr),
		adapter:         adapter,
	}, nil
}

// WatchPayments listens for blockchain events and triggers coffee commands
func (l *Listener) WatchPayments(interruptChannel chan os.Signal) {
	// Define the threshold for the coffee order: 0.000263 ETH in wei
	threshold := big.NewInt(263000000000000)

	// Define the signature of the OrderCoffee event (indexed parameters and data)
	// For Solidity, the event signature is the keccak256 hash of the event definition
	eventSignature := []byte("OrderCoffee(address,uint256)")
	eventSignatureHash := common.BytesToHash(crypto.Keccak256(eventSignature))

	query := ethereum.FilterQuery{
		Addresses: []common.Address{l.contractAddress},
		Topics:    [][]common.Hash{{eventSignatureHash}},
	}

	logs := make(chan types.Log)
	sub, err := l.client.SubscribeFilterLogs(context.Background(), query, logs)
	if err != nil {
		log.Fatalf("Failed to subscribe to Ethereum logs: %v", err)
	}
	defer sub.Unsubscribe()

	log.Println("Listening for coffee payments...")

	for {
		select {
		case err := <-sub.Err():
			log.Println("Subscription error:", err)
		case vLog := <-logs:
			log.Println("New payment detected! Processing...")

			// Check if the log is for the OrderCoffee event
			if !(len(vLog.Topics) == 3 && vLog.Topics[0] == eventSignatureHash) {
				log.Println("Wrong topic, ignoring event ...")
				continue
			}

			// Extract the user address and payment amount from the event log
			user := common.HexToAddress(vLog.Topics[1].Hex())
			amount := new(big.Int).SetBytes(vLog.Data)

			log.Printf("New coffee order detected from %s for %s ETH!", user.Hex(), amount.String())

			if amount.Cmp(threshold) >= 0 {
				log.Println("Payment confirmed. Making coffee...")
				err := l.adapter.MakeCoffee()
				if err != nil {
					log.Println("Failed to make coffee:", err)
				}
			} else {
				log.Println("Payment too low, ignoring.")
			}
		case <-interruptChannel:
			log.Println("Interrupt signal received, shutting down listener...")
			return
		}
	}
}
