use std::fs::File;
use tar::Builder;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::error::Error;
use colored::*;

use crate::exceptions::exceptions::{throw_gel_global_exception, GelExceptions};

//pub(crate) fn check_deps(_args: String) -> Result

pub(crate) fn handle_create(_arg: String) -> Result<(), Box<dyn Error>> {
    if _arg == "" || _arg == "." {
        eprintln!("{} {}", "Error: Please specify a folder name".red(), "gel create <folder_name>".red());
        throw_gel_global_exception(
            GelExceptions::GelGlobalExceptions("undefined folder name (no such file or directory)".to_string())
        );
    }
    let folder_name = _arg;
    let gel = File::create(format!("{}.gel", folder_name))?;

    let enc = GzEncoder::new(gel, Compression::default());
    let mut builder = Builder::new(enc);

    builder.append_dir_all("", folder_name.clone())?;
    if builder.finish()? == () {
        println!("{} {}", "Success: Created".green(), format!("{}.gel", folder_name).green());
        Ok(())
    } else {
        eprintln!("{} {}", "Error: Failed to create".red(), format!("{}.gel", folder_name).red());
        std::process::exit(1)
    }
}