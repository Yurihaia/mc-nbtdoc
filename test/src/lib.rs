#![type_length_limit="7000000"]

#[cfg(test)]
mod tests {
	const TARGET: &str = "../minecraft";

	use nbtdoc::Root;
	use glob::glob;
	use std::fs;

	#[test]
	fn check_syntax() {
		let paths = glob("../minecraft/**/*.nbtdoc").unwrap();
		let mut panic = false;
		for path in paths {
			let path = path.unwrap();
			let ress = fs::read_to_string(&path).unwrap();
			let res = nbtdoc::parse::root::<nom::error::VerboseError<&str>>(&ress);
			match res {
				Ok(_) => (),
				Err(e) => match e {
					nom::Err::Error(e) | nom::Err::Failure(e) => {
						eprintln!(
							"Error at file {}: {}",
							path.to_str().unwrap(),
							nom::error::convert_error(&ress, e)
						);
						panic = true;
					},
					_ => panic!()
				}
			}
		}
		if panic {
			panic!()
		}
	}

	#[test]
	fn check_resolve() {
		let mut root = Root::new();
		root.add_root_module(TARGET, &nbtdoc::DefaultFileProvider).unwrap();
	}
}