use std::process;

use bip321::{BitcoinURI, types::{NoopHandler, Value}};


pub fn run(uri: String) {
    let result = uri.parse::<BitcoinURI<NoopHandler>>();
 
    match result {
        Ok(_) => (),
        Err(e) => { 
            println!("Error: {}", e);
            process::exit(1)
        }
    }

    let props = result.clone().unwrap();
    let address = props.address.map(|p| p.inner()
                                        .clone().assume_checked().to_string())
                                            .unwrap_or("None".to_string());
    
    let amount = props.amount.map(|a| format!("{} BTC", a.0 as f64 / 100_000_000.0))
                                                .unwrap_or("None".to_string());
    
    let label = props.label.unwrap_or("None".to_string());
    let message = props.message.unwrap_or("None".to_string());
    let pop = props.pop.map(|f| f.get_value())
                                                    .unwrap_or("None".to_string());

    let instructions = props.instructions;
    let unknowns = props.unknown;

    println!("Address:      {}", address);
    println!("Amount:       {}", amount);
    println!("Label:        {}", label);
    println!("Message:      {}", message);
    println!("Pop:          {}", pop);
    
    if !instructions.is_empty() {
        println!("Instructions:");
        for instruction in instructions {
            println!("        -{:?}", instruction)
        }
    } else {
        println!("Instructions:     None")
    }
    
    
    if !unknowns.is_empty() {
        println!("Unknowns:");
        for unknown in unknowns {
            println!("        -{:?}", unknown)
        }
    } else {
        println!("Unknown:      None")
    }
    process::exit(0)
}