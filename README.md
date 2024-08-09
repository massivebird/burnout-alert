# Burnout alert sim

Simulates the cancellable alerts of Burnout Revenge (PS2, 2005).

Written in Rust 🦀

## How it works

Messages are stored in a queue and are processed in FIFO order. Messages are animated on the screen, "fading in" and "fading out" one character at a time. The fade out animation is _cancellable_ and is skipped if there is a message waiting to be processed.
