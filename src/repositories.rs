use std::fs::{DirEntry, read_dir, metadata};
use std::path::Path;

use errors::Error;

pub fn parse_owners_with_repos_from_folder(folder_path: &Path) -> Result<Vec<OwnerTree>, Error> {

		let mut owners = vec![];

		let owner_dir_list = try!(read_dir(folder_path));

		for owner_entry in owner_dir_list {

			let owner_entry = try!(owner_entry);

			if try!(metadata(owner_entry.path())).is_dir() {

				let mut owner_instance = OwnerTree{
					owner: String::from(owner_entry.path().file_name().unwrap().to_str().unwrap()),
					repositories: vec![],
				};

				let repo_dir_list = try!(read_dir(&owner_entry.path()));

				for repo_entry in repo_dir_list {
					let repo_entry = try!(repo_entry);

						if try!(metadata(repo_entry.path())).is_dir() {

							try!(owner_instance.add_repo(String::from(repo_entry.path().file_name().unwrap().to_str().unwrap())));

						}
				}

				owners.push(owner_instance);
			}

		}

		return Ok(owners);
}


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
		return repo_names;
	}
}

pub struct RepositoryTree {
	pub repository_name: String,
}
