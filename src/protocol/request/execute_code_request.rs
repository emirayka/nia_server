use std::convert::TryFrom;

use crate::error::NiaServerError;

use crate::protocol::GetRequestType;
use crate::protocol::RequestType;

#[derive(Debug, Clone)]
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

impl TryFrom<nia_protocol_rust::ExecuteCodeRequest> for NiaExecuteCodeRequest {
    type Error = NiaServerError;

    fn try_from(
        execute_code_request: nia_protocol_rust::ExecuteCodeRequest,
    ) -> Result<Self, Self::Error> {
        let code = execute_code_request.get_code();

        Ok(NiaExecuteCodeRequest::new(code))
    }
}
