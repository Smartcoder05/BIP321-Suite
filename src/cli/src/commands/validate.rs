use std::process;

use bip321::{BitcoinURI, types::NoopHandler};

pub fn run(uri: String){
    let result = uri.parse::<BitcoinURI<NoopHandler>>();

    match result {
        Ok(_) => println!("Valid URI"),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1)
        }
    }

    process::exit(0)
}