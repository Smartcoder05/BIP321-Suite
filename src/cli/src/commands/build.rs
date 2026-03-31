use bip321::{BitcoinURI, PaymentInstruction, PopParam, parser::validate_address, types::{NoopHandler}};

pub struct BuildOptions {
    pub address: Option<String>,
    pub amount: Option<String>,
    pub label: Option<String>,
    pub pop: Option<String>,
    pub message: Option<String>,
    pub lnd: Option<String>,
    pub lno: Option<String>,
    pub sp: Option<String>,
}

fn handle_instruction(value: Option<String>, variant: fn(String) -> PaymentInstruction
) -> PaymentInstruction {

    let real_value = value.unwrap_or_default();
    if real_value.is_empty() {
        return PaymentInstruction::Empty;
    }
    variant(real_value.to_string())
}


fn handle_pop(
    value: Option<String>, 
    variant: fn(String) -> PopParam
) -> Option<PopParam>{
    let pop_value = value.unwrap_or_default();
    if pop_value.is_empty() {
        return None
    }
    Some(variant(pop_value.to_string()))
}

pub fn run
(option: BuildOptions) {
    
    let mut uri_struct = BitcoinURI::<NoopHandler>::default();
    
    
    uri_struct.address = validate_address(&option.address.unwrap()).unwrap();
    uri_struct.amount = if let Some(amt) = option.amount {
        amt.parse().ok()
    } else {
        None
    };
    uri_struct.label = option.label;
    uri_struct.message = option.message;
    let uri_lnd = handle_instruction(option.lnd, PaymentInstruction::Lightning);
    let uri_lno = handle_instruction(option.lno,PaymentInstruction::Bolt12);
    let uri_sp = handle_instruction(option.sp, PaymentInstruction::SilentPayment);
    let instructions = vec![uri_lnd, uri_lno, uri_sp];
    uri_struct.pop = handle_pop(option.pop, PopParam::Optional);

    uri_struct.instructions = instructions;
    let uri = uri_struct.to_string();

    println!("{}", uri)
}