use fs_extra::dir::CopyOptions;
use include_dir::{include_dir, Dir, DirEntry};
use owo_colors::OwoColorize;
use temp_dir::TempDir;

use std::{
    path::PathBuf,
    process::exit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

static TEMPLATE_DIR: Dir<'_> = include_dir!("templates/");

fn main() {
    let should_terminate = Arc::new(AtomicBool::new(false));

    {
        let terminate_signal = Arc::clone(&should_terminate);
        ctrlc::set_handler(move || {
            terminate_signal.store(true, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    cliclack::clear_screen().unwrap();

    cliclack::intro("CM-IV CLI".on_cyan().black()).unwrap();

    let path = match cliclack::input("Where should we create your project?")
        .placeholder("./sparkling-solid")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Please enter a path.")
            } else if !input.starts_with("./") {
                Err("Please enter a relative path")
            } else {
                Ok(())
            }
        })
        .interact()
    {
        Ok(path) => path,
        Err(_) => {
            if should_terminate.load(Ordering::SeqCst) {
                cliclack::log::remark("\nGracefully exiting...").unwrap();
                exit(0);
            } else {
                cliclack::log::error("\nError with input, exiting...").unwrap();
                exit(1);
            }
        }
    };

    let kind = match cliclack::select(format!("Pick a template to install in '{path}'"))
        .initial_value("astro")
        .item("astro", "AstroJS", "")
        .item("nextjs", "NextJS", "")
        .item("redwood-ts", "RedwoodJS", "")
        .item("blitz-ts", "BlitzJS", "")
        .item("adonis-ts-web", "AdonisJS", "")
        .item("preact-vite-ts", "Preact Vite TS", "")
        .item("solid-vite-ts", "Solid Vite TS", "")
        .item("svelte-vite-ts", "Svelte Vite TS", "")
        .interact()
    {
        Ok(kind) => kind,
        Err(_) => {
            if should_terminate.load(Ordering::SeqCst) {
                cliclack::log::remark("\nGracefully exiting...").unwrap();
                exit(0);
            } else {
                cliclack::log::error("\nError with selection, exiting...").unwrap();
                exit(1);
            }
        }
    };

    let dirs = TEMPLATE_DIR.entries();

    copy_template(path, kind, dirs);
}

fn copy_template(install_path: String, kind: &str, dirs: &[DirEntry]) {
    for entry in dirs {
        if entry.path().display().to_string().contains(kind) {
            let d = TempDir::new().unwrap();

            TEMPLATE_DIR.extract(d.path()).unwrap();

            let extracted_template_path = d.path().join(kind);

            let target_path = std::path::Path::new(&format!(
                "{}/{}",
                std::env::current_dir().unwrap().display(),
                install_path
            ))
            .to_path_buf();

            if let Err(e) = std::fs::create_dir_all(&target_path) {
                eprintln!("Failed to create target directory: {}", e);
                continue;
            }

            let mut entries: Vec<PathBuf> = vec![];

            if let Ok(dir_entries) = std::fs::read_dir(&extracted_template_path) {
                for entry in dir_entries {
                    if let Ok(entry) = entry {
                        entries.push(entry.path());
                    }
                }
            }

            match fs_extra::copy_items(&entries, &target_path, &CopyOptions::new()) {
                Ok(_) => {
                    cliclack::outro("All finished, thanks for using CM-IV CLI!".green()).unwrap();
                }
                Err(e) => {
                    eprintln!("Failed to copy files: {}", e);
                }
            }
        }
    }
}
