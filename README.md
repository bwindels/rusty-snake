start with hardest part in c, polling the keyboard with a timeout.
Then port this to rust and build a snake game around it.

## C prototype

- [x] write keyboard polling code using kqueue
- [x] make kqueue timeout work properly
- [x] measure elapsed time of kevent call to support a timer
- [x] set terminal in raw mode to prevent stdin buffering
- [x] write code to position cursor

## Rust application

port c prototype to rust, and start porting snake-js code on top of this, using mainly stack allocation.
