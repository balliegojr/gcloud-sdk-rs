/// The representation of a key managed by the API Keys API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Key {
    /// Output only. The resource name of the key.
    /// The `name` has the form:
    /// `projects/<PROJECT_NUMBER>/locations/global/keys/<KEY_ID>`.
    /// For example:
    /// `projects/123456867718/locations/global/keys/b7ff1f9f-8275-410a-94dd-3855ee9b5dd2`
    ///
    /// NOTE: Key is a global resource; hence the only supported value for
    /// location is `global`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Unique id in UUID4 format.
    #[prost(string, tag = "5")]
    pub uid: ::prost::alloc::string::String,
    /// Human-readable display name of this key that you can modify.
    /// The maximum length is 63 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. An encrypted and signed value held by this key.
    /// This field can be accessed only through the `GetKeyString` method.
    #[prost(string, tag = "3")]
    pub key_string: ::prost::alloc::string::String,
    /// Output only. A timestamp identifying the time this key was originally
    /// created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. A timestamp identifying the time this key was last
    /// updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. A timestamp when this key was deleted. If the resource is not deleted,
    /// this must be empty.
    #[prost(message, optional, tag = "7")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Annotations is an unstructured key-value map stored with a policy that
    /// may be set by external tools to store and retrieve arbitrary metadata.
    /// They are not queryable and should be preserved when modifying objects.
    #[prost(map = "string, string", tag = "8")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Key restrictions.
    #[prost(message, optional, tag = "9")]
    pub restrictions: ::core::option::Option<Restrictions>,
    /// Output only. A checksum computed by the server based on the current value of the Key
    /// resource. This may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    /// See <https://google.aip.dev/154.>
    #[prost(string, tag = "11")]
    pub etag: ::prost::alloc::string::String,
}
/// Describes the restrictions on the key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Restrictions {
    /// A restriction for a specific service and optionally one or
    /// more specific methods. Requests are allowed if they
    /// match any of these restrictions. If no restrictions are
    /// specified, all targets are allowed.
    #[prost(message, repeated, tag = "5")]
    pub api_targets: ::prost::alloc::vec::Vec<ApiTarget>,
    /// The websites, IP addresses, Android apps, or iOS apps (the clients) that
    /// are allowed to use the key. You can specify only one type of client
    /// restrictions per key.
    #[prost(oneof = "restrictions::ClientRestrictions", tags = "1, 2, 3, 4")]
    pub client_restrictions: ::core::option::Option<restrictions::ClientRestrictions>,
}
/// Nested message and enum types in `Restrictions`.
pub mod restrictions {
    /// The websites, IP addresses, Android apps, or iOS apps (the clients) that
    /// are allowed to use the key. You can specify only one type of client
    /// restrictions per key.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClientRestrictions {
        /// The HTTP referrers (websites) that are allowed to use the key.
        #[prost(message, tag = "1")]
        BrowserKeyRestrictions(super::BrowserKeyRestrictions),
        /// The IP addresses of callers that are allowed to use the key.
        #[prost(message, tag = "2")]
        ServerKeyRestrictions(super::ServerKeyRestrictions),
        /// The Android apps that are allowed to use the key.
        #[prost(message, tag = "3")]
        AndroidKeyRestrictions(super::AndroidKeyRestrictions),
        /// The iOS apps that are allowed to use the key.
        #[prost(message, tag = "4")]
        IosKeyRestrictions(super::IosKeyRestrictions),
    }
}
/// The HTTP referrers (websites) that are allowed to use the key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrowserKeyRestrictions {
    /// A list of regular expressions for the referrer URLs that are allowed
    /// to make API calls with this key.
    #[prost(string, repeated, tag = "1")]
    pub allowed_referrers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The IP addresses of callers that are allowed to use the key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerKeyRestrictions {
    /// A list of the caller IP addresses that are allowed to make API calls
    /// with this key.
    #[prost(string, repeated, tag = "1")]
    pub allowed_ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The Android apps that are allowed to use the key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidKeyRestrictions {
    /// A list of Android applications that are allowed to make API calls with
    /// this key.
    #[prost(message, repeated, tag = "1")]
    pub allowed_applications: ::prost::alloc::vec::Vec<AndroidApplication>,
}
/// Identifier of an Android application for key use.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidApplication {
    /// The SHA1 fingerprint of the application. For example, both sha1 formats are
    /// acceptable : DA:39:A3:EE:5E:6B:4B:0D:32:55:BF:EF:95:60:18:90:AF:D8:07:09 or
    /// DA39A3EE5E6B4B0D3255BFEF95601890AFD80709.
    /// Output format is the latter.
    #[prost(string, tag = "1")]
    pub sha1_fingerprint: ::prost::alloc::string::String,
    /// The package name of the application.
    #[prost(string, tag = "2")]
    pub package_name: ::prost::alloc::string::String,
}
/// The iOS apps that are allowed to use the key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosKeyRestrictions {
    /// A list of bundle IDs that are allowed when making API calls with this key.
    #[prost(string, repeated, tag = "1")]
    pub allowed_bundle_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A restriction for a specific service and optionally one or multiple
/// specific methods. Both fields are case insensitive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiTarget {
    /// The service for this restriction. It should be the canonical
    /// service name, for example: `translate.googleapis.com`.
    /// You can use [`gcloud services list`](/sdk/gcloud/reference/services/list)
    /// to get a list of services that are enabled in the project.
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    /// Optional. List of one or more methods that can be called.
    /// If empty, all methods for the service are allowed. A wildcard
    /// (*) can be used as the last symbol.
    /// Valid examples:
    ///    `google.cloud.translate.v2.TranslateService.GetSupportedLanguage`
    ///    `TranslateText`
    ///    `Get*`
    ///    `translate.googleapis.com.Get*`
    #[prost(string, repeated, tag = "2")]
    pub methods: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for `CreateKey` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyRequest {
    /// Required. The project in which the API key is created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The API key fields to set at creation time.
    /// You can configure only the `display_name`, `restrictions`, and
    /// `annotations` fields.
    #[prost(message, optional, tag = "2")]
    pub key: ::core::option::Option<Key>,
    /// User specified key id (optional). If specified, it will become the final
    /// component of the key resource name.
    ///
    /// The id must be unique within the project, must conform with RFC-1034,
    /// is restricted to lower-cased letters, and has a maximum length of 63
    /// characters. In another word, the id must match the regular
    /// expression: `\[a-z]([a-z0-9-]{0,61}[a-z0-9\])?`.
    ///
    /// The id must NOT be a UUID-like string.
    #[prost(string, tag = "3")]
    pub key_id: ::prost::alloc::string::String,
}
/// Request message for `ListKeys` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKeysRequest {
    /// Required. Lists all API keys associated with this project.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Specifies the maximum number of results to be returned at a time.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Requests a specific page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Indicate that keys deleted in the past 30 days should also be
    /// returned.
    #[prost(bool, tag = "6")]
    pub show_deleted: bool,
}
/// Response message for `ListKeys` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKeysResponse {
    /// A list of API keys.
    #[prost(message, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<Key>,
    /// The pagination token for the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `GetKey` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyRequest {
    /// Required. The resource name of the API key to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `GetKeyString` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyStringRequest {
    /// Required. The resource name of the API key to be retrieved.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for `GetKeyString` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyStringResponse {
    /// An encrypted and signed value of the key.
    #[prost(string, tag = "1")]
    pub key_string: ::prost::alloc::string::String,
}
/// Request message for `UpdateKey` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateKeyRequest {
    /// Required. Set the `name` field to the resource name of the API key to be
    /// updated. You can update only the `display_name`, `restrictions`, and
    /// `annotations` fields.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<Key>,
    /// The field mask specifies which fields to be updated as part of this
    /// request. All other fields are ignored.
    /// Mutable fields are: `display_name`, `restrictions`, and `annotations`.
    /// If an update mask is not provided, the service treats it as an implied mask
    /// equivalent to all allowed fields that are set on the wire. If the field
    /// mask has a special value "*", the service treats it equivalent to replace
    /// all allowed mutable fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for `DeleteKey` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteKeyRequest {
    /// Required. The resource name of the API key to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The etag known to the client for the expected state of the key.
    /// This is to be used for optimistic concurrency.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for `UndeleteKey` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteKeyRequest {
    /// Required. The resource name of the API key to be undeleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `LookupKey` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupKeyRequest {
    /// Required. Finds the project that owns the key string value.
    #[prost(string, tag = "1")]
    pub key_string: ::prost::alloc::string::String,
}
/// Response message for `LookupKey` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupKeyResponse {
    /// The project that owns the key with the value specified in the request.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The resource name of the API key. If the API key has been purged,
    /// resource name is empty.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod api_keys_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages the API keys associated with projects.
    #[derive(Debug, Clone)]
    pub struct ApiKeysClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ApiKeysClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ApiKeysClient<T>
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
        ) -> ApiKeysClient<InterceptedService<T, F>>
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
            ApiKeysClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new API key.
        ///
        /// NOTE: Key is a global resource; hence the only supported value for
        /// location is `global`.
        pub async fn create_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateKeyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.api.apikeys.v2.ApiKeys/CreateKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the API keys owned by a project. The key string of the API key
        /// isn't included in the response.
        ///
        /// NOTE: Key is a global resource; hence the only supported value for
        /// location is `global`.
        pub async fn list_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListKeysRequest>,
        ) -> Result<tonic::Response<super::ListKeysResponse>, tonic::Status> {
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
                "/google.api.apikeys.v2.ApiKeys/ListKeys",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the metadata for an API key. The key string of the API key
        /// isn't included in the response.
        ///
        /// NOTE: Key is a global resource; hence the only supported value for
        /// location is `global`.
        pub async fn get_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeyRequest>,
        ) -> Result<tonic::Response<super::Key>, tonic::Status> {
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
                "/google.api.apikeys.v2.ApiKeys/GetKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get the key string for an API key.
        ///
        /// NOTE: Key is a global resource; hence the only supported value for
        /// location is `global`.
        pub async fn get_key_string(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeyStringRequest>,
        ) -> Result<tonic::Response<super::GetKeyStringResponse>, tonic::Status> {
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
                "/google.api.apikeys.v2.ApiKeys/GetKeyString",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Patches the modifiable fields of an API key.
        /// The key string of the API key isn't included in the response.
        ///
        /// NOTE: Key is a global resource; hence the only supported value for
        /// location is `global`.
        pub async fn update_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateKeyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.api.apikeys.v2.ApiKeys/UpdateKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an API key. Deleted key can be retrieved within 30 days of
        /// deletion. Afterward, key will be purged from the project.
        ///
        /// NOTE: Key is a global resource; hence the only supported value for
        /// location is `global`.
        pub async fn delete_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteKeyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.api.apikeys.v2.ApiKeys/DeleteKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Undeletes an API key which was deleted within 30 days.
        ///
        /// NOTE: Key is a global resource; hence the only supported value for
        /// location is `global`.
        pub async fn undelete_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteKeyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.api.apikeys.v2.ApiKeys/UndeleteKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Find the parent project and resource name of the API
        /// key that matches the key string in the request. If the API key has been
        /// purged, resource name will not be set.
        /// The service account must have the `apikeys.keys.lookup` permission
        /// on the parent project.
        pub async fn lookup_key(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupKeyRequest>,
        ) -> Result<tonic::Response<super::LookupKeyResponse>, tonic::Status> {
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
                "/google.api.apikeys.v2.ApiKeys/LookupKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
