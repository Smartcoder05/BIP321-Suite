use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum URIError {
    #[error("Invalid Address")]
    InvalidAddress,
    #[error("No instruction")]
    EmptyURINoInstruction,
    #[error("Duplicate Param")]
    DuplicateParam(String),
    #[error("Unknown Required Param")]
    UnknownRequiredParam(String),
    #[error("Invalid Amount")]
    InvalidAmount(String),
    #[error("Forbidden Pop Scheme")]
    ForbiddenPopScheme(String),
    #[error("Both Pop And Req Pop")]
    BothPopAndReqPop,
    #[error("req-pop parameter present but this wallet cannot fulfill this callback")]
    RequiredPopNotSupported,
    #[error("Silent Payment Error")]
    SilentPaymentError(String),
    #[error("Payjoin Error")]
    PayjoinError(String),
    #[error("Invalid Pop Parameter")]
    InvalidPopParameter(String),
}
