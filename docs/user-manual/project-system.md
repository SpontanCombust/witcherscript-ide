# Project System

The game during script compilation needs to think only about assembling one big blob of code that will be then compiled. Places where it looks for said code are predefined and limited. Aside from the testing stage, developing a script mod often means working in a completely unrelated workspace directory, which can additionally be stored remotely using a version control system like Git. WitcherScript IDE bridges the gap between those workspaces and scripts in the game directory by introducing a project system.

Projects organize WitcherScript code into seperable packages which can link with each other.
Recognized content structures are:

 - [`witcherscript.toml` projects](#project)
 - [Redkit projects](#redkit-project)
 - [raw content directories](#raw-content-directory)


## Project

A WitcherScript project that WitcherScript IDE establishes is comprised of two things: a [manifest](#manifest-format) and a scripts directory. 
The manifest is a TOML file by convention called `witcherscript.toml`. It contains basic information about the project like its name and what are its dependencies. 
The scripts directory is a subfolder literally called *"scripts"*, which contains all of project's WitcherScript files. The location of that folder can be configured in the manifest.

Creating a manifest for your script mod is mandatory if you want to use more advanced code features like go to definition (available soon<!--TODO remove when ready-->). Without a manifest you are limited to syntax highlighting and syntax analysis.


## Manifest format

:exclamation: ***Beware***: *format of the manifest may change in the future. Look out for breaking changes section in the changelog.*

Every manifest is composed of following sections:

- [content](#the-content-table) *:
    - [name](#the-name-field) *
    - [version](#the-version-field) *
    - [game_version](#the-game_version-field) *
    - [authors](#the-authors-field)
    - [scripts_root](#the-scripts_root-field)
- [dependencies](#the-dependencies-table)

\* table/field is required

### The `[content]` table
Project metadata establishing basic information on how its content is named and structured.

#### The `name` field
Name of the project. It must begin with an ASCII letter or underscore and contain only alphanumerical ASCII characters or underscores. Spaces are not allowed.

#### The `version` field
Version of the project. It must follow [semantic versioning](https://semver.org/) format.

#### The `game_version` field
Version of Witcher 3 with which this project is compatible. It can be a range of versions.
The field does not require any specific format at the moment, but it will change in the future.

#### The `authors` field
An array of authors of this project. Their form can be completely arbitrary.
This field is optional.

#### The `scripts_root` field
Relative path to the scripts root directory. This field is optional and defaults to `"./scripts"`.


### The `[dependencies]` table
This table contains key-value pairs of dependency specifications like so: 

```toml
dependency_name = value
```

The key `dependency_name` specifies the name of the foreign content. This name corresponds to the `name` field in the `[content]` table if the dependency is a project or parent directory if its a raw content.

`value` specifies where the content should be looked for. It appears in multiple forms:

- boolean `true`/`false` - content should be searched for in [repositories](#content-repositories) based upon its name. If the value is `false` the dependency will be ignored. Example:
```toml
repoDependency = true
```

- inline table `{ path = "path/to/content" }` - content should be looked for in a specific directory. It is advised to use a relative path. Name of the content pointed to by the path must also match with name of the dependency written before that. Example:
```toml
specificDependency = { path = "../dependencies/specificDependency" }
``` 


## Project example

Example of a simple script project structure:

```text
SuperSpeedMod/
├─ witcherscript.toml
├─ scripts/
    ├─ game/
    │  ├─ r4Player.ws
    ├─ local/
        ├─ super_speed.ws
```

```toml
# SuperSpeedMod/witcherscript.toml

[content]
name = "modSuperSpeed"
version = "1.0.0"
authors = [ 'Yours truly' ]
game_version = "4.04"

[dependencies]
content0 = true # added by default
modMovement = { path = "../MovementMod" }
```

Note that you do not have to create a manifest or even an entire project by hand. You can use *`"Initialize/Create WitcherScript project..."`* commands in the editor to either create a manifest in already existing directory or create an entirely new project directory.



## REDKit project

WitcherScript IDE is able to detect projects created using the REDKit modding tool. These projects contain a `.w3edit`, which acts as a solution file for the whole project.
Working with REDKit projects still requires to set the path to the game in extension's settings.

REDKit project naturally can't use any scripts that are not part of it or the vanilla game (*content0*). If you want to use other mods as dependencies for the project consider creating a `witcherscript.toml` manifest and filling the `[dependencies]` table instead.


## Raw content directory

"Raw content" is the term by which the IDE refers to directories containing game files, among which are scripts residing in *"scripts"* directory.
Examples of such directories in Witcher&nbsp;3's game directory include *"content0"*, which contains all vanilla scripts, and any mods in the *"Mods"* directory that contain WitcherScript.

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

Project directory can be seen as an extension of a content directory, as alongside scripts it also includes the manifest. 
Raw content recognized by the IDE purely for user convenience to not force anyone to needlesly create manifest files for directories, whose identity can be recognized from the context of their placement in the file system (case in point, again, the *"content0"* directory).

Name of raw content is derived from the name of its root directory.



## Content repositories

"Content repositories" are directories that can contain script contents, either raw or projects. Commonly used repositories are *"Witcher 3/content"* and *"Witcher 3/Mods"*. Repositories can be configured via editor's settings.