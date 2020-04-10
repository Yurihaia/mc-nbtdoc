#![type_length_limit = "60000000"]

#[cfg(test)]
mod tests {
    const TARGET: &str = "../minecraft";

    use glob::glob;
    use nbtdoc::Root;
    use nbtdoc::{identifier::Identifier, CompoundExtend, NbtValue};
    use std::collections::HashSet;
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
                Ok((i, _)) => {
                    if !i.trim().is_empty() {
                        eprintln!("Error at end of file {}: {}", path.to_str().unwrap(), i);
                        panic = true;
                    }
                }
                Err(e) => match e {
                    nom::Err::Error(e) | nom::Err::Failure(e) => {
                        eprintln!(
                            "Error at file {}: {}",
                            path.to_str().unwrap(),
                            nom::error::convert_error(&ress, e)
                        );
                        panic = true;
                    }
                    _ => panic!(),
                },
            }
        }
        if panic {
            panic!()
        }
    }

    #[test]
    fn check_resolve() {
        let mut root = Root::new();
        root.add_root_module(TARGET, &nbtdoc::DefaultFileProvider)
            .unwrap();
    }

    #[test]
    fn get_custom_registries() {
        let mut root = Root::new();
        root.add_root_module(TARGET, &nbtdoc::DefaultFileProvider)
            .unwrap();
        let mut hs = HashSet::new();
        for x in root.get_compounds() {
            for f in x.fields.values() {
                match &f.nbttype {
                    NbtValue::Index { target, .. } | NbtValue::Id(target) => {
                        if root.get_registry(target).is_none() {
                            hs.insert(target.clone());
                        }
                    }
                    _ => (),
                }
            }
            if let Some(CompoundExtend::Registry { target, .. }) = &x.supers {
                if root.get_registry(target).is_none() {
                    hs.insert(target.clone());
                }
            }
        }
        let known = include_str!("./registries.txt")
            .split('\n')
            .filter_map(|v| {
                let mut split = v.split(':');
                Some(Identifier::new(split.next()?.into(), split.next()?.into()))
            })
            .collect::<HashSet<_>>();
        let hs = hs.difference(&known).collect::<Vec<_>>();
        assert!(
            hs.is_empty(),
            "\nCustom defined registries needed:\n{}\n",
            hs.into_iter()
                .map(|v| format!("  - {}", v))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}
