use git2::Repository;

fn main() -> Result<(), git2::Error> {
	let repo = Repository::open(".")?;
	let src = repo.find_submodule("mc-nbtdoc")?.open()?;
	let head_commit = repo.head()?.peel_to_commit()?;
	println!("current head: {}", head_commit.id());
	let mut tags = src.tag_names(None)?.iter().filter_map(|t| if t?.ends_with("-gen") {
		None
	} else {
		Some(t?.to_owned())
	}).map(
		|t| {
			let v = src.revparse_single(&format!("refs/tags/{}", t))?;
			Ok((t, v))
		}
	).collect::<Result<Vec<_>, git2::Error>>()?;
	tags.sort_by(|(_, a), (_, b)| 
		a.peel_to_commit()
			.unwrap()
			.time()
			.cmp(
				&b.peel_to_commit()
				.unwrap()
				.time())
	);

	let sig = git2::Signature::now(
		"MrYurihi",
		"mryurihi2003@gmail.com"
	)?;
	for (n, o) in tags {
		src.checkout_tree(
			&o,
			None
		)?;
		src.set_head(&format!("refs/tags/{}", n))?;
		let files = mc_nbtdoc_gen::gen();
		let mut index = repo.index()?;
		for f in files {
			index.add_path(&f)?;
		}
		let tree = index.write_tree()?;
		let commit = repo.commit(
			Some("HEAD"),
			&sig,
			&sig,
			&format!("Rebuild {} at {}", n, head_commit.id()),
			&repo.find_tree(tree)?,
			&[&repo.head()?.peel_to_commit()?]
		)?;
		repo.tag_lightweight(&format!("{}-gen", n), &repo.find_object(commit, None)?, true)?;
	}
	Ok(())
}