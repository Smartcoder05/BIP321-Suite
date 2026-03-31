use bip321::{build_callback, types::{BitcoinURI, NoopHandler}};

#[test]
fn test_build_callback() {
    let uri = "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?pop=initiatingapp%3a";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    let pop_param = result.pop.unwrap();
    let hexdata = "HEX_ENCODED_TRANSACTION";
    let callback = build_callback(&pop_param, "onchain", hexdata).unwrap();
    assert_eq!(callback, "initiatingapp:onchain=$HEX_ENCODED_TRANSACTION");
}

#[test]
fn test_build_callback_with_encoded_pop() {
    let uri = "bitcoin:?lightning=lnbc420bogusinvoice&pop=callbackuri%3abody%3fpop=";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    let pop_param = result.pop.unwrap();
    let hexdata = "HEX_ENCODED_PAYMENT_PREIMAGE";
    let callback = build_callback(&pop_param, "lightning", hexdata).unwrap();
    assert_eq!(callback, "callbackuri:body?pop=lightning=$HEX_ENCODED_PAYMENT_PREIMAGE");
}

#[test]
fn test_build_callback_double_with_onchain() {
    let uri = "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?lightning=lnbc420bogusinvoice&pop=app%3a%3f";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    let pop_param = result.pop.unwrap();
    let hexdata = "HEX_ENCODED_TRANSACTION";
    let callback = build_callback(&pop_param, "onchain", hexdata).unwrap();
    assert_eq!(callback, "app:?onchain=$HEX_ENCODED_TRANSACTION");
}

#[test]
fn test_build_callback_double_with_lightning() {
    let uri = "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?lightning=lnbc420bogusinvoice&pop=app%3a%3f";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    let pop_param = result.pop.unwrap();
    let hexdata = "HEX_ENCODED_TRANSACTION";
    let callback = build_callback(&pop_param, "lightning", hexdata).unwrap();
    assert_eq!(callback, "app:?lightning=$HEX_ENCODED_TRANSACTION");
}

#[test]
fn test_build_callback_with_invalid_scheme() {
    let uri = "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?pop=https%3aiwantyouripaddress.com";
    let result = uri.parse::<BitcoinURI<NoopHandler>>().unwrap();
    let pop_param = result.pop.unwrap();
    let hexdata = "HEX_ENCODED_TRANSACTION";
    let callback_result = build_callback(&pop_param, "onchain", hexdata);
    assert!(callback_result.is_err());
}
