use fs_more::directory::{DirectoryCopyOptions, copy_directory};
use include_dir::{Dir, DirEntry, include_dir};
use owo_colors::OwoColorize;
use temp_dir::TempDir;
use velvetio::{ask, choose};

static TEMPLATE_DIR: Dir<'_> = include_dir!("templates/");

fn main() {
    println!("{}", "CMIV-CLI\n".cyan());

    let path = ask!(
        "Where should we create your project?",
        validate: |s: &String| s.len() > 0 && s.starts_with("./"),
        error: "You must enter a relative path"
    );

    let choices = &[
        "astro",
        "nextjs",
        "redwood-ts",
        "adonis-ts-web",
        "preact-vite-ts",
        "solid-vite-ts",
        "svelte-vite-ts",
    ];

    let kind = choose!("Pick a template to install", choices);

    let dirs = TEMPLATE_DIR.entries();

    copy_template(&path, kind, dirs);
}

fn copy_template(install_path: &str, kind: &str, dirs: &[DirEntry]) {
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

            match copy_directory(
                extracted_template_path,
                target_path,
                DirectoryCopyOptions::default(),
            ) {
                Ok(_) => {
                    println!("{}", "\nAll finished, thanks for using CM-IV CLI!".green());
                }
                Err(e) => {
                    eprintln!("Error copying directory: {e}");
                }
            }
        }
    }
}
