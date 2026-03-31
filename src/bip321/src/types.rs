use std::{str::FromStr};
use std::fmt::{Debug, Display};
use bitcoin::{Address, Network, address::NetworkUnchecked};
use crate::build;
use crate::{error::URIError, parse};

pub trait Value {
    fn get_value(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum BitcoinAddress {
    Base58(Address<NetworkUnchecked>),
    Bech32m(Address<NetworkUnchecked>),
    Bech32(Address<NetworkUnchecked>),
}

impl BitcoinAddress {
    pub fn inner(&self) -> &Address<NetworkUnchecked> {
        match self {
            BitcoinAddress::Base58(address) => address,
            BitcoinAddress::Bech32(address) => address,
            BitcoinAddress::Bech32m(address) => address
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Amount (pub u64);
impl FromStr for Amount {
    type Err = URIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let btc:f64 = s.parse::<f64>().map_err(|_| URIError::InvalidAmount(s.to_string()))?;
        let satoshis = (btc * 100_000_000.0).round() as u64;
        Ok(Amount(satoshis))
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum PopParam {
    Empty,
    Optional(String),
    Required(String),
}
impl PopParam {
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}
impl Value for PopParam {
    fn get_value(&self) -> String {
        match self {
            PopParam::Required(s) => s.to_string(),
            PopParam::Optional(s) => s.to_string(),
            PopParam::Empty => "".to_string()
        }
    }
}

pub trait PaymentResolver {
    type Output;
    fn supports(instruction: &PaymentInstruction) -> bool;
    fn resolve(instruction: &PaymentInstruction) -> Option<Self::Output>;
    fn network() -> Option<Network>;
}



#[derive(Debug, Clone, PartialEq)]
pub enum PaymentInstruction {
    Empty,
    Onchain(BitcoinAddress),
    Lightning(String),
    Bolt12(String),
    SilentPayment(String),
    Segwit(Vec<BitcoinAddress>),
    Unknown(String, String),
}

impl Value for PaymentInstruction {
    fn get_value(&self) -> String {
        match self {
            PaymentInstruction::Bolt12(s) => format!("lno={}", s),
            PaymentInstruction::Lightning(s) => format!("ligthning={}", s),
            PaymentInstruction::Onchain(s) => s.inner().clone()
                                                                            .assume_checked()
                                                                            .to_string(),
            PaymentInstruction::Segwit(s) => {
                s.iter().map(|x| "bc=".to_owned() + &x.inner().clone()
                                                                            .assume_checked()
                                                                            .to_string())
                                                                            .collect::<Vec<String>>()
                                                                            .join("&")
            },
            PaymentInstruction::SilentPayment(s) => format!("sp={}",s),
            PaymentInstruction::Unknown(key, value ) => {
                key.to_string() + "=" + value
            },
            PaymentInstruction::Empty => String::new(),

        }
    }
}

pub trait PaymentInstructionHandler: Default + Debug {
    fn handle(&mut self, key: &str, value: &str) -> Result<(), URIError>;
    fn is_supported(&self, key: &str) -> bool;
    fn is_empty(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BitcoinURI<H: PaymentInstructionHandler> {
    pub address: Option<BitcoinAddress>,
    pub amount: Option<Amount>,
    pub label: Option<String>,
    pub message: Option<String>,
    pub pop: Option<PopParam>,
    pub instructions: Vec<PaymentInstruction>,
    pub unknown: Vec<(String, String)>,
    pub extra: Option<H>
}

impl<H: PaymentInstructionHandler> BitcoinURI<H> {
    pub fn resolve_instruction<T, R: PaymentResolver<Output = T>>(&self) -> Option<T> {
        for instruction in &self.instructions {
            if R::supports(instruction) {
                return R::resolve(instruction);
            }
        }
        None
    }

    pub fn resolve_instructions<T, R: PaymentResolver<Output = T>>(&self) -> Vec<T> {
        let mut results = Vec::new();
        for instruction in &self.instructions {
            if R::supports(instruction) {
                if let PaymentInstruction::Onchain(addr) = instruction
                    && let Some(network) = R::network() {
                        let checked = addr.inner().clone()
                                    .require_network(network);
                        if checked.is_err() {
                            continue; // Skip this instruction if the network doesn't match     
                    }
                }

            
            if let Some(result) = R::resolve(instruction) {
                    results.push(result);
            }
        }
        
    }
    results
}}

impl<H: PaymentInstructionHandler + Default> FromStr for BitcoinURI<H> {
    type Err = URIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s, H::default())
    }
}

impl<H: PaymentInstructionHandler + Default> Display for BitcoinURI<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uri = build(self);
        write!(f, "{}", uri)
    }
}


// This struct used as a default struct for PaymentInstructionHandler
#[derive(Default, Clone, Debug)]
pub struct NoopHandler;

impl PaymentInstructionHandler for NoopHandler {
    fn handle(&mut self, _key: &str, _value: &str) -> Result<(), URIError> {
        Ok(())
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn is_supported(&self, _key: &str) -> bool {
        false
    }
}
