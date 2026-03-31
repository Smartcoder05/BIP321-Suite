use bip321::{PaymentInstruction, PaymentResolver, types::{Value, NoopHandler}, BitcoinURI};
use bitcoin::Network;

struct MainnetOnchainResolver;

impl PaymentResolver for MainnetOnchainResolver {
    type Output = String;
    fn supports(instruction: &PaymentInstruction) -> bool {
        matches!(instruction, PaymentInstruction::Onchain(_))
    }

    fn resolve(instruction: &PaymentInstruction) -> Option<Self::Output> {
        if !instruction.get_value().is_empty() {
            return Some(instruction.get_value())
        }
        None 
    }

    fn network() -> Option<Network> {
        return Some(Network::Bitcoin);
    }
}

#[test]
fn test_valid_network() {
    let uri = "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    let resolved = result.resolve_instructions::<String, MainnetOnchainResolver>();
    assert_eq!(resolved[0], "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa")
}

#[test]
fn test_invalid_network() {
    let uri = "bitcoin:tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    let resolved = result.resolve_instructions::<String, MainnetOnchainResolver>();
    assert_eq!(resolved.len(), 0)
}