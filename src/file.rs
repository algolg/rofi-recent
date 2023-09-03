use std::path::PathBuf;

use dirs::home_dir;
use urlencoding::decode;

#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
    pub filename: String,
    pub output: String,
    pub date: String,
}

impl File {
    // add path to file output
    pub fn add_path(&mut self) {
        self.output = format!(
            " <i><small>{}</small></i> {}",
            decode(&self.path.to_str().unwrap())
                .unwrap()
                .to_string()
                .split(&self.filename)
                .nth(0)
                .unwrap()
                .split("file://")
                .nth(1)
                .unwrap_or("")
                .replace(home_dir().unwrap().to_str().unwrap(), "~")
                .replace("&", "&amp;"),
            &self.output,
        )
    }
}
