use crate::{PaymentInstruction, types::{Amount, BitcoinAddress, BitcoinURI, PaymentInstructionHandler, PopParam, Value}};
use urlencoding::encode;

pub fn build<H: PaymentInstructionHandler>(uri: &BitcoinURI<H>) -> String {
    let body = build_body(uri);
    let query = build_query(uri);
    match query.is_empty() {
        true => format!("bitcoin:{}", body),
        false => format!("bitcoin:{}?{}", body, query)
     }
}

fn build_body<H: PaymentInstructionHandler>(uri: &BitcoinURI<H>) ->String {
    let body = uri.address.clone();

    match body {
        Some(BitcoinAddress::Base58(s)) => s.assume_checked().to_string(),
        Some(BitcoinAddress::Bech32(s)) => s.assume_checked().to_string(),
        Some(BitcoinAddress::Bech32m(s)) => s.assume_checked().to_string(),
        None => String::new(),
    }
}

fn satoshi_to_btc_converter(amount: &Amount) -> String{
    (amount.0 as f64 / 100_000_000.0).to_string()
}



fn build_query<H: PaymentInstructionHandler>(uri: &BitcoinURI<H>) -> String {
    let mut query: Vec<String> = vec![];
    if let Some(s) = &uri.amount {
        query.push("amount=".to_string() + &satoshi_to_btc_converter(s));
    } 

    if let Some(s) = &uri.label {
        query.push("label=".to_string() + &encode(s))
    }

    if let Some(s) = &uri.message {
        query.push("message=".to_string() + &encode(s));
    }

    match &uri.pop {
        Some(PopParam::Required(s)) => query.push("req-pop=".to_string() + s),
        Some(PopParam::Optional(s)) => query.push("pop=".to_string() + s),
        _ => (),
    }
    for instruction in &uri.instructions {
        if matches!(instruction, PaymentInstruction::Onchain(_)) {
            continue;
        }
        let value = instruction.get_value();
        query.push(value);
    }

    // let some_instrcution = &uri.instructions.get
    query.iter().filter(|s| !s.is_empty()).cloned().collect::<Vec<String>>().join("&")

}
