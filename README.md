# Rust Hello Quad

A simple [Rust](https://www.rust-lang.org) application that renders a textured quad with OpenGL using:

- gl
- glutin
- image

![demo](https://github.com/peerhenry/rust_hello_quad/blob/master/Capture.PNG)

The image is loaded from the resources folder. Using "cargo run", the environment path is the project directory. In order for the executable to have access to the resources, the folder "resources" must be copied to the executable directory, either manually or by a custom made build task.

## Instructions

Run the program using `cargo run`. You can look around by moving the mouse (you may need to move it around a bit to find the quad).

Controls:
* W: forward
* S: backward
* A: left
* D: right