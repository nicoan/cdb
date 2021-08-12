mod cli;
mod commands;
mod bookmarks;

use std::process::exit;
use std::io::{ Error, ErrorKind };
use structopt::StructOpt;
use cli::CommandLineArgs;
use bookmarks::Bookmark;
use commands::*;

fn get_bookmark_from_args(mut args: Vec<String>) -> Result<Bookmark, Error> {
    let alias = args.pop()
        .ok_or(Error::new(ErrorKind::Other, "There was an error reading the alias"))?;
    let path = args.pop()
        .ok_or(Error::new(ErrorKind::Other, "There was an error reading the path"))?;

    Ok(Bookmark::new(path, alias))
}

fn main() {
    let command = CommandLineArgs::from_args();

    let result = match command {
        CommandLineArgs { list: true, .. } => list_bookmarks(),
        CommandLineArgs { find: Some(Some(pattern)), .. } => find_bookmark(pattern),
        CommandLineArgs { find: Some(None), .. } => list_bookmarks_names(),
        CommandLineArgs { show: Some(bookmark), .. } => show_bookmark(bookmark),
        CommandLineArgs { update: Some(args), .. } => {
            get_bookmark_from_args(args)
                .and_then(update_bookmark)
        }
        CommandLineArgs { add: Some(args), .. } => {
            get_bookmark_from_args(args)
                .and_then(add_bookmark)
        },
        CommandLineArgs { remove: Some(bookmark), .. } => remove_bookmark(bookmark),
        CommandLineArgs { bookmark: Some(b), .. } => change_directory(b),
        CommandLineArgs { bookmark: None, .. } => Err(
            Error::new(
                ErrorKind::InvalidInput,
                "Must specify a bookmark name. For more information type \"cdb --help\""
                )
            )
    };

    exit(match result {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error: {}", e);
            1
        },
    });
}
