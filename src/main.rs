use fork::{daemon, Fork};
use linked_hash_map::LinkedHashMap;
use regex::Regex;
use std::env;
use std::process::Command;
use urlencoding::decode;

mod recently_used_xbel;

static NUM_OF_FILES: usize = 5;

struct File {
    path: String,
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args_vec: Vec<String> = env::args().collect();
    let args: String = args_vec
        .join(" ")
        .split("rofi-recent ")
        .nth(1)
        .unwrap_or("")
        .to_string();
    let recently_used = recently_used_xbel::parse_file()?;
    let get_app = Regex::new(r"[a-zA-Z/]+").unwrap();

    // reverse the order of bookmarks to put the most recent bookmarks at the top
    let rev_bookmarks: Vec<_> = recently_used.bookmarks.iter().rev().collect();
    // create a HashMap to store file names
    let mut files = LinkedHashMap::<String, Vec<File>>::new();
    for bookmark in rev_bookmarks {
        for app in bookmark.info.metadata.app_parent.apps.iter() {
            let cmd = get_app.find(&app.exec).unwrap().as_str();
            if !files.contains_key(cmd) {
                files.insert(cmd.to_string(), Vec::new());
            }
            if files.get(cmd).unwrap().len() < NUM_OF_FILES {
                let ele = File {
                    path: decode(&bookmark.href)?.to_string(),
                    output: format!(
                        "{}\0icon\x1f{}\x1fmeta\x1f{} {}",
                        // split directory and file name to only show file name
                        decode(&bookmark.href)?.split("/").last().unwrap_or(""),
                        // get icon name by replacing forward slashes in type with dashes
                        bookmark.info.metadata.mime_type.mimetype.replace("/", "-"),
                        // these two fields are added solely to make searching more thorough
                        app.name,
                        app.exec
                    ),
                };
                files.get_mut(cmd).unwrap().push(ele);
            }
        }
    }
    if args.eq("") {
        printer(files);
    } else {
        run(files, args);
    }
    Ok(())
}

// prints a list files from the HashMap
fn printer(files: LinkedHashMap<String, Vec<File>>) -> () {
    for (k, v) in files {
        for ele in v {
            println!("{} {}", k, ele.output);
        }
    }
}

// opens the chosen file
fn run(files: LinkedHashMap<String, Vec<File>>, choice: String) -> () {
    // the first part of choice, which contains the program command
    let cmd = choice.split_whitespace().next().unwrap_or("");
    let short_path: String = choice[(cmd.chars().count() + 1)..].to_string();
    // the rest of the choice, which contains the file name
    let file_vec = files.get(cmd).unwrap();
    // variable cmd is used to find the vector of files from the LinkedHashMap
    let mut path: Option<&str> = None;
    // the path variable will store the full path to the file

    // compares each file in file_vec with the short_path to look for a match
    for file in file_vec {
        if file.path.contains(&short_path) {
            path = Some(&file.path);
            break;
        }
    }

    // if path has a value, a command is executed to open the file
    if let Some(_value) = path {
        if let Ok(Fork::Child) = daemon(false, false) {
            Command::new(cmd)
                .arg(path.unwrap())
                .output()
                .expect("no such file or directory");
        }
    } else {
        println!("error: no path");
    }
}
