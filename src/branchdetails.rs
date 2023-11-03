use std::io;

use git2::{Branch, Mailmap, Oid};

pub struct BranchDetails {
    name: String,
    oid: Oid,
    short_id: String,
    author_name: String,
    author_email: String,
    summary: String,
}

impl BranchDetails {
    pub fn get_details(branch: Branch, mailmap: &Mailmap) -> Result<BranchDetails, io::Error> {
        let branch_name = branch.name().unwrap().unwrap().to_string();
        let branch_ref = branch.into_reference();
        let commit = branch_ref.peel_to_commit().unwrap().clone();
        let author = commit.author_with_mailmap(mailmap).unwrap().clone();

        let oid = commit.id().clone();
        let short_id = String::from_utf8(commit.as_object().short_id().unwrap().to_vec()).unwrap();
        let summary = commit.summary().unwrap().to_string();
        let author_name = author.name().unwrap().to_string();
        let author_email = author.email().unwrap().to_string();

        Ok(BranchDetails {
            name: branch_name,
            oid,
            short_id,
            author_name,
            author_email,
            summary,
        })
    }

    pub fn to_string(&self) -> String {
        let line = format!(
            "{:} {} \"{}\" - {}|{}",
            self.short_id,
            self.author_name,
            self.author_email,
            self.name,
            self.summary
        );
        line
    }

    pub fn get_oid(&self) -> Oid {
        self.oid
    }
}

