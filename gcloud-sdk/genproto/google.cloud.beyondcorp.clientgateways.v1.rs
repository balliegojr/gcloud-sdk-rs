/// Message describing ClientGateway object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientGateway {
    /// Required. name of resource. The name is ignored during creation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. [Output only] Create time stamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. [Output only] Update time stamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The operational state of the gateway.
    #[prost(enumeration = "client_gateway::State", tag = "4")]
    pub state: i32,
    /// Output only. A unique identifier for the instance generated by the system.
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
    /// Output only. The client connector service name that the client gateway is
    /// associated to. Client Connector Services, named as follows:
    ///    `projects/{project_id}/locations/{location_id}/client_connector_services/{client_connector_service_id}`.
    #[prost(string, tag = "6")]
    pub client_connector_service: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ClientGateway`.
pub mod client_gateway {
    /// Represents the different states of a gateway.
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
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Gateway is being created.
        Creating = 1,
        /// Gateway is being updated.
        Updating = 2,
        /// Gateway is being deleted.
        Deleting = 3,
        /// Gateway is running.
        Running = 4,
        /// Gateway is down and may be restored in the future.
        /// This happens when CCFE sends ProjectState = OFF.
        Down = 5,
        /// ClientGateway encountered an error and is in indeterministic state.
        Error = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Running => "RUNNING",
                State::Down => "DOWN",
                State::Error => "ERROR",
            }
        }
    }
}
/// Message for requesting list of ClientGateways.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClientGatewaysRequest {
    /// Required. Parent value for ListClientGatewaysRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing ClientGateways.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClientGatewaysResponse {
    /// The list of ClientGateway.
    #[prost(message, repeated, tag = "1")]
    pub client_gateways: ::prost::alloc::vec::Vec<ClientGateway>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a ClientGateway.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClientGatewayRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a ClientGateway.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClientGatewayRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. User-settable client gateway resource ID.
    ///   * Must start with a letter.
    ///   * Must contain between 4-63 characters from `/\[a-z][0-9\]-/`.
    ///   * Must end with a number or a letter.
    #[prost(string, tag = "2")]
    pub client_gateway_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub client_gateway: ::core::option::Option<ClientGateway>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// Message for deleting a ClientGateway
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClientGatewayRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientGatewayOperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have been cancelled successfully
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod client_gateways_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// ## API Overview
    ///
    /// The `beyondcorp.googleapis.com` service implements the Google Cloud
    /// BeyondCorp API.
    ///
    /// ## Data Model
    ///
    /// The ClientGatewaysService exposes the following resources:
    ///
    /// * Client Gateways, named as follows:
    ///   `projects/{project_id}/locations/{location_id}/clientGateways/{client_gateway_id}`.
    #[derive(Debug, Clone)]
    pub struct ClientGatewaysServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ClientGatewaysServiceClient<tonic::transport::Channel> {
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
    impl<T> ClientGatewaysServiceClient<T>
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
        ) -> ClientGatewaysServiceClient<InterceptedService<T, F>>
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
            ClientGatewaysServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists ClientGateways in a given project and location.
        pub async fn list_client_gateways(
            &mut self,
            request: impl tonic::IntoRequest<super::ListClientGatewaysRequest>,
        ) -> Result<tonic::Response<super::ListClientGatewaysResponse>, tonic::Status> {
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
                "/google.cloud.beyondcorp.clientgateways.v1.ClientGatewaysService/ListClientGateways",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single ClientGateway.
        pub async fn get_client_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClientGatewayRequest>,
        ) -> Result<tonic::Response<super::ClientGateway>, tonic::Status> {
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
                "/google.cloud.beyondcorp.clientgateways.v1.ClientGatewaysService/GetClientGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new ClientGateway in a given project and location.
        pub async fn create_client_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClientGatewayRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.beyondcorp.clientgateways.v1.ClientGatewaysService/CreateClientGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single ClientGateway.
        pub async fn delete_client_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClientGatewayRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.beyondcorp.clientgateways.v1.ClientGatewaysService/DeleteClientGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
