use prost::Message;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Message)]
pub struct ProtoFlattenedSerializerFieldT {
    #[prost(int32, optional, tag = "1")]
    pub var_type_sym: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub var_name_sym: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub bit_count: Option<i32>,
    #[prost(float, optional, tag = "4")]
    pub low_value: Option<f32>,
    #[prost(float, optional, tag = "5")]
    pub high_value: Option<f32>,
    #[prost(int32, optional, tag = "6")]
    pub encode_flags: Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub field_serializer_name_sym: Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub field_serializer_version: Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub send_node_sym: Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub var_encoder_sym: Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub var_serializer_sym: Option<i32>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Message)]
pub struct ProtoFlattenedSerializerT {
    #[prost(int32, optional, tag = "1")]
    pub serializer_name_sym: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub serializer_version: Option<i32>,
    #[prost(int32, repeated, tag = "3")]
    pub fields_index: Vec<i32>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Message)]
pub struct CsvcMsgFlattenedSerializer {
    #[prost(message, repeated, tag = "1")]
    pub serializers: Vec<ProtoFlattenedSerializerT>,
    #[prost(string, repeated, tag = "2")]
    pub symbols: Vec<String>,
    #[prost(message, repeated, tag = "3")]
    pub fields: Vec<ProtoFlattenedSerializerFieldT>,
}
