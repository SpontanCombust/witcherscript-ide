<!-- Warning: This file is automatically copied from docs/user-manual/changelog.md into editors/vscode/CHANGELOG.md when changes are made to it! -->

# Changelog


## v0.2.1
REDKit project support & fixes

### Features
- Added support for REDKit projects

### Fixes
- Fixed script analysis not being reloaded when file was saved
- Fixed duplicated syntax errors for code inside functions
- Fixed `defaults` blocks not having syntax analysis
- Fixed diagnostics not displaying immediately when a manifest file was changed and saved
- Fixed content not being detected if it resided in the root of a repository directory

### Documentation
- Moved copyright information directly into the "About" page
- Made a dedicated page for "Getting started" which is available from the main page of user manual
- Updated "Project System" with REDKit project information

### Other
- Importing a script file now automatically opens it in the editor
- Opening a new/unknown manifest file now doesn't trigger content graph rebuild and a manual file save action is required to trigger it
- More descriptive errors diagnostics when linking content dependencies
- The extension now also gets published to Eclipse's Open VSX Registry at   
<https://open-vsx.org/extension/SpontanCombust/witcherscript-ide>


## v0.2.0
Project system update

### Features
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