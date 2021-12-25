use clap::{AppSettings, Parser, Subcommand};
use std::{io::BufRead, path::PathBuf};
use web3::{
    contract::{Contract, Options},
    types::Address,
    Transport,
};

#[derive(Parser)]
#[clap(name = "ens-name-search")]
#[clap(about = "ENS name availability searcher")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check the availability of one name
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Single { name: String },

    /// Check the availability of many names, given by a line-delimited file, or "-" for stdin
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Batch {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
}

async fn available<T: Transport>(
    controller_contract: &Contract<T>,
    name: &str,
) -> web3::contract::Result<bool> {
    let name = name
        .to_string()
        .to_lowercase()
        .replace(|c| !char::is_alphanumeric(c), "");
    if name.chars().count() < 3 {
        return Ok(false);
    }
    let res = controller_contract.query("available", (name,), None, Options::default(), None);
    let avail: bool = res.await?;
    Ok(avail)
}

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let args = Cli::parse();

    let transport_uri =
        std::env::var("ETH_NODE_RPC").expect("Missing ETH_NODE_RPC environment variable");
    let transport = web3::transports::WebSocket::new(&transport_uri).await?;
    let web3 = web3::Web3::new(transport);
    let controller_address: Address = "283Af0B28c62C092C9727F1Ee09c02CA627EB7F5".parse().unwrap();
    let controller_contract = Contract::from_json(
        web3.eth(),
        controller_address,
        include_bytes!("../resources/controller_abi.json"),
    )?;

    match &args.command {
        Commands::Single { name } => {
            let res = available(&controller_contract, name);
            let is_available = res.await?;
            if is_available {
                println!("{}.eth is available", name);
            } else {
                println!("{}.eth is not available", name);
            }
        }
        Commands::Batch { path } => {
            match &path.to_str().unwrap() {
                // Accept from stdin
                &"-" => {
                    process_batch_stdin(&controller_contract).await.unwrap();
                }

                // Accept line-delimited file
                _ => {
                    let path = path.as_path();
                    if !path.exists() {
                        eprintln!("\"{}\" does not exist", path.display());
                        std::process::exit(1);
                    }
                    let f = std::fs::File::open(path).unwrap_or_else(|e| {
                        eprintln!("Could not open file at \"{}\": {:?}", path.display(), e);
                        std::process::exit(1);
                    });
                    process_batch(&controller_contract, f).await.unwrap();
                }
            }
        }
    }
    Ok(())
}

async fn process_batch<T: Transport>(
    controller_contract: &Contract<T>,
    file_handle: impl std::io::Read,
) -> std::io::Result<()> {
    let reader = std::io::BufReader::new(file_handle);

    for line in reader.lines() {
        let line = clean_name(&line?);
        let avail = available(&controller_contract, &line).await.unwrap();
        if avail {
            println!("{}", line);
        }
    }
    Ok(())
}

async fn process_batch_stdin<T: Transport>(
    controller_contract: &Contract<T>,
) -> std::io::Result<()> {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = clean_name(&line?);
        let avail = available(&controller_contract, &line).await.unwrap();
        if avail {
            println!("{}", line);
        }
    }
    Ok(())
}

fn clean_name(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .replace(|c| !char::is_alphanumeric(c), "")
}
