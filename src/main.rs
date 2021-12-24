use web3::{
    contract::{Contract, Options},
    types::Address,
    Transport,
};

async fn available<T: Transport>(
    controller_contract: &Contract<T>,
    name: &str,
) -> web3::contract::Result<bool> {
    let name = name.to_string();
    let res = controller_contract.query("available", (name,), None, Options::default(), None);
    let avail: bool = res.await?;
    Ok(avail)
}

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        std::process::exit(1);
    }
    let name = &args[1];
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
    let res = available(&controller_contract, name);
    let is_available = res.await?;
    if is_available {
        println!("{}.eth is available", name);
    } else {
        println!("{}.eth is not available", name);
    }
    Ok(())
}
