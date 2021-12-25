use clap::{AppSettings, Parser, Subcommand};
use std::path::PathBuf;
use web3::{
    contract::{Contract, Options},
    types::Address,
    Transport,
};

mod file_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let f = File::open(path)?;
            let reader = io::BufReader::new(f);
            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();
            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}

#[derive(Parser)]
#[clap(name = "ens-name-search")]
#[clap(about = "ENS name availability searcher")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Check the availability of one name
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Single { name: String },

    // Check the availability of many names, given by a line-deliminted file
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
    let CONTROLLER_ADDRESS: Address = "283Af0B28c62C092C9727F1Ee09c02CA627EB7F5".parse().unwrap();
    let controller_contract = Contract::from_json(
        web3.eth(),
        CONTROLLER_ADDRESS,
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
            let path = path.as_path();
            if !path.exists() {
                eprintln!("\"{}\" does not exist", path.display());
            }
            process_batch(&controller_contract, path).await.unwrap();
        }
    }
    Ok(())
}

async fn process_batch<T: Transport>(
    controller_contract: &Contract<T>,
    path: impl AsRef<std::path::Path>,
) -> std::io::Result<()> {
    let mut reader = file_reader::BufReader::open(path)?;
    let mut buffer = String::new();

    while let Some(line) = reader.read_line(&mut buffer) {
        let line = line?;
        let line = clean_name(line);
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
