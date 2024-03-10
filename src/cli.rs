use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Text that will bounce on the screen
    #[clap(global = true)]
    pub text: String,
    /// Speed of text movement
    #[clap(short, long, default_value_t = 24)]
    pub moves_per_second: u32,
}
