<!-- Warning: This file is automatically copied from docs/user-manual/changelog.md into editors/vscode/CHANGELOG.md when changes are made to it! -->

# Changelog


## v0.3.2
**Hotfix**

### Features
- Added support for XOR binary operator
- Added support for BITAND assignment operator 
- Added support for BITOR assignment operator 
- Removed support for modulo assignment operator as it is not recognized by the compiler


## v0.3.1
**Annotation support**

### Features
- Added support for code annotations introduced Witcher 3 update `4.04a_REDkit_Update_2` on 06.06.2024
- Added more descriptive errors when adding improper specifiers in code (e.g. access modifier for struct fields)

### Fixes
- Established a specific order in which configuration settings get shown. They should no longer be listed in seemingly random order
- New projects now should get automatically detected when they get created inside the workspace
- Improved syntax highlighting of tooltips involving primitive types, special variables (e.g. `this`) and global variables (e.g. `thePlayer`)
- Fixed not being able to use "Create/Initialize project" commands when not already being in a workspace with existing script project(s)

### Commands
- Added a debug command to inspect Concrete Syntax Tree of a script

### Other changes
- Message shown when trying to access language features from the level of a non-project script will now direct to the main "User manual" page instead of "Project System" subpage
- Reorganized LSP server and client code for readability


## v0.3.0
**Code symbols update**

With this release we also establish an acronym for the project, that being ***WIDE*** (**W**itcherScript **I**ntegrated **D**evelopment **E**nvironment). 
It is also finally getting some visual branding!

### Features
- Added Go to definition/declaration feature [#13](https://github.com/SpontanCombust/witcherscript-ide/issues/13)
- Added Hover tooltips feature [#7](https://github.com/SpontanCombust/witcherscript-ide/issues/7)
- Added Document Symbols feature [#26](https://github.com/SpontanCombust/witcherscript-ide/issues/26)
- Added Selection Range feature [#27](https://github.com/SpontanCombust/witcherscript-ide/issues/27)
- Added `witcherscript.toml` schema, which can be used by [`Even Better TOML`](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) extension if you have it installed [#16](https://github.com/SpontanCombust/witcherscript-ide/issues/16)
- Added multiple definition detection
- Added unique identifiers to diagnostics, which point to the documentation

### Fixes
- Fixed code text not synchronizing properly when saving a file
- Fixed unusual line endings in some vanilla script files causing parsing errors [#31](https://github.com/SpontanCombust/witcherscript-ide/issues/31)
- Fixed diagnostic for invalid project dependency path not being displayed
- Fixed scripts not getting updated if their version was rolled back (modified date was made older than the previous date)
- Fixed change to the script root path in the manifest file not getting detected and handled
- Disallowed contents to specify themselves as their own dependency
- Lessened the chance of the server getting deadlocked

### Commands
- Added debug command "Show script symbols" to get a view of all symbols coming from current script file
- Added "Clear global cache" debug command, which clears global persistant data saved by the VSCode extension. Useful for testing by developers

### Configuration
- Added "Enable debug features" setting, which prevents debug features such as commands from being available if not enabled. By default this is disabled. [#25](https://github.com/SpontanCombust/witcherscript-ide/issues/25)
- Added a setting to enable or disable the language server, which allows you to disable everything except syntax highlighting if you want to [#38](https://github.com/SpontanCombust/witcherscript-ide/issues/38)
- Added a setting to enable or disable syntax analysis in the language server
  
### Other changes
- Trying to access more than very basic features such as go to definition outside of a script project should now result in showing a warning message explaining as to why that can't be done. [#33](https://github.com/SpontanCombust/witcherscript-ide/issues/33)
- Added more possible automatic `scripts_root` subdirectory candidates for new projects [#35](https://github.com/SpontanCombust/witcherscript-ide/issues/35)
- Improved UX by explicitly displaying which window is vanilla and which is for the mod in script diff view
- Improved UX by automatically reacting to newly created, removed or renamed script files, some cases still require explicit file save to refresh though [#39](https://github.com/SpontanCombust/witcherscript-ide/issues/39)
- Added native content directory, which contains all symbols available in WitcherScript, but not explicitly declared. This directory is shipped together with the Language Server
- Improved AST traversal and text retrieval performance through better memory management
- Added issue templates to the repository

### Documentation
- Added "Diagnostic Index" page detailing all diagnostics that can be appear in the editor [#36](https://github.com/SpontanCombust/witcherscript-ide/issues/36)
- Added FAQ page
- Added more demo media showing WIDE's capabilities


## v0.2.1
**REDKit project support & fixes**

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
**Project system update**

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
**Initial release**

### Editor
- Added syntax highlighting
- Added basic syntactical analysis