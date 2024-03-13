use std::path::PathBuf;

use dirs::home_dir;
use htmlescape::encode_minimal;
use urlencoding::decode;

#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
    pub path_added: bool,
    pub filename: String,
    pub output: String,
    pub date: String,
}

impl File {
    // add path to file output
    pub fn add_path(&mut self) {
        if self.path_added == true {
            return;
        }
        let path_str = self.path.to_str().unwrap();
        self.output = format!(
            " <i><small>{}</small></i> {}",
            encode_minimal(
                &decode(path_str)
                    .unwrap()
                    // remove file uri and filename from path
                    .to_string()[("file://".len())..(path_str.len() - self.filename.len())]
                    .replace(home_dir().unwrap().to_str().unwrap(), "~")
            ),
            &self.output
        );
        self.path_added = true;
    }
}

pub fn format_output(icon: &str, absolute_path: &str, app_name: &str, app_exec: &str, mimetype: &str) -> String {
    format!(
        "\0icon\x1f{}\x1finfo\x1f{}\x1fmeta\x1f{}",
        // get icon name by replacing forward slashes in type with dashes
        icon,
        // store the absolute file path in ROFI_INFO environment variable
        absolute_path,
        // these three fields are added solely to make searching more thorough
        format_output_tail(app_name, app_exec, mimetype)
    )
}

pub fn format_output_tail(app_name: &str, app_exec: &str, mimetype: &str) -> String {
    format!(
        " {} {} {}",
        app_name,
        app_exec,
        mimetype
    )
}
