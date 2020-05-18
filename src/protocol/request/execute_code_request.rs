use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::RequestType;
use crate::protocol::{GetRequestType, Serializable};
use nia_protocol_rust::ExecuteCodeRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaExecuteCodeRequest {
    code: String,
}

impl NiaExecuteCodeRequest {
    pub fn new<S>(code: S) -> NiaExecuteCodeRequest
    where
        S: Into<String>,
    {
        NiaExecuteCodeRequest { code: code.into() }
    }

    pub fn get_code(self) -> String {
        self.code
    }
}

impl Serializable<NiaExecuteCodeRequest, nia_protocol_rust::ExecuteCodeRequest>
    for NiaExecuteCodeRequest
{
    fn to_pb(&self) -> ExecuteCodeRequest {
        let mut execute_code_request_pb =
            nia_protocol_rust::ExecuteCodeRequest::new();

        execute_code_request_pb
            .set_code(protobuf::Chars::from(self.code.clone()));

        execute_code_request_pb
    }

    fn from_pb(
        object_pb: ExecuteCodeRequest,
    ) -> NiaServerResult<NiaExecuteCodeRequest> {
        let mut execute_code_request =
            NiaExecuteCodeRequest::new(object_pb.get_code());

        Ok(execute_code_request)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaExecuteCodeRequest::new("(println \"kek\")");

        let bytes = expected.to_bytes().unwrap();
        let result = NiaExecuteCodeRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
