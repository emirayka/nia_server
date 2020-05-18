use crate::error::{from_protobuf_error, NiaServerError, NiaServerResult};
use protobuf::Message;

pub trait Serializable<NiaType, PBType>
where
    PBType: protobuf::Message,
{
    fn to_pb(&self) -> PBType;
    fn from_pb(object_pb: PBType) -> NiaServerResult<NiaType>;

    fn to_bytes(&self) -> Result<Vec<u8>, NiaServerError> {
        let object_pb = self.to_pb();

        let bytes = object_pb.write_to_bytes().map_err(from_protobuf_error)?;

        Ok(bytes)
    }

    fn from_bytes(bytes: Vec<u8>) -> NiaServerResult<NiaType> {
        let mut object_pb = PBType::new();

        object_pb
            .merge_from_bytes(bytes.as_slice())
            .map_err(from_protobuf_error)?;

        let object_nia = Self::from_pb(object_pb)?;

        Ok(object_nia)
    }
}
