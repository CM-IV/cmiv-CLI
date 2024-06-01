use include_dir::{include_dir, Dir, DirEntry};
use owo_colors::OwoColorize;
use temp_dir::TempDir;

use std::path::Path;

static TEMPLATE_DIR: Dir<'_> = include_dir!("templates/");

fn main() {
    ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");

    cliclack::clear_screen().unwrap();

    cliclack::intro("CM-IV CLI".on_cyan().black()).unwrap();

    let path = cliclack::input("Where should we create your project?")
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
        .unwrap();

    let kind = cliclack::select(format!("Pick a template to install in '{path}'"))
        .initial_value("astro")
        .item("astro", "Astro", "")
        .item("nextjs", "NextJS", "")
        .item("preact-vite-ts", "Preact Vite TS", "")
        .item("solid-vite-ts", "Solid Vite TS", "")
        .interact()
        .unwrap();

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
