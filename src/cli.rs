use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Change Directory to Bookmark",
    about = "A tool to bookmark directories in the command line.",
    usage = "cdb [FLAG] [OPTION] [--] [bookmark]"
)]
pub struct CommandLineArgs {
    /// List all bookmarks
    #[structopt(short, long)]
    pub list: bool,

    /// Outputs where <bookmark> is pointing
    #[structopt(short, long, empty_values = false, value_names = &["bookmark"])]
    pub show: Option<String>,

    /// Adds a directory bookmark with name <bookmark> pointing at <path>
    #[structopt(short, long, max_values = 2, min_values = 2, value_names = &["bookmark", "path"])]
    pub add: Option<Vec<String>>,

    /// Updates the path pointed by <bookmark>
    #[structopt(short, long, max_values = 2, min_values = 2, value_names = &["bookmark", "new_path"])]
    pub update: Option<Vec<String>>,

    /// Removes a directory bookmark with name <bookmark>
    #[structopt(short, long, empty_values = false, value_names = &["bookmark"])]
    pub remove: Option<String>,

    /// List all the bookmark names that start with <pattern>
    #[structopt(short, long, empty_values = false, value_names = &["pattern"])]
    pub find: Option<Option<String>>,

    /// Bookmark name
    #[structopt(empty_values = false)]
    pub bookmark: Option<String>,
}
