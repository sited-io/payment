#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketBoothResponse {
    #[prost(string, tag = "1")]
    pub market_booth_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub created_at: i64,
    #[prost(int64, tag = "4")]
    pub updated_at: i64,
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "6")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub image_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMarketBoothRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMarketBoothResponse {
    #[prost(message, optional, tag = "1")]
    pub market_booth: ::core::option::Option<MarketBoothResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMarketBoothRequest {
    #[prost(string, tag = "1")]
    pub market_booth_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMarketBoothResponse {
    #[prost(message, optional, tag = "1")]
    pub market_booth: ::core::option::Option<MarketBoothResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketBoothsOrderBy {
    #[prost(enumeration = "MarketBoothsOrderByField", tag = "1")]
    pub field: i32,
    #[prost(enumeration = "super::super::ordering::v1::Direction", tag = "2")]
    pub direction: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketBoothsFilter {
    #[prost(enumeration = "MarketBoothsFilterField", tag = "1")]
    pub field: i32,
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMarketBoothsRequest {
    #[prost(string, optional, tag = "1")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
    #[prost(message, optional, tag = "3")]
    pub order_by: ::core::option::Option<MarketBoothsOrderBy>,
    #[prost(message, optional, tag = "4")]
    pub filter: ::core::option::Option<MarketBoothsFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMarketBoothsResponse {
    #[prost(message, repeated, tag = "1")]
    pub market_booths: ::prost::alloc::vec::Vec<MarketBoothResponse>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMarketBoothRequest {
    #[prost(string, tag = "1")]
    pub market_booth_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMarketBoothResponse {
    #[prost(message, optional, tag = "1")]
    pub market_booth: ::core::option::Option<MarketBoothResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMarketBoothRequest {
    #[prost(string, tag = "1")]
    pub market_booth_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMarketBoothResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateImageOfMarketBoothRequest {
    #[prost(string, tag = "1")]
    pub market_booth_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<super::super::media::v1::MediaUpload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateImageOfMarketBoothResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveImageFromMarketBoothRequest {
    #[prost(string, tag = "1")]
    pub market_booth_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveImageFromMarketBoothResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MarketBoothsOrderByField {
    Unspecified = 0,
    CreatedAt = 1,
    UpdatedAt = 2,
    Name = 3,
    Random = 4,
}
impl MarketBoothsOrderByField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MarketBoothsOrderByField::Unspecified => {
                "MARKET_BOOTHS_ORDER_BY_FIELD_UNSPECIFIED"
            }
            MarketBoothsOrderByField::CreatedAt => {
                "MARKET_BOOTHS_ORDER_BY_FIELD_CREATED_AT"
            }
            MarketBoothsOrderByField::UpdatedAt => {
                "MARKET_BOOTHS_ORDER_BY_FIELD_UPDATED_AT"
            }
            MarketBoothsOrderByField::Name => "MARKET_BOOTHS_ORDER_BY_FIELD_NAME",
            MarketBoothsOrderByField::Random => "MARKET_BOOTHS_ORDER_BY_FIELD_RANDOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MARKET_BOOTHS_ORDER_BY_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "MARKET_BOOTHS_ORDER_BY_FIELD_CREATED_AT" => Some(Self::CreatedAt),
            "MARKET_BOOTHS_ORDER_BY_FIELD_UPDATED_AT" => Some(Self::UpdatedAt),
            "MARKET_BOOTHS_ORDER_BY_FIELD_NAME" => Some(Self::Name),
            "MARKET_BOOTHS_ORDER_BY_FIELD_RANDOM" => Some(Self::Random),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MarketBoothsFilterField {
    Unspecified = 0,
    Name = 1,
    Description = 2,
    NameAndDescription = 3,
}
impl MarketBoothsFilterField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MarketBoothsFilterField::Unspecified => {
                "MARKET_BOOTHS_FILTER_FIELD_UNSPECIFIED"
            }
            MarketBoothsFilterField::Name => "MARKET_BOOTHS_FILTER_FIELD_NAME",
            MarketBoothsFilterField::Description => {
                "MARKET_BOOTHS_FILTER_FIELD_DESCRIPTION"
            }
            MarketBoothsFilterField::NameAndDescription => {
                "MARKET_BOOTHS_FILTER_FIELD_NAME_AND_DESCRIPTION"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MARKET_BOOTHS_FILTER_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "MARKET_BOOTHS_FILTER_FIELD_NAME" => Some(Self::Name),
            "MARKET_BOOTHS_FILTER_FIELD_DESCRIPTION" => Some(Self::Description),
            "MARKET_BOOTHS_FILTER_FIELD_NAME_AND_DESCRIPTION" => {
                Some(Self::NameAndDescription)
            }
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod market_booth_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MarketBoothServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MarketBoothServiceClient<tonic::transport::Channel> {
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
    impl<T> MarketBoothServiceClient<T>
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
        ) -> MarketBoothServiceClient<InterceptedService<T, F>>
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
            MarketBoothServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_market_booth(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMarketBoothRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateMarketBoothResponse>,
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
                "/peoplesmarkets.commerce.v1.MarketBoothService/CreateMarketBooth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.MarketBoothService",
                        "CreateMarketBooth",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_market_booth(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMarketBoothRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMarketBoothResponse>,
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
                "/peoplesmarkets.commerce.v1.MarketBoothService/GetMarketBooth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.MarketBoothService",
                        "GetMarketBooth",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_market_booths(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMarketBoothsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMarketBoothsResponse>,
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
                "/peoplesmarkets.commerce.v1.MarketBoothService/ListMarketBooths",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.MarketBoothService",
                        "ListMarketBooths",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_market_booth(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMarketBoothRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMarketBoothResponse>,
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
                "/peoplesmarkets.commerce.v1.MarketBoothService/UpdateMarketBooth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.MarketBoothService",
                        "UpdateMarketBooth",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_market_booth(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMarketBoothRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMarketBoothResponse>,
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
                "/peoplesmarkets.commerce.v1.MarketBoothService/DeleteMarketBooth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.MarketBoothService",
                        "DeleteMarketBooth",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_image_of_market_booth(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateImageOfMarketBoothRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateImageOfMarketBoothResponse>,
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
                "/peoplesmarkets.commerce.v1.MarketBoothService/UpdateImageOfMarketBooth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.MarketBoothService",
                        "UpdateImageOfMarketBooth",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_image_from_market_booth(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveImageFromMarketBoothRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveImageFromMarketBoothResponse>,
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
                "/peoplesmarkets.commerce.v1.MarketBoothService/RemoveImageFromMarketBooth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.commerce.v1.MarketBoothService",
                        "RemoveImageFromMarketBooth",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
