mod commands;
mod file_ops;
mod models;
mod scraper;
#[macro_use]
extern crate colour;

use structopt::StructOpt;
use std::path::PathBuf;
use crate::commands::{list, init, add, update};

#[derive(Debug, StructOpt)]
#[structopt(name = "Manga updater", about = "A CLI tool to show updated manga chapters.")]
struct Cli {
    //The command can be list, add [url], remove [url], update [url/all] (coming soon)
    //By default, it takes nothing to return the last chapters of the stored mangas.
    #[structopt(default_value="list", help="Available commands: list, add [url], remove [url]")]
    command: String,

    //The URL to the manga to add / remove. Can be [all] in the case of update.
    #[structopt(help="The URL to the manga page on manganelo.")]
    url: Option<String>,

    //A path is optional (used mainly for debug purposes), and indicates the file containing the URLs.
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    path: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    match args.command.as_str() {
        "list" => list(args.path).await,
        "init" => init(args.path),
        "add" => add(args.path, args.url).await,
        "update" => update(args.path, args.url).await,
        _ => println!("Argument out of range.")
    }
    return

}
