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

fn init(arg: Cli) -> std::io::Result<()> {
    let config_path = Path::new(&arg.global_args.config_path);

    if !config_path.exists() {
        let config = Config::default();
        let toml = toml::to_string(&config).unwrap();
        if config_path.parent().is_some() {
            fs::create_dir_all(config_path.parent().unwrap())?;
        }
        let mut file = fs::File::create(config_path)?;
        file.write_all(toml.as_bytes())?;
        println!(
            "Created default configuration file at '{}'.",
            config_path.display()
        );
        Ok(())
    } else {
        println!(
            "Configuration file '{}' already exists.",
            config_path.display()
        );
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Add(arg) => add(arg)?,
        Commands::Init => init(args)?,
    };

    Ok(())
}
