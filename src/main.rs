use std::collections::HashMap;
use std::process::Command;

use clap::Parser;
use fork::{daemon, Fork};
use regex::Regex;
use urlencoding::decode;

mod recently_used_xbel;

#[derive(Parser,Default,Debug)]
#[clap(trailing_var_arg = true)]
struct Arguments {
    /// Max number of recent files to list per program (0 = unlimited)
    #[clap(short, long, default_value_t = 5)]
    limit: usize,

    /// Command to run (program + file).
    /// Excluding this arg will print a list of recently-used files
    #[clap(required = false)]
    command: Vec<String>,
}

struct File {
    path: String,
    output: String,
    date: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();
    let recently_used = recently_used_xbel::parse_file()?;
    let get_app = Regex::new(r"[a-zA-Z/]+").unwrap();

    // create a HashMap to store file names
    let mut files = HashMap::<String, Vec<File>>::new();
    // iterates through the bookmarks
    for bookmark in recently_used.bookmarks {
        // application type, which is used for the icon and (sometimes) the program name
        let mimetype: String = bookmark.info.metadata.mime_type.mimetype;
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
            // adds the command as a key to the HashMap if it doesn't already exist
            if !files.contains_key(cmd) {
                files.insert(cmd.to_string(), Vec::new());
            } else {
                for v in files.get_mut(cmd).unwrap() {
                    if fname.eq(&v.output.split("\0icon").next().unwrap_or("").to_string()) {
                        v.output += &format!(
                            " {} {} {}",
                            &app.name,
                            &app.exec,
                            &mimetype.split("application/x-").nth(1).unwrap_or("")
                        );
                        exists = true;
                        break;
                    }
                }
            }
            // adds Files as values to the HashMap if there are still spaces remaining
            if !exists {
                let ele = File {
                    path: decode(&bookmark.href)?.to_string(),
                    output: format!(
                        "{}\0icon\x1f{}\x1fmeta\x1f{} {} {}",
                        // use file name
                        fname,
                        // get icon name by replacing forward slashes in type with dashes
                        mimetype.replace("/", "-"),
                        // these three fields are added solely to make searching more thorough
                        app.name,
                        app.exec,
                        mimetype.split("application/x-").nth(1).unwrap_or("")
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
        if args.limit > 0 {
            files.get_mut(&cmd).unwrap().truncate(args.limit);
        }
    }
    // without any args, the program will use the HashMap to print a list
    if args.command.is_empty() {
        printer(files);
    }
    // with args, the program will attempt to open the file in the desired program
    else {
        run(files, args.command);
    }
    Ok(())
}

// prints a list of files from the HashMap
fn printer(files: HashMap<String, Vec<File>>) {
    for (k, v) in files {
        for ele in v {
            println!("{} {}", k, ele.output);
        }
    }
}

// opens the chosen file in the desired program
fn run(files: HashMap<String, Vec<File>>, command: Vec<String>) {
    // stores the name of the program to open a file in
    let program = command[0].to_string();
    // stores the short path of the file
    let short_path = command[1..].join(" ");
    // the program variable is used to find the vector of files from the HashMap
    let file_vec = files.get(&program).unwrap();
    // the path variable will store the full path to the file
    let mut path: Option<&str> = None;
    // joining Strings in filename

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
                Command::new(program)
                    .arg(format!("{}", value.split("file://").nth(1).unwrap()))
                    .output()
                    .expect("no such file or directory");
            }
        }
        None => eprintln!("error: no path"),
    }
}
