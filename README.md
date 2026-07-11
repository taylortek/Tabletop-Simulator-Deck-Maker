# Tabletop-Simulator-Deck-Maker
A tts deck maker (made in rust) that's simple and doesn't force you to include blank card spaces.

# About
I made this very simple project to kill two birds with one stone.
1. I wanted to get better at rust
2. I wanted an easier alternative to the deck editor provided by Tabletop Simulator. Specifically, I wanted a program that didn't add white spaces at the end of every card sheet, and I wanted to just set one grid size for every sheet without manually entering it for every sheet.

I accomplished both of those goals, and I hope someone out there finds this useful in some way.

# Using the program
You're probably going to want to use the precompiled binary for windows (check the releases), but if you really want to compile it then you absolutely can!

## Steps
1. Double click the exe
2. Select all the card images you want for your deck (they must all have the same dimensions)
3. Select/Create the folder you want to output your sheets to
4. Follow the prompts in the terminal
   1. Enter how many columns you want.
   2. Enter how many rows you want.
   3. Enter the name you want for your sheets (the sheets will be named "{your_name}(001...).jpg")
5. Wait a few seconds

And you're done! Just import to tts or print!

# Compiling
I'll probably add a guide here soon, but it's pretty self explanatory if you've every compiled a rust project before.

# Thank You!
I hope you enjoy.
