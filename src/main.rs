use std::env;
use std::process::Command;

use fork::{daemon, Fork};
use linked_hash_map::LinkedHashMap;
use regex::Regex;
use urlencoding::decode;

mod recently_used_xbel;

// setting this to false will remove the limit on the number of files per program
const LIMIT: bool = true;
// changing this value will change the limit on the number of files per program
const NUM_OF_FILES: usize = 5;

struct File {
    path: String,
    output: String,
    date: String,
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

    // create a HashMap to store file names
    let mut files = LinkedHashMap::<String, Vec<File>>::new();
    // iterates through the bookmarks
    for bookmark in recently_used.bookmarks {
        for app in bookmark.info.metadata.app_parent.apps.iter() {
            // uses Regex to find the command without parameters
            let cmd = get_app.find(&app.exec).unwrap().as_str();
            // split directory and file name to only show file name
            let fname: String = decode(&bookmark.href)?
                .split("/")
                .last()
                .unwrap_or("")
                .to_string();
            // create a boolean to prevent duplicate entries
            let mut exists = false;
            // adds the command as a key to the LinkedHashMap if it doesn't already exist
            if !files.contains_key(cmd) {
                files.insert(cmd.to_string(), Vec::new());
            } else {
                for v in files.get_mut(cmd).unwrap() {
                    if fname.eq(&v.output.split("\0icon").next().unwrap_or("").to_string()) {
                        v.output += &(" ".to_owned() + &app.name + &app.exec);
                        exists = true;
                        break;
                    }
                }
            }
            // adds Files as values to the LinkedHashMap if there are still spaces remaining
            if !exists {
                let ele = File {
                    path: decode(&bookmark.href)?.to_string(),
                    output: format!(
                        "{}\0icon\x1f{}\x1fmeta\x1f{} {}",
                        // use file name
                        fname,
                        // get icon name by replacing forward slashes in type with dashes
                        bookmark.info.metadata.mime_type.mimetype.replace("/", "-"),
                        // these two fields are added solely to make searching more thorough
                        app.name,
                        app.exec
                    ),
                    date: bookmark.modified.to_string(),
                };
                files.get_mut(cmd).unwrap().push(ele);
            }
        }
    }
    // sorts the files with most-recently-used files at the top and truncates the list if needed
    let cmds: Vec<_> = files.keys().cloned().collect();
    for cmd in cmds {
        files
            .get_mut(&cmd)
            .unwrap()
            .sort_by(|a, b| b.date.cmp(&a.date));
        if LIMIT {
            files.get_mut(&cmd).unwrap().truncate(NUM_OF_FILES);
        }
    }
    // without any args, the program will use the LinkedHashMap to print a list
    if args.is_empty() {
        printer(files);
    }
    // with args, the program will attempt to open the file in the desired program
    else {
        run(files, args);
    }
    Ok(())
}

// prints a list files from the HashMap
fn printer(files: LinkedHashMap<String, Vec<File>>) {
    for (k, v) in files {
        for ele in v {
            println!("{} {}", k, ele.output);
        }
    }
}

// opens the chosen file in the desired program
fn run(files: LinkedHashMap<String, Vec<File>>, choice: String) {
    // the first part of choice, which contains the program command
    let cmd = choice.split_whitespace().next().unwrap_or("");
    // the rest of the choice, which contains the file name
    let short_path: String = choice[(cmd.chars().count() + 1)..].to_string();
    // variable cmd is used to find the vector of files from the LinkedHashMap
    let file_vec = files.get(cmd).unwrap();
    // the path variable will store the full path to the file
    let mut path: Option<&str> = None;

    // compares each file in file_vec with the short_path to look for a match
    for file in file_vec {
        if file.path.contains(&short_path) {
            path = Some(&file.path);
            break;
        }
    }

    // if path has a value, a command is executed in the background to open the file
    match path {
        Some(value) => {
            if let Ok(Fork::Child) = daemon(false, false) {
                Command::new(cmd)
                    .arg(format!("{}", value.split("file://").nth(1).unwrap()))
                    .output()
                    .expect("no such file or directory");
            }
        }
        None => eprintln!("error: no path"),
    }
}
