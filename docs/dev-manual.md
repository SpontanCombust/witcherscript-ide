# Developer manual

Here you will find the information you need if you want to contribute to this project's creation or just want to compile the project yourself.


## Project structure
- `.cargo` - Cargo configuration to enable xtasks
- `.vscode` - VSCode specific files with debugging configurations 
- `crates` - Rust packages forming WIDE. The main crate is `lsp`, which contains language server implementation
- `docs` - project documentation from which this website is built
- `editors` - contains implementations of WitcherScript language client, currently just `vscode` client written in TypeScript
- `media` - all sorts of visual assets used throughout the repository
- `schemas` - contains `witcherscript.toml` schema
- `xtask` - code for build commands to speed up development


## Building the project
You will need [Rust with Cargo](https://rustup.rs/) to build the server and [node.js with npm](https://nodejs.org/en) to build the client.

Project utilises the `xtask` convention of writing build scripts in Rust. To call a build script simply run `cargo xtask {command}` in the root project directory. You can run `cargo xtask --help` to see all the available commands and what they do.

Currently available xtask commands:

- `prep-server` - build and copy LSP server executable into VSCode's extension directory
    - `--release` - should LSP be built with optimised release profile
    - `--target` - compilation target triple, e.g. x86_64-pc-windows-msvc
- `prep-client` - build VSCode client
    - `--watch` - whether client should be continuously watched for changes made to it and rebuilt 
    - `--fast` - whether client should be built instantly by skipping `npm ci` step
- `package` - build and package VSCode extension into a .vsix file
    - `--out` - output path for the .vsix file; default is "./witcherscript-ide.vsix"
    - `--target` - VSCode extension target, e.g. win32-x64
    - `--pre-release` - mark the package as pre-release
- `install` - build, package and install the VSCode extension locally


## Debugging
Use VSCode to debug the client and server. The project provides launch configurations for both of them:

- `Launch Client` - launches the extension host session of the client. Client needs to be built first!
- `Attach to Server` - attaches to the currently running server process created by the client. You need the [CodeLLDB extension](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) for this.


## Building docs
Documentation of the project is generated using MKDocs. To locally serve and test the website:
1. Make sure you have [python3 with pip](https://www.python.org/downloads/) installed
2. Install MKDocs and dependencies
```sh
pip install mkdocs
pip install mkdocs-material
```
3. Serve the website
```sh
mkdocs serve
```

To learn more about MKDocs check out their website at <https://www.mkdocs.org/>.


## Contributing
Contributions to the project are welcome. First create an issue or a PR to discuss the change you want to make and ensure that it doesn't conflict with any future plans or core features. The active development branch is `dev`.

You can access the [project board](https://github.com/users/SpontanCombust/projects/2/views/1) conveniently detailing what has been done and future plans.
The board does not contain everything though and some issues might just be annotated with TODO or FIXME comments. For this I highly recommend the [Todo Tree](https://marketplace.visualstudio.com/items?itemName=Gruntfuggly.todo-tree) extension.