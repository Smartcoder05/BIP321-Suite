use bech32::Bech32m;
use bech32::primitives::decode::CheckedHrpstring;
use bip321::URIError;

#[derive(Debug)]
pub enum Network {
    Empty,
    Mainnet,
    Testnet,
}

pub fn validate_address(addr: String) -> Result<(Network, String), URIError>{
    let decoded = CheckedHrpstring::new::<Bech32m>(&addr)
        .map_err(|_| URIError::SilentPaymentError(addr.clone()))?;

    let network: Network = match decoded.hrp().to_string().as_str() {
        "sp" => {
            Network::Testnet
        },
        "tsp" => {
            Network::Mainnet
        },
        _ => {
            Network::Empty
        }
    };
    Ok((network, addr))
}

