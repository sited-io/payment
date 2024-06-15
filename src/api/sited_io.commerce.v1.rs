#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopCustomizationResponse {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub created_at: u64,
    #[prost(uint64, tag = "4")]
    pub updated_at: u64,
    #[prost(string, optional, tag = "5")]
    pub logo_image_light_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub logo_image_dark_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub banner_image_light_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub banner_image_dark_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub primary_color: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "ShopLayoutType", tag = "10")]
    pub layout_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutShopCustomizationRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub primary_color: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "ShopLayoutType", tag = "3")]
    pub layout_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutShopCustomizationResponse {
    #[prost(message, optional, tag = "1")]
    pub shop_customization: ::core::option::Option<ShopCustomizationResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopCustomizationRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopCustomizationResponse {
    #[prost(message, optional, tag = "1")]
    pub shop_customization: ::core::option::Option<ShopCustomizationResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShopCustomizationRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShopCustomizationResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutBannerImageToShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<super::super::media::v1::MediaUpload>,
    #[prost(message, optional, tag = "3")]
    pub image_dark: ::core::option::Option<super::super::media::v1::MediaUpload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutBannerImageToShopResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveBannerImageFromShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveBannerImageFromShopResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutLogoImageToShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<super::super::media::v1::MediaUpload>,
    #[prost(message, optional, tag = "3")]
    pub image_dark: ::core::option::Option<super::super::media::v1::MediaUpload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutLogoImageToShopResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLogoImageFromShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLogoImageFromShopResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShopLayoutType {
    Unspecified = 0,
    Fead = 1,
    OfferList = 2,
}
impl ShopLayoutType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShopLayoutType::Unspecified => "SHOP_LAYOUT_TYPE_UNSPECIFIED",
            ShopLayoutType::Fead => "SHOP_LAYOUT_TYPE_FEAD",
            ShopLayoutType::OfferList => "SHOP_LAYOUT_TYPE_OFFER_LIST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SHOP_LAYOUT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SHOP_LAYOUT_TYPE_FEAD" => Some(Self::Fead),
            "SHOP_LAYOUT_TYPE_OFFER_LIST" => Some(Self::OfferList),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod shop_customization_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ShopCustomizationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ShopCustomizationServiceClient<tonic::transport::Channel> {
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
    impl<T> ShopCustomizationServiceClient<T>
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
        ) -> ShopCustomizationServiceClient<InterceptedService<T, F>>
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
            ShopCustomizationServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        pub async fn put_shop_customization(
            &mut self,
            request: impl tonic::IntoRequest<super::PutShopCustomizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutShopCustomizationResponse>,
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
                "/sited_io.commerce.v1.ShopCustomizationService/PutShopCustomization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v1.ShopCustomizationService",
                        "PutShopCustomization",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_shop_customization(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShopCustomizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetShopCustomizationResponse>,
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
                "/sited_io.commerce.v1.ShopCustomizationService/GetShopCustomization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v1.ShopCustomizationService",
                        "GetShopCustomization",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_shop_customization(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteShopCustomizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteShopCustomizationResponse>,
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
                "/sited_io.commerce.v1.ShopCustomizationService/DeleteShopCustomization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v1.ShopCustomizationService",
                        "DeleteShopCustomization",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn put_banner_image_to_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::PutBannerImageToShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutBannerImageToShopResponse>,
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
                "/sited_io.commerce.v1.ShopCustomizationService/PutBannerImageToShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v1.ShopCustomizationService",
                        "PutBannerImageToShop",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_banner_image_from_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveBannerImageFromShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveBannerImageFromShopResponse>,
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
                "/sited_io.commerce.v1.ShopCustomizationService/RemoveBannerImageFromShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v1.ShopCustomizationService",
                        "RemoveBannerImageFromShop",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn put_logo_image_to_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::PutLogoImageToShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutLogoImageToShopResponse>,
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
                "/sited_io.commerce.v1.ShopCustomizationService/PutLogoImageToShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v1.ShopCustomizationService",
                        "PutLogoImageToShop",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_logo_image_from_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveLogoImageFromShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveLogoImageFromShopResponse>,
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
                "/sited_io.commerce.v1.ShopCustomizationService/RemoveLogoImageFromShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v1.ShopCustomizationService",
                        "RemoveLogoImageFromShop",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopResponse {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub created_at: u64,
    #[prost(uint64, tag = "4")]
    pub updated_at: u64,
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub slug: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "7")]
    pub domain: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, tag = "9")]
    pub platform_fee_percent: u32,
    #[prost(uint32, tag = "10")]
    pub minimum_platform_fee_cent: u32,
    #[prost(message, optional, tag = "11")]
    pub customization: ::core::option::Option<ShopCustomizationResponse>,
    #[prost(bool, tag = "12")]
    pub is_active: bool,
    #[prost(string, optional, tag = "13")]
    pub contact_email_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub client_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateShopRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub slug: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub platform_fee_percent: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub minimum_platform_fee_cent: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateShopResponse {
    #[prost(message, optional, tag = "1")]
    pub shop: ::core::option::Option<ShopResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopRequest {
    #[prost(string, optional, tag = "1")]
    pub shop_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub extended: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "3")]
    pub slug: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub domain: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopResponse {
    #[prost(message, optional, tag = "1")]
    pub shop: ::core::option::Option<ShopResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopsOrderBy {
    #[prost(enumeration = "ShopsOrderByField", tag = "1")]
    pub field: i32,
    #[prost(enumeration = "super::super::ordering::v1::Direction", tag = "2")]
    pub direction: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopsFilter {
    #[prost(enumeration = "ShopsFilterField", tag = "1")]
    pub field: i32,
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShopsRequest {
    #[prost(string, optional, tag = "1")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
    #[prost(message, optional, tag = "3")]
    pub order_by: ::core::option::Option<ShopsOrderBy>,
    #[prost(message, optional, tag = "4")]
    pub filter: ::core::option::Option<ShopsFilter>,
    #[prost(bool, optional, tag = "5")]
    pub extended: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShopsResponse {
    #[prost(message, repeated, tag = "1")]
    pub shops: ::prost::alloc::vec::Vec<ShopResponse>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub slug: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "5")]
    pub platform_fee_percent: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub minimum_platform_fee_cent: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "7")]
    pub is_active: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "8")]
    pub contact_email_address: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateShopResponse {
    #[prost(message, optional, tag = "1")]
    pub shop: ::core::option::Option<ShopResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShopResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShopsOrderByField {
    Unspecified = 0,
    CreatedAt = 1,
    UpdatedAt = 2,
    Name = 3,
    Random = 4,
}
impl ShopsOrderByField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShopsOrderByField::Unspecified => "SHOPS_ORDER_BY_FIELD_UNSPECIFIED",
            ShopsOrderByField::CreatedAt => "SHOPS_ORDER_BY_FIELD_CREATED_AT",
            ShopsOrderByField::UpdatedAt => "SHOPS_ORDER_BY_FIELD_UPDATED_AT",
            ShopsOrderByField::Name => "SHOPS_ORDER_BY_FIELD_NAME",
            ShopsOrderByField::Random => "SHOPS_ORDER_BY_FIELD_RANDOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SHOPS_ORDER_BY_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "SHOPS_ORDER_BY_FIELD_CREATED_AT" => Some(Self::CreatedAt),
            "SHOPS_ORDER_BY_FIELD_UPDATED_AT" => Some(Self::UpdatedAt),
            "SHOPS_ORDER_BY_FIELD_NAME" => Some(Self::Name),
            "SHOPS_ORDER_BY_FIELD_RANDOM" => Some(Self::Random),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShopsFilterField {
    Unspecified = 0,
    Name = 1,
    Description = 2,
    NameAndDescription = 3,
}
impl ShopsFilterField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShopsFilterField::Unspecified => "SHOPS_FILTER_FIELD_UNSPECIFIED",
            ShopsFilterField::Name => "SHOPS_FILTER_FIELD_NAME",
            ShopsFilterField::Description => "SHOPS_FILTER_FIELD_DESCRIPTION",
            ShopsFilterField::NameAndDescription => {
                "SHOPS_FILTER_FIELD_NAME_AND_DESCRIPTION"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SHOPS_FILTER_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "SHOPS_FILTER_FIELD_NAME" => Some(Self::Name),
            "SHOPS_FILTER_FIELD_DESCRIPTION" => Some(Self::Description),
            "SHOPS_FILTER_FIELD_NAME_AND_DESCRIPTION" => Some(Self::NameAndDescription),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod shop_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ShopServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ShopServiceClient<tonic::transport::Channel> {
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
    impl<T> ShopServiceClient<T>
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
        ) -> ShopServiceClient<InterceptedService<T, F>>
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
            ShopServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateShopResponse>,
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
                "/sited_io.commerce.v1.ShopService/CreateShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v1.ShopService", "CreateShop"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetShopResponse>,
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
                "/sited_io.commerce.v1.ShopService/GetShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sited_io.commerce.v1.ShopService", "GetShop"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_shops(
            &mut self,
            request: impl tonic::IntoRequest<super::ListShopsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListShopsResponse>,
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
                "/sited_io.commerce.v1.ShopService/ListShops",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v1.ShopService", "ListShops"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateShopResponse>,
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
                "/sited_io.commerce.v1.ShopService/UpdateShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v1.ShopService", "UpdateShop"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_shop(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteShopResponse>,
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
                "/sited_io.commerce.v1.ShopService/DeleteShop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v1.ShopService", "DeleteShop"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Recurring {
    #[prost(enumeration = "RecurringInterval", tag = "1")]
    pub interval: i32,
    #[prost(uint32, tag = "2")]
    pub interval_count: u32,
    #[prost(uint32, optional, tag = "3")]
    pub trial_period_days: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Price {
    #[prost(enumeration = "Currency", tag = "1")]
    pub currency: i32,
    #[prost(enumeration = "PriceType", tag = "2")]
    pub price_type: i32,
    #[prost(enumeration = "PriceBillingScheme", tag = "3")]
    pub billing_scheme: i32,
    #[prost(uint32, tag = "4")]
    pub unit_amount: u32,
    #[prost(message, optional, tag = "5")]
    pub recurring: ::core::option::Option<Recurring>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Currency {
    Unspecified = 0,
    Eur = 1,
}
impl Currency {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Currency::Unspecified => "CURRENCY_UNSPECIFIED",
            Currency::Eur => "CURRENCY_EUR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CURRENCY_UNSPECIFIED" => Some(Self::Unspecified),
            "CURRENCY_EUR" => Some(Self::Eur),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceType {
    Unspecified = 0,
    OneTime = 1,
    Recurring = 2,
}
impl PriceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceType::Unspecified => "PRICE_TYPE_UNSPECIFIED",
            PriceType::OneTime => "PRICE_TYPE_ONE_TIME",
            PriceType::Recurring => "PRICE_TYPE_RECURRING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE_TYPE_ONE_TIME" => Some(Self::OneTime),
            "PRICE_TYPE_RECURRING" => Some(Self::Recurring),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceBillingScheme {
    Unspecified = 0,
    PerUnit = 1,
}
impl PriceBillingScheme {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceBillingScheme::Unspecified => "PRICE_BILLING_SCHEME_UNSPECIFIED",
            PriceBillingScheme::PerUnit => "PRICE_BILLING_SCHEME_PER_UNIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_BILLING_SCHEME_UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE_BILLING_SCHEME_PER_UNIT" => Some(Self::PerUnit),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RecurringInterval {
    Unspecified = 0,
    Day = 1,
    Week = 2,
    Month = 3,
    Year = 4,
}
impl RecurringInterval {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RecurringInterval::Unspecified => "RECURRING_INTERVAL_UNSPECIFIED",
            RecurringInterval::Day => "RECURRING_INTERVAL_DAY",
            RecurringInterval::Week => "RECURRING_INTERVAL_WEEK",
            RecurringInterval::Month => "RECURRING_INTERVAL_MONTH",
            RecurringInterval::Year => "RECURRING_INTERVAL_YEAR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RECURRING_INTERVAL_UNSPECIFIED" => Some(Self::Unspecified),
            "RECURRING_INTERVAL_DAY" => Some(Self::Day),
            "RECURRING_INTERVAL_WEEK" => Some(Self::Week),
            "RECURRING_INTERVAL_MONTH" => Some(Self::Month),
            "RECURRING_INTERVAL_YEAR" => Some(Self::Year),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferResponse {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shop_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub shop_slug: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub shop_domain: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "6")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "7")]
    pub created_at: i64,
    #[prost(int64, tag = "8")]
    pub updated_at: i64,
    #[prost(string, tag = "9")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "11")]
    pub is_active: bool,
    #[prost(bool, tag = "12")]
    pub is_featured: bool,
    #[prost(enumeration = "OfferType", tag = "13")]
    pub r#type: i32,
    #[prost(message, repeated, tag = "14")]
    pub images: ::prost::alloc::vec::Vec<OfferImageResponse>,
    #[prost(message, optional, tag = "15")]
    pub price: ::core::option::Option<Price>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferImageResponse {
    #[prost(string, tag = "1")]
    pub offer_image_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub image_url: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub ordering: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOfferRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "OfferType", tag = "4")]
    pub r#type: i32,
    #[prost(bool, tag = "5")]
    pub is_featured: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOfferResponse {
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<OfferResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOfferResponse {
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<OfferResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMyOfferResponse {
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<OfferResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OffersOrderBy {
    #[prost(enumeration = "OffersOrderByField", tag = "1")]
    pub field: i32,
    #[prost(enumeration = "super::super::ordering::v1::Direction", tag = "2")]
    pub direction: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OffersFilter {
    #[prost(enumeration = "OffersFilterField", tag = "1")]
    pub field: i32,
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOffersRequest {
    #[prost(string, optional, tag = "1")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub shop_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<
        super::super::pagination::v1::PaginationRequest,
    >,
    #[prost(message, optional, tag = "4")]
    pub order_by: ::core::option::Option<OffersOrderBy>,
    #[prost(message, optional, tag = "5")]
    pub filter: ::core::option::Option<OffersFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOffersResponse {
    #[prost(message, repeated, tag = "1")]
    pub offers: ::prost::alloc::vec::Vec<OfferResponse>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::pagination::v1::PaginationResponse,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "4")]
    pub is_active: ::core::option::Option<bool>,
    #[prost(enumeration = "OfferType", optional, tag = "5")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "6")]
    pub is_featured: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOfferResponse {
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<OfferResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOfferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddImageToOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<super::super::media::v1::MediaUpload>,
    #[prost(int64, tag = "3")]
    pub ordering: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddImageToOfferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveImageFromOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_image_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveImageFromOfferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutPriceToOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<Price>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutPriceToOfferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePriceFromOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePriceFromOfferResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OfferType {
    Unspecified = 0,
    Physical = 1,
    Digital = 2,
    Service = 3,
}
impl OfferType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OfferType::Unspecified => "OFFER_TYPE_UNSPECIFIED",
            OfferType::Physical => "OFFER_TYPE_PHYSICAL",
            OfferType::Digital => "OFFER_TYPE_DIGITAL",
            OfferType::Service => "OFFER_TYPE_SERVICE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OFFER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "OFFER_TYPE_PHYSICAL" => Some(Self::Physical),
            "OFFER_TYPE_DIGITAL" => Some(Self::Digital),
            "OFFER_TYPE_SERVICE" => Some(Self::Service),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OffersOrderByField {
    Unspecified = 0,
    CreatedAt = 1,
    UpdatedAt = 2,
    Name = 3,
    Random = 4,
}
impl OffersOrderByField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OffersOrderByField::Unspecified => "OFFERS_ORDER_BY_FIELD_UNSPECIFIED",
            OffersOrderByField::CreatedAt => "OFFERS_ORDER_BY_FIELD_CREATED_AT",
            OffersOrderByField::UpdatedAt => "OFFERS_ORDER_BY_FIELD_UPDATED_AT",
            OffersOrderByField::Name => "OFFERS_ORDER_BY_FIELD_NAME",
            OffersOrderByField::Random => "OFFERS_ORDER_BY_FIELD_RANDOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OFFERS_ORDER_BY_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "OFFERS_ORDER_BY_FIELD_CREATED_AT" => Some(Self::CreatedAt),
            "OFFERS_ORDER_BY_FIELD_UPDATED_AT" => Some(Self::UpdatedAt),
            "OFFERS_ORDER_BY_FIELD_NAME" => Some(Self::Name),
            "OFFERS_ORDER_BY_FIELD_RANDOM" => Some(Self::Random),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OffersFilterField {
    Unspecified = 0,
    Name = 1,
    Description = 2,
    NameAndDescription = 3,
    Type = 4,
    IsFeatured = 5,
}
impl OffersFilterField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OffersFilterField::Unspecified => "OFFERS_FILTER_FIELD_UNSPECIFIED",
            OffersFilterField::Name => "OFFERS_FILTER_FIELD_NAME",
            OffersFilterField::Description => "OFFERS_FILTER_FIELD_DESCRIPTION",
            OffersFilterField::NameAndDescription => {
                "OFFERS_FILTER_FIELD_NAME_AND_DESCRIPTION"
            }
            OffersFilterField::Type => "OFFERS_FILTER_FIELD_TYPE",
            OffersFilterField::IsFeatured => "OFFERS_FILTER_FIELD_IS_FEATURED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OFFERS_FILTER_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "OFFERS_FILTER_FIELD_NAME" => Some(Self::Name),
            "OFFERS_FILTER_FIELD_DESCRIPTION" => Some(Self::Description),
            "OFFERS_FILTER_FIELD_NAME_AND_DESCRIPTION" => Some(Self::NameAndDescription),
            "OFFERS_FILTER_FIELD_TYPE" => Some(Self::Type),
            "OFFERS_FILTER_FIELD_IS_FEATURED" => Some(Self::IsFeatured),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod offer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OfferServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OfferServiceClient<tonic::transport::Channel> {
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
    impl<T> OfferServiceClient<T>
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
        ) -> OfferServiceClient<InterceptedService<T, F>>
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
            OfferServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOfferResponse>,
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
                "/sited_io.commerce.v1.OfferService/CreateOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v1.OfferService", "CreateOffer"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOfferResponse>,
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
                "/sited_io.commerce.v1.OfferService/GetOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v1.OfferService", "GetOffer"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_my_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMyOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMyOfferResponse>,
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
                "/sited_io.commerce.v1.OfferService/GetMyOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v1.OfferService", "GetMyOffer"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_offers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOffersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOffersResponse>,
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
                "/sited_io.commerce.v1.OfferService/ListOffers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v1.OfferService", "ListOffers"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOfferResponse>,
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
                "/sited_io.commerce.v1.OfferService/UpdateOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v1.OfferService", "UpdateOffer"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteOfferResponse>,
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
                "/sited_io.commerce.v1.OfferService/DeleteOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sited_io.commerce.v1.OfferService", "DeleteOffer"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_image_to_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::AddImageToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddImageToOfferResponse>,
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
                "/sited_io.commerce.v1.OfferService/AddImageToOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v1.OfferService",
                        "AddImageToOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_image_from_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveImageFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveImageFromOfferResponse>,
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
                "/sited_io.commerce.v1.OfferService/RemoveImageFromOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v1.OfferService",
                        "RemoveImageFromOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn put_price_to_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::PutPriceToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutPriceToOfferResponse>,
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
                "/sited_io.commerce.v1.OfferService/PutPriceToOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v1.OfferService",
                        "PutPriceToOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_price_from_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::RemovePriceFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemovePriceFromOfferResponse>,
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
                "/sited_io.commerce.v1.OfferService/RemovePriceFromOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v1.OfferService",
                        "RemovePriceFromOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShippingRateResponse {
    #[prost(string, tag = "1")]
    pub shipping_rate_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub amount: u32,
    #[prost(enumeration = "Currency", tag = "5")]
    pub currency: i32,
    #[prost(bool, tag = "6")]
    pub all_countries: bool,
    #[prost(enumeration = "ShippingCountry", repeated, tag = "7")]
    pub specific_countries: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutShippingRateRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub amount: u32,
    #[prost(enumeration = "Currency", tag = "3")]
    pub currency: i32,
    #[prost(bool, tag = "4")]
    pub all_countries: bool,
    #[prost(enumeration = "ShippingCountry", repeated, tag = "5")]
    pub specific_countries: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutShippingRateResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShippingRateRequest {
    #[prost(string, optional, tag = "1")]
    pub offer_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShippingRateResponse {
    #[prost(message, optional, tag = "1")]
    pub shipping_rate: ::core::option::Option<ShippingRateResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShippingRateRequest {
    #[prost(string, tag = "1")]
    pub shipping_rate_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShippingRateResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShippingCountry {
    Unspecified = 0,
    Ac = 1,
    Ad = 2,
    Ae = 3,
    Af = 4,
    Ag = 5,
    Ai = 6,
    Al = 7,
    Am = 8,
    Ao = 9,
    Aq = 10,
    Ar = 11,
    At = 12,
    Au = 13,
    Aw = 14,
    Ax = 15,
    Az = 16,
    Ba = 17,
    Bb = 18,
    Bd = 19,
    Be = 20,
    Bf = 21,
    Bg = 22,
    Bh = 23,
    Bi = 24,
    Bj = 25,
    Bl = 26,
    Bm = 27,
    Bn = 28,
    Bo = 29,
    Bq = 30,
    Br = 31,
    Bs = 32,
    Bt = 33,
    Bv = 34,
    Bw = 35,
    By = 36,
    Bz = 37,
    Ca = 38,
    Cd = 39,
    Cf = 40,
    Cg = 41,
    Ch = 42,
    Ci = 43,
    Ck = 44,
    Cl = 45,
    Cm = 46,
    Cn = 47,
    Co = 48,
    Cr = 49,
    Cv = 50,
    Cw = 51,
    Cy = 52,
    Cz = 53,
    De = 54,
    Dj = 55,
    Dk = 56,
    Dm = 57,
    Do = 58,
    Dz = 59,
    Ec = 60,
    Ee = 61,
    Eg = 62,
    Eh = 63,
    Er = 64,
    Es = 65,
    Et = 66,
    Fi = 67,
    Fj = 68,
    Fk = 69,
    Fo = 70,
    Fr = 71,
    Ga = 72,
    Gb = 73,
    Gd = 74,
    Ge = 75,
    Gf = 76,
    Gg = 77,
    Gh = 78,
    Gi = 79,
    Gl = 80,
    Gm = 81,
    Gn = 82,
    Gp = 83,
    Gq = 84,
    Gr = 85,
    Gs = 86,
    Gt = 87,
    Gu = 88,
    Gw = 89,
    Gy = 90,
    Hk = 91,
    Hn = 92,
    Hr = 93,
    Ht = 94,
    Hu = 95,
    Id = 96,
    Ie = 97,
    Il = 98,
    Im = 99,
    In = 100,
    Io = 101,
    Iq = 102,
    Is = 103,
    It = 104,
    Je = 105,
    Jm = 106,
    Jo = 107,
    Jp = 108,
    Ke = 109,
    Kg = 110,
    Kh = 111,
    Ki = 112,
    Km = 113,
    Kn = 114,
    Kr = 115,
    Kw = 116,
    Ky = 117,
    La = 118,
    Lb = 119,
    Lc = 120,
    Li = 121,
    Lk = 122,
    Lr = 123,
    Ls = 124,
    Lt = 125,
    Lu = 126,
    Lv = 127,
    Ly = 128,
    Ma = 129,
    Mc = 130,
    Md = 131,
    Me = 132,
    Mf = 133,
    Mg = 134,
    Mk = 135,
    Ml = 136,
    Mm = 137,
    Mn = 138,
    Mo = 139,
    Mq = 140,
    Mr = 141,
    Ms = 142,
    Mt = 143,
    Mu = 144,
    Mv = 145,
    Mw = 146,
    Mx = 147,
    My = 148,
    Mz = 149,
    Na = 150,
    Nc = 151,
    Ne = 152,
    Ng = 153,
    Ni = 154,
    Nl = 155,
    No = 156,
    Np = 157,
    Nr = 158,
    Nu = 159,
    Nz = 160,
    Om = 161,
    Pa = 162,
    Pe = 163,
    Pf = 164,
    Pg = 165,
    Ph = 166,
    Pk = 167,
    Pl = 168,
    Pm = 169,
    Pn = 170,
    Pr = 171,
    Ps = 172,
    Pt = 173,
    Py = 174,
    Qa = 175,
    Re = 176,
    Ro = 177,
    Rs = 178,
    Ru = 179,
    Rw = 180,
    Sa = 181,
    Sb = 182,
    Sc = 183,
    Se = 184,
    Sg = 185,
    Sh = 186,
    Si = 187,
    Sj = 188,
    Sk = 189,
    Sl = 190,
    Sm = 191,
    Sn = 192,
    So = 193,
    Sr = 194,
    Ss = 195,
    St = 196,
    Sv = 197,
    Sx = 198,
    Sz = 199,
    Ta = 200,
    Tc = 201,
    Td = 202,
    Tf = 203,
    Tg = 204,
    Th = 205,
    Tj = 206,
    Tk = 207,
    Tl = 208,
    Tm = 209,
    Tn = 210,
    To = 211,
    Tr = 212,
    Tt = 213,
    Tv = 214,
    Tw = 215,
    Tz = 216,
    Ua = 217,
    Ug = 218,
    Us = 219,
    Uy = 220,
    Uz = 221,
    Va = 222,
    Vc = 223,
    Ve = 224,
    Vg = 225,
    Vn = 226,
    Vu = 227,
    Wf = 228,
    Ws = 229,
    Xk = 230,
    Ye = 231,
    Yt = 232,
    Za = 233,
    Zm = 234,
    Zw = 235,
    Zz = 236,
}
impl ShippingCountry {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShippingCountry::Unspecified => "SHIPPING_COUNTRY_UNSPECIFIED",
            ShippingCountry::Ac => "SHIPPING_COUNTRY_AC",
            ShippingCountry::Ad => "SHIPPING_COUNTRY_AD",
            ShippingCountry::Ae => "SHIPPING_COUNTRY_AE",
            ShippingCountry::Af => "SHIPPING_COUNTRY_AF",
            ShippingCountry::Ag => "SHIPPING_COUNTRY_AG",
            ShippingCountry::Ai => "SHIPPING_COUNTRY_AI",
            ShippingCountry::Al => "SHIPPING_COUNTRY_AL",
            ShippingCountry::Am => "SHIPPING_COUNTRY_AM",
            ShippingCountry::Ao => "SHIPPING_COUNTRY_AO",
            ShippingCountry::Aq => "SHIPPING_COUNTRY_AQ",
            ShippingCountry::Ar => "SHIPPING_COUNTRY_AR",
            ShippingCountry::At => "SHIPPING_COUNTRY_AT",
            ShippingCountry::Au => "SHIPPING_COUNTRY_AU",
            ShippingCountry::Aw => "SHIPPING_COUNTRY_AW",
            ShippingCountry::Ax => "SHIPPING_COUNTRY_AX",
            ShippingCountry::Az => "SHIPPING_COUNTRY_AZ",
            ShippingCountry::Ba => "SHIPPING_COUNTRY_BA",
            ShippingCountry::Bb => "SHIPPING_COUNTRY_BB",
            ShippingCountry::Bd => "SHIPPING_COUNTRY_BD",
            ShippingCountry::Be => "SHIPPING_COUNTRY_BE",
            ShippingCountry::Bf => "SHIPPING_COUNTRY_BF",
            ShippingCountry::Bg => "SHIPPING_COUNTRY_BG",
            ShippingCountry::Bh => "SHIPPING_COUNTRY_BH",
            ShippingCountry::Bi => "SHIPPING_COUNTRY_BI",
            ShippingCountry::Bj => "SHIPPING_COUNTRY_BJ",
            ShippingCountry::Bl => "SHIPPING_COUNTRY_BL",
            ShippingCountry::Bm => "SHIPPING_COUNTRY_BM",
            ShippingCountry::Bn => "SHIPPING_COUNTRY_BN",
            ShippingCountry::Bo => "SHIPPING_COUNTRY_BO",
            ShippingCountry::Bq => "SHIPPING_COUNTRY_BQ",
            ShippingCountry::Br => "SHIPPING_COUNTRY_BR",
            ShippingCountry::Bs => "SHIPPING_COUNTRY_BS",
            ShippingCountry::Bt => "SHIPPING_COUNTRY_BT",
            ShippingCountry::Bv => "SHIPPING_COUNTRY_BV",
            ShippingCountry::Bw => "SHIPPING_COUNTRY_BW",
            ShippingCountry::By => "SHIPPING_COUNTRY_BY",
            ShippingCountry::Bz => "SHIPPING_COUNTRY_BZ",
            ShippingCountry::Ca => "SHIPPING_COUNTRY_CA",
            ShippingCountry::Cd => "SHIPPING_COUNTRY_CD",
            ShippingCountry::Cf => "SHIPPING_COUNTRY_CF",
            ShippingCountry::Cg => "SHIPPING_COUNTRY_CG",
            ShippingCountry::Ch => "SHIPPING_COUNTRY_CH",
            ShippingCountry::Ci => "SHIPPING_COUNTRY_CI",
            ShippingCountry::Ck => "SHIPPING_COUNTRY_CK",
            ShippingCountry::Cl => "SHIPPING_COUNTRY_CL",
            ShippingCountry::Cm => "SHIPPING_COUNTRY_CM",
            ShippingCountry::Cn => "SHIPPING_COUNTRY_CN",
            ShippingCountry::Co => "SHIPPING_COUNTRY_CO",
            ShippingCountry::Cr => "SHIPPING_COUNTRY_CR",
            ShippingCountry::Cv => "SHIPPING_COUNTRY_CV",
            ShippingCountry::Cw => "SHIPPING_COUNTRY_CW",
            ShippingCountry::Cy => "SHIPPING_COUNTRY_CY",
            ShippingCountry::Cz => "SHIPPING_COUNTRY_CZ",
            ShippingCountry::De => "SHIPPING_COUNTRY_DE",
            ShippingCountry::Dj => "SHIPPING_COUNTRY_DJ",
            ShippingCountry::Dk => "SHIPPING_COUNTRY_DK",
            ShippingCountry::Dm => "SHIPPING_COUNTRY_DM",
            ShippingCountry::Do => "SHIPPING_COUNTRY_DO",
            ShippingCountry::Dz => "SHIPPING_COUNTRY_DZ",
            ShippingCountry::Ec => "SHIPPING_COUNTRY_EC",
            ShippingCountry::Ee => "SHIPPING_COUNTRY_EE",
            ShippingCountry::Eg => "SHIPPING_COUNTRY_EG",
            ShippingCountry::Eh => "SHIPPING_COUNTRY_EH",
            ShippingCountry::Er => "SHIPPING_COUNTRY_ER",
            ShippingCountry::Es => "SHIPPING_COUNTRY_ES",
            ShippingCountry::Et => "SHIPPING_COUNTRY_ET",
            ShippingCountry::Fi => "SHIPPING_COUNTRY_FI",
            ShippingCountry::Fj => "SHIPPING_COUNTRY_FJ",
            ShippingCountry::Fk => "SHIPPING_COUNTRY_FK",
            ShippingCountry::Fo => "SHIPPING_COUNTRY_FO",
            ShippingCountry::Fr => "SHIPPING_COUNTRY_FR",
            ShippingCountry::Ga => "SHIPPING_COUNTRY_GA",
            ShippingCountry::Gb => "SHIPPING_COUNTRY_GB",
            ShippingCountry::Gd => "SHIPPING_COUNTRY_GD",
            ShippingCountry::Ge => "SHIPPING_COUNTRY_GE",
            ShippingCountry::Gf => "SHIPPING_COUNTRY_GF",
            ShippingCountry::Gg => "SHIPPING_COUNTRY_GG",
            ShippingCountry::Gh => "SHIPPING_COUNTRY_GH",
            ShippingCountry::Gi => "SHIPPING_COUNTRY_GI",
            ShippingCountry::Gl => "SHIPPING_COUNTRY_GL",
            ShippingCountry::Gm => "SHIPPING_COUNTRY_GM",
            ShippingCountry::Gn => "SHIPPING_COUNTRY_GN",
            ShippingCountry::Gp => "SHIPPING_COUNTRY_GP",
            ShippingCountry::Gq => "SHIPPING_COUNTRY_GQ",
            ShippingCountry::Gr => "SHIPPING_COUNTRY_GR",
            ShippingCountry::Gs => "SHIPPING_COUNTRY_GS",
            ShippingCountry::Gt => "SHIPPING_COUNTRY_GT",
            ShippingCountry::Gu => "SHIPPING_COUNTRY_GU",
            ShippingCountry::Gw => "SHIPPING_COUNTRY_GW",
            ShippingCountry::Gy => "SHIPPING_COUNTRY_GY",
            ShippingCountry::Hk => "SHIPPING_COUNTRY_HK",
            ShippingCountry::Hn => "SHIPPING_COUNTRY_HN",
            ShippingCountry::Hr => "SHIPPING_COUNTRY_HR",
            ShippingCountry::Ht => "SHIPPING_COUNTRY_HT",
            ShippingCountry::Hu => "SHIPPING_COUNTRY_HU",
            ShippingCountry::Id => "SHIPPING_COUNTRY_ID",
            ShippingCountry::Ie => "SHIPPING_COUNTRY_IE",
            ShippingCountry::Il => "SHIPPING_COUNTRY_IL",
            ShippingCountry::Im => "SHIPPING_COUNTRY_IM",
            ShippingCountry::In => "SHIPPING_COUNTRY_IN",
            ShippingCountry::Io => "SHIPPING_COUNTRY_IO",
            ShippingCountry::Iq => "SHIPPING_COUNTRY_IQ",
            ShippingCountry::Is => "SHIPPING_COUNTRY_IS",
            ShippingCountry::It => "SHIPPING_COUNTRY_IT",
            ShippingCountry::Je => "SHIPPING_COUNTRY_JE",
            ShippingCountry::Jm => "SHIPPING_COUNTRY_JM",
            ShippingCountry::Jo => "SHIPPING_COUNTRY_JO",
            ShippingCountry::Jp => "SHIPPING_COUNTRY_JP",
            ShippingCountry::Ke => "SHIPPING_COUNTRY_KE",
            ShippingCountry::Kg => "SHIPPING_COUNTRY_KG",
            ShippingCountry::Kh => "SHIPPING_COUNTRY_KH",
            ShippingCountry::Ki => "SHIPPING_COUNTRY_KI",
            ShippingCountry::Km => "SHIPPING_COUNTRY_KM",
            ShippingCountry::Kn => "SHIPPING_COUNTRY_KN",
            ShippingCountry::Kr => "SHIPPING_COUNTRY_KR",
            ShippingCountry::Kw => "SHIPPING_COUNTRY_KW",
            ShippingCountry::Ky => "SHIPPING_COUNTRY_KY",
            ShippingCountry::La => "SHIPPING_COUNTRY_LA",
            ShippingCountry::Lb => "SHIPPING_COUNTRY_LB",
            ShippingCountry::Lc => "SHIPPING_COUNTRY_LC",
            ShippingCountry::Li => "SHIPPING_COUNTRY_LI",
            ShippingCountry::Lk => "SHIPPING_COUNTRY_LK",
            ShippingCountry::Lr => "SHIPPING_COUNTRY_LR",
            ShippingCountry::Ls => "SHIPPING_COUNTRY_LS",
            ShippingCountry::Lt => "SHIPPING_COUNTRY_LT",
            ShippingCountry::Lu => "SHIPPING_COUNTRY_LU",
            ShippingCountry::Lv => "SHIPPING_COUNTRY_LV",
            ShippingCountry::Ly => "SHIPPING_COUNTRY_LY",
            ShippingCountry::Ma => "SHIPPING_COUNTRY_MA",
            ShippingCountry::Mc => "SHIPPING_COUNTRY_MC",
            ShippingCountry::Md => "SHIPPING_COUNTRY_MD",
            ShippingCountry::Me => "SHIPPING_COUNTRY_ME",
            ShippingCountry::Mf => "SHIPPING_COUNTRY_MF",
            ShippingCountry::Mg => "SHIPPING_COUNTRY_MG",
            ShippingCountry::Mk => "SHIPPING_COUNTRY_MK",
            ShippingCountry::Ml => "SHIPPING_COUNTRY_ML",
            ShippingCountry::Mm => "SHIPPING_COUNTRY_MM",
            ShippingCountry::Mn => "SHIPPING_COUNTRY_MN",
            ShippingCountry::Mo => "SHIPPING_COUNTRY_MO",
            ShippingCountry::Mq => "SHIPPING_COUNTRY_MQ",
            ShippingCountry::Mr => "SHIPPING_COUNTRY_MR",
            ShippingCountry::Ms => "SHIPPING_COUNTRY_MS",
            ShippingCountry::Mt => "SHIPPING_COUNTRY_MT",
            ShippingCountry::Mu => "SHIPPING_COUNTRY_MU",
            ShippingCountry::Mv => "SHIPPING_COUNTRY_MV",
            ShippingCountry::Mw => "SHIPPING_COUNTRY_MW",
            ShippingCountry::Mx => "SHIPPING_COUNTRY_MX",
            ShippingCountry::My => "SHIPPING_COUNTRY_MY",
            ShippingCountry::Mz => "SHIPPING_COUNTRY_MZ",
            ShippingCountry::Na => "SHIPPING_COUNTRY_NA",
            ShippingCountry::Nc => "SHIPPING_COUNTRY_NC",
            ShippingCountry::Ne => "SHIPPING_COUNTRY_NE",
            ShippingCountry::Ng => "SHIPPING_COUNTRY_NG",
            ShippingCountry::Ni => "SHIPPING_COUNTRY_NI",
            ShippingCountry::Nl => "SHIPPING_COUNTRY_NL",
            ShippingCountry::No => "SHIPPING_COUNTRY_NO",
            ShippingCountry::Np => "SHIPPING_COUNTRY_NP",
            ShippingCountry::Nr => "SHIPPING_COUNTRY_NR",
            ShippingCountry::Nu => "SHIPPING_COUNTRY_NU",
            ShippingCountry::Nz => "SHIPPING_COUNTRY_NZ",
            ShippingCountry::Om => "SHIPPING_COUNTRY_OM",
            ShippingCountry::Pa => "SHIPPING_COUNTRY_PA",
            ShippingCountry::Pe => "SHIPPING_COUNTRY_PE",
            ShippingCountry::Pf => "SHIPPING_COUNTRY_PF",
            ShippingCountry::Pg => "SHIPPING_COUNTRY_PG",
            ShippingCountry::Ph => "SHIPPING_COUNTRY_PH",
            ShippingCountry::Pk => "SHIPPING_COUNTRY_PK",
            ShippingCountry::Pl => "SHIPPING_COUNTRY_PL",
            ShippingCountry::Pm => "SHIPPING_COUNTRY_PM",
            ShippingCountry::Pn => "SHIPPING_COUNTRY_PN",
            ShippingCountry::Pr => "SHIPPING_COUNTRY_PR",
            ShippingCountry::Ps => "SHIPPING_COUNTRY_PS",
            ShippingCountry::Pt => "SHIPPING_COUNTRY_PT",
            ShippingCountry::Py => "SHIPPING_COUNTRY_PY",
            ShippingCountry::Qa => "SHIPPING_COUNTRY_QA",
            ShippingCountry::Re => "SHIPPING_COUNTRY_RE",
            ShippingCountry::Ro => "SHIPPING_COUNTRY_RO",
            ShippingCountry::Rs => "SHIPPING_COUNTRY_RS",
            ShippingCountry::Ru => "SHIPPING_COUNTRY_RU",
            ShippingCountry::Rw => "SHIPPING_COUNTRY_RW",
            ShippingCountry::Sa => "SHIPPING_COUNTRY_SA",
            ShippingCountry::Sb => "SHIPPING_COUNTRY_SB",
            ShippingCountry::Sc => "SHIPPING_COUNTRY_SC",
            ShippingCountry::Se => "SHIPPING_COUNTRY_SE",
            ShippingCountry::Sg => "SHIPPING_COUNTRY_SG",
            ShippingCountry::Sh => "SHIPPING_COUNTRY_SH",
            ShippingCountry::Si => "SHIPPING_COUNTRY_SI",
            ShippingCountry::Sj => "SHIPPING_COUNTRY_SJ",
            ShippingCountry::Sk => "SHIPPING_COUNTRY_SK",
            ShippingCountry::Sl => "SHIPPING_COUNTRY_SL",
            ShippingCountry::Sm => "SHIPPING_COUNTRY_SM",
            ShippingCountry::Sn => "SHIPPING_COUNTRY_SN",
            ShippingCountry::So => "SHIPPING_COUNTRY_SO",
            ShippingCountry::Sr => "SHIPPING_COUNTRY_SR",
            ShippingCountry::Ss => "SHIPPING_COUNTRY_SS",
            ShippingCountry::St => "SHIPPING_COUNTRY_ST",
            ShippingCountry::Sv => "SHIPPING_COUNTRY_SV",
            ShippingCountry::Sx => "SHIPPING_COUNTRY_SX",
            ShippingCountry::Sz => "SHIPPING_COUNTRY_SZ",
            ShippingCountry::Ta => "SHIPPING_COUNTRY_TA",
            ShippingCountry::Tc => "SHIPPING_COUNTRY_TC",
            ShippingCountry::Td => "SHIPPING_COUNTRY_TD",
            ShippingCountry::Tf => "SHIPPING_COUNTRY_TF",
            ShippingCountry::Tg => "SHIPPING_COUNTRY_TG",
            ShippingCountry::Th => "SHIPPING_COUNTRY_TH",
            ShippingCountry::Tj => "SHIPPING_COUNTRY_TJ",
            ShippingCountry::Tk => "SHIPPING_COUNTRY_TK",
            ShippingCountry::Tl => "SHIPPING_COUNTRY_TL",
            ShippingCountry::Tm => "SHIPPING_COUNTRY_TM",
            ShippingCountry::Tn => "SHIPPING_COUNTRY_TN",
            ShippingCountry::To => "SHIPPING_COUNTRY_TO",
            ShippingCountry::Tr => "SHIPPING_COUNTRY_TR",
            ShippingCountry::Tt => "SHIPPING_COUNTRY_TT",
            ShippingCountry::Tv => "SHIPPING_COUNTRY_TV",
            ShippingCountry::Tw => "SHIPPING_COUNTRY_TW",
            ShippingCountry::Tz => "SHIPPING_COUNTRY_TZ",
            ShippingCountry::Ua => "SHIPPING_COUNTRY_UA",
            ShippingCountry::Ug => "SHIPPING_COUNTRY_UG",
            ShippingCountry::Us => "SHIPPING_COUNTRY_US",
            ShippingCountry::Uy => "SHIPPING_COUNTRY_UY",
            ShippingCountry::Uz => "SHIPPING_COUNTRY_UZ",
            ShippingCountry::Va => "SHIPPING_COUNTRY_VA",
            ShippingCountry::Vc => "SHIPPING_COUNTRY_VC",
            ShippingCountry::Ve => "SHIPPING_COUNTRY_VE",
            ShippingCountry::Vg => "SHIPPING_COUNTRY_VG",
            ShippingCountry::Vn => "SHIPPING_COUNTRY_VN",
            ShippingCountry::Vu => "SHIPPING_COUNTRY_VU",
            ShippingCountry::Wf => "SHIPPING_COUNTRY_WF",
            ShippingCountry::Ws => "SHIPPING_COUNTRY_WS",
            ShippingCountry::Xk => "SHIPPING_COUNTRY_XK",
            ShippingCountry::Ye => "SHIPPING_COUNTRY_YE",
            ShippingCountry::Yt => "SHIPPING_COUNTRY_YT",
            ShippingCountry::Za => "SHIPPING_COUNTRY_ZA",
            ShippingCountry::Zm => "SHIPPING_COUNTRY_ZM",
            ShippingCountry::Zw => "SHIPPING_COUNTRY_ZW",
            ShippingCountry::Zz => "SHIPPING_COUNTRY_ZZ",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SHIPPING_COUNTRY_UNSPECIFIED" => Some(Self::Unspecified),
            "SHIPPING_COUNTRY_AC" => Some(Self::Ac),
            "SHIPPING_COUNTRY_AD" => Some(Self::Ad),
            "SHIPPING_COUNTRY_AE" => Some(Self::Ae),
            "SHIPPING_COUNTRY_AF" => Some(Self::Af),
            "SHIPPING_COUNTRY_AG" => Some(Self::Ag),
            "SHIPPING_COUNTRY_AI" => Some(Self::Ai),
            "SHIPPING_COUNTRY_AL" => Some(Self::Al),
            "SHIPPING_COUNTRY_AM" => Some(Self::Am),
            "SHIPPING_COUNTRY_AO" => Some(Self::Ao),
            "SHIPPING_COUNTRY_AQ" => Some(Self::Aq),
            "SHIPPING_COUNTRY_AR" => Some(Self::Ar),
            "SHIPPING_COUNTRY_AT" => Some(Self::At),
            "SHIPPING_COUNTRY_AU" => Some(Self::Au),
            "SHIPPING_COUNTRY_AW" => Some(Self::Aw),
            "SHIPPING_COUNTRY_AX" => Some(Self::Ax),
            "SHIPPING_COUNTRY_AZ" => Some(Self::Az),
            "SHIPPING_COUNTRY_BA" => Some(Self::Ba),
            "SHIPPING_COUNTRY_BB" => Some(Self::Bb),
            "SHIPPING_COUNTRY_BD" => Some(Self::Bd),
            "SHIPPING_COUNTRY_BE" => Some(Self::Be),
            "SHIPPING_COUNTRY_BF" => Some(Self::Bf),
            "SHIPPING_COUNTRY_BG" => Some(Self::Bg),
            "SHIPPING_COUNTRY_BH" => Some(Self::Bh),
            "SHIPPING_COUNTRY_BI" => Some(Self::Bi),
            "SHIPPING_COUNTRY_BJ" => Some(Self::Bj),
            "SHIPPING_COUNTRY_BL" => Some(Self::Bl),
            "SHIPPING_COUNTRY_BM" => Some(Self::Bm),
            "SHIPPING_COUNTRY_BN" => Some(Self::Bn),
            "SHIPPING_COUNTRY_BO" => Some(Self::Bo),
            "SHIPPING_COUNTRY_BQ" => Some(Self::Bq),
            "SHIPPING_COUNTRY_BR" => Some(Self::Br),
            "SHIPPING_COUNTRY_BS" => Some(Self::Bs),
            "SHIPPING_COUNTRY_BT" => Some(Self::Bt),
            "SHIPPING_COUNTRY_BV" => Some(Self::Bv),
            "SHIPPING_COUNTRY_BW" => Some(Self::Bw),
            "SHIPPING_COUNTRY_BY" => Some(Self::By),
            "SHIPPING_COUNTRY_BZ" => Some(Self::Bz),
            "SHIPPING_COUNTRY_CA" => Some(Self::Ca),
            "SHIPPING_COUNTRY_CD" => Some(Self::Cd),
            "SHIPPING_COUNTRY_CF" => Some(Self::Cf),
            "SHIPPING_COUNTRY_CG" => Some(Self::Cg),
            "SHIPPING_COUNTRY_CH" => Some(Self::Ch),
            "SHIPPING_COUNTRY_CI" => Some(Self::Ci),
            "SHIPPING_COUNTRY_CK" => Some(Self::Ck),
            "SHIPPING_COUNTRY_CL" => Some(Self::Cl),
            "SHIPPING_COUNTRY_CM" => Some(Self::Cm),
            "SHIPPING_COUNTRY_CN" => Some(Self::Cn),
            "SHIPPING_COUNTRY_CO" => Some(Self::Co),
            "SHIPPING_COUNTRY_CR" => Some(Self::Cr),
            "SHIPPING_COUNTRY_CV" => Some(Self::Cv),
            "SHIPPING_COUNTRY_CW" => Some(Self::Cw),
            "SHIPPING_COUNTRY_CY" => Some(Self::Cy),
            "SHIPPING_COUNTRY_CZ" => Some(Self::Cz),
            "SHIPPING_COUNTRY_DE" => Some(Self::De),
            "SHIPPING_COUNTRY_DJ" => Some(Self::Dj),
            "SHIPPING_COUNTRY_DK" => Some(Self::Dk),
            "SHIPPING_COUNTRY_DM" => Some(Self::Dm),
            "SHIPPING_COUNTRY_DO" => Some(Self::Do),
            "SHIPPING_COUNTRY_DZ" => Some(Self::Dz),
            "SHIPPING_COUNTRY_EC" => Some(Self::Ec),
            "SHIPPING_COUNTRY_EE" => Some(Self::Ee),
            "SHIPPING_COUNTRY_EG" => Some(Self::Eg),
            "SHIPPING_COUNTRY_EH" => Some(Self::Eh),
            "SHIPPING_COUNTRY_ER" => Some(Self::Er),
            "SHIPPING_COUNTRY_ES" => Some(Self::Es),
            "SHIPPING_COUNTRY_ET" => Some(Self::Et),
            "SHIPPING_COUNTRY_FI" => Some(Self::Fi),
            "SHIPPING_COUNTRY_FJ" => Some(Self::Fj),
            "SHIPPING_COUNTRY_FK" => Some(Self::Fk),
            "SHIPPING_COUNTRY_FO" => Some(Self::Fo),
            "SHIPPING_COUNTRY_FR" => Some(Self::Fr),
            "SHIPPING_COUNTRY_GA" => Some(Self::Ga),
            "SHIPPING_COUNTRY_GB" => Some(Self::Gb),
            "SHIPPING_COUNTRY_GD" => Some(Self::Gd),
            "SHIPPING_COUNTRY_GE" => Some(Self::Ge),
            "SHIPPING_COUNTRY_GF" => Some(Self::Gf),
            "SHIPPING_COUNTRY_GG" => Some(Self::Gg),
            "SHIPPING_COUNTRY_GH" => Some(Self::Gh),
            "SHIPPING_COUNTRY_GI" => Some(Self::Gi),
            "SHIPPING_COUNTRY_GL" => Some(Self::Gl),
            "SHIPPING_COUNTRY_GM" => Some(Self::Gm),
            "SHIPPING_COUNTRY_GN" => Some(Self::Gn),
            "SHIPPING_COUNTRY_GP" => Some(Self::Gp),
            "SHIPPING_COUNTRY_GQ" => Some(Self::Gq),
            "SHIPPING_COUNTRY_GR" => Some(Self::Gr),
            "SHIPPING_COUNTRY_GS" => Some(Self::Gs),
            "SHIPPING_COUNTRY_GT" => Some(Self::Gt),
            "SHIPPING_COUNTRY_GU" => Some(Self::Gu),
            "SHIPPING_COUNTRY_GW" => Some(Self::Gw),
            "SHIPPING_COUNTRY_GY" => Some(Self::Gy),
            "SHIPPING_COUNTRY_HK" => Some(Self::Hk),
            "SHIPPING_COUNTRY_HN" => Some(Self::Hn),
            "SHIPPING_COUNTRY_HR" => Some(Self::Hr),
            "SHIPPING_COUNTRY_HT" => Some(Self::Ht),
            "SHIPPING_COUNTRY_HU" => Some(Self::Hu),
            "SHIPPING_COUNTRY_ID" => Some(Self::Id),
            "SHIPPING_COUNTRY_IE" => Some(Self::Ie),
            "SHIPPING_COUNTRY_IL" => Some(Self::Il),
            "SHIPPING_COUNTRY_IM" => Some(Self::Im),
            "SHIPPING_COUNTRY_IN" => Some(Self::In),
            "SHIPPING_COUNTRY_IO" => Some(Self::Io),
            "SHIPPING_COUNTRY_IQ" => Some(Self::Iq),
            "SHIPPING_COUNTRY_IS" => Some(Self::Is),
            "SHIPPING_COUNTRY_IT" => Some(Self::It),
            "SHIPPING_COUNTRY_JE" => Some(Self::Je),
            "SHIPPING_COUNTRY_JM" => Some(Self::Jm),
            "SHIPPING_COUNTRY_JO" => Some(Self::Jo),
            "SHIPPING_COUNTRY_JP" => Some(Self::Jp),
            "SHIPPING_COUNTRY_KE" => Some(Self::Ke),
            "SHIPPING_COUNTRY_KG" => Some(Self::Kg),
            "SHIPPING_COUNTRY_KH" => Some(Self::Kh),
            "SHIPPING_COUNTRY_KI" => Some(Self::Ki),
            "SHIPPING_COUNTRY_KM" => Some(Self::Km),
            "SHIPPING_COUNTRY_KN" => Some(Self::Kn),
            "SHIPPING_COUNTRY_KR" => Some(Self::Kr),
            "SHIPPING_COUNTRY_KW" => Some(Self::Kw),
            "SHIPPING_COUNTRY_KY" => Some(Self::Ky),
            "SHIPPING_COUNTRY_LA" => Some(Self::La),
            "SHIPPING_COUNTRY_LB" => Some(Self::Lb),
            "SHIPPING_COUNTRY_LC" => Some(Self::Lc),
            "SHIPPING_COUNTRY_LI" => Some(Self::Li),
            "SHIPPING_COUNTRY_LK" => Some(Self::Lk),
            "SHIPPING_COUNTRY_LR" => Some(Self::Lr),
            "SHIPPING_COUNTRY_LS" => Some(Self::Ls),
            "SHIPPING_COUNTRY_LT" => Some(Self::Lt),
            "SHIPPING_COUNTRY_LU" => Some(Self::Lu),
            "SHIPPING_COUNTRY_LV" => Some(Self::Lv),
            "SHIPPING_COUNTRY_LY" => Some(Self::Ly),
            "SHIPPING_COUNTRY_MA" => Some(Self::Ma),
            "SHIPPING_COUNTRY_MC" => Some(Self::Mc),
            "SHIPPING_COUNTRY_MD" => Some(Self::Md),
            "SHIPPING_COUNTRY_ME" => Some(Self::Me),
            "SHIPPING_COUNTRY_MF" => Some(Self::Mf),
            "SHIPPING_COUNTRY_MG" => Some(Self::Mg),
            "SHIPPING_COUNTRY_MK" => Some(Self::Mk),
            "SHIPPING_COUNTRY_ML" => Some(Self::Ml),
            "SHIPPING_COUNTRY_MM" => Some(Self::Mm),
            "SHIPPING_COUNTRY_MN" => Some(Self::Mn),
            "SHIPPING_COUNTRY_MO" => Some(Self::Mo),
            "SHIPPING_COUNTRY_MQ" => Some(Self::Mq),
            "SHIPPING_COUNTRY_MR" => Some(Self::Mr),
            "SHIPPING_COUNTRY_MS" => Some(Self::Ms),
            "SHIPPING_COUNTRY_MT" => Some(Self::Mt),
            "SHIPPING_COUNTRY_MU" => Some(Self::Mu),
            "SHIPPING_COUNTRY_MV" => Some(Self::Mv),
            "SHIPPING_COUNTRY_MW" => Some(Self::Mw),
            "SHIPPING_COUNTRY_MX" => Some(Self::Mx),
            "SHIPPING_COUNTRY_MY" => Some(Self::My),
            "SHIPPING_COUNTRY_MZ" => Some(Self::Mz),
            "SHIPPING_COUNTRY_NA" => Some(Self::Na),
            "SHIPPING_COUNTRY_NC" => Some(Self::Nc),
            "SHIPPING_COUNTRY_NE" => Some(Self::Ne),
            "SHIPPING_COUNTRY_NG" => Some(Self::Ng),
            "SHIPPING_COUNTRY_NI" => Some(Self::Ni),
            "SHIPPING_COUNTRY_NL" => Some(Self::Nl),
            "SHIPPING_COUNTRY_NO" => Some(Self::No),
            "SHIPPING_COUNTRY_NP" => Some(Self::Np),
            "SHIPPING_COUNTRY_NR" => Some(Self::Nr),
            "SHIPPING_COUNTRY_NU" => Some(Self::Nu),
            "SHIPPING_COUNTRY_NZ" => Some(Self::Nz),
            "SHIPPING_COUNTRY_OM" => Some(Self::Om),
            "SHIPPING_COUNTRY_PA" => Some(Self::Pa),
            "SHIPPING_COUNTRY_PE" => Some(Self::Pe),
            "SHIPPING_COUNTRY_PF" => Some(Self::Pf),
            "SHIPPING_COUNTRY_PG" => Some(Self::Pg),
            "SHIPPING_COUNTRY_PH" => Some(Self::Ph),
            "SHIPPING_COUNTRY_PK" => Some(Self::Pk),
            "SHIPPING_COUNTRY_PL" => Some(Self::Pl),
            "SHIPPING_COUNTRY_PM" => Some(Self::Pm),
            "SHIPPING_COUNTRY_PN" => Some(Self::Pn),
            "SHIPPING_COUNTRY_PR" => Some(Self::Pr),
            "SHIPPING_COUNTRY_PS" => Some(Self::Ps),
            "SHIPPING_COUNTRY_PT" => Some(Self::Pt),
            "SHIPPING_COUNTRY_PY" => Some(Self::Py),
            "SHIPPING_COUNTRY_QA" => Some(Self::Qa),
            "SHIPPING_COUNTRY_RE" => Some(Self::Re),
            "SHIPPING_COUNTRY_RO" => Some(Self::Ro),
            "SHIPPING_COUNTRY_RS" => Some(Self::Rs),
            "SHIPPING_COUNTRY_RU" => Some(Self::Ru),
            "SHIPPING_COUNTRY_RW" => Some(Self::Rw),
            "SHIPPING_COUNTRY_SA" => Some(Self::Sa),
            "SHIPPING_COUNTRY_SB" => Some(Self::Sb),
            "SHIPPING_COUNTRY_SC" => Some(Self::Sc),
            "SHIPPING_COUNTRY_SE" => Some(Self::Se),
            "SHIPPING_COUNTRY_SG" => Some(Self::Sg),
            "SHIPPING_COUNTRY_SH" => Some(Self::Sh),
            "SHIPPING_COUNTRY_SI" => Some(Self::Si),
            "SHIPPING_COUNTRY_SJ" => Some(Self::Sj),
            "SHIPPING_COUNTRY_SK" => Some(Self::Sk),
            "SHIPPING_COUNTRY_SL" => Some(Self::Sl),
            "SHIPPING_COUNTRY_SM" => Some(Self::Sm),
            "SHIPPING_COUNTRY_SN" => Some(Self::Sn),
            "SHIPPING_COUNTRY_SO" => Some(Self::So),
            "SHIPPING_COUNTRY_SR" => Some(Self::Sr),
            "SHIPPING_COUNTRY_SS" => Some(Self::Ss),
            "SHIPPING_COUNTRY_ST" => Some(Self::St),
            "SHIPPING_COUNTRY_SV" => Some(Self::Sv),
            "SHIPPING_COUNTRY_SX" => Some(Self::Sx),
            "SHIPPING_COUNTRY_SZ" => Some(Self::Sz),
            "SHIPPING_COUNTRY_TA" => Some(Self::Ta),
            "SHIPPING_COUNTRY_TC" => Some(Self::Tc),
            "SHIPPING_COUNTRY_TD" => Some(Self::Td),
            "SHIPPING_COUNTRY_TF" => Some(Self::Tf),
            "SHIPPING_COUNTRY_TG" => Some(Self::Tg),
            "SHIPPING_COUNTRY_TH" => Some(Self::Th),
            "SHIPPING_COUNTRY_TJ" => Some(Self::Tj),
            "SHIPPING_COUNTRY_TK" => Some(Self::Tk),
            "SHIPPING_COUNTRY_TL" => Some(Self::Tl),
            "SHIPPING_COUNTRY_TM" => Some(Self::Tm),
            "SHIPPING_COUNTRY_TN" => Some(Self::Tn),
            "SHIPPING_COUNTRY_TO" => Some(Self::To),
            "SHIPPING_COUNTRY_TR" => Some(Self::Tr),
            "SHIPPING_COUNTRY_TT" => Some(Self::Tt),
            "SHIPPING_COUNTRY_TV" => Some(Self::Tv),
            "SHIPPING_COUNTRY_TW" => Some(Self::Tw),
            "SHIPPING_COUNTRY_TZ" => Some(Self::Tz),
            "SHIPPING_COUNTRY_UA" => Some(Self::Ua),
            "SHIPPING_COUNTRY_UG" => Some(Self::Ug),
            "SHIPPING_COUNTRY_US" => Some(Self::Us),
            "SHIPPING_COUNTRY_UY" => Some(Self::Uy),
            "SHIPPING_COUNTRY_UZ" => Some(Self::Uz),
            "SHIPPING_COUNTRY_VA" => Some(Self::Va),
            "SHIPPING_COUNTRY_VC" => Some(Self::Vc),
            "SHIPPING_COUNTRY_VE" => Some(Self::Ve),
            "SHIPPING_COUNTRY_VG" => Some(Self::Vg),
            "SHIPPING_COUNTRY_VN" => Some(Self::Vn),
            "SHIPPING_COUNTRY_VU" => Some(Self::Vu),
            "SHIPPING_COUNTRY_WF" => Some(Self::Wf),
            "SHIPPING_COUNTRY_WS" => Some(Self::Ws),
            "SHIPPING_COUNTRY_XK" => Some(Self::Xk),
            "SHIPPING_COUNTRY_YE" => Some(Self::Ye),
            "SHIPPING_COUNTRY_YT" => Some(Self::Yt),
            "SHIPPING_COUNTRY_ZA" => Some(Self::Za),
            "SHIPPING_COUNTRY_ZM" => Some(Self::Zm),
            "SHIPPING_COUNTRY_ZW" => Some(Self::Zw),
            "SHIPPING_COUNTRY_ZZ" => Some(Self::Zz),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod shipping_rate_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ShippingRateServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ShippingRateServiceClient<tonic::transport::Channel> {
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
    impl<T> ShippingRateServiceClient<T>
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
        ) -> ShippingRateServiceClient<InterceptedService<T, F>>
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
            ShippingRateServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn put_shipping_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::PutShippingRateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutShippingRateResponse>,
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
                "/sited_io.commerce.v1.ShippingRateService/PutShippingRate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v1.ShippingRateService",
                        "PutShippingRate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_shipping_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShippingRateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetShippingRateResponse>,
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
                "/sited_io.commerce.v1.ShippingRateService/GetShippingRate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v1.ShippingRateService",
                        "GetShippingRate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_shipping_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteShippingRateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteShippingRateResponse>,
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
                "/sited_io.commerce.v1.ShippingRateService/DeleteShippingRate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sited_io.commerce.v1.ShippingRateService",
                        "DeleteShippingRate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
