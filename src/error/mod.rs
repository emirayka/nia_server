use nia_error::Error;
use std::borrow::Borrow;

#[derive(Clone, Debug)]
pub enum NiaServerError {
    UnknownError(String),
    InvalidRequestError(String),
    DeserializationError(String),
    InterpreterError(String),

    ProtobufError(),
}

impl NiaServerError {
    pub fn get_message(&self) -> String {
        match (self) {
            NiaServerError::UnknownError(s) => s.clone(),
            NiaServerError::InvalidRequestError(s) => s.clone(),
            NiaServerError::DeserializationError(s) => s.clone(),
            NiaServerError::InterpreterError(s) => s.clone(),

            NiaServerError::ProtobufError() => String::from("Protobuf error"),
        }
    }

    pub fn unknown<S>(message: S) -> NiaServerError
    where
        S: Into<String>,
    {
        NiaServerError::UnknownError(message.into())
    }

    pub fn invalid_request<S>(message: S) -> NiaServerError
    where
        S: Into<String>,
    {
        NiaServerError::InvalidRequestError(message.into())
    }

    pub fn deserialization_error<S>(message: S) -> NiaServerError
    where
        S: Into<String>,
    {
        NiaServerError::DeserializationError(message.into())
    }

    pub fn interpreter_execution<S>(message: S) -> NiaServerError
    where
        S: Into<String>,
    {
        NiaServerError::InterpreterError(message.into())
    }

    pub fn protobuf_error(
        protobuf_error: protobuf::ProtobufError,
    ) -> NiaServerError {
        NiaServerError::ProtobufError()
    }
}

impl<T> From<NiaServerError> for Result<T, NiaServerError> {
    fn from(error: NiaServerError) -> Self {
        Err(error)
    }
}

pub fn from_interpreter_error(
    interpreter_error: nia_interpreter_core::Error,
) -> NiaServerError {
    let error_message = interpreter_error.to_string();

    NiaServerError::InterpreterError(error_message)
}

pub fn from_protobuf_error(
    protobuf_error: protobuf::ProtobufError,
) -> NiaServerError {
    NiaServerError::ProtobufError()
}

pub type NiaServerResult<T> = Result<T, NiaServerError>;
