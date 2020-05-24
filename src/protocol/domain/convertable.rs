use crate::error::NiaServerResult;

pub trait NiaConvertable<NiaServerType, NiaInterpreterType> {
    fn to_interpreter_repr(&self) -> NiaInterpreterType;
    fn from_interpreter_repr(
        object_pb: &NiaInterpreterType,
    ) -> NiaServerResult<NiaServerType>;
}
