# Rust-TicTacToe
<img src="images/TicTacToe.png" height=200 /> <img src="https://www.freecodecamp.org/news/content/images/2021/01/rust-mascot.png" height=200 />

I created a small Rust console program to play tictactoe against your friends or an ai.

## Startup
When you start the program you'll be greeted by a mode selection.

**vs**: Play against another player.<br>
**ai**: Play against the minimax player.<br>
&emsp;If you choose **ai**, then you will have to choose what to play as:<br>
&emsp;Either as *x* to begin.<br>
&emsp;Or as *o* to play second.<br>

When playing you will have to input the column and then the row you want to make your move.
After you do either the next player will have to input their move or the ai will calculate it's next move.
The ai will then spit out a move and a random number called: Best value. This value is basically the chances it thinks that it's going to win.<br>
&emsp;If it's playing as X, the higher the better.<br>
&emsp;If it's playing as O, the lower the better.

## Further Reading
[Geeks for Geeks](https://www.geeksforgeeks.org/minimax-algorithm-in-game-theory-set-1-introduction/)<br>
[Sebastian Lague](https://www.youtube.com/watch?v=l-hh51ncgDI&ab_channel=SebastianLague)<br>
[The Coding Train](https://www.youtube.com/watch?v=trKjYdBASyQ&ab_channel=TheCodingTrain)<br>
