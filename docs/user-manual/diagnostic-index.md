<!-- Beware!!! WIDE's diagnostic code descriptions link to this page. -->
<!-- Make sure that chapters for each diagnostic have correct name. -->


# Diagnostic index


## **Project System**

---

### `invalid-project-manifest`

Project manifest (`witcherscript.toml`) could not be parsed due to syntax error or missing properties.

```toml title="witcherscript.toml" hl_lines="5"
[content] 
version = "1.0.0"
authors = []
game_version = "4.04"
# (1)

[dependencies]
```

1. Missing field "name" in table \[content].


---

### `invalid-project-name`

The "name" field in `witcherscript.toml` manifest file is incorrect. The name must follow a [specific format](project-system.md/#the-name-field).

```toml title="witcherscript.toml" hl_lines="2"
[content]
name = "modCośWięcej" # (1)
```

1. Should not contain non-English characters. Use "modCosWiecej" instead.


---

### `invalid-redkit-project-manifest`

REDkit project's `.w3edit` file could not be parsed.
This could happen if you manually edited the `.w3edit` file that is created for a REDkit project. This file is edited automatically by the REDkit when needed and you shouldn't edit it yourself.

```json title="exampleMod.w3edit" hl_lines="2"
{ 
    "name": "exampleMod" // (1)
    "version": "1.0.0",
    "gameVersion": "4.04",
```

1. Syntax error, missing ",".


---

### `project-dependency-path-not-found`

Dependency in the `witcherscript.toml` manifest file could not be found at a specified path. This can happen if either a) the path does not exist or b) the path exists, but there is not script content there.

```toml title="C:\Modding\modExample\witcherscript.toml" hl_lines="2"
[dependencies]
modSharedUtils = { path = "../modSharedUtils" } # (1)
```

1. Path "C:\Modding\modSharedUtils" does not exist or does not contain any script content.


---

### `project-dependency-name-not-found`

Dependency in the `witcherscript.toml` manifest file could not be found in any of the [repositories](project-system.md#content-repositories). Make sure that the name of the dependency is correct. It should correspond to the name of the project or name of the directory if it's raw content.

```toml title="witcherscript.toml" hl_lines="2"
[dependencies]
modSharedUtils = true # (1)
```

1. No content with name "modSharedUtils" could be found in any of the repositories.


---

### `project-dependency-name-not-found-at-path`

Dependency in the `witcherscript.toml` manifest file was found at a given path, but the name key does not match with the "name" field in dependency's manifest.

```toml title="witcherscript.toml" hl_lines="2"
[dependencies]
mod_shared_utils = { path = "../modSharedUtils" } # (1)
```

1. Expected for example `#!toml modSharedUtils = { path = "../modSharedUtils" }`


---

### `project-self-dependency`

You've made content point to itself as its own dependency. Make sure to specify a correct path if it's a path dependency or remove the entry entirely if it's a repository dependency.

```toml title="witcherscript.toml" hl_lines="5 6"
[content]
name = "helloWorld"

[dependencies]
helloWorld = { path = "." } # (1)
helloWorld = true # (2)
```

1. A path self-dependency
2. This is an error if content itself is inside a repository

---

### `multiple-matching-project-dependencies`

A repository dependency was found, but in multiple places. WIDE has no idea which one to choose. This can happen if you have added multiple repository paths in the configuration that share script content with the same name.  
A good example would be having two `content0` repository paths configured: one from game installation and other is from the 1.21 version of the game with commented code.

```toml title="witcherscript.toml" hl_lines="2"
[dependencies]
content0 = true # (1)
```

1. "content0" was found in game installation and some other, manually configured repository.


---



</br>

## **Syntax Analysis**

---

### `missing-syntax`

Some element of the WitcherScript syntax was missing.

```ts linenums="1" hl_lines="2"
latent function testLatent() {
    while () { // (1)
        Sleep(1);
        break;
    }
}
```

1. Missing expression for `while`'s condition


---

### `invalid-syntax`

Diagnostic used for all other syntax error cases. Syntactical analysis is very basic at the moment and can't communicate more complex cases. 


---



</br>

## **Contextual Syntax Analysis**

---

### `incompatible-specifier`

Specifiers are keywords that tell the WitcherScript compiler to give a code symbol some additional properties.
Different kinds of symbols can only take a predefined set of specifiers. A state for example cannot be at the same time a statemachine and thus it won't accept a `statemachine` specifier.

```ts linenums="1" hl_lines="1"
statemachine state Idle in Monster { // (1)
    //...
}
```

1. A state cannot be marked with `statemachine`


---

### `incompatible-function-flavour`

Functions can additionally be marked with specifiers that you could call "flavours". They give them special attributes, like exposing them to the debug console if you add the `exec` keyword to the function. At most only one flavour can be specified. Some flavours can only be used in certain contexts, for example the aformentioned `exec` can only be used for global functions.

```ts linenums="1" hl_lines="4"
class CR4Player {
    //...

    public exec function LogHealth() { // (1)
        LogChannel('Health', IntToString(this.GetHealth()));
    }
}
```

1. `exec` cannot be used with a class method. Move the function to the global scope.


---

### `repeated-specifier`

Repeating the same specifier for one code symbol is not allowed.

```ts linenums="1" hl_lines="1"
public saved public var piesEaten: int; // (1) 
```

1. Repeated specifier `public` for field `piesEaten`.


---

### `multiple-access-modifiers`

Access modifiers are keywords that change the visibility of a field or method. This is a common feature in object oriented languages like WitcherScript. 
Available access modifiers are `private`, `protected` and `public`. Only one of them can be used in the declaration.

```ts linenums="1" hl_lines="1"
protected public function MakeDinner() { // (1) 
    // ...
} 
```

1. Can't use both `protected` and `public` access modifiers. Use only one of these two.

You can read more about access modifiers in programming languages [here](https://en.wikipedia.org/wiki/Access_modifiers).


---

### `invalid-annotation`

Detected a use of an unknown annotation. See the official WitcherScript guide for REDkit to know which annotations are avaialble.

```ts linenums="1" hl_lines="1"
@addFunction(CR4Player) // (1)
function MountDragon(force: bool) {
    // ...
}
```

1. Used unknown `@addFunction` annotation. Did you mean `@addMethod`?


---

### `invalid-annotation-placement`

Annotations can only be used in the global context. Using them inside classes for example is erroneous.

```ts linenums="1" hl_lines="2"
class Cannon {
    @addField(Ship) // (1)
    public var ammunition: int;
    // ...
}
```

1. Using annotations inside classes is invalid. Add the field outside of the class definition.


---

### `missing-annotation-argument`

Some annotations require an argument. For example the `@wrapMethod` annotation requires a type argument that will decide which class's method will be wrapped.

```ts linenums="1" hl_lines="1"
@wrapMethod // (1)
function OnSpawned() {
    // ...
}
```

1. Missing class name


---

### `incompatible-annotation`

Annotations expect a specific code fragment below.

```ts linenums="1" hl_lines="1"
@addField(CR4Player) // (1)
function SetE3Hairstyle() {
    // ...
}
```

1. `@addField` annotation expects a var declaration.


---

### `global-scope-var-decl`

WitcherScript does not support global variable declarations. The only context when it is valid is after the `@addField` annotation.

```ts linenums="1" hl_lines="10"
class Mod1 {
    // ...
}

class Mod2 {
    // ...
}


var modsInstalled: int; // (1)
```

1. Variable declaration not allowed here.


---

### `invalid-local-var-placement`

The WitcherScript compiler is not very flexible when it comes to local variable declarations. You are forced to declare all of them at the start of a function before doing anything else, even when it comes to such trivial cases like for loop iterator. This is similar to early C language standards, where declarations could only be done at the beginning of a scope.

```ts linenums="1" hl_lines="3"
function LogHeldSwordInCombat() {
    if (thePlayer.IsInCombatState()) {
        var id: SItemUniqueId; // (1)
        id = thePlayer.inv.GetCurrentlyHeldSword();
        LogChannel('TEST', "Sword item name: " + thePlayer.inv.GetItemName(id));
    }
}
```

1. Variable must be done before the "if" statement.


---



</br>

## **Symbol Analysis**

---

### `symbol-name-taken`

A code symbol (type, function, etc.) has been defined multiple types with the same name inside a content.

```ts linenums="1" hl_lines="1 9"
function doFoo() {
    // ...
}

function doBar() {
    // ...
}

function doFoo(a: int) { // (1)
    // ...
}
```

1. Global function "doFoo" has already been defined on line 1. Function overloading is not available in WitcherScript.


Some contexts allow the same name to be used again. An example would be a class method having the same name as a global function.
In that case if you were to use a function of that name within class's body, WitcherScript compiler would pick the function defined within the class.

```ts linenums="1" hl_lines="1 11"
function doFoo(a: int, b: string) {
    // ...
}

class MyClass {
    function doFoo(s: string) {
        // ...
    }

    function testFoo() {
        doFoo("Hello"); // (1)
    }
}
```

1. Compiler parses code without errors and picks the function defined within the class even if it has the same name as the global function from line 1.


---

### `missing-type-arg`

WitcherScript does not offer a way to create your own generic types. It does however have syntax of using them akin to languages like C++ and Java. There to instantiate a variable of a generic type you would write `#!java List<int> myList`, where `#!java List` is the generic type and `#!java int` is the type argument placed between angled brackets.  

The only type in WitcherScript with properties of a generic type is the `array` type, which takes one type argument. Not supplying that type argument is an error.

```ts linenums="1" hl_lines="1"
var intArray: array; // (1)
```

1. `array` requires a type argument, like `<int>`. So you should write `array<int>`.

CDPR probably originally intended to be able to create your own generic types, but they ran out of time. That's because it would be easier to distinguish array-like types using square brackets (e.g. `[int]` or `int[]`) or something similar instead of having to reserve the `array` identifier just for this purpose.



---

### `unnecessary-type-arg`

The only type in WitcherScript with properties of a generic type is the `array` type. No other types can take any type arguments.  

Also see [missing-type-arg](#missing-type-arg).

```ts linenums="1" hl_lines="1"
var player: CR4Player<Ciri>; // (1)
```

1. Class `CR4Player` does not take any type arguments. Remove `<Ciri>`.


---

### `same-content-annotation`

Annotations are meant to extend types already existing in the Witcher 3 script code base or types defined in other mods. Even if it may be possible to `@wrapMethod` that is defined in the same mod WIDE discourages this behaviour in favour of simply editing those types instead of using annotations. 

```ts linenums="1" title="modSkillFramework/content/scripts/skill_framework.ws"
class SkillFramework {
    public function DefineSkill(skillName: name) {
        // ...
    }
}
```

```ts linenums="1" hl_lines="2"
@wrapMethod(SkillFramework)
function DefineSkill(skillName: name) { // (1)
    LogChannel('SF', "Skill defined: " + skillName);
    wrappedMethod(skillName);
}
```

1. Putting this in the same mod will work for the script compiler, but will yield undefined behaviour for WIDE.


---



</br>

## **Workspace Symbol Analysis**

---

### `symbol-name-taken-in-dependency`

A code symbol (type, function, etc.) has already been defined in a content that is a dependency to this content.

```ts linenums="1" title="content0/scripts/game/player/playerCheats.ws"
exec function RestoreStamina( optional val : int )
{	
	// ...
}
```

```ts linenums="1" title="modFoodStamina/scripts/local/staminaManager.ws" hl_lines="3"
// ...

function RestoreStamina() // (1)
{	
	// ...
}
```

1. Global function "RestoreStamina" has already been defined in content "content0"

See also [`symbol-name-taken`](#symbol-name-taken).


---