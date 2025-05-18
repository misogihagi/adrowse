use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
pub struct GlobalArgs {
    #[arg(default_value = ".adr/config", global = true, long)]
    pub config_path: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    global_args: GlobalArgs,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Parser)]
struct AddArg {
    #[clap(flatten)]
    global_args: GlobalArgs,

    #[arg(short, long)]
    path: Option<String>,

    #[arg(short, long)]
    template: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Add(AddArg),
    Init,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    locale: String,
    path: String,
    padding: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            locale: "en".to_string(),
            path: "docs/decisions".to_string(), // teamwork-advice-for-adrs
            padding: 5,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
