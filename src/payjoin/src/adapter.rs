use bip321::URIError;
use url::Url;

pub fn validate_pj_url(input: String) -> Result<Url, URIError> {
    let url = Url::parse(&input)
        .map_err(|_| URIError::PayjoinError("Invalid URL".to_string()))?;
    
    let scheme = url.scheme();
    let host = url.host_str();
    match scheme {
        "https" => {},
        "http" => {
            if host != Some("localhost") {
                return Err(URIError::PayjoinError("Invalid URL".to_string()));
            } 
        }
        _ => return Err(URIError::PayjoinError("Invalid URL".to_string()))
    }
  
  Ok(url)
}