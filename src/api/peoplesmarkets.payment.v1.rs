#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StripeAccount {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub stripe_account_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StripeAccountDetails {
    #[prost(bool, tag = "1")]
    pub charges_enabled: bool,
    #[prost(bool, tag = "2")]
    pub details_submitted: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountResponse {
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<StripeAccount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountLinkRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub refresh_url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub return_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountLinkResponse {
    #[prost(string, tag = "1")]
    pub link: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountResponse {
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<StripeAccount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountDetailsRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountDetailsResponse {
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<StripeAccount>,
    #[prost(message, optional, tag = "2")]
    pub details: ::core::option::Option<StripeAccountDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCheckoutSessionRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub success_url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub cancel_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCheckoutSessionResponse {
    #[prost(string, tag = "1")]
    pub link: ::prost::alloc::string::String,
}
/// Generated server implementations.
pub mod stripe_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StripeServiceServer.
    #[async_trait]
    pub trait StripeService: Send + Sync + 'static {
        async fn create_account(
            &self,
            request: tonic::Request<super::CreateAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAccountResponse>,
            tonic::Status,
        >;
        async fn create_account_link(
            &self,
            request: tonic::Request<super::CreateAccountLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAccountLinkResponse>,
            tonic::Status,
        >;
        async fn get_account(
            &self,
            request: tonic::Request<super::GetAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountResponse>,
            tonic::Status,
        >;
        async fn get_account_details(
            &self,
            request: tonic::Request<super::GetAccountDetailsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountDetailsResponse>,
            tonic::Status,
        >;
        async fn create_checkout_session(
            &self,
            request: tonic::Request<super::CreateCheckoutSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCheckoutSessionResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct StripeServiceServer<T: StripeService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: StripeService> StripeServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StripeServiceServer<T>
    where
        T: StripeService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/peoplesmarkets.payment.v1.StripeService/CreateAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAccountSvc<T: StripeService>(pub Arc<T>);
                    impl<
                        T: StripeService,
                    > tonic::server::UnaryService<super::CreateAccountRequest>
                    for CreateAccountSvc<T> {
                        type Response = super::CreateAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StripeService>::create_account(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.payment.v1.StripeService/CreateAccountLink" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAccountLinkSvc<T: StripeService>(pub Arc<T>);
                    impl<
                        T: StripeService,
                    > tonic::server::UnaryService<super::CreateAccountLinkRequest>
                    for CreateAccountLinkSvc<T> {
                        type Response = super::CreateAccountLinkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAccountLinkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StripeService>::create_account_link(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateAccountLinkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.payment.v1.StripeService/GetAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountSvc<T: StripeService>(pub Arc<T>);
                    impl<
                        T: StripeService,
                    > tonic::server::UnaryService<super::GetAccountRequest>
                    for GetAccountSvc<T> {
                        type Response = super::GetAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StripeService>::get_account(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.payment.v1.StripeService/GetAccountDetails" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountDetailsSvc<T: StripeService>(pub Arc<T>);
                    impl<
                        T: StripeService,
                    > tonic::server::UnaryService<super::GetAccountDetailsRequest>
                    for GetAccountDetailsSvc<T> {
                        type Response = super::GetAccountDetailsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountDetailsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StripeService>::get_account_details(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountDetailsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.payment.v1.StripeService/CreateCheckoutSession" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCheckoutSessionSvc<T: StripeService>(pub Arc<T>);
                    impl<
                        T: StripeService,
                    > tonic::server::UnaryService<super::CreateCheckoutSessionRequest>
                    for CreateCheckoutSessionSvc<T> {
                        type Response = super::CreateCheckoutSessionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCheckoutSessionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StripeService>::create_checkout_session(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateCheckoutSessionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: StripeService> Clone for StripeServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: StripeService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: StripeService> tonic::server::NamedService for StripeServiceServer<T> {
        const NAME: &'static str = "peoplesmarkets.payment.v1.StripeService";
    }
}
