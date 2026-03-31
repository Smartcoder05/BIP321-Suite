use crate::{error::URIError, types::{PopParam, Value}};
use urlencoding::decode;

pub fn validate_pop_scheme(pop: &PopParam) -> Result<String, URIError> {
    let pop_value = pop.get_value();
    let decoded_pop = decode(&pop_value).unwrap();

    let (before, _) = decoded_pop.split_once(":").unwrap_or((&decoded_pop, ""));

    match before {
        "https" | "http" | "file" | "javascript" | "mailto" => Err(URIError::ForbiddenPopScheme(before.to_string())),
        _ => Ok(decoded_pop.to_string())
    }
}

pub fn build_callback(pop: &PopParam, method: &str, hex: &str) -> Result<String, URIError> {
    let pop_value = pop.get_value();
    let pop_pay = validate_pop_scheme(pop)?;

    let decoded_pop = decode(&pop_pay)
        .map_err(|_| URIError::InvalidPopParameter(pop_value.clone()))
        .unwrap();

    let call_back = format!("{}{}=${}", decoded_pop, method, hex);
    Ok(call_back)
}