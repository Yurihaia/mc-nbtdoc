# Contributing

While anyones help is greatly appreciated, to make sure that
the styling in this repo is consistent, before you make a PR
**please** check that your code follows the style guidelines
detailed below

## Style Guidelines

* All indentation in the code **must** be tabs (`	`)
* Opening curly brackets (`{`) must be on the same line
  as the `compound` keyword and the ID.
* Closing curly brackets (`}`) must be on their own line
* Even though trailing commas are allowed in the specs, they should not be used
  In this repository
* The colon in a `compound` field (`:`) must have no spaces before it, and one space afterwards
* When using a range bind (`@`), there must be one space on both sides
* The range token (`..`) must not have any spaces on either side
* Float tokens (`0.1`, `5.13`, etc) must have a number before the period, and cannot end with a period
* All non-minecraft specific identifiers must be in `PascalCase`, and cannot have numbers in it
* The enum type def (ex `(string)`) must have no spaces before it and one space after
* Doc comments (`///`) must have a space after the slashes,
  and must start with an *uppercase* letter (where applicable obviously)
* `descibes` definitions have to have their targets on multiple lines,
  unless they only have a single target in which case they should be on a single line
  with the brackets having *no* spaces between them and the minecraft id
* **Judgement**: If a repo owner says to fix the styling of some element, *please do*

Examples
```
compound MyCompound {
	myfield: float @ 0.1..5.13
}
```

```
enum(string) MyEnum {
	VarOne = "hello",
	VarTwo = "world"
}
```

```
MyCompound describes minecraft:entity[
	minecraft:sheep,
	minecraft:cow
];

MyOtherCompound describes minecraft:entity[minecraft:chicken];
```