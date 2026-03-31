use bip321::{BitcoinURI, types::NoopHandler};

#[test]
fn test_duplicate_label_is_invalid() {
    let uri = "bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?label=Luke-Jr&label=Matt";
    let result = uri.parse::<BitcoinURI<NoopHandler>>();
    assert!(result.is_err());
}

#[test]
fn test_duplicate_amount_is_invalid() {
    let uri = "bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=42&amount=10";
    let result = uri.parse::<BitcoinURI<NoopHandler>>();
    assert!(result.is_err())
}

#[test]
fn test_duplicate_same_amount_is_invalid() {
    let uri = "bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=42&amount=42";
    let result = uri.parse::<BitcoinURI<NoopHandler>>();
    assert!(result.is_err())
}

#[test]
fn test_duplicate_pop_is_invalid() {
    let uri = "bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?pop=callback%3a&req-pop=callback%3a";
    let result = uri.parse::<BitcoinURI<NoopHandler>>();
    assert!(result.is_err())
}

#[test]
fn test_segwit_address() {
    let uri = "bitcoin:?bc=tb1qghfhmd4zh7ncpmxl3qzhmq566jk8ckq4gafnmg";
    let result = uri.parse::<BitcoinURI<NoopHandler>>();
    assert!(result.is_err())
}

#[test]
fn test_unknown_req_param() {
    let uri = "bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?req-somethingyoudontunderstand=50&req-somethingelseyoudontget=999";
    let result = uri.parse::<BitcoinURI<NoopHandler>>();
    assert!(result.is_err())
}

#[test]
fn test_unknown_req_param_with_invalid_callback() {
    let uri = "bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?req-pop=https%3aevilwebsite.com";
    let result = uri.parse::<BitcoinURI<NoopHandler>>();
    assert!(result.is_err())
}
