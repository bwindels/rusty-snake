## Remaining tasks

 - [x] write and pass tests for Snake struct
 - [x] write game loop in SnakeApp struct
 - [ ] write and pass tests for SnakeGame struct
 - [x] make Game trait that determines polling timeout and can handle a keypress
 - [x] implement partial redrawing with only changed parts without clearing the screen
 - [x] turn SnakeApp into GameLoop, that talks to instance of Game trait
 - [x] implement SnakeGame::new_apple using rand library
 - [ ] draw head differently so it stays clear which way the snakes moves if it's head is next to it's body
 - [ ] write and pass tests for Field struct
 - [ ] add padding support to (Term)Screen::draw_text()
 - [ ] write and pass tetst for TermScreen
 - [x] write score counting
 - [x] write snake moving faster as score goes up
 - [ ] port input and timer to linux
