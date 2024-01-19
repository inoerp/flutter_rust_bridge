use crate::app::system::error::no_value::NoValueFoundError;
use std::sync::mpsc;
pub type CommunicationChannel = (
    mpsc::Sender<Result<String, NoValueFoundError>>,
    mpsc::Receiver<Result<String, NoValueFoundError>>,
);