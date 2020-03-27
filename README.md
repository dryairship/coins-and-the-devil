# coins-and-the-devil

The idea of the game was taken from [here](https://blog.tanyakhovanova.com/2020/03/a-game-with-the-devil). 

The description of the game is as follows:

> You are playing a game with the Devil. There are n coins in a line, each showing either H (heads) or T (tails). Whenever the rightmost coin is H, you decide its new orientation and move it to the leftmost position. Whenever the rightmost coin is T, the Devil decides its new orientation and moves it to the leftmost position. This process repeats until all coins face the same way, at which point you win. What’s the winning strategy? 

In this implementation, H has been replaced with 0 and T has been replaced with 1.

## Building
```
cargo build
```

## Running
```
cargo run
```
or you can build it first and then execute the generated binary file.

