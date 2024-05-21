<!-- Warning: This file is automatically copied from docs/user-manual/changelog.md into editors/vscode/CHANGELOG.md when changes are made to it! -->

# Changelog


## v0.3.0-dev.3

Almost feature complete "go to definition" & hover tooltips

### Features
- Expanded "Go to definition" to almost all code symbols including expressions (only missing handling of state base types) [#13](https://github.com/SpontanCombust/witcherscript-ide/issues/13)
- Added "Go to declaration" pointing to the first declaration of the function in the inheritance tree [#13](https://github.com/SpontanCombust/witcherscript-ide/issues/13)
- Added hover feature displaying code symbol information [#7](https://github.com/SpontanCombust/witcherscript-ide/issues/7)
- Made `{unknown}` the default text to display for unknown code symbols
- Added native content directory, which contains all symbols available in WitcherScript, but not explicitly declared. This directory is shipped together with the Language Server.
- Diagnostics are now published together with an identifier, e.g. `symbol-name-taken`

### Fixes
- Fixed code text not synchronizing properly when saving a file
- Lessened the chance of deadlocking by freeing resources earlier



## v0.3.0-dev.2

### Features
- Added go to definition feature, for now working for type identifiers and special vars (`this`, `super` in most cases, `parent`, `virtual_parent`). [#13](https://github.com/SpontanCombust/witcherscript-ide/issues/13)

### Commands
- Added `witcherscript-ide.debug.clearGlobalState` debug command, which clears global persistant data saved by the VSCode extension. Useful for testing by developers.

### Configuration
- Added "Enable debug features" setting, which prevents debug features such as commands from being available if not enabled. The default value is false. [#25](https://github.com/SpontanCombust/witcherscript-ide/issues/25)

### Other
- Trying to access more than very basic features such as go to definition outside of a script project should now result in showing a warning message explaining as to why that can't be done. [#33](https://github.com/SpontanCombust/witcherscript-ide/issues/33)


## v0.3.0-dev.1
This is the first pre-release of the version that will finally bring "Go to" code features to the extension.
If you find any criticial bugs make sure to submit an issue. Thank you!

### Features
- Added code symbol scanning. This enables the IDE to know about all the types, functions and data declared in workspace scripts. The use of it is limited for now [#9](https://github.com/SpontanCombust/witcherscript-ide/issues/9)
- Added Document Symbols feature [#26](https://github.com/SpontanCombust/witcherscript-ide/issues/26)
- Added Selection Range feature [#27](https://github.com/SpontanCombust/witcherscript-ide/issues/27)
- Added `witcherscript.toml` schema, which can be used by [`Even Better TOML`](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) extension if you have it installed [#16](https://github.com/SpontanCombust/witcherscript-ide/issues/16)
- Some diagnostics now have linked information to them, e.g. diagnostic for multiple definitions will point to the original declaration

### Commands
- Added debug command "Show script symbols" to get a view of all symbols coming from current script file

### Other
- Improved AST traversal and text retrieval performance through better memory management
- Lessened the chance of code deadlock for diagnostics by using HashMap behind a Mutex instead of DashMap


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