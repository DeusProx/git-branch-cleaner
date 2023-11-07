use std::io::{Result, stdout, Write, Stdout};

use crossterm::ExecutableCommand;
use crossterm::event::{self, KeyCode, KeyEventKind};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

use git_branch_cleaner::{BranchDetails, get_domain, List};
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::Paragraph;

use git2::{Repository, FetchOptions, Cred};
use ssh2_config::{SshConfig, ParseRule};

fn main() -> Result<()> {

    let repo = Repository::open("./").unwrap();
    let mut remote = repo.find_remote("origin").unwrap();
    let _refs = repo.references().unwrap();

    // Callbacks
    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.transfer_progress(|stats| {
        if stats.received_objects() == stats.total_objects() {
            print!(
                "Resolving deltas {}/{}\r",
                stats.indexed_deltas(),
                stats.total_deltas()
            );
        } else if stats.total_objects() > 0 {
            print!(
                "Received {}/{} objects ({}) in {} bytes\r",
                stats.received_objects(),
                stats.total_objects(),
                stats.indexed_objects(),
                stats.received_bytes()
            );
        }
        stdout().flush().unwrap(); // Hint: Needs to import std::io::Write so flush will work
        true
    });
    callbacks.credentials(|url, username_from_url, _allowed_types| {
        let domain = get_domain(url).unwrap();
        let cred = get_cred(domain, username_from_url.unwrap()).unwrap();
        Ok(cred)
    });

    // Fetch
    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks);
    // Always fetch all tags.
    // Perform a download and also update tips
    fo.download_tags(git2::AutotagOption::All);
    _ = remote.fetch(&["main"], Some(&mut fo), None);

    /*
    // Fetch head for each ref
    let _ = repo.fetchhead_foreach(|name, url, oid, is_merge_ref| {
        let bla = std::str::from_utf8(url).unwrap().to_string();
        println!("fetch: {} {} {}", name, bla, oid.to_string());
        is_merge_ref
    });
    */

    // Get necessary details of all branches
    let mailmap = repo.mailmap().unwrap();
    let branches = repo.branches(None).unwrap();
    // let branches = repo.branches(Some(git2::BranchType::Remote)).unwrap();

    let details = branches.map(|branch_result| {
        let branch = branch_result.unwrap().0;
        let branchdetails = BranchDetails::get_details(branch, &mailmap).unwrap();
        branchdetails
    });

    let list: List<BranchDetails> = List::new(details.collect::<Vec<BranchDetails>>());

    // Render TUI
    let mut terminal = term_setup().unwrap();
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(format!("Welcome to Git Branch Cleaner! (Leave by pressing q)\n{}", list)), area);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(event::KeyEvent {
                kind: KeyEventKind::Press,
                code: KeyCode::Char(char),
                ..
            }) = event::read()? {
                match char {
                    'q' => break,
                    _ => {}
                }
            }
        }
    }
    term_terminate(terminal)
}

pub fn term_setup() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}

pub fn term_terminate(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    terminal.show_cursor()?;
    terminal.clear()?;
    // print!("\x1B[2J\x1B[1;1H");
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

pub fn get_cred(domain: &str, username_from_url: &str) -> Result<Cred> {
    let config = SshConfig::parse_default_file(ParseRule::STRICT).expect("Failed to parse configuration");
    let github_config = config.query(domain);
    let identity_file_paths = github_config.identity_file.unwrap();
    let private_key_path = identity_file_paths.first().unwrap().to_path_buf();
    // println!("identity file path: {};", private_key_path.display());

    let cred = Cred::ssh_key(
        username_from_url,
        None,
        &private_key_path,
        None,
    ).unwrap();
    Ok(cred)
}

