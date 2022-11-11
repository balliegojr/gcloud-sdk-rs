/// Request message for CreateInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. Parent resource of the Instance, of the form: `projects/*/locations/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Identifier to assign to the Instance. Must be unique within scope of the
    /// parent resource.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Required. The Instance.
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
}
/// Request message for DeleteInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. The name of the Instance to delete.
    /// Format: `projects/*/locations/*/instances/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. The name of the Instance to retrieve.
    /// Format: `projects/*/locations/*/instances/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub cancellation_requested: bool,
    /// API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// An Instance represents the instance resources of the Registry.
/// Currently, only one instance is allowed for each project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Format: `projects/*/locations/*/instance`.
    /// Currently only `locations/global` is supported.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation timestamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update timestamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the Instance.
    #[prost(enumeration = "instance::State", tag = "4")]
    pub state: i32,
    /// Output only. Extra information of Instance.State if the state is `FAILED`.
    #[prost(string, tag = "5")]
    pub state_message: ::prost::alloc::string::String,
    /// Required. Config of the Instance.
    #[prost(message, optional, tag = "6")]
    pub config: ::core::option::Option<instance::Config>,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Available configurations to provision an Instance.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Config {
        /// Output only. The GCP location where the Instance resides.
        #[prost(string, tag = "1")]
        pub location: ::prost::alloc::string::String,
        /// Required. The Customer Managed Encryption Key (CMEK) used for data encryption.
        /// The CMEK name should follow the format of
        /// `projects/(\[^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/\]+)`,
        /// where the `location` must match InstanceConfig.location.
        #[prost(string, tag = "2")]
        pub cmek_key_name: ::prost::alloc::string::String,
    }
    /// State of the Instance.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// The Instance has not been initialized or has been deleted.
        Inactive = 1,
        /// The Instance is being created.
        Creating = 2,
        /// The Instance has been created and is ready for use.
        Active = 3,
        /// The Instance is being updated.
        Updating = 4,
        /// The Instance is being deleted.
        Deleting = 5,
        /// The Instance encountered an error during a state change.
        Failed = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Inactive => "INACTIVE",
                State::Creating => "CREATING",
                State::Active => "ACTIVE",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Failed => "FAILED",
            }
        }
    }
}
/// Generated client implementations.
pub mod provisioning_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The service that is used for managing the data plane provisioning of the
    /// Registry.
    #[derive(Debug, Clone)]
    pub struct ProvisioningClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProvisioningClient<tonic::transport::Channel> {
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
    impl<T> ProvisioningClient<T>
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
        ) -> ProvisioningClient<InterceptedService<T, F>>
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
            ProvisioningClient::new(InterceptedService::new(inner, interceptor))
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
        /// Provisions instance resources for the Registry.
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
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
                "/google.cloud.apigeeregistry.v1.Provisioning/CreateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the Registry instance.
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
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
                "/google.cloud.apigeeregistry.v1.Provisioning/DeleteInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Instance.
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Provisioning/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A top-level description of an API.
/// Produced by producers and are commitments to provide services.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Api {
    /// Resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human-meaningful name.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// A detailed description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Creation timestamp.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update timestamp.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A user-definable description of the availability of this service.
    /// Format: free-form, but we expect single words that describe availability,
    /// e.g., "NONE", "TESTING", "PREVIEW", "GENERAL", "DEPRECATED", "SHUTDOWN".
    #[prost(string, tag = "6")]
    pub availability: ::prost::alloc::string::String,
    /// The recommended version of the API.
    /// Format: `apis/{api}/versions/{version}`
    #[prost(string, tag = "7")]
    pub recommended_version: ::prost::alloc::string::String,
    /// The recommended deployment of the API.
    /// Format: `apis/{api}/deployments/{deployment}`
    #[prost(string, tag = "8")]
    pub recommended_deployment: ::prost::alloc::string::String,
    /// Labels attach identifying metadata to resources. Identifying metadata can
    /// be used to filter list operations.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores, and dashes. International characters are allowed.
    /// No more than 64 user labels can be associated with one resource (System
    /// labels are excluded).
    ///
    /// See <https://goo.gl/xmQnxf> for more information and examples of labels.
    /// System reserved label keys are prefixed with
    /// `apigeeregistry.googleapis.com/` and cannot be changed.
    #[prost(map = "string, string", tag = "9")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Annotations attach non-identifying metadata to resources.
    ///
    /// Annotation keys and values are less restricted than those of labels, but
    /// should be generally used for small values of broad interest. Larger, topic-
    /// specific metadata should be stored in Artifacts.
    #[prost(map = "string, string", tag = "10")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Describes a particular version of an API.
/// ApiVersions are what consumers actually use.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiVersion {
    /// Resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human-meaningful name.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// A detailed description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Creation timestamp.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update timestamp.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A user-definable description of the lifecycle phase of this API version.
    /// Format: free-form, but we expect single words that describe API maturity,
    /// e.g., "CONCEPT", "DESIGN", "DEVELOPMENT", "STAGING", "PRODUCTION",
    /// "DEPRECATED", "RETIRED".
    #[prost(string, tag = "6")]
    pub state: ::prost::alloc::string::String,
    /// Labels attach identifying metadata to resources. Identifying metadata can
    /// be used to filter list operations.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    /// No more than 64 user labels can be associated with one resource (System
    /// labels are excluded).
    ///
    /// See <https://goo.gl/xmQnxf> for more information and examples of labels.
    /// System reserved label keys are prefixed with
    /// `apigeeregistry.googleapis.com/` and cannot be changed.
    #[prost(map = "string, string", tag = "7")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Annotations attach non-identifying metadata to resources.
    ///
    /// Annotation keys and values are less restricted than those of labels, but
    /// should be generally used for small values of broad interest. Larger, topic-
    /// specific metadata should be stored in Artifacts.
    #[prost(map = "string, string", tag = "8")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Describes a version of an API in a structured way.
/// ApiSpecs provide formal descriptions that consumers can use to use a version.
/// ApiSpec resources are intended to be fully-resolved descriptions of an
/// ApiVersion. When specs consist of multiple files, these should be bundled
/// together (e.g., in a zip archive) and stored as a unit. Multiple specs can
/// exist to provide representations in different API description formats.
/// Synchronization of these representations would be provided by tooling and
/// background services.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiSpec {
    /// Resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A possibly-hierarchical name used to refer to the spec from other specs.
    #[prost(string, tag = "2")]
    pub filename: ::prost::alloc::string::String,
    /// A detailed description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Immutable. The revision ID of the spec.
    /// A new revision is committed whenever the spec contents are changed.
    /// The format is an 8-character hexadecimal string.
    #[prost(string, tag = "4")]
    pub revision_id: ::prost::alloc::string::String,
    /// Output only. Creation timestamp; when the spec resource was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Revision creation timestamp; when the represented revision was created.
    #[prost(message, optional, tag = "6")]
    pub revision_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update timestamp: when the represented revision was last modified.
    #[prost(message, optional, tag = "7")]
    pub revision_update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A style (format) descriptor for this spec that is specified as a Media Type
    /// (<https://en.wikipedia.org/wiki/Media_type>). Possible values include
    /// `application/vnd.apigee.proto`, `application/vnd.apigee.openapi`, and
    /// `application/vnd.apigee.graphql`, with possible suffixes representing
    /// compression types. These hypothetical names are defined in the vendor tree
    /// defined in RFC6838 (<https://tools.ietf.org/html/rfc6838>) and are not final.
    /// Content types can specify compression. Currently only GZip compression is
    /// supported (indicated with "+gzip").
    #[prost(string, tag = "8")]
    pub mime_type: ::prost::alloc::string::String,
    /// Output only. The size of the spec file in bytes. If the spec is gzipped, this is the
    /// size of the uncompressed spec.
    #[prost(int32, tag = "9")]
    pub size_bytes: i32,
    /// Output only. A SHA-256 hash of the spec's contents. If the spec is gzipped, this is
    /// the hash of the uncompressed spec.
    #[prost(string, tag = "10")]
    pub hash: ::prost::alloc::string::String,
    /// The original source URI of the spec (if one exists).
    /// This is an external location that can be used for reference purposes
    /// but which may not be authoritative since this external resource may
    /// change after the spec is retrieved.
    #[prost(string, tag = "11")]
    pub source_uri: ::prost::alloc::string::String,
    /// Input only. The contents of the spec.
    /// Provided by API callers when specs are created or updated.
    /// To access the contents of a spec, use GetApiSpecContents.
    #[prost(bytes = "vec", tag = "12")]
    pub contents: ::prost::alloc::vec::Vec<u8>,
    /// Labels attach identifying metadata to resources. Identifying metadata can
    /// be used to filter list operations.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    /// No more than 64 user labels can be associated with one resource (System
    /// labels are excluded).
    ///
    /// See <https://goo.gl/xmQnxf> for more information and examples of labels.
    /// System reserved label keys are prefixed with
    /// `apigeeregistry.googleapis.com/` and cannot be changed.
    #[prost(map = "string, string", tag = "14")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Annotations attach non-identifying metadata to resources.
    ///
    /// Annotation keys and values are less restricted than those of labels, but
    /// should be generally used for small values of broad interest. Larger, topic-
    /// specific metadata should be stored in Artifacts.
    #[prost(map = "string, string", tag = "15")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Describes a service running at particular address that
/// provides a particular version of an API. ApiDeployments have revisions which
/// correspond to different configurations of a single deployment in time.
/// Revision identifiers should be updated whenever the served API spec or
/// endpoint address changes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiDeployment {
    /// Resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human-meaningful name.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// A detailed description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Immutable. The revision ID of the deployment.
    /// A new revision is committed whenever the deployment contents are changed.
    /// The format is an 8-character hexadecimal string.
    #[prost(string, tag = "4")]
    pub revision_id: ::prost::alloc::string::String,
    /// Output only. Creation timestamp; when the deployment resource was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Revision creation timestamp; when the represented revision was created.
    #[prost(message, optional, tag = "6")]
    pub revision_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update timestamp: when the represented revision was last modified.
    #[prost(message, optional, tag = "7")]
    pub revision_update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The full resource name (including revision ID) of the spec of the API being
    /// served by the deployment. Changes to this value will update the revision.
    /// Format: `apis/{api}/deployments/{deployment}`
    #[prost(string, tag = "8")]
    pub api_spec_revision: ::prost::alloc::string::String,
    /// The address where the deployment is serving. Changes to this value will
    /// update the revision.
    #[prost(string, tag = "9")]
    pub endpoint_uri: ::prost::alloc::string::String,
    /// The address of the external channel of the API (e.g., the Developer
    /// Portal). Changes to this value will not affect the revision.
    #[prost(string, tag = "10")]
    pub external_channel_uri: ::prost::alloc::string::String,
    /// Text briefly identifying the intended audience of the API. Changes to this
    /// value will not affect the revision.
    #[prost(string, tag = "11")]
    pub intended_audience: ::prost::alloc::string::String,
    /// Text briefly describing how to access the endpoint. Changes to this value
    /// will not affect the revision.
    #[prost(string, tag = "12")]
    pub access_guidance: ::prost::alloc::string::String,
    /// Labels attach identifying metadata to resources. Identifying metadata can
    /// be used to filter list operations.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    /// No more than 64 user labels can be associated with one resource (System
    /// labels are excluded).
    ///
    /// See <https://goo.gl/xmQnxf> for more information and examples of labels.
    /// System reserved label keys are prefixed with
    /// `apigeeregistry.googleapis.com/` and cannot be changed.
    #[prost(map = "string, string", tag = "14")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Annotations attach non-identifying metadata to resources.
    ///
    /// Annotation keys and values are less restricted than those of labels, but
    /// should be generally used for small values of broad interest. Larger, topic-
    /// specific metadata should be stored in Artifacts.
    #[prost(map = "string, string", tag = "15")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Artifacts of resources. Artifacts are unique (single-value) per resource
/// and are used to store metadata that is too large or numerous to be stored
/// directly on the resource. Since artifacts are stored separately from parent
/// resources, they should generally be used for metadata that is needed
/// infrequently, i.e., not for display in primary views of the resource but
/// perhaps displayed or downloaded upon request. The `ListArtifacts` method
/// allows artifacts to be quickly enumerated and checked for presence without
/// downloading their (potentially-large) contents.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Artifact {
    /// Resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation timestamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update timestamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A content type specifier for the artifact.
    /// Content type specifiers are Media Types
    /// (<https://en.wikipedia.org/wiki/Media_type>) with a possible "schema"
    /// parameter that specifies a schema for the stored information.
    /// Content types can specify compression. Currently only GZip compression is
    /// supported (indicated with "+gzip").
    #[prost(string, tag = "4")]
    pub mime_type: ::prost::alloc::string::String,
    /// Output only. The size of the artifact in bytes. If the artifact is gzipped, this is
    /// the size of the uncompressed artifact.
    #[prost(int32, tag = "5")]
    pub size_bytes: i32,
    /// Output only. A SHA-256 hash of the artifact's contents. If the artifact is gzipped,
    /// this is the hash of the uncompressed artifact.
    #[prost(string, tag = "6")]
    pub hash: ::prost::alloc::string::String,
    /// Input only. The contents of the artifact.
    /// Provided by API callers when artifacts are created or replaced.
    /// To access the contents of an artifact, use GetArtifactContents.
    #[prost(bytes = "vec", tag = "7")]
    pub contents: ::prost::alloc::vec::Vec<u8>,
}
/// Request message for ListApis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApisRequest {
    /// Required. The parent, which owns this collection of APIs.
    /// Format: `projects/*/locations/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of APIs to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 values will be returned.
    /// The maximum is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListApis` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListApis` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that can be used to filter the list. Filters use the Common
    /// Expression Language and can refer to all message fields.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for ListApis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApisResponse {
    /// The APIs from the specified publisher.
    #[prost(message, repeated, tag = "1")]
    pub apis: ::prost::alloc::vec::Vec<Api>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetApi.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiRequest {
    /// Required. The name of the API to retrieve.
    /// Format: `projects/*/locations/*/apis/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateApi.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiRequest {
    /// Required. The parent, which owns this collection of APIs.
    /// Format: `projects/*/locations/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The API to create.
    #[prost(message, optional, tag = "2")]
    pub api: ::core::option::Option<Api>,
    /// Required. The ID to use for the API, which will become the final component of
    /// the API's resource name.
    ///
    /// This value should be 4-63 characters, and valid characters
    /// are /\[a-z][0-9\]-/.
    ///
    /// Following AIP-162, IDs must not have the form of a UUID.
    #[prost(string, tag = "3")]
    pub api_id: ::prost::alloc::string::String,
}
/// Request message for UpdateApi.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApiRequest {
    /// Required. The API to update.
    ///
    /// The `name` field is used to identify the API to update.
    /// Format: `projects/*/locations/*/apis/*`
    #[prost(message, optional, tag = "1")]
    pub api: ::core::option::Option<Api>,
    /// The list of fields to be updated. If omitted, all fields are updated that
    /// are set in the request message (fields set to default values are ignored).
    /// If an asterisk "*" is specified, all fields are updated, including fields
    /// that are unspecified/default in the request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set to true, and the API is not found, a new API will be created.
    /// In this situation, `update_mask` is ignored.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
}
/// Request message for DeleteApi.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiRequest {
    /// Required. The name of the API to delete.
    /// Format: `projects/*/locations/*/apis/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set to true, any child resources will also be deleted.
    /// (Otherwise, the request will only work if there are no child resources.)
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for ListApiVersions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApiVersionsRequest {
    /// Required. The parent, which owns this collection of versions.
    /// Format: `projects/*/locations/*/apis/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of versions to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 values will be returned.
    /// The maximum is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListApiVersions` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListApiVersions` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that can be used to filter the list. Filters use the Common
    /// Expression Language and can refer to all message fields.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for ListApiVersions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApiVersionsResponse {
    /// The versions from the specified publisher.
    #[prost(message, repeated, tag = "1")]
    pub api_versions: ::prost::alloc::vec::Vec<ApiVersion>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetApiVersion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiVersionRequest {
    /// Required. The name of the version to retrieve.
    /// Format: `projects/*/locations/*/apis/*/versions/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateApiVersion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiVersionRequest {
    /// Required. The parent, which owns this collection of versions.
    /// Format: `projects/*/locations/*/apis/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The version to create.
    #[prost(message, optional, tag = "2")]
    pub api_version: ::core::option::Option<ApiVersion>,
    /// Required. The ID to use for the version, which will become the final component of
    /// the version's resource name.
    ///
    /// This value should be 1-63 characters, and valid characters
    /// are /\[a-z][0-9\]-/.
    ///
    /// Following AIP-162, IDs must not have the form of a UUID.
    #[prost(string, tag = "3")]
    pub api_version_id: ::prost::alloc::string::String,
}
/// Request message for UpdateApiVersion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApiVersionRequest {
    /// Required. The version to update.
    ///
    /// The `name` field is used to identify the version to update.
    /// Format: `projects/*/locations/*/apis/*/versions/*`
    #[prost(message, optional, tag = "1")]
    pub api_version: ::core::option::Option<ApiVersion>,
    /// The list of fields to be updated. If omitted, all fields are updated that
    /// are set in the request message (fields set to default values are ignored).
    /// If an asterisk "*" is specified, all fields are updated, including fields
    /// that are unspecified/default in the request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set to true, and the version is not found, a new version will be
    /// created. In this situation, `update_mask` is ignored.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
}
/// Request message for DeleteApiVersion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiVersionRequest {
    /// Required. The name of the version to delete.
    /// Format: `projects/*/locations/*/apis/*/versions/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set to true, any child resources will also be deleted.
    /// (Otherwise, the request will only work if there are no child resources.)
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for ListApiSpecs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApiSpecsRequest {
    /// Required. The parent, which owns this collection of specs.
    /// Format: `projects/*/locations/*/apis/*/versions/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of specs to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 values will be returned.
    /// The maximum is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListApiSpecs` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListApiSpecs` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that can be used to filter the list. Filters use the Common
    /// Expression Language and can refer to all message fields except contents.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for ListApiSpecs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApiSpecsResponse {
    /// The specs from the specified publisher.
    #[prost(message, repeated, tag = "1")]
    pub api_specs: ::prost::alloc::vec::Vec<ApiSpec>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetApiSpec.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiSpecRequest {
    /// Required. The name of the spec to retrieve.
    /// Format: `projects/*/locations/*/apis/*/versions/*/specs/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetApiSpecContents.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiSpecContentsRequest {
    /// Required. The name of the spec whose contents should be retrieved.
    /// Format: `projects/*/locations/*/apis/*/versions/*/specs/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateApiSpec.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiSpecRequest {
    /// Required. The parent, which owns this collection of specs.
    /// Format: `projects/*/locations/*/apis/*/versions/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The spec to create.
    #[prost(message, optional, tag = "2")]
    pub api_spec: ::core::option::Option<ApiSpec>,
    /// Required. The ID to use for the spec, which will become the final component of
    /// the spec's resource name.
    ///
    /// This value should be 4-63 characters, and valid characters
    /// are /\[a-z][0-9\]-/.
    ///
    /// Following AIP-162, IDs must not have the form of a UUID.
    #[prost(string, tag = "3")]
    pub api_spec_id: ::prost::alloc::string::String,
}
/// Request message for UpdateApiSpec.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApiSpecRequest {
    /// Required. The spec to update.
    ///
    /// The `name` field is used to identify the spec to update.
    /// Format: `projects/*/locations/*/apis/*/versions/*/specs/*`
    #[prost(message, optional, tag = "1")]
    pub api_spec: ::core::option::Option<ApiSpec>,
    /// The list of fields to be updated. If omitted, all fields are updated that
    /// are set in the request message (fields set to default values are ignored).
    /// If an asterisk "*" is specified, all fields are updated, including fields
    /// that are unspecified/default in the request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set to true, and the spec is not found, a new spec will be created.
    /// In this situation, `update_mask` is ignored.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
}
/// Request message for DeleteApiSpec.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiSpecRequest {
    /// Required. The name of the spec to delete.
    /// Format: `projects/*/locations/*/apis/*/versions/*/specs/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set to true, any child resources will also be deleted.
    /// (Otherwise, the request will only work if there are no child resources.)
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for TagApiSpecRevision.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagApiSpecRevisionRequest {
    /// Required. The name of the spec to be tagged, including the revision ID.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The tag to apply.
    /// The tag should be at most 40 characters, and match `\[a-z][a-z0-9-\]{3,39}`.
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
}
/// Request message for ListApiSpecRevisions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApiSpecRevisionsRequest {
    /// Required. The name of the spec to list revisions for.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The maximum number of revisions to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token, received from a previous ListApiSpecRevisions call.
    /// Provide this to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListApiSpecRevisionsResponse.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApiSpecRevisionsResponse {
    /// The revisions of the spec.
    #[prost(message, repeated, tag = "1")]
    pub api_specs: ::prost::alloc::vec::Vec<ApiSpec>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for RollbackApiSpec.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackApiSpecRequest {
    /// Required. The spec being rolled back.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The revision ID to roll back to.
    /// It must be a revision of the same spec.
    ///
    ///    Example: `c7cfa2a8`
    #[prost(string, tag = "2")]
    pub revision_id: ::prost::alloc::string::String,
}
/// Request message for DeleteApiSpecRevision.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiSpecRevisionRequest {
    /// Required. The name of the spec revision to be deleted,
    /// with a revision ID explicitly included.
    ///
    /// Example:
    /// `projects/sample/locations/global/apis/petstore/versions/1.0.0/specs/openapi.yaml@c7cfa2a8`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListApiDeployments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApiDeploymentsRequest {
    /// Required. The parent, which owns this collection of deployments.
    /// Format: `projects/*/locations/*/apis/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of deployments to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 values will be returned.
    /// The maximum is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListApiDeployments` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListApiDeployments` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that can be used to filter the list. Filters use the Common
    /// Expression Language and can refer to all message fields.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for ListApiDeployments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApiDeploymentsResponse {
    /// The deployments from the specified publisher.
    #[prost(message, repeated, tag = "1")]
    pub api_deployments: ::prost::alloc::vec::Vec<ApiDeployment>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetApiDeployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiDeploymentRequest {
    /// Required. The name of the deployment to retrieve.
    /// Format: `projects/*/locations/*/apis/*/deployments/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateApiDeployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiDeploymentRequest {
    /// Required. The parent, which owns this collection of deployments.
    /// Format: `projects/*/locations/*/apis/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The deployment to create.
    #[prost(message, optional, tag = "2")]
    pub api_deployment: ::core::option::Option<ApiDeployment>,
    /// Required. The ID to use for the deployment, which will become the final component of
    /// the deployment's resource name.
    ///
    /// This value should be 4-63 characters, and valid characters
    /// are /\[a-z][0-9\]-/.
    ///
    /// Following AIP-162, IDs must not have the form of a UUID.
    #[prost(string, tag = "3")]
    pub api_deployment_id: ::prost::alloc::string::String,
}
/// Request message for UpdateApiDeployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApiDeploymentRequest {
    /// Required. The deployment to update.
    ///
    /// The `name` field is used to identify the deployment to update.
    /// Format: `projects/*/locations/*/apis/*/deployments/*`
    #[prost(message, optional, tag = "1")]
    pub api_deployment: ::core::option::Option<ApiDeployment>,
    /// The list of fields to be updated. If omitted, all fields are updated that
    /// are set in the request message (fields set to default values are ignored).
    /// If an asterisk "*" is specified, all fields are updated, including fields
    /// that are unspecified/default in the request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set to true, and the deployment is not found, a new deployment will be
    /// created. In this situation, `update_mask` is ignored.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
}
/// Request message for DeleteApiDeployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiDeploymentRequest {
    /// Required. The name of the deployment to delete.
    /// Format: `projects/*/locations/*/apis/*/deployments/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set to true, any child resources will also be deleted.
    /// (Otherwise, the request will only work if there are no child resources.)
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for TagApiDeploymentRevision.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagApiDeploymentRevisionRequest {
    /// Required. The name of the deployment to be tagged, including the revision ID.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The tag to apply.
    /// The tag should be at most 40 characters, and match `\[a-z][a-z0-9-\]{3,39}`.
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
}
/// Request message for ListApiDeploymentRevisions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApiDeploymentRevisionsRequest {
    /// Required. The name of the deployment to list revisions for.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The maximum number of revisions to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token, received from a previous ListApiDeploymentRevisions call.
    /// Provide this to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListApiDeploymentRevisionsResponse.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApiDeploymentRevisionsResponse {
    /// The revisions of the deployment.
    #[prost(message, repeated, tag = "1")]
    pub api_deployments: ::prost::alloc::vec::Vec<ApiDeployment>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for RollbackApiDeployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackApiDeploymentRequest {
    /// Required. The deployment being rolled back.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The revision ID to roll back to.
    /// It must be a revision of the same deployment.
    ///
    ///    Example: `c7cfa2a8`
    #[prost(string, tag = "2")]
    pub revision_id: ::prost::alloc::string::String,
}
/// Request message for DeleteApiDeploymentRevision.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiDeploymentRevisionRequest {
    /// Required. The name of the deployment revision to be deleted,
    /// with a revision ID explicitly included.
    ///
    /// Example:
    /// `projects/sample/locations/global/apis/petstore/deployments/prod@c7cfa2a8`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListArtifacts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListArtifactsRequest {
    /// Required. The parent, which owns this collection of artifacts.
    /// Format: `{parent}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of artifacts to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 values will be returned.
    /// The maximum is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListArtifacts` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListArtifacts` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that can be used to filter the list. Filters use the Common
    /// Expression Language and can refer to all message fields except contents.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for ListArtifacts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListArtifactsResponse {
    /// The artifacts from the specified publisher.
    #[prost(message, repeated, tag = "1")]
    pub artifacts: ::prost::alloc::vec::Vec<Artifact>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetArtifact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetArtifactRequest {
    /// Required. The name of the artifact to retrieve.
    /// Format: `{parent}/artifacts/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetArtifactContents.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetArtifactContentsRequest {
    /// Required. The name of the artifact whose contents should be retrieved.
    /// Format: `{parent}/artifacts/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateArtifact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateArtifactRequest {
    /// Required. The parent, which owns this collection of artifacts.
    /// Format: `{parent}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The artifact to create.
    #[prost(message, optional, tag = "2")]
    pub artifact: ::core::option::Option<Artifact>,
    /// Required. The ID to use for the artifact, which will become the final component of
    /// the artifact's resource name.
    ///
    /// This value should be 4-63 characters, and valid characters
    /// are /\[a-z][0-9\]-/.
    ///
    /// Following AIP-162, IDs must not have the form of a UUID.
    #[prost(string, tag = "3")]
    pub artifact_id: ::prost::alloc::string::String,
}
/// Request message for ReplaceArtifact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceArtifactRequest {
    /// Required. The artifact to replace.
    ///
    /// The `name` field is used to identify the artifact to replace.
    /// Format: `{parent}/artifacts/*`
    #[prost(message, optional, tag = "1")]
    pub artifact: ::core::option::Option<Artifact>,
}
/// Request message for DeleteArtifact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteArtifactRequest {
    /// Required. The name of the artifact to delete.
    /// Format: `{parent}/artifacts/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod registry_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Registry service allows teams to manage descriptions of APIs.
    #[derive(Debug, Clone)]
    pub struct RegistryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RegistryClient<tonic::transport::Channel> {
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
    impl<T> RegistryClient<T>
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
        ) -> RegistryClient<InterceptedService<T, F>>
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
            RegistryClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns matching APIs.
        pub async fn list_apis(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApisRequest>,
        ) -> Result<tonic::Response<super::ListApisResponse>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/ListApis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a specified API.
        pub async fn get_api(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiRequest>,
        ) -> Result<tonic::Response<super::Api>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/GetApi",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a specified API.
        pub async fn create_api(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApiRequest>,
        ) -> Result<tonic::Response<super::Api>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/CreateApi",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Used to modify a specified API.
        pub async fn update_api(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApiRequest>,
        ) -> Result<tonic::Response<super::Api>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/UpdateApi",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Removes a specified API and all of the resources that it
        /// owns.
        pub async fn delete_api(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/DeleteApi",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns matching versions.
        pub async fn list_api_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApiVersionsRequest>,
        ) -> Result<tonic::Response<super::ListApiVersionsResponse>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/ListApiVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a specified version.
        pub async fn get_api_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiVersionRequest>,
        ) -> Result<tonic::Response<super::ApiVersion>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/GetApiVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a specified version.
        pub async fn create_api_version(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApiVersionRequest>,
        ) -> Result<tonic::Response<super::ApiVersion>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/CreateApiVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Used to modify a specified version.
        pub async fn update_api_version(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApiVersionRequest>,
        ) -> Result<tonic::Response<super::ApiVersion>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/UpdateApiVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Removes a specified version and all of the resources that
        /// it owns.
        pub async fn delete_api_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiVersionRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/DeleteApiVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns matching specs.
        pub async fn list_api_specs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApiSpecsRequest>,
        ) -> Result<tonic::Response<super::ListApiSpecsResponse>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/ListApiSpecs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a specified spec.
        pub async fn get_api_spec(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiSpecRequest>,
        ) -> Result<tonic::Response<super::ApiSpec>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/GetApiSpec",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the contents of a specified spec.
        /// If specs are stored with GZip compression, the default behavior
        /// is to return the spec uncompressed (the mime_type response field
        /// indicates the exact format returned).
        pub async fn get_api_spec_contents(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiSpecContentsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::api::HttpBody>,
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
                "/google.cloud.apigeeregistry.v1.Registry/GetApiSpecContents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a specified spec.
        pub async fn create_api_spec(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApiSpecRequest>,
        ) -> Result<tonic::Response<super::ApiSpec>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/CreateApiSpec",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Used to modify a specified spec.
        pub async fn update_api_spec(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApiSpecRequest>,
        ) -> Result<tonic::Response<super::ApiSpec>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/UpdateApiSpec",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Removes a specified spec, all revisions, and all child
        /// resources (e.g., artifacts).
        pub async fn delete_api_spec(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiSpecRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/DeleteApiSpec",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Adds a tag to a specified revision of a spec.
        pub async fn tag_api_spec_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::TagApiSpecRevisionRequest>,
        ) -> Result<tonic::Response<super::ApiSpec>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/TagApiSpecRevision",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all revisions of a spec.
        /// Revisions are returned in descending order of revision creation time.
        pub async fn list_api_spec_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApiSpecRevisionsRequest>,
        ) -> Result<
            tonic::Response<super::ListApiSpecRevisionsResponse>,
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
                "/google.cloud.apigeeregistry.v1.Registry/ListApiSpecRevisions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the current revision to a specified prior revision.
        /// Note that this creates a new revision with a new revision ID.
        pub async fn rollback_api_spec(
            &mut self,
            request: impl tonic::IntoRequest<super::RollbackApiSpecRequest>,
        ) -> Result<tonic::Response<super::ApiSpec>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/RollbackApiSpec",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a revision of a spec.
        pub async fn delete_api_spec_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiSpecRevisionRequest>,
        ) -> Result<tonic::Response<super::ApiSpec>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/DeleteApiSpecRevision",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns matching deployments.
        pub async fn list_api_deployments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApiDeploymentsRequest>,
        ) -> Result<tonic::Response<super::ListApiDeploymentsResponse>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/ListApiDeployments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a specified deployment.
        pub async fn get_api_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiDeploymentRequest>,
        ) -> Result<tonic::Response<super::ApiDeployment>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/GetApiDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a specified deployment.
        pub async fn create_api_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApiDeploymentRequest>,
        ) -> Result<tonic::Response<super::ApiDeployment>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/CreateApiDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Used to modify a specified deployment.
        pub async fn update_api_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApiDeploymentRequest>,
        ) -> Result<tonic::Response<super::ApiDeployment>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/UpdateApiDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Removes a specified deployment, all revisions, and all
        /// child resources (e.g., artifacts).
        pub async fn delete_api_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiDeploymentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/DeleteApiDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Adds a tag to a specified revision of a
        /// deployment.
        pub async fn tag_api_deployment_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::TagApiDeploymentRevisionRequest>,
        ) -> Result<tonic::Response<super::ApiDeployment>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/TagApiDeploymentRevision",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all revisions of a deployment.
        /// Revisions are returned in descending order of revision creation time.
        pub async fn list_api_deployment_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApiDeploymentRevisionsRequest>,
        ) -> Result<
            tonic::Response<super::ListApiDeploymentRevisionsResponse>,
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
                "/google.cloud.apigeeregistry.v1.Registry/ListApiDeploymentRevisions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the current revision to a specified prior
        /// revision. Note that this creates a new revision with a new revision ID.
        pub async fn rollback_api_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::RollbackApiDeploymentRequest>,
        ) -> Result<tonic::Response<super::ApiDeployment>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/RollbackApiDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a revision of a deployment.
        pub async fn delete_api_deployment_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiDeploymentRevisionRequest>,
        ) -> Result<tonic::Response<super::ApiDeployment>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/DeleteApiDeploymentRevision",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns matching artifacts.
        pub async fn list_artifacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListArtifactsRequest>,
        ) -> Result<tonic::Response<super::ListArtifactsResponse>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/ListArtifacts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a specified artifact.
        pub async fn get_artifact(
            &mut self,
            request: impl tonic::IntoRequest<super::GetArtifactRequest>,
        ) -> Result<tonic::Response<super::Artifact>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/GetArtifact",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the contents of a specified artifact.
        /// If artifacts are stored with GZip compression, the default behavior
        /// is to return the artifact uncompressed (the mime_type response field
        /// indicates the exact format returned).
        pub async fn get_artifact_contents(
            &mut self,
            request: impl tonic::IntoRequest<super::GetArtifactContentsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::api::HttpBody>,
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
                "/google.cloud.apigeeregistry.v1.Registry/GetArtifactContents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a specified artifact.
        pub async fn create_artifact(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateArtifactRequest>,
        ) -> Result<tonic::Response<super::Artifact>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/CreateArtifact",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Used to replace a specified artifact.
        pub async fn replace_artifact(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplaceArtifactRequest>,
        ) -> Result<tonic::Response<super::Artifact>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/ReplaceArtifact",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Removes a specified artifact.
        pub async fn delete_artifact(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteArtifactRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.apigeeregistry.v1.Registry/DeleteArtifact",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
