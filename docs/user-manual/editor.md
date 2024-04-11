# Editor

## Quick Start

1. Install the extension from the [marketplace](https://marketplace.visualstudio.com/items?itemName=SpontanCombust.witcherscript-ide) or grab a .vsix file for your OS from the [releases](https://github.com/SpontanCombust/witcherscript-ide/releases) page.
2. Configure paths to existing vanilla and mod scripts by going to settings (File > Preferences > Settings). Start typeing "witcherscript" and look for "Game Directory" and "Content Repositories" settings.
3. Open the command prompt using Ctrl + Shift + P and start typeing "witcherscript". Choose a command to create or initialize a WitcherScript project.

<!--TODO add a video demonstration-->


## Features

- syntax highlighting
- basic syntactical analysis
- initializing or creating new script projects
- importing and comparing scripts with their vanilla counterparts


## Commands

### `witcherscript-ide.projects.init`
*"Initialize a WitcherScript project in existing directory..."*  
Will establish a basic file structure of a WitcherScript project in a given existing directory. Initial name of the project is picked based upon the directory name.

### `witcherscript-ide.projects.create` 
*"Create a new WitcherScript project..."*  
Will create a new directory and establish a basic file structure of a WitcherScript project inside it.

### `witcherscript-ide.scripts.importVanilla`
*"Import scripts from vanilla..."*  
Invokes a file chooser to pick vanilla scripts you want to import into your project. If more than one WitcherScript project exists in the workspace, prompts the user to choose the project.  
Command requires that the `content0` content is known and is a dependency to a given project.

### `witcherscript-ide.scripts.diffVanilla`
*"Compare this script with vanilla counterpart"*  
Shows a difference view between the original vanilla script and the imported, modified script in the project.

### `witcherscript-ide.debug.showScriptAst`
*"Show script AST"*  
A debugging command. Shows the Abstract Syntax Tree  of the currently focused on script as it is uderstood by the language server.

### `witcherscript-ide.debug.contentGraphDot`
*"Show content graph"*  
A debugging command. Shows the graph in Graphviz .dot format representing the overall content dependency graph of the workspace.


## Configuration

### `witcherscript-ide.gameDirectory`
Path to Witcher 3's root game directory (containing bin, content, Mods folders etc.). This will effectively add `content` and `Mods` folders to content repositories.

### `witcherscript-ide.contentRepositories`
Paths to custom directories containing WitcherScript contents. Useful when not having Witcher 3 installed on your local machine or when simply storing scripts outside of game directory.
