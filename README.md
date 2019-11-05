# MC-NBTDOC
`mc-nbtdoc` is a repository for schemas of Minecraft's NBT format,
including entities, blocks, and items.

## Validation
The main point of these docs is to provide a way for language services
(such as [mcfunction-rs](https://github.com/Levertion/mcfunction-rs))
to be able to give in depth and complete information about a certain NBT tag.  
  
Validation can be a tricky process, but should be much simpler than in 
[mc-nbt-paths](https://github.com/MrYurih/mc-nbt-paths). Each compound tag is
described in its own definition, which looks similar to Rust's struct definitions.
For values which can take only certain values, an `enum` can be used.  
Doc comments, which show the description of certain objects start with `///`. Regular comments
start with `//`. Doc comments are valid before `compound` and `enum` definitions, and on their fields.  
For more information on validation, the [validation.md](https://github.com/MrYurihi/nbtdoc-rs/blob/master/docs/validation.md)
file can server as a reference point.

## Versioning
All of the data in this repo is under a strict versioning scheme. For each Minecraft release, snapshot, and pre-release, a 
new tag is added to the repo with the name found in the
[version_manifest.json](https://launchermeta.mojang.com/mc/game/version_manifest.json) file.  
If any problems are found in the repo, the changes will not be moved under the version tag until the next version tag. The 
only exception is for releases, all critical bugs found will be corrected as soon as possible, and the tag will be updated 
to match. Any non-critical bugs will not trigger a new tag update, but will still me committed to `master`. While it is 
discoraged to move a tag, this will happen when nescessary. Any non-data changes to this repository will never trigger an 
update to a tag.

## Format
The repo [nbtdoc-rs](https://github.com/MrYurihi/nbtdoc-rs)
should serve as the main reference point for any offshoots of parser,
and an official grammar definition will be put here soon.
The documentation of the format can be found [here](https://github.com/MrYurihi/nbtdoc-rs/blob/master/docs/format.md).
This is a best-effort documentation, and is not the official documentation to go by, but it *should* be accurate.

### Serialized Formats
In case of users that do not have a library to parse and resolve the data, serialized forms of the data have been made 
available in the [generated](https://github.com/MrYurihi/mc-nbtdoc/tree/generated) branch of this repository. The version
tags to that branch will match the version tags described [above](#versioning), with `-gen` appended on to the version from
the official version manifest. All of the generated files will be located in the `build` directory, each with the name of
`generated.<ext>`, and `generated.pretty.<ext>`, where `<ext>` is the specified extension. The `.pretty` file contains 
pretty printed data when applicable.  
The current extensions are:
 * `.json` (with pretty printing)

The format for the serialized data can be quite counterintuitive, so a TypeScript declaration file for the format can be
found at [json_format.d.ts](https://github.com/MrYurihi/nbtdoc-rs/blob/master/docs/json_format.d.ts). While this data is 
especially useful for the JSON data, it should generally apply to all other serialized forms.

## Contributing
Help on this repo is welcomed and encouraged, but make sure you follow the style guidelines at
[CONTRIBUTING](CONTRIBUTING.md).