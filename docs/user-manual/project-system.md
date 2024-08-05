# Project System

The game during script compilation needs to think only about assembling one big blob of code that will be then compiled. Places where it looks for said code are predefined and limited. Aside from the testing stage, developing a script mod often means working in a completely unrelated workspace directory, which can additionally be stored remotely using a version control system like Git. WIDE bridges the gap between those workspaces and scripts in the game directory by introducing a project system. 

The project system does not affect the process of packaging and installing mods to the game. For now that is left to the developer to figure out for themselves, this includes any dependencies that the mod may need. You can either write your own batch scripts or use REDkit built-in packaging functionality.

Projects organize WitcherScript code into seperable packages which can link with each other.
Recognized content structures are:

 - [Redkit projects](#redkit-project)
 - [WIDE projects](#wide-project)
 - [Raw content directories](#raw-content-directory)


## REDKit project

WitcherScript IDE is able to detect projects created using the REDKit modding tool. These projects contain a `.w3edit`, which acts as a solution file for the whole project.
Working with REDKit projects requires you to set the path to the game in extension's settings.

REDKit project naturally can't use any scripts that are not part of it or the vanilla game (*content0*) unless you manually edit the depot. Until REDkit will be able to support script dependencies more easily if you want to get code suggestions from other mods consider initializing [WIDE project](#wide-project) in the workspace directory and filling the `[dependencies]` table.

:material-information-outline: If both `*.w3edit` and `witcherscript.toml` are present in the directory, it will be treated as a WIDE project.


## WIDE Project

The WitcherScript project format that WIDE establishes is comprised of two things: a [manifest file](#manifest-format) and a scripts directory. 
The manifest is a TOML file by convention called `witcherscript.toml`. It contains basic information about the project like its name and what are its dependencies. 
The scripts directory is a subfolder literally called *"scripts"*, which contains all of project's WitcherScript files. The location of that folder can be configured in the manifest.

Creating a manifest for your script mod is mandatory if you want to use more advanced code features like go to definition. Without a manifest you are limited to syntax highlighting and basic syntax analysis.

:material-information-outline: To quickly create a new project or initialize one in an existing project directory use *`"Initialize/Create WitcherScript project..."`* commands from the dashboard view.


Example of a simple WIDE project setup:

```text
SuperSpeedMod/
├─ witcherscript.toml
├─ scripts/
    ├─ game/
    │  ├─ r4Player.ws
    ├─ local/
        ├─ super_speed.ws
```

```toml title="SuperSpeedMod/witcherscript.toml"
[content]
name = "modSuperSpeed"
version = "1.0.0"
authors = [ 'Yours truly' ]
game_version = "4.04"

[dependencies]
content0 = true # added by default
modMovement = { path = "../MovementMod" }
```


## Manifest format

:material-alert-outline: ***Beware***: *format of the manifest may change in the future. Look out for breaking changes section in the changelog.*

Every WIDE project manifest is composed of following sections:

- [content](#the-content-table) *:
    - [name](#the-name-field) *
    - [description](#the-description-field)
    - [version](#the-version-field) *
    - [game_version](#the-game_version-field) *
    - [authors](#the-authors-field)
    - [scripts_root](#the-scripts_root-field)
- [dependencies](#the-dependencies-table) *

\* table/field is required

:material-information-outline: If you have [`Even Better TOML`](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) extension installed it will provide you with auto-completion and hints when editing the manifest file.


### The `[content]` table
Project metadata establishing basic information on how its content is named and structured.

#### The `name` field
Name of the project. It must begin with an ASCII letter or underscore and contain only alphanumerical ASCII characters or underscores. Spaces are not allowed.

#### The `description` field
Short description of the project. This field is optional.

#### The `version` field
Version of the project. It must follow [semantic versioning](https://semver.org/) format.

#### The `game_version` field
Version of Witcher 3 with which this project is compatible.
The field does not require any specific format at the moment, but it may change in the future.

#### The `authors` field
An array of authors of this project. Their form can be completely arbitrary.
This field is optional.

#### The `scripts_root` field
Relative path to the scripts root directory. This field is optional and defaults to `"./scripts"`.


### The `[dependencies]` table
This table contains key-value pairs of dependency specifications. It may be left empty, but it's still required.

Format of allowed values is as follows: 

```toml
dependency_name = value
```

The key `dependency_name` specifies the name of another content. This name corresponds to the `name` field in the `[content]` table if the dependency is a project or parent directory name if it's a raw content.

`value` specifies where the content should be looked for. Accepted forms are:

- boolean `true`/`false` - content should be searched for in [repositories](#content-repositories) based upon its name. If the value is `false` the dependency will be ignored. Example:
```toml
repoDependency = true
```

- inline table `{ path = "path/to/content" }` - content should be looked for in a specific directory. The path can be absolute or relative to manifest's directory, but it is advised to use the latter. Name of the content pointed to by the path must also match with name of the dependency written before that. Example:
```toml
pathDependency = { path = "../dependencies/pathDependency" }
``` 


## Raw content directory

"Raw content" is the term by which WIDE refers to directories containing game files, among which are scripts residing in *"scripts"* directory.
Examples of such directories in Witcher&nbsp;3's game directory include *"content0"*, which contains all vanilla scripts, and any mods in the *"Mods"* directory that contain WitcherScript.

:material-information-outline: In order for raw content directories to be found by WIDE you need to configure [content repositories](#content-repositories).

Recognized folder patterns are:

1. *"scripts"* directly inside the folder (example: *"content0"*)
```text
{content_name}/
├─ scripts/
   ├─ **/*.ws
```

2. *"scripts"* inside an intermediary *"content"* directory (examples: script mod in the *"Mods"* directory)
```text
{content_name}/
├─ content/
   ├─ scripts/
      ├─ **/*.ws
```

Name of raw content is derived from the name of its root directory.

Project directories can be thought of as an extension of a raw content directories, as alongside scripts they also includes the manifest. 
Raw content is recognized by WIDE purely for user convenience to not force anyone to needlesly create manifest files for directories, whose identity can be recognized from the context of their placement in the file system (case in point, again, the *"content0"* directory).


## Content repositories

"Content repositories" are directories that directly inside them contain raw or project content directories. Commonly used repositories are *"Witcher 3/content"* and *"Witcher 3/Mods"*. Repositories can be configured via [`witcherscript-ide.gameDirectory`](./editor.md#witcherscript-idegamedirectory) and [`witcherscript-ide.contentRepositories`](./editor.md#witcherscript-idecontentrepositories) settings in editor.


## Vanilla content dependency

When setting vanilla scripts as a dependency for your project make sure that the name of the content they are found in is called exactly "content0". WIDE distinguishes this way the vanilla content from any other modded content. If you configure it to be found in your game directory with `witcherscript-ide.gameDirectory` setting and don't modify it in any way, it should be valid without a need for any further actions.  
If you however store your vanilla scripts in a seperate folder make sure that either that folder is called "content0" or create a manifest for it with the following content inside it:
```toml
[content]
name = "content0"
# version etc...

[dependencies]
# this table should be empty
```