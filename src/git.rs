use anyhow::Result;
use git2::Repository;

pub struct GitHelper {
    repo: Repository,
}

#[derive(Clone, Debug)]
pub struct Commit {
    pub full_hash: String,
    pub message: String,
    pub diff: String,
    pub author_name: String,
    pub author_email: String,
    pub date: String,
}

impl GitHelper {
    pub fn new() -> Result<Self> {
        let repo = Repository::discover(".")?;
        Ok(Self { repo })
    }

    pub fn get_diff(&self, staged: bool) -> Result<String> {
        let head = self.repo.head()?;
        let tree = head.peel_to_tree()?;

        let mut opts = git2::DiffOptions::new();
        opts.context_lines(0);

        let index = self.repo.index()?;
        let diff = self.repo.diff_tree_to_index(
            Some(&tree),
            if staged { Some(&index) } else { None },
            Some(&mut opts),
        )?;

        let mut diff_text = String::new();
        diff.print(git2::DiffFormat::Patch, |_, _, line| {
            diff_text.push_str(std::str::from_utf8(line.content()).unwrap());
            true
        })?;

        Ok(diff_text)
    }

    pub fn is_valid_hash(&self, hash: &str) -> bool {
        self.repo.revparse_single(hash).is_ok()
    }

    pub fn get_commit_from_hash(&self, reference: &str) -> Result<Commit, git2::Error> {
        let commit = self.repo.revparse_single(reference)?;
        let commit = commit.peel_to_commit()?;
        let tree = commit.tree()?;

        let diff = self.repo.diff_tree_to_tree(None, Some(&tree), None)?;

        let mut diff_text = String::new();
        diff.print(git2::DiffFormat::Patch, |_, _, line| {
            diff_text.push_str(std::str::from_utf8(line.content()).unwrap());
            true
        })?;

        let author = commit.author();
        let author_name = author.name().unwrap_or("").to_string();
        let author_email = author.email().unwrap_or("").to_string();

        Ok(Commit {
            full_hash: commit.id().to_string(),
            message: commit.summary().unwrap_or("").to_string(),
            diff: diff_text,
            author_name,
            author_email,
            date: commit.time().seconds().to_string(),
        })
    }
}
