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

    if let Err(e) = cliclack::clear_screen() {
        eprintln!("Error clearing screen: {}", e);
        exit(1);
    }

    if let Err(e) = cliclack::intro("CM-IV CLI".on_cyan().black()) {
        eprintln!("Error showing intro: {}", e);
        exit(1);
    }

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
        Err(e) => {
            if should_terminate.load(Ordering::SeqCst) {
                println!("\nGracefully exiting...");
                exit(0);
            } else {
                eprintln!("Error with input: {}", e);
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
        Err(e) => {
            if should_terminate.load(Ordering::SeqCst) {
                println!("Gracefully exiting...");
                exit(0);
            } else {
                eprintln!("Error with selection: {}", e);
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

            if !extracted_template_path.exists() {
                eprintln!(
                    "Template path does not exist: {}",
                    extracted_template_path.display()
                );
                continue;
            }

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
            } else {
                eprintln!(
                    "Failed to read directory: {}",
                    extracted_template_path.display()
                );
                continue;
            }

            let options = CopyOptions::new();

            match fs_extra::copy_items(&entries, &target_path, &options) {
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
