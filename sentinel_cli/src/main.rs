use crate::workspace::{WORKSPACE_CARGO_TOML, WORKSPACE_LIB_RS, WORKSPACE_MANIFEST_TOML};
use clap::{Parser, Subcommand};
use sentinel_core::dag::manifest::{Manifest, PlanError};

mod workspace;

const MANIFEST_PATH: &'static str = "./manifest.toml";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init { workspace_name: String },
    Plan,
    Apply,
}

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Some(Commands::Init { workspace_name }) => init(workspace_name).unwrap(),
        Some(Commands::Plan) => plan().unwrap(),
        Some(Commands::Apply) => apply(),
        None => println!("not a command"),
    }
}

fn init(workspace_name: String) -> anyhow::Result<()> {
    std::fs::create_dir_all(format!("{}/udfs/src", workspace_name))?;

    std::fs::write(
        format!("{}/manifest.toml", workspace_name),
        WORKSPACE_MANIFEST_TOML,
    )?;
    std::fs::write(
        format!("{}/udfs/Cargo.toml", workspace_name),
        WORKSPACE_CARGO_TOML,
    )?;
    std::fs::write(
        format!("{}/udfs/src/lib.rs", workspace_name),
        WORKSPACE_LIB_RS,
    )?;

    Ok(())
}

fn plan() -> Result<(), PlanError> {
    let manifest_str = std::fs::read_to_string(MANIFEST_PATH)?;
    let manifest = toml::from_str::<Manifest>(&manifest_str)?;
    let sorted_dag = sentinel_core::dag::plan::plan_dag(&manifest)?;
    let result = sorted_dag
        .iter()
        .map(|op| op.udf_name.as_str())
        .collect::<Vec<_>>()
        .join(" -> ");
    println!("{}", result);

    let plan_toml = std::path::Path::new("./").join("plan.toml");
    std::fs::copy(MANIFEST_PATH, plan_toml)?;

    Ok(())
}

fn apply() {}
