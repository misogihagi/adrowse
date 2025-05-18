use clap::{Parser, Subcommand};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct TemplateNotFoundError {
    path: String,
}

#[derive(Debug)]
enum AdrowseError {
    TemplateNotFound(TemplateNotFoundError),
}

impl From<AdrowseError> for String {
    fn from(_value: AdrowseError) -> Self {
        match _value {
            AdrowseError::TemplateNotFound(err) => {
                eprintln!("ADR template not found at '{}'.", &err.path);
                format!("ADR template not found at '{}'.", &err.path)
            }
        }
    }
}

impl std::error::Error for AdrowseError {}
impl std::fmt::Display for AdrowseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdrowseError::TemplateNotFound(_) => write!(f, "ADR template not found"),
        }
    }
}

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

fn find_max_number(dir_path: &Path) -> Result<u32, std::io::Error> {
    let entries = fs::read_dir(dir_path)?;
    let mut max_number: u32 = 1;

    for entry in entries {
        let path = entry?.path();

        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            if let Some(prefix) = file_name.split('-').next() {
                if let Ok(number) = prefix.parse::<u32>() {
                    if number > max_number {
                        max_number = number
                    }
                }
            }
        }
    }

    Ok(max_number)
}

fn load_config(config_path: &str) -> Result<Config, String> {
    match fs::read_to_string(config_path) {
        Ok(contents) => {
            toml::from_str(&contents).map_err(|e| format!("Failed to parse TOML format: {}", e))
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                Err(format!("Configuration file not found: {}", config_path))
            }
            other_error => Err(format!(
                "Failed to read configuration file: {}",
                other_error
            )),
        },
    }
}

fn add(arg: AddArg) -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = load_config(&arg.global_args.config_path)?;

    let title: String = Input::new()
        .with_prompt("Title of the ADR?")
        .interact_text()?;

    let template_index = if arg.template.is_some() {
        0
    } else {
        Select::new()
            .with_prompt("What do you choose?")
            .items(&TEMPLATES_INDEX)
            .interact()?
    };

    let dir_name_str = arg.path.unwrap_or(config.path);
    let dir_name = Path::new(&dir_name_str);
    let file_name = format!(
        "{:0pad$}-{}.md",
        find_max_number(dir_name)? + 1,
        title,
        pad = From::from(config.padding)
    );

    if arg.template.is_some() {
        let adr_path = Path::new(arg.template.as_ref().unwrap());

        if !adr_path.exists() {
            return Err(Box::new(AdrowseError::TemplateNotFound(
                TemplateNotFoundError {
                    path: format!("ADR template not found at '{}'.", &adr_path.display()),
                },
            )));
        }
    }
    let contents = if arg.template.is_some() {
        &fs::read_to_string(&arg.template.unwrap())?
    } else {
        TEMPLATES.get_templates(&config.locale)?[template_index]
    };

    let adr_path = dir_name.join(file_name);
    fs::write(&adr_path, contents).unwrap();

    println!("Created ADR template at '{}'.", &adr_path.display());
    Ok(())
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
