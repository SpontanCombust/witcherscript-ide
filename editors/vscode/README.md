# WitcherScript

VSCode client for WitcherScript language server.

<!--TODO add a video demonstration-->

## Quick Start
1. Configure paths to existing vanilla and mod scripts by going to settings (File > Preferences > Settings). Start typeing "witcherscript" and look for "Game Directory" and "Project Repositories" settings.
2. Open the command prompt using Ctrl + Shift + P and start typeing "witcherscript". Choose a command to create a WitcherScript project.


## Features
- syntax highlighting
- basic syntactical analysis
- importing and comparing scripts with their vanilla counterparts


## Configuration

**witcherscript-ide.gameDirectory** <br>
Path to Witcher 3's root game directory (containing bin, content, Mods folders etc.). This will effectively add `content` and `Mods` folders to content repositories.

**witcherscript-ide.contentRepositories** <br>
Paths to custom directories containing WitcherScript contents. Useful when not having Witcher 3 installed on your local machine or when simply storing scripts outside of game directory.


## Known Issues
This extension is meant to replace [vscode-witcherscript](https://marketplace.visualstudio.com/items?itemName=nicollasricas.vscode-witcherscript).
It is necessary to uninstall that extension first if you have it installed.

<!--
## Requirements
None at the moment.

## Breaking release Notes

-->

---
