<!-- Warning: This file is automatically copied from docs/user-manual/changelog.md into editors/vscode/CHANGELOG.md when changes are made to it! -->

# Changelog


## v0.2.0
Project system update

### Editor
- Added a project system for WitcherScript, check the user manual for details
- Added workspace-wide analysis thanks to the project system
- Improved syntax highlighting 
- Fixed syntactical analysis for more obscure grammar. The grammar used should now be 100% compatible with vanilla code

### Commands
- Added a command to initialize a WitcherScript project in an existing directory
- Added a command to create a WitcherScript project in a new directory
- Added a command to import a vanilla script into the project
- Added a command to compare a script with the original vanilla counterpart
- (Debug) Added a command to inspect the AST of a script file
- (Debug) Added a command to inspect the dependency graph of projects in the workspace 

### Configuration
- Added `witcherscript-ide.gameDirectory` setting
- Added `witcherscript-ide.contentRepositories` setting


## v0.1.0
Initial release

### Editor
- Added syntax highlighting
- Added basic syntactical analysis