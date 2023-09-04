use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::Command;

use clap::Parser;
use fork::{daemon, Fork};
use htmlescape::encode_minimal;
use itertools::Itertools;
use regex::Regex;
use urlencoding::decode;

use crate::file::File;

mod file;
mod recently_used_xbel;

#[derive(Parser, Default, Debug)]
#[clap(trailing_var_arg = true)]
struct Arguments {
    /// Max number of recent files to list per program
    /// (0 = unlimited)
    #[clap(short, long, default_value_t = 5, verbatim_doc_comment)]
    limit: usize,

    /// Program(s) to exclude
    /// Take word-for-word from rofi-recent's output
    /// If excluding multiple programs, encase in quotes with a space between each program
    #[clap(short, long, default_value = "", hide_default_value = true, verbatim_doc_comment)]
    exclude: String,

    /// Shows paths for all files
    #[clap(short, long, default_value_t = false, num_args = 0, verbatim_doc_comment)]
    show_all_paths: bool,

    /// Command to run (program + file)
    /// Excluding this arg will print a list of recently-used files
    #[clap(required = false, num_args = 1.., value_delimiter = ' ', verbatim_doc_comment)]
    command: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // stores program names and their corresponding files
    let mut files = HashMap::<String, Vec<File>>::new();
    // parse args here in order to quickly skip files from excluded programs
    let args = Arguments::parse();
    let excluded: Vec<&str> = args.exclude.split(" ").collect::<Vec<_>>();
    // iterates through the bookmarks
    let recently_used = recently_used_xbel::parse_file()?;
    // create a HashMap to store file names
    for bookmark in recently_used.bookmarks {
        // application type, which is used for the icon and (sometimes) the program name
        let mimetype: String = bookmark.info.metadata.mime_type.mimetype;
        // Regex to find command without parameters
        let get_app = Regex::new(r"[a-zA-Z/]+").unwrap();
        for app in bookmark.info.metadata.app_parent.apps.iter() {
            // stores the command without parameters
            let cmd = get_app.find(&app.exec).unwrap().as_str();
            // if cmd is one of the excluded programs, then skip this file
            if excluded.contains(&cmd) {
                continue;
            }
            // store path and filename to variables
            let path = Path::new(&bookmark.href);
            let fname = path.file_name().unwrap().to_str().unwrap().to_string();
            // create a boolean to prevent duplicate entries
            let mut exists = false;
            // adds the command as a key to the HashMap if it doesn't already exist
            if !files.contains_key(cmd) {
                files.insert(cmd.to_string(), Vec::new());
            } else {
                for v in files.get_mut(cmd).unwrap() {
                    if path.eq(&v.path) {
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
                let mut ele = File {
                    path: path.to_path_buf(),
                    filename: decode(&fname)?.to_string(),
                    output: format!(
                        "\0icon\x1f{}\x1finfo\x1f{}\x1fmeta\x1f{} {} {}",
                        // get icon name by replacing forward slashes in type with dashes
                        mimetype.replace("/", "-"),
                        // store the absolute file path in ROFI_INFO environment variable
                        decode(&path.to_str().unwrap())?.to_string(),
                        // these three fields are added solely to make searching more thorough
                        app.name,
                        app.exec,
                        mimetype.split("application/x-").nth(1).unwrap_or("")
                    ),
                    date: bookmark.modified.to_string(),
                };
                // add paths to all outputs if flag was given
                if args.show_all_paths {
                    ele.add_path();
                }
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
    // adds paths to outputs if files of the same program have the same name
    if !args.show_all_paths {
        let mut all_need_path: Vec<(String, [usize; 2])> = Vec::new();
        for (k, v) in files.iter() {
            all_need_path.append(&mut need_path(&k, &v));
        }
        for (k, i) in all_need_path {
            files.get_mut(&k).unwrap().get_mut(i[0]).unwrap().add_path();
            files.get_mut(&k).unwrap().get_mut(i[1]).unwrap().add_path();
        }
    }

    // without any args, the program will use the HashMap to print a list
    if args.command.is_empty() {
        printer(files);
    }
    // with args, the program will attempt to open the file in the desired program
    else {
        run(args.command);
    }
    Ok(())
}

fn need_path(k: &String, v: &Vec<File>) -> Vec<(String, [usize; 2])> {
    let mut need_path = Vec::new();
    let it = 0..v.len();
    for pair in it.combinations(2) {
        let first: usize = pair.first().unwrap().to_owned();
        let last: usize = pair.last().unwrap().to_owned();
        if v.get(first).unwrap().filename == v.get(last).unwrap().filename {
            need_path.push((k.clone(), [first, last]));
        }
    }
    need_path
}

// prints a list of files from the HashMap
fn printer(files: HashMap<String, Vec<File>>) {
    println!("\0markup-rows\x1ftrue\n");
    for (k, v) in files {
        for ele in v {
            println!("{} {} {}", k, encode_minimal(&ele.filename), ele.output);
        }
    }
}

// opens the chosen file in the desired program
fn run(command: Vec<String>) {
    // stores the name of the program to open a file in
    let program = command[0].to_string();
    // the path variable will store the full path to the file
    let path = env::var("ROFI_INFO");

    // if path has a value, a command is executed in the background to open the file
    match path {
        Ok(value) => {
            if let Ok(Fork::Child) = daemon(false, false) {
                Command::new(program)
                    .arg(format!("{}", value.split("file://").nth(1).unwrap()))
                    .output()
                    .expect("no such file or directory");
            }
        }
        Err(_e) => println!("error: no path"),
    }
}
