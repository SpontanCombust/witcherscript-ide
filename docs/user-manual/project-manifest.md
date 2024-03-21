# Script Project Manifest

In order for the IDE to know all the important information needed to analyze scripts a project needs a manifest file.
This file is written in TOML format and by convention is called `witcherscript.toml`.

Beware: format of the manifest may change in the future. Look out for breaking changes section in the changelog.

## Format:
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
This table contains key-value pairs of dependency specifications. The kay is always the name of the foreign content. The value can take multiple forms:

- boolean `true`/`false` - means the content should be looked for based upon its name. If the value is `false` the dependency will be ignored.
- `{ path = "path/to/content" }` - content should be looked for in a specific directory on disk. The path can be either relative or absolute.


## Example
Contents of an exemplary `witcherscript.toml` manifest:

```toml
[content]
name = "modSuperSpeed"
version = "1.0.0"
authors = [ 'Yours truly' ]
game_version = "4.04"

[dependencies]
content0 = true # added by default
modMovement = { path = "../modMovement" }
```

Note that you do not have to create a manifest file by hand. You can use one of the "create" commands in the editor.