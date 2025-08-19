use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::Command;

use clap::Parser;
use fork::{daemon, Fork};
use itertools::Itertools;
use recently_used_xbel::RecentlyUsed;
use url::Url;

use crate::file::{format_output, format_output_tail, File};

mod arguments;
mod file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = arguments::Arguments::parse();
    let excluded: Vec<&str> = args.exclude.split(" ").collect::<Vec<_>>();

    let recently_used = recently_used_xbel::parse_file()?;
    let mut files = store_files(recently_used, args.show_all_paths, &excluded)?;

    sort_and_truncate_files(&mut files, args.limit);

    // adds paths to outputs if files of the same program have the same filename
    if !args.show_all_paths {
        show_paths_when_needed(&mut files);
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

fn store_files(
    recently_used: RecentlyUsed,
    show_all_paths: bool,
    excluded: &Vec<&str>,
) -> Result<HashMap<String, Vec<File>>, Box<dyn std::error::Error>> {
    let mut files = HashMap::<String, Vec<File>>::new();
    // let get_app = Regex::new(r"[a-zA-Z/]+").unwrap();
    for bookmark in recently_used.bookmarks {
        // application type, which is used for the icon and (sometimes) the program name
        let info = match bookmark.info {
            None => continue,
            Some(x) => x,
        };
        let mimetype: String = match info.metadata.mime_type {
            None => "application/x-generic".to_string(),
            Some(x) => x.mime_type,
        };
        for app in info.metadata.applications.applications.iter() {
            // stores the command without parameters
            let cmd: &str = &app.exec[1..].split(" ").next().unwrap();
            // if cmd is one of the excluded programs, then skip this file
            if (*excluded).contains(&cmd) {
                continue;
            }
            // store path and filename to variables
            let url = Url::parse(&bookmark.href)?;
            let path: PathBuf = match url.to_file_path() {
                Err(_) => {
                    continue;
                }
                Ok(x) => x,
            };
            let fname: &str = match path.file_name() {
                None => "/",
                Some(value) => value.to_str().expect("/"),
            };
            // create a boolean to prevent duplicate entries
            let mut exists = false;
            // adds the command as a key to the HashMap if it doesn't already exist
            if !files.contains_key(cmd) {
                files.insert(cmd.to_string(), Vec::new());
            } else {
                for v in files.get_mut(cmd).unwrap() {
                    if path.eq(&v.path) {
                        v.output += &format_output_tail(
                            &app.name,
                            &app.exec,
                            &mimetype.split("application/x-").nth(1).unwrap_or(""),
                        );
                        exists = true;
                        break;
                    }
                }
            }
            if exists {
                continue;
            }
            // adds Files as values to the HashMap
            let mut ele = File {
                path: path.to_path_buf(),
                path_added: false,
                filename: fname.to_string(),
                output: format_output(
                    &mimetype.replace("/", "-"),
                    &path.to_str().unwrap(),
                    &app.name,
                    &app.exec,
                    mimetype.split("application/x-").nth(1).unwrap_or(""),
                ),
                date: bookmark.modified.to_string(),
            };
            // add paths to all outputs if flag was given
            if show_all_paths {
                ele.add_path();
            }
            files.get_mut(cmd).unwrap().push(ele);
        }
    }
    return Ok(files);
}

// sorts the files with most-recently-used files at the top and truncates the list if needed
fn sort_and_truncate_files(files: &mut HashMap<String, Vec<File>>, limit: usize) {
    let cmds: Vec<_> = files.keys().cloned().collect();
    for cmd in cmds {
        files
            .get_mut(&cmd)
            .unwrap()
            .sort_by(|a, b| b.date.cmp(&a.date));
        if limit > 0 {
            files.get_mut(&cmd).unwrap().truncate(limit);
        }
    }
}

// adds paths to files with non-unique names
fn show_paths_when_needed(files: &mut HashMap<String, Vec<File>>) {
    let mut all_need_path: Vec<(String, [usize; 2])> = Vec::new();
    for (k, v) in files.iter() {
        all_need_path.append(&mut need_path(&k, &v));
    }
    for (k, i) in all_need_path {
        files.get_mut(&k).unwrap().get_mut(i[0]).unwrap().add_path();
        files.get_mut(&k).unwrap().get_mut(i[1]).unwrap().add_path();
    }
}

// returns a vector that refers to the files which need paths added
// return value contains the command name and file numbers of the files which need paths added
fn need_path(k: &String, v: &Vec<File>) -> Vec<(String, [usize; 2])> {
    let mut need_path = Vec::new();
    let it = 0..(v.len());
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
            println!("{} {} {}", k, ele.filename, ele.output);
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
                    .arg(format!("{}", &value))
                    .output()
                    .expect("no such file or directory");
            }
        }
        Err(_e) => println!("error: no path"),
    }
}
