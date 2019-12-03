#![type_length_limit="60000000"]

#[cfg(test)]
mod tests {
	const TARGET: &str = "../minecraft";

	use nbtdoc::Root;
	use nbtdoc::NbtValue;
	use std::collections::HashSet;
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
				Ok((i, _)) => if i.trim().len() > 0 {
					eprintln!(
						"Error at end of file {}: {}",
						path.to_str().unwrap(),
						i
					);
					panic = true;
				},
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

	#[test]
	fn get_custom_registries() {
		let mut root = Root::new();
		root.add_root_module(TARGET, &nbtdoc::DefaultFileProvider).unwrap();
		let mut hs = HashSet::new();
		for x in root.get_compounds() {
			for (_, f) in &x.fields {
				match &f.nbttype {
					NbtValue::Index { target, path: _ } => if root.get_registry(target).is_none() {
						hs.insert(target);
					},
					_ => ()
				}
			}
		}
		for x in hs {
			eprintln!("{} is not a described registry", x)
		}
	}
}