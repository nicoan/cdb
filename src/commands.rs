use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io::{ SeekFrom, Seek, Error, ErrorKind };
use crate::bookmarks::{ Bookmark, Bookmarks };
use home::home_dir;
use serde_json;
use std::path::PathBuf;

fn get_config_dir() -> Result<PathBuf, Error> {
    let mut path = home_dir()
        .ok_or(Error::new(ErrorKind::Other, "There was an error looking for the home directory."))?;
    path.push(".config");
    path.push("cdb");

    Ok(path)
}

fn get_config_path() -> Result<String, Error> {
    let mut config_dir = get_config_dir()?;
    config_dir.push("config.json");

    config_dir
        .into_os_string()
        .into_string()
        .map_err(|_| Error::new(ErrorKind::Other, "There was an error getting the config path."))
}

fn open_config_file() -> Result<fs::File, Error> {
    // Get directory where config file is located
    let config_dir = get_config_dir()?
        .into_os_string()
        .into_string()
        .map_err(|_| Error::new(ErrorKind::Other, "There was an error getting the config directory."))?;

    // We create it if it does not exists
    fs::create_dir_all(config_dir)?;

    // Get file path
    let config_path = get_config_path()?;
    // And open it with read and write permission
    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config_path)
}

fn deserialize_config_file(mut file: &fs::File) -> Result<Bookmarks, Error> {
    // Rewind before
    file.seek(SeekFrom::Start(0))?;
    // Deserialize
    let bookmarks = match serde_json::from_reader(file) {
        Ok(bookmarks) => bookmarks,
        Err(e) if e.is_eof() => HashMap::new(),
        Err(e) => Err(e)?,
    };
    // Rewind after
    file.seek(SeekFrom::Start(0))?;

    Ok(bookmarks)
}

fn get_absolute_path(bookmark: &Bookmark) -> Result<String, Error> {
    fs::canonicalize(PathBuf::from(&bookmark.path))?
        .into_os_string()
        .into_string()
        .map_err(|_| Error::new(
            ErrorKind::Other,
            format!("There was an error getting the absolute path for \"{}\".", &bookmark.path))
        )
}

fn check_bookmark_is_valid(bookmark: &Bookmark) -> Result<(), Error> {
    // Check bookmark name does not start with hypen
    if bookmark.alias.chars().nth(0).unwrap() == '-' {
        return Err(Error::new(ErrorKind::InvalidInput, "Bookmark name must not start with \"-\"."));
    }

    // Check if given path exists
    if !Path::new(&bookmark.path).exists() {
        return Err(Error::new(ErrorKind::NotFound, format!("Directory \"{}\" does not exists.", &bookmark.path)));
    }

    // Check if path given is a directory
    let path_metadata = fs::metadata(&bookmark.path)?;
    if !path_metadata.is_dir() {
        return Err(Error::new(ErrorKind::InvalidInput, format!("Path \"{}\" is not a directory.", &bookmark.path)));
    }

    Ok(())
}

pub fn add_bookmark(bookmark: Bookmark) -> Result<(), Error> {
    // Open config file
    let file = open_config_file()?;

    // Read its contents and deserialize them
    let mut bookmarks = deserialize_config_file(&file)?;

    // If the bookmark exsits, we return an error.
    if bookmarks.contains_key(&bookmark.alias) {
        return Err(Error::new(ErrorKind::AlreadyExists, format!("Bookmark \"{}\" already exists.", &bookmark.alias)));
    }

    // Check bookmark is valid
    check_bookmark_is_valid(&bookmark)?;

    // Write the new bookmark
    let absolute_bookmark_path = get_absolute_path(&bookmark)?;
    bookmarks.insert(bookmark.alias, absolute_bookmark_path);
    serde_json::to_writer(&file, &bookmarks)?;

    Ok(())
}

pub fn update_bookmark(bookmark: Bookmark) -> Result<(), Error> {
    // Open config file
    let file = open_config_file()?;

    // Read its contents and deserialize them
    let mut bookmarks = deserialize_config_file(&file)?;

    // If the bookmark does not exsits, we return an error.
    if !bookmarks.contains_key(&bookmark.alias) {
        return Err(Error::new(ErrorKind::NotFound, format!("Bookmark \"{}\" does not exists.", &bookmark.alias)));
    }

    // Check bookmark is valid
    check_bookmark_is_valid(&bookmark)?;

    // Update the entry
    bookmarks.insert(bookmark.alias, bookmark.path);
    file.set_len(0)?;
    serde_json::to_writer(&file, &bookmarks)?;

    Ok(())
}

pub fn remove_bookmark(alias: String) -> Result<(), Error> {
    // Open config file
    let file = open_config_file()?;

    // Read its contents and deserialize them
    let mut bookmarks = deserialize_config_file(&file)?;

    // If the bookmark exsits, we return an error.
    if !bookmarks.contains_key(&alias) {
        return Err(Error::new(ErrorKind::NotFound, format!("Bookmark \"{}\" does not exists.", &alias)));
    }

    // Write the new bookmark
    bookmarks.remove(&alias);

    // Truncate the file so we write all the content (otherwise we might leave trash behind)
    file.set_len(0)?;
    serde_json::to_writer(&file, &bookmarks)?;

    Ok(())
}

pub fn show_bookmark(alias: String) -> Result<(), Error> {
    // Open config file
    let file = open_config_file()?;

    // Read its contents and deserialize them
    let bookmarks = deserialize_config_file(&file)?;

    // If the bookmark exsits, we return an error.
    if let Some(bookmark) = bookmarks.get(&alias) {
        println!("{}", bookmark);
        return  Ok(());
    }

    Err(Error::new(ErrorKind::NotFound, format!("Bookmark \"{}\" does not exists.", &alias)))
}

pub fn list_bookmarks() -> Result<(), Error> {
    // Open config file
    let file = open_config_file()?;

    // Read its contents and deserialize them
    let bookmarks = deserialize_config_file(&file)?;

    if bookmarks.is_empty() {
        println!("There are no bookmarks yet!");
        return Ok(());
    }

    // Calculate longest bookmark length to pad the result
    let longest_bookmark = bookmarks
        .keys()
        .map(|k| k.len())
        .max()
        .ok_or(Error::new(ErrorKind::Other, "There was an error listing the bookmarks"))?;

    // We need the home directory for replacing it by "~" symbol
    let home = home_dir()
        .ok_or(Error::new(ErrorKind::Other, "There was an error listing the bookmarks"))?
        .into_os_string()
        .into_string()
        .map_err(|_| Error::new(ErrorKind::Other, "There was an error listing the bookmarks"))?;

    for (key, value) in bookmarks.into_iter() {
        println!("{:<longest_bookmark$} -> {}", key, value.replace(&home, "~"), longest_bookmark = longest_bookmark);
    }

    Ok(())
}

pub fn find_bookmark(pattern: String) -> Result<(), Error> {
    // Open config file
    let file = open_config_file()?;

    // Read its contents and deserialize them
    let bookmarks = deserialize_config_file(&file)?;

    let filtered_aliases = bookmarks
        .keys()
        .filter(|a| a.starts_with(pattern.as_str()));

    for alias in filtered_aliases {
        println!("{}", alias);
    }

    Ok(())
}

pub fn list_bookmarks_names() -> Result<(), Error> {
    // Open config file
    let file = open_config_file()?;

    // Read its contents and deserialize them
    let bookmarks = deserialize_config_file(&file)?;

    bookmarks
        .keys()
        .for_each(|a| println!("{}", a));

    Ok(())
}

pub fn change_directory(alias: String) -> Result<(), Error> {
    // Open config file
    let file = open_config_file()?;

    // Read its contents and deserialize them
    let bookmarks = deserialize_config_file(&file)?;

    let path = bookmarks.get(&alias)
        .ok_or(Error::new(ErrorKind::NotFound, format!("Bookmark \"{}\" not found.", &alias)))?;

    // We just print where we want to go, the actual directory change is managed by the shell
    // script
    println!("CDB {}", &path);

    Ok(())
}
