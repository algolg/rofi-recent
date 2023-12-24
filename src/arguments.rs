use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(trailing_var_arg = true)]
pub struct Arguments {
    /// Max number of recent files to list per program
    /// (0 = unlimited)
    #[clap(short, long, default_value_t = 5, verbatim_doc_comment)]
    pub limit: usize,

    /// Program(s) to exclude
    /// Take word-for-word from rofi-recent's output
    /// If excluding multiple programs, encase in quotes with a space between each program
    #[clap(
        short,
        long,
        default_value = "",
        hide_default_value = true,
        verbatim_doc_comment
    )]
    pub exclude: String,

    /// Shows paths for all files
    #[clap(
        short,
        long,
        default_value_t = false,
        num_args = 0,
        verbatim_doc_comment
    )]
    pub show_all_paths: bool,

    /// Command to run (program + file)
    /// Excluding this arg will print a list of recently-used files
    #[clap(required = false, num_args = 1.., value_delimiter = ' ', verbatim_doc_comment)]
    pub command: Vec<String>,
}
