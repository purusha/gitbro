extern crate log;

use chrono::{DateTime, Utc};
use chrono::offset::TimeZone;
use git2::{BranchType, Time};
use std::time::Instant;
use clap::Parser;

use crate::abc::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of local reporisoty git
    #[arg(short, long)]
    path: String,
}

pub fn resolve() {
    let before_revwalk = Instant::now();

    let args: Args = Args::parse();    
    let path: &std::path::Path = check_directory(&args.path).unwrap();

    let repo = resolve_repo_git(path).unwrap();
    info!("found git repository on {:?}", path);

    let branches = repo.branches(Some(BranchType::Remote)).unwrap();
    info!("remote branches {:?}", branches.count());

    let branches2 = repo.branches(Some(BranchType::Local)).unwrap();
    info!("local branches {:?}", branches2.count());    

    let mut revwalk = repo.revwalk().unwrap();
    let _ = revwalk.push_head();

    //non posso stampare il numero di elementi ???
    //info!("revwalk {:?}", revwalk.count());   

    let _ = revwalk.set_sorting(git2::Sort::TIME | git2::Sort::REVERSE);

    for rev in revwalk {
        let oid = rev.unwrap();

        let commit = repo.find_commit(oid).unwrap();
        let message = commit.summary_bytes().unwrap_or_else(|| commit.message_bytes());
        info!("{} = {}", commit.id(), String::from_utf8_lossy(message));

        let when = &commit.author().when();
        info!("\twhen {:?}", convert_git_time_to_datetime(when));

        if commit.parents().len() == 1 {
            let parent = commit.parent(0).unwrap();
            info!("\tparent {}", parent.id());
        } 

        info!("");
    }

    let after_revwalk = Instant::now();
    info!("Revwalk time: {:?}", after_revwalk.duration_since(before_revwalk));
    
}

fn convert_git_time_to_datetime(git_time: &Time) -> DateTime<Utc> {
    #![allow(warnings)]
    
    Utc.timestamp(git_time.seconds() + i64::from(git_time.offset_minutes()) * 60, 0)
}
