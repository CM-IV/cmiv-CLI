use include_dir::{include_dir, Dir, DirEntry};
use owo_colors::OwoColorize;
use temp_dir::TempDir;

use std::path::Path;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::process::exit;

static TEMPLATE_DIR: Dir<'_> = include_dir!("templates/");

fn main() {
    let should_terminate = Arc::new(AtomicBool::new(false));

    {
        let terminate_signal = Arc::clone(&should_terminate);
        ctrlc::set_handler(move || {
            terminate_signal.store(true, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
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
    .item("astro", "Astro", "")
    .item("nextjs", "NextJS", "")
    .item("preact-vite-ts", "Preact Vite TS", "")
    .item("solid-vite-ts", "Solid Vite TS", "")
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

    copy_template(path, kind, dirs)
}

fn copy_template(install_path: String, kind: &str, dirs: &[DirEntry]) {
    for entry in dirs {
        if entry.path().display().to_string().contains(kind) {

            let d = TempDir::new().unwrap();

            TEMPLATE_DIR
            .extract(d.path())
            .unwrap();

            let extracted_template_path = d.path().join(kind);

            if !extracted_template_path.exists() {
                eprintln!("Template path does not exist: {}", extracted_template_path.display());
                continue;
            }

            let target_path = std::path::Path::new(&format!("{}/{}", std::env::current_dir().unwrap().display(), install_path)).to_path_buf();

            if let Err(e) = std::fs::create_dir_all(&target_path) {
                eprintln!("Failed to create target directory: {}", e);
                continue;
            }

            match copy_files_recursively(&extracted_template_path, target_path.as_path()) {
                Ok(_) => {
                    cliclack::outro("All finished, thanks for using CM-IV CLI!".green()).unwrap();
                },
                Err(e) => {
                    eprintln!("Failed to recursively copy files: {}", e);
                }
            };

        }
    }
}

fn copy_files_recursively(from: &Path, to: &Path) -> Result<(), std::io::Error> {
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            copy_files_recursively(&path, to)?;
        } else if path.is_file() {
            let file_name = match path.file_name() {
                Some(name) => name,
                None => continue,
            };
            let target_file = to.join(file_name);
            std::fs::copy(&path, target_file)?;
        }
    }
    Ok(())
}
