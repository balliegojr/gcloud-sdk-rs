#[doc = r" Generated client implementations."]
pub mod routes_alpha_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Routes Preferred API."]
    pub struct RoutesAlphaClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RoutesAlphaClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Returns the primary route along with optional alternate routes, given a set"]
        #[doc = " of terminal and intermediate waypoints."]
        #[doc = ""]
        #[doc = " **NOTE:** This method requires that you specify a response field mask in"]
        #[doc = " the input. You can provide the response field mask by using the URL"]
        #[doc = " parameter `$fields` or `fields`, or by using the HTTP/gRPC header"]
        #[doc = " `X-Goog-FieldMask` (see the [available URL parameters and"]
        #[doc = " headers](https://cloud.google.com/apis/docs/system-parameters). The value"]
        #[doc = " is a comma separated list of field paths. See this detailed documentation"]
        #[doc = " about [how to construct the field"]
        #[doc = " paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto)."]
        #[doc = ""]
        #[doc = " For example, in this method:"]
        #[doc = ""]
        #[doc = " * Field mask of all available fields (for manual inspection):"]
        #[doc = "   `X-Goog-FieldMask: *`"]
        #[doc = " * Field mask of Route-level duration, distance, and polyline (an example"]
        #[doc = " production setup):"]
        #[doc = "   `X-Goog-FieldMask:"]
        #[doc = "   routes.duration,routes.distanceMeters,routes.polyline.encodedPolyline`"]
        #[doc = ""]
        #[doc = " Google discourages the use of the wildcard (`*`) response field mask, or"]
        #[doc = " specifying the field mask at the top level (`routes`), because:"]
        #[doc = ""]
        #[doc = " * Selecting only the fields that you need helps our server save computation"]
        #[doc = " cycles, allowing us to return the result to you with a lower latency."]
        #[doc = " * Selecting only the fields that you need in your production job ensures"]
        #[doc = " stable latency performance. We might add more response fields in the"]
        #[doc = " future, and those new fields might require extra computation time. If you"]
        #[doc = " select all fields, or if you select all fields at the top level, then you"]
        #[doc = " might experience performance degradation because any new field we add will"]
        #[doc = " be automatically included in the response."]
        #[doc = " * Selecting only the fields that you need results in a smaller response"]
        #[doc = " size, and thus higher network throughput."]
        pub async fn compute_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::super::v1::ComputeRoutesRequest>,
        ) -> Result<tonic::Response<super::super::v1::ComputeRoutesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.routes.v1alpha.RoutesAlpha/ComputeRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Takes in a list of origins and destinations and returns a stream containing"]
        #[doc = " route information for each combination of origin and destination."]
        #[doc = ""]
        #[doc = " **NOTE:** This method requires that you specify a response field mask in"]
        #[doc = " the input. You can provide the response field mask by using the URL"]
        #[doc = " parameter `$fields` or `fields`, or by using the HTTP/gRPC header"]
        #[doc = " `X-Goog-FieldMask` (see the [available URL parameters and"]
        #[doc = " headers](https://cloud.google.com/apis/docs/system-parameters). The value"]
        #[doc = " is a comma separated list of field paths. See this detailed documentation"]
        #[doc = " about [how to construct the field"]
        #[doc = " paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto)."]
        #[doc = ""]
        #[doc = " For example, in this method:"]
        #[doc = ""]
        #[doc = " * Field mask of all available fields (for manual inspection):"]
        #[doc = "   `X-Goog-FieldMask: *`"]
        #[doc = " * Field mask of route durations, distances, element status, condition, and"]
        #[doc = "   element indices (an example production setup):"]
        #[doc = "   `X-Goog-FieldMask:"]
        #[doc = "   originIndex,destinationIndex,status,condition,distanceMeters,duration`"]
        #[doc = ""]
        #[doc = " It is critical that you include `status` in your field mask as otherwise"]
        #[doc = " all messages will appear to be OK. Google discourages the use of the"]
        #[doc = " wildcard (`*`) response field mask, because:"]
        #[doc = ""]
        #[doc = " * Selecting only the fields that you need helps our server save computation"]
        #[doc = " cycles, allowing us to return the result to you with a lower latency."]
        #[doc = " * Selecting only the fields that you need in your production job ensures"]
        #[doc = " stable latency performance. We might add more response fields in the"]
        #[doc = " future, and those new fields might require extra computation time. If you"]
        #[doc = " select all fields, or if you select all fields at the top level, then you"]
        #[doc = " might experience performance degradation because any new field we add will"]
        #[doc = " be automatically included in the response."]
        #[doc = " * Selecting only the fields that you need results in a smaller response"]
        #[doc = " size, and thus higher network throughput."]
        pub async fn compute_route_matrix(
            &mut self,
            request: impl tonic::IntoRequest<super::super::v1::ComputeRouteMatrixRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::v1::RouteMatrixElement>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.routes.v1alpha.RoutesAlpha/ComputeRouteMatrix",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for RoutesAlphaClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for RoutesAlphaClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "RoutesAlphaClient {{ ... }}")
        }
    }
}
