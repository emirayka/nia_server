use crate::protocol::RequestType;

pub trait GetRequestType {
    fn get_request_type(&self) -> RequestType;
}
