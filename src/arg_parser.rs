use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Word that you need to search
    #[clap()]
    pub word: String,

    /// Path of the file where you need to search
    #[clap()]
    pub path: Vec<std::path::PathBuf>,

    /// Ignore case distinctions in patterns and data
    #[clap(
        short = 'i',
        long = "ignore-case",
        help_heading = "Pattern selection and interpretation"
    )]
    pub ignore_case: bool,
}

pub fn parse() -> Args {
    Args::parse()
}
