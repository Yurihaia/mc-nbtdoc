#[cfg(test)]
mod tests {
	const TARGET: &str = "../minecraft";

	use nbtdoc::Root;
	use glob::glob;
	use std::fs;

	#[test]
	fn check_syntax() {
		let paths = glob("../minecraft/**/*.nbtdoc").unwrap();
		for path in paths {
			let res = fs::read_to_string(path.unwrap()).unwrap();
			let res = nbtdoc::parse::root(&res).unwrap();
			assert_eq!(res.0.trim(), "");
		}
	}

	#[test]
	fn check_resolve() {
		let mut root = Root::new();
		root.add_root_module(TARGET, &nbtdoc::DefaultFileProvider).unwrap();
	}
}