use crate::pop::validate_pop_scheme;
use crate::types::{Amount, BitcoinAddress, BitcoinURI, PaymentInstruction, PaymentInstructionHandler, PopParam};
use crate::error::URIError;
use urlencoding::decode;
use bitcoin::{Address, address::{AddressType::{P2pkh, P2tr, P2wpkh, P2wsh, P2a, P2sh}, NetworkUnchecked}};

pub fn parse<H: PaymentInstructionHandler>(input: &str, handler: H) -> Result<BitcoinURI<H>, URIError> {
    let after_scheme = strip_scheme(input)?;
    let (body, query) = split_body_and_query(after_scheme);
    let query_pairs = parse_query_pairs(query)?;
    
    classify_params(body, query_pairs, handler)
}

// strip the scheme
fn strip_scheme(input: &str) -> Result<&str, URIError> {
    let (before, after) = input.split_once(":").unwrap_or((input, ""));
     if before.to_lowercase() != "bitcoin" {
        return Err(URIError::InvalidAddress);
     }

     Ok(after)
}

// split body(address) and query
fn split_body_and_query(after: &str) -> (&str, &str) {
    let (body, query) = after.split_once("?").unwrap_or((after, ""));
    (body, query)
}

// parse the query pairs
fn parse_query_pairs(query: &str) -> Result<Vec<(String, String)>, URIError> {
    let mut query_pairs:Vec<(String, String)> = vec![];
    let queries:Vec<&str> = query.split("&").collect();
    for params in queries {
        let (key, value) = params.split_once('=').unwrap_or((params, ""));
        let query_tuple = (key.to_lowercase(), value.to_lowercase());
        query_pairs.push(query_tuple);
    }

    Ok(query_pairs)
}

pub fn validate_address(body: &str) -> Result<Option<BitcoinAddress>, URIError> {
    let address = body.parse::<Address<NetworkUnchecked>>()
        .map_err(|_| URIError::InvalidAddress)?;

    let checked = address.clone().assume_checked();
    let network = match checked.address_type() {
        Some(P2pkh) | Some(P2sh) =>{ BitcoinAddress::Base58(address)},
        Some(P2wpkh) | Some(P2wsh) => BitcoinAddress::Bech32(address),
        Some(P2a) | Some(P2tr) => BitcoinAddress::Bech32m(address),
        _ => return Err(URIError::InvalidAddress),
    };

    Ok(Some(network))
}
// classify and validate each key

//  helper functions
fn handle_amount(value: &str, amount: &mut Option<Amount>) -> Result<(), URIError> {
    if amount.is_some() {
        return Err(URIError::DuplicateParam("amount".to_string()));
    }
    *amount = value.parse().ok();
    Ok(())
}

fn handle_label(value: &str, label: &mut Option<String>) -> Result<(), URIError> {
    if label.is_some() {
        return Err(URIError::DuplicateParam("amount".to_string()));
    }
    *label = Some(decode(value).unwrap().to_string());
    Ok(())
}

fn handle_message(value: &str, message: &mut Option<String>) -> Result<(), URIError> {
    if message.is_some() {
        return Err(URIError::DuplicateParam("amount".to_string()));
    }
    *message = Some(decode(value).unwrap().to_string());
    Ok(())
}

fn handle_pop(
    value: &str, 
    pop: &mut Option<PopParam>,
    variant: fn(String) -> PopParam
) -> Result<(), URIError> {
    if pop.is_some() {
        return Err(URIError::DuplicateParam("pop".to_string()));
    }

    *pop = Some(variant(value.to_string()));
    Ok(())
}

fn handle_req_pop(
    value: &str, 
    pop: &mut Option<PopParam>,
    variant: fn(String) -> PopParam
) -> Result<(), URIError> {
    if pop.is_some() {
        return Err(URIError::BothPopAndReqPop);
    }

    if validate_pop_scheme(&PopParam::Required(value.to_string())).is_err() {
        return Err(URIError::RequiredPopNotSupported);
    }

    *pop = Some(variant(value.to_string()));
    Ok(())
}

fn handle_instruction(
    value: &str, 
    instructions: &mut Vec<PaymentInstruction>, 
    variant: fn(String) -> PaymentInstruction
) -> Result<(), URIError> {
    instructions.push(variant(value.to_string()));
    Ok(())
}

fn handle_segwit(key: &str, value: &str, segwit: &mut Vec<BitcoinAddress>) -> Result<(), URIError>{
    if !value.starts_with(key) {
        return Err(URIError::InvalidAddress);
    } 

    let address = match validate_address(value)? {
        Some(addr) => addr,
        None => return Err(URIError::InvalidAddress)
    };

    segwit.push(address);
    Ok(())
    
}

fn handle_unknown<H: PaymentInstructionHandler>(key: &str, 
    value: &str, 
    unknown: &mut Vec<(String, String)>, 
    handler: &mut H
) -> Result<(), URIError> {
    let unknown_pairs: (String, String) = (key.to_string(), value.to_string());
    let key = &unknown_pairs.0;
    if handler.is_supported(key){
        handler.handle(key, value)?
    } else if key.starts_with("req-") {
        return Err(URIError::UnknownRequiredParam(key.to_string()));
    } else{
      unknown.push(unknown_pairs);  
    }
    
    Ok(())
}

fn classify_params<H: PaymentInstructionHandler>(
    body: &str, 
    pairs: Vec<(String, String)>,
    mut handler: H
) -> Result<BitcoinURI<H>, URIError> {
    
    let address: Option<BitcoinAddress> = if !body.is_empty() {
        validate_address(body)?
    } else {
        None
    };
       
    let mut amount:Option<Amount> = None;
    let mut label: Option<String> = None;
    let mut message: Option<String> = None;
    let mut pop: Option<PopParam> = None;
    let mut instructions: Vec<PaymentInstruction> = vec![];
    let mut segwit: Vec<BitcoinAddress> = vec![];
    let mut unknown: Vec<(String, String)> = vec![];

    if !body.is_empty() {
        let inner_address = match address.clone() {
        Some(addr) => addr,
        None => return Err(URIError::InvalidAddress)
    };
        instructions.push(PaymentInstruction::Onchain(inner_address));
    }
    
    for (key, value) in pairs {
        match key.as_str(){
            "amount" => handle_amount(&value, &mut amount)?,
            "label" => handle_label(&value, &mut label)?,
            "message" => handle_message(&value, &mut message)?,
            "pop" => handle_pop(&value, &mut pop, PopParam::Optional)?,
            "req-pop" => handle_req_pop(&value, &mut pop, PopParam::Required)?,
            "lightning" => handle_instruction(&value, &mut instructions, PaymentInstruction::Lightning)?,
            "lno" => handle_instruction(&value, &mut instructions, PaymentInstruction::Bolt12)?,
            "sp" => handle_instruction(&value, &mut instructions, PaymentInstruction::SilentPayment)?,
            "bc" | "tb" => handle_segwit(&key, &value, &mut segwit)?,
            _ => handle_unknown(&key, &value, &mut unknown, &mut handler)?,
        }
    }

    if !(segwit.is_empty()) {
        instructions.push(PaymentInstruction::Segwit(segwit));
    }
    let extra = if !handler.is_empty() {
        Some(handler)
    } else {
        None
    };
    
    Ok(BitcoinURI{address, amount, label, message, pop, instructions, unknown, extra})
}
