use clap::Parser;

pub fn parse() -> Args {
    Args::parse()
}

#[derive(Parser, Debug, Clone)]
pub struct Filter {
    #[arg(long, value_name = "KEY")]
    pub filter_tag_key: Vec<String>,
    #[arg(long, value_name = "VALUE")]
    pub filter_tag_value: Vec<String>,
    #[arg(long, value_name = "KEY=VALUE")]
    pub filter_tag_key_value: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    // Profile name to use in ~/.osc/config.json
    #[arg(long, short = 'p', default_value_t = String::from("default"))]
    pub profile: String,
    #[arg(long, default_value_t = false)]
    pub debug: bool,
    #[arg(value_enum, long, default_value_t = OutputFormat::Hour)]
    pub format: OutputFormat,
    #[arg(long, short = 'o')]
    pub output: Option<String>,
    #[arg(long, short = 'i')]
    pub input: Option<String>,
    #[command(flatten)]
    pub filter: Option<Filter>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Hour,
    Month,
    Json,
    Csv,
}
