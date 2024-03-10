use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Text that will bounce on the screen
    #[clap(global = true)]
    pub text: String,
}
