mod commands;
mod file_ops;
mod models;
mod scraper;
#[macro_use]
extern crate colour;

use structopt::StructOpt;
use std::path::PathBuf;
use crate::commands::{list, init, add, update, export, import};

/// The CLI struct to store the different commands and parameters used by the app.
#[derive(Debug, StructOpt)]
#[structopt(name = "Manga updater", about = "A CLI tool to show updated manga chapters.")]
struct Cli {
    //The command can be list, add [url], remove [url], update [url/all] (coming soon)
    //By default, it takes nothing to return the last chapters of the stored mangas.
    #[structopt(default_value="list",
    help="Available commands: list, add [url], remove [url], export [-e path]. For more info, refer to the doc.")]
    command: String,

    //The URL to the manga to add / remove. Can be [all] in the case of update.
    #[structopt(help="The URL to the manga page on manganelo. Can be all for update")]
    argument: Option<String>,

    //A path is optional (used mainly for debug purposes), and indicates the file containing the URLs.
    #[structopt(short = "p", long = "path", parse(from_os_str),
    help="The path to the CSV file to use. Overrides default.")]
    path: Option<PathBuf>,

    //The path to export the CSV file to, or import from.
    #[structopt(short = "e", long = "external", parse(from_os_str),
    help="The path to export/import a CSV file from the application.")]
    external_file: Option<PathBuf>,

    #[structopt(short="o", long="overwrite",
    help="Specifies if the current database must be overwritten. Usable only with import command.")]
    overwrite: bool,

    #[structopt(short="n", long="new", help="Display only new chapters.")]
    new: bool
}

/// Entry point of the application.
/// Matches the argument given at the start, and redirect to the correct command.
#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    match args.command.as_str() {
        "list" => list(args.path, args.new).await,
        "init" => init(args.path),
        "add" => add(args.path, args.argument).await,
        "update" => update(args.path, args.argument).await,
        "export" => export(args.path, args.external_file),
        "import" => import(args.external_file, args.path, args.overwrite),
        _ => println!("Argument out of range. Try running --h or -h.")
    }
    return

}
