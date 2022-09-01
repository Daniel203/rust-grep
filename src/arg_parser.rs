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

    /// Select non-matching lines
    #[clap(short = 'v', long = "invert-match", help_heading = "Miscellaneous")]
    pub invert_match: bool,

    /// stop after NUM selected lines
    #[clap(
        short = 'm',
        long = "max-count",
        help_heading = "Output control",
        default_value_t = u32::MAX
    )]
    pub max_count: u32,
}

pub fn parse() -> Args {
    Args::parse()
}
