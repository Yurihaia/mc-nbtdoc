#![type_length_limit="60000000"]

const SOURCE: &str = "mc-nbtdoc/minecraft/.";
const DEST: &str = "build/.";

use std::fs::File;
use std::path::{Path, PathBuf};

use serde_json::ser::PrettyFormatter;

use serde::Serialize;

pub fn gen() -> Vec<PathBuf> {
	let mut root = nbtdoc::Root::new();
	root.add_root_module(SOURCE, &nbtdoc::DefaultFileProvider).unwrap();
	let dest = Path::new(DEST);
	serde_json::to_writer(
		File::create(dest.join("generated.json")).unwrap(),
		&root
	).unwrap();
	root.serialize(&mut serde_json::Serializer::with_formatter(
		File::create(dest.join("generated.pretty.json")).unwrap(),
		PrettyFormatter::with_indent(b"\t")
	)).unwrap();
	vec![
		dest.join("generated.json"),
		dest.join("generated.pretty.json")
	]
}
