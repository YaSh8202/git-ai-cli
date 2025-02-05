use git2::Repository;
use anyhow::Result;

pub struct GitHelper {
    repo: Repository,
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
            if staged { 
                Some(&index)
            } else { 
                None 
            },
            Some(&mut opts)
        )?;

        let mut diff_text = String::new();
        diff.print(git2::DiffFormat::Patch, |_, _, line| {
            diff_text.push_str(std::str::from_utf8(line.content()).unwrap());
            true
        })?;

        Ok(diff_text)
    }
}