#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaResponse {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub offer_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub created_at: i64,
    #[prost(int64, tag = "6")]
    pub updated_at: i64,
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "9")]
    pub ordering: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaUpload {
    #[prost(string, tag = "1")]
    pub content_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMediaRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub file: ::core::option::Option<MediaUpload>,
    #[prost(string, tag = "4")]
    pub file_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMediaResponse {
    #[prost(message, optional, tag = "1")]
    pub media: ::core::option::Option<MediaResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMediaRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMediaResponse {
    #[prost(message, optional, tag = "1")]
    pub media: ::core::option::Option<MediaResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadMediaRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadMediaResponse {
    #[prost(string, tag = "1")]
    pub download_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaOrderBy {
    #[prost(enumeration = "MediaOrderByField", tag = "1")]
    pub field: i32,
    #[prost(enumeration = "super::super::ordering::v1::Direction", tag = "2")]
    pub direction: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaFilter {
    #[prost(enumeration = "MediaFilterField", tag = "1")]
    pub field: i32,
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMediaRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
    #[prost(message, optional, tag = "3")]
    pub order_by: ::core::option::Option<MediaOrderBy>,
    #[prost(message, optional, tag = "4")]
    pub filter: ::core::option::Option<MediaFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMediaResponse {
    #[prost(message, repeated, tag = "1")]
    pub medias: ::prost::alloc::vec::Vec<MediaResponse>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessibleMediaRequest {
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
    #[prost(message, optional, tag = "3")]
    pub order_by: ::core::option::Option<MediaOrderBy>,
    #[prost(message, optional, tag = "4")]
    pub filter: ::core::option::Option<MediaFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessibleMediaResponse {
    #[prost(message, repeated, tag = "1")]
    pub medias: ::prost::alloc::vec::Vec<MediaResponse>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMediaRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub file: ::core::option::Option<MediaUpload>,
    #[prost(string, optional, tag = "4")]
    pub file_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMediaResponse {
    #[prost(message, optional, tag = "1")]
    pub media: ::core::option::Option<MediaResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMediaRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMediaResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitiateMultipartUploadRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub content_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitiateMultipartUploadResponse {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub upload_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutMultipartChunkRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub upload_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub part_number: u32,
    #[prost(bytes = "vec", tag = "4")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Part {
    #[prost(uint32, tag = "1")]
    pub part_number: u32,
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutMultipartChunkResponse {
    #[prost(message, optional, tag = "1")]
    pub part: ::core::option::Option<Part>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteMultipartUploadRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub upload_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub parts: ::prost::alloc::vec::Vec<Part>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteMultipartUploadResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMediaToOfferRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(int64, optional, tag = "3")]
    pub ordering: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMediaToOfferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMediaOfferOrderingRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub ordering: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMediaOfferOrderingResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMediaFromOfferRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub offer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMediaFromOfferResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MediaOrderByField {
    Unspecified = 0,
    CreatedAt = 1,
    UpdatedAt = 2,
    Ordering = 3,
}
impl MediaOrderByField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MediaOrderByField::Unspecified => "MEDIA_ORDER_BY_FIELD_UNSPECIFIED",
            MediaOrderByField::CreatedAt => "MEDIA_ORDER_BY_FIELD_CREATED_AT",
            MediaOrderByField::UpdatedAt => "MEDIA_ORDER_BY_FIELD_UPDATED_AT",
            MediaOrderByField::Ordering => "MEDIA_ORDER_BY_FIELD_ORDERING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MEDIA_ORDER_BY_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "MEDIA_ORDER_BY_FIELD_CREATED_AT" => Some(Self::CreatedAt),
            "MEDIA_ORDER_BY_FIELD_UPDATED_AT" => Some(Self::UpdatedAt),
            "MEDIA_ORDER_BY_FIELD_ORDERING" => Some(Self::Ordering),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MediaFilterField {
    Unspecified = 0,
    Name = 1,
    OfferId = 2,
}
impl MediaFilterField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MediaFilterField::Unspecified => "MEDIA_FILTER_FIELD_UNSPECIFIED",
            MediaFilterField::Name => "MEDIA_FILTER_FIELD_NAME",
            MediaFilterField::OfferId => "MEDIA_FILTER_FIELD_OFFER_ID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MEDIA_FILTER_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "MEDIA_FILTER_FIELD_NAME" => Some(Self::Name),
            "MEDIA_FILTER_FIELD_OFFER_ID" => Some(Self::OfferId),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod media_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MediaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MediaServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MediaServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MediaServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MediaServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn create_media(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateMediaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaService/CreateMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaService",
                        "CreateMedia",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_media(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMediaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaService/GetMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("peoplesmarkets.media.v1.MediaService", "GetMedia"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn download_media(
            &mut self,
            request: impl tonic::IntoRequest<super::DownloadMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DownloadMediaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaService/DownloadMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaService",
                        "DownloadMedia",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_media(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMediaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaService/ListMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("peoplesmarkets.media.v1.MediaService", "ListMedia"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_accessible_media(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccessibleMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAccessibleMediaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaService/ListAccessibleMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaService",
                        "ListAccessibleMedia",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_media(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMediaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaService/UpdateMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaService",
                        "UpdateMedia",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_media(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMediaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaService/DeleteMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaService",
                        "DeleteMedia",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn initiate_multipart_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::InitiateMultipartUploadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InitiateMultipartUploadResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaService/InitiateMultipartUpload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaService",
                        "InitiateMultipartUpload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn put_multipart_chunk(
            &mut self,
            request: impl tonic::IntoRequest<super::PutMultipartChunkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutMultipartChunkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaService/PutMultipartChunk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaService",
                        "PutMultipartChunk",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn complete_multipart_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteMultipartUploadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompleteMultipartUploadResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaService/CompleteMultipartUpload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaService",
                        "CompleteMultipartUpload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_media_to_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::AddMediaToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddMediaToOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaService/AddMediaToOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaService",
                        "AddMediaToOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_media_offer_ordering(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMediaOfferOrderingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMediaOfferOrderingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaService/UpdateMediaOfferOrdering",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaService",
                        "UpdateMediaOfferOrdering",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_media_from_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMediaFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMediaFromOfferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaService/RemoveMediaFromOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaService",
                        "RemoveMediaFromOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
