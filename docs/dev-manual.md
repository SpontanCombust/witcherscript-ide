# Developer manual

Here you will find the information you need if you want to contribute to this project's creation or just want to compile the project yourself.


## Project structure
- `.vscode` - VSCode specific files with debugging configurations 
- `crates` - server Rust code. The main crate is `lsp`, which contains language server implementation
- `docs` - project documentation from which this website is built
- `editors` - contains implementations of WitcherScript language client, currently just `vscode` client written in TypeScript
- `xtask` - code for build commands to speed up development


## Building the project
You will need [Rust with Cargo](https://rustup.rs/) to build the server and [node.js with npm](https://nodejs.org/en) to build the client.

Project utilises the `xtask` convention of writing build scripts in Rust. To call a build script simply run `cargo xtask {command}` in the root project directory. You can run `cargo xtask --help` to see all the available commands and what they do.

Currently available xtask commands:

- `prep-server` - build and copy LSP server executable into VSCode's extension directory
    - `--release` - should LSP be built with optimised release profile
    - `--target` - compilation target triple
- `prep-client` - build VSCode client
    - `--watch` - whether client should be continuously watched for changes made to it and rebuilt 
- `package` - build and package VSCode extension into a .vsix file
    - `--out-dir` - output directory for the .vsix file; default is the current working directory
    - `--out-name` - name of the output file without the extension; default is "witcherscript-ide"
- `install` - build, package and install the VSCode extension


## Debugging
Use VSCode to debug the client and server. The project provides launch configurations for both of them:

- `Launch Client` - launches the extension host session of the client. Client needs to be built first!
- `Attach to Server` - attaches to the currently running server process created by the client. You need the [CodeLLDB extension](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) for this.


## The project board
You can access the [project board](https://github.com/users/SpontanCombust/projects/2/views/1) conveniently detailing what has been done and future plans.


## Contributing
Contributions to the project are welcome. First create an issue or a PR to discuss the change you want to make and ensure that it doesn't conflict with any future plans or core features. The active development branch is `dev`.

You can also catch me in the [Wolven Workshop](https://discord.gg/S3HjaY65uh) Discord server.