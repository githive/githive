
use errors::Error;

pub struct OwnerTree {
	pub owner: String,
	pub repositories: Vec<RepositoryTree>,
}

impl OwnerTree {
	pub fn add_repo(&mut self, repository_name: String) -> Result<(), Error> {

		let repo = RepositoryTree {
			repository_name: repository_name,
		};

		self.repositories.push(repo);
		Ok(())
	}

	pub fn get_repo_names(self) -> Vec<String> {
		let mut repo_names = vec![];

		for repo in self.repositories {

			let mut repo_path = String::from("/");
			repo_path.push_str(&self.owner);
			repo_path.push_str("/");
			repo_path.push_str(&repo.repository_name);

			repo_names.push(repo_path);
		}
		return (repo_names);
	}
}

pub struct RepositoryTree {
	pub repository_name: String,
}
