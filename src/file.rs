use std::path::PathBuf;

use dirs::home_dir;
use htmlescape::encode_minimal;
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
            encode_minimal(
                &decode(&self.path.to_str().unwrap())
                    .unwrap()
                    .to_string()
                    .split(&self.filename)
                    .nth(0)
                    .unwrap()
                    .split("file://")
                    .nth(1)
                    .unwrap_or("")
                    .replace(home_dir().unwrap().to_str().unwrap(), "~")
            ),
            &self.output,
        )
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
