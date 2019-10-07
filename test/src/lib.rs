#[cfg(test)]
mod tests {
	const TARGET: &str = "../minecraft";

	use nbtdoc::Root;
	use glob::glob;
	use std::fs;

	// This is a very sketchy function, but it actually works
	fn str_before<'a>(src: &'a str, target: &'a str) -> Option<&'a str> {
		if target.as_ptr() < src.as_ptr() {
			None
		} else {
			let offset = target.as_ptr() as usize - src.as_ptr() as usize;
			src.get(0..offset)
		}
	}

	#[test]
	fn check_syntax() {
		let paths = glob("../minecraft/**/*.nbtdoc").unwrap();
		let mut fail = vec![];
		for path in paths {
			let path = path.unwrap();
			let ress = fs::read_to_string(&path).unwrap();
			let res = nbtdoc::parse::root(&ress);
			match res {
				Ok((v, _)) => assert_eq!(v, ""),
				Err(nom::Err::Error((r, e))) | Err(nom::Err::Failure((r, e))) => {
					let offset = str_before(ress.as_str(), r).unwrap().trim_end();
					let line = offset.split('\n').count();
					let col = offset.split('\n').next_back().unwrap().len();
					let pathstr = path.to_str().unwrap();
					eprintln!(
						"-   Error {} in file {} at {}:{}",
						e.description(),
						pathstr,
						line,
						col
					);
					fail.push(String::from(pathstr));
				},
				_ => panic!()
			}
		}
		panic!()
	}

	#[test]
	fn check_resolve() {
		let mut root = Root::new();
		root.add_root_module(TARGET, &nbtdoc::DefaultFileProvider).unwrap();
	}
}