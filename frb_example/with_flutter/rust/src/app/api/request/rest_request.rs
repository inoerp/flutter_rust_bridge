use std::error::Error;
pub trait RestRequest {
    fn complete_request()-> Result<String, Box<dyn Error>> ;
}