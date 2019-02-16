import random

print ("Guess the number!!! \n")

secret_number = random.randint(1,100)

while True:
    guess = int(input("Please input your guess \n"))
    print("Your Guess: ", guess)
    if (guess > secret_number):
        print("Too Big!")
    elif (guess < secret_number):
        print("Too Small!")
    else:
        print("You Win!")
        break        


