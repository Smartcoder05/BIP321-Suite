use std::process;
use bip321::{BitcoinURI, types::NoopHandler, pop::build_callback};

pub fn run(uri: String, method: String, proof: String) {
    let result = uri.parse::<BitcoinURI<NoopHandler>>();

    match result {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1)
        }
    }

    let pop_value = result.unwrap().pop.unwrap();
    let pop = build_callback(&pop_value, &method, &proof);
    if pop.is_err() {
        println!("Invalid Pop");
        process::exit(1)
    }
    match pop {
        Ok(_) => println!("The pop is: {:?}", pop.unwrap()),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1)
        }
    }

    process::exit(0)
}