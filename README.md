# MC-NBTDOC
`mc-nbtdoc` is a repository for schemas of Minecraft's NBT format,
including entities, blocks, and items.

## Format
The repo [nbtdoc-rs](https://github.com/MrYurihi/nbtdoc-rs)
should serve as the main reference point for any offshoots of parser,
and an official grammar definition will be put here soon.
The documentation of the format can be found [here](https://github.com/MrYurihi/nbtdoc-rs/blob/master/docs/format.md)

## Validation
The main point of these docs is to provide a way for language services
(such as [mcfunction-rs](https://github.com/Levertion/mcfunction-rs))
to be able to give in depth and complete information about a certain NBT tag.  
  
Validation can be a tricky process, but should be much simpler than in 
[mc-nbt-paths](https://github.com/MrYurih/mc-nbt-paths). Each compound tag is
described in its own definition, which looks similar to Rust's struct definitions.
For values which can take only certain values, an `enum` can be used.  
Doc comments, which show the description of certain objects start with `///`. Regular comments
start with `//`. Doc comments are valid before `compound` and `enum` definitions, and on their fields

## Contributing
Help on this repo is welcomed and encouraged, but make sure you follow the style guidelines at [CONTRIBUTING](CONTRIBUTING.md)
