use bip321::{BitcoinAddress, PaymentInstruction, types::NoopHandler, BitcoinURI};

#[test]
fn test_valid_address() {
    let uri = "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    let result = uri.parse::<BitcoinURI<NoopHandler>>();
    let b_uri = result.clone().unwrap();
    let temp_addr = result.clone().unwrap().address.unwrap();
    let address = match  temp_addr {
        bip321::BitcoinAddress::Base58(s) => s,
        bip321::BitcoinAddress::Bech32(s) => s,
        bip321::BitcoinAddress::Bech32m(s) => s
    }; 
    assert_eq!(b_uri.to_string(), uri);
    assert_eq!(address.assume_checked().to_string(), "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_valid_address_with_label() {
    let uri = "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?label=Luke-Jr";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    let address = match result.address.unwrap() {
        BitcoinAddress::Base58(s) => s,
        BitcoinAddress::Bech32(s) => s,
        BitcoinAddress::Bech32m(s) => s
    };

    let label = result.label.unwrap();
    assert_eq!(label.as_str(), "Luke-Jr".to_lowercase());
    assert_eq!(address.assume_checked().to_string(), "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string());
}

#[test]
fn test_valid_address_label_amount() {
    let uri = "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?amount=20.3&label=Luke-Jr";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    let address = match result.address.unwrap() {
        BitcoinAddress::Base58(s) => s,
        BitcoinAddress::Bech32(s) => s,
        BitcoinAddress::Bech32m(s) => s
    }; 
    let label = result.label.unwrap();
    let amount = result.amount.unwrap();
    assert_eq!(label.as_str(), "Luke-Jr".to_lowercase());
    assert_eq!(address.assume_checked().to_string(), "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string());
    assert_eq!(amount.0, 2030000000);
}

#[test]
fn test_valid_address_message_amount() {
    let uri = "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?amount=50&label=Luke-Jr&message=Donation%20for%20project%20xyz";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    let address = match result.address.unwrap() {
        bip321::BitcoinAddress::Base58(s) => s,
        bip321::BitcoinAddress::Bech32(s) => s,
        bip321::BitcoinAddress::Bech32m(s) => s
    }; 
    let label = result.label.unwrap();
    let message = result.message.unwrap();
    let amount = result.amount.unwrap();
    assert_eq!(label.as_str(), "Luke-Jr".to_lowercase());
    assert_eq!(message.as_str(), "Donation for project xyz".to_lowercase());
    assert_eq!(address.assume_checked().to_string(), "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string());
    assert_eq!(amount.0, 5000000000);
}

#[test]
fn test_bolt11_with_onchain_fallback() {
    let uri = "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?lightning=lnbc420bogusinvoice";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    assert!(matches!(
        &result.instructions[1], PaymentInstruction::Lightning(s) if s == "lnbc420bogusinvoice"
    ));

    assert!(matches!(
        &result.instructions[0], 
        PaymentInstruction::Onchain(s) if s.inner().clone()
                                           .assume_checked()
                                           .to_string() == "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
    ))
}

#[test]
fn test_bolt11_with_no_fallback() {
    let uri = "bitcoin:?lightning=lnbc420bogusinvoice";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    assert!(matches!(
        &result.instructions[0], 
        PaymentInstruction::Lightning(s) if s == "lnbc420bogusinvoice"
    ));
}

#[test]
fn test_bolt12_with_no_fallback() {
    let uri = "bitcoin:?lno=lno1bogusoffer";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    assert!(matches!(
        &result.instructions[0], PaymentInstruction::Bolt12(s) if s == "lno1bogusoffer"
    ));
}

#[test]
fn test_bolt12_sp_with_no_fallback() {
    let uri = "bitcoin:?lno=lno1bogusoffer&sp=sp1qsilentpayment";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    assert!(matches!(
        &result.instructions[0], PaymentInstruction::Bolt12(s) if s == "lno1bogusoffer"
    ));

    assert!(matches!(
        &result.instructions[1], PaymentInstruction::SilentPayment(s) if s == "sp1qsilentpayment"
    ));
}

#[test]
fn test_sp_with_no_fallback() {
    let uri = "bitcoin:?sp=sp1qsilentpayment";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();

    assert!(matches!(
        &result.instructions[0], PaymentInstruction::SilentPayment(s) if s == "sp1qsilentpayment"
    ));
}

#[test]
fn test_bolt12_sp_with_fallback() {
    let uri = "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?sp=sp1qsilentpayment";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    assert!(matches!(
        &result.instructions[0], 
        PaymentInstruction::Onchain(s) if s.inner().clone()
                                            .assume_checked()
                                            .to_string() == "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
    ));

    assert!(matches!(
        &result.instructions[1], PaymentInstruction::SilentPayment(s) if s == "sp1qsilentpayment"
    ));
}

#[test]
fn test_variables_not_understood() {
    let uri = "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?somethingyoudontunderstand=50&somethingelseyoudontget=999";
    let result = uri.parse::<BitcoinURI<NoopHandler>>();
    assert!(result.is_ok())

}

#[test]
fn test_multiple_segwit_address() {
    let uri = "bitcoin:?bc=bc1qufgy354j3kmvuch987xe4s40836x3h0lg8f5n2&bc=bc1p5swkugezn97763tl0yty6556856uug0q6jflljvep9m4p7339x5qzyrh4g";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    assert!(matches!(
        &result.instructions[0],
        PaymentInstruction::Segwit(v) if v.len() == 2
    ))
}


#[test]
fn test_case_sensitivity_with_fallback() {
    let uri = "BITCOIN:BC1QUFGY354J3KMVUCH987XE4S40836X3H0LG8F5N2?BC=BC1P5SWKUGEZN97763TL0YTY6556856UUG0Q6JFLLJVEP9M4P7339X5QZYRH4G";
    let result = uri.parse::<BitcoinURI<NoopHandler>>();
    assert!(result.is_ok())
}

#[test]
fn test_case_sensitivity_with_no_fallback() {
    let uri = "BITCOIN:?BC=BC1QUFGY354J3KMVUCH987XE4S40836X3H0LG8F5N2&BC=BC1P5SWKUGEZN97763TL0YTY6556856UUG0Q6JFLLJVEP9M4P7339X5QZYRH4G";
    let result = uri.parse::<BitcoinURI<NoopHandler>>();
    assert!(result.is_ok())
}

#[test]
fn test_tb_parameter() {
    let uri = "bitcoin:?tb=tb1qghfhmd4zh7ncpmxl3qzhmq566jk8ckq4gafnmg";
    let result = uri.parse::<BitcoinURI<NoopHandler>>();
    assert!(result.is_ok())
}
