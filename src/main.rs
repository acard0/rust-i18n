use anyhow::Error;
use clap::{Arg, Command};

use std::{collections::HashMap, path::Path};

use rust_i18n_extract::{extractor, generator, iter};
mod config;

#[macro_use]
extern crate serde_derive;

const APP_NAME: &str = "rust-i18n";
const ABOUT: &str = r#"Rust I18n command for help you simply to extract all untranslated texts from soruce code.

It will iter all Rust files in and extract all untranslated texts that used `t!` macro.
And then generate a YAML file and merge for existing texts.

https://github.com/longbridgeapp/rust-i18n
"#;

fn main() -> Result<(), Error> {
    let extract_command = Command::new("i18n")
        .about("Extract all untranslated I18n texts from soruce code")
        .version(clap::crate_version!())
        .arg(
            Arg::new("source")
                .help("Path of your Rust crate root and Cargo.toml")
                .default_value("./"),
        );

    let app = Command::new(APP_NAME)
        .bin_name("cargo")
        .about(ABOUT)
        .subcommand(extract_command)
        .get_matches();

    let mut results = HashMap::new();

    #[allow(clippy::single_match)]
    match app.subcommand() {
        Some(("i18n", sub_m)) => {
            let source_path = sub_m.get_one::<String>("source").expect("Missing source path");

            let cfg = config::load(std::path::Path::new(source_path))?;

            iter::iter_crate(source_path, |path, source| {
                extractor::extract(&mut results, path, source)
            })?;

            let mut messages: Vec<_> = results.values().collect();
            messages.sort_by_key(|m| m.index);

            let mut has_error = false;

            let output_path = Path::new(source_path).join(&cfg.load_path);

            for available_locale in cfg.available_locales.into_iter() {
                let result = generator::generate(&output_path, &available_locale, messages.clone());
                if result.is_err() {
                    has_error = true;
                }
            }

            if has_error {
                std::process::exit(1);
            }
        }
        _ => {}
    }

    Ok(())
}


