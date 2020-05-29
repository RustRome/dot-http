use crate::response_handler::{DefaultResponse, ResponseHandler};

pub struct DefaultResponseHandler;

impl ResponseHandler for DefaultResponseHandler {
    type Response = DefaultResponse;
}
