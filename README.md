# Rust Hello Quad

A simple Rust application that renders a textured quad with OpenGL using:

- gl
- glutin
- image

The image is loaded from the resources folder. Using "cargo run", the environment path is the project directory. In order for the executable to have access to the resources, the folder "resources" must be copied to the executable directory, either manually or by a custom made build task.

### VsCode

As the project was built using VsCode, a folder '.vscode' with a 'tasks.json' file is included for easy build and debug run.