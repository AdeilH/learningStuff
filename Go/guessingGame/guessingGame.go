package main

import (
	"fmt"
	"math/rand"
)

func main() {
	fmt.Println("Guess the Number!!! ")
	secretNumber := rand.Intn(100)
	guess := 0
	for {
		fmt.Println("Please input your guess ")
		_, err := fmt.Scan(&guess)
		if err != nil {
			fmt.Println(err)
		}
		fmt.Println("Your guess is", guess)

		if guess > secretNumber {
			fmt.Println("Too Big!")
		}
		if guess < secretNumber {
			fmt.Println("Too Small!")
		}
		if guess == secretNumber {
			fmt.Println("You Win")
			break
		}

	}

}
