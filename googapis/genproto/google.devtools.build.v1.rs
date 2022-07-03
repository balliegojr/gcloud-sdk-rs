/// Status used for both invocation attempt and overall build completion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildStatus {
    /// The end result.
    #[prost(enumeration = "build_status::Result", tag = "1")]
    pub result: i32,
    /// Final invocation ID of the build, if there was one.
    /// This field is only set on a status in BuildFinished event.
    #[prost(string, tag = "3")]
    pub final_invocation_id: ::prost::alloc::string::String,
    /// Build tool exit code. Integer value returned by the executed build tool.
    /// Might not be available in some cases, e.g., a build timeout.
    #[prost(message, optional, tag = "4")]
    pub build_tool_exit_code: ::core::option::Option<i32>,
    /// Fine-grained diagnostic information to complement the status.
    #[prost(message, optional, tag = "2")]
    pub details: ::core::option::Option<::prost_types::Any>,
}
/// Nested message and enum types in `BuildStatus`.
pub mod build_status {
    /// The end result of the Build.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unspecified or unknown.
        UnknownStatus = 0,
        /// Build was successful and tests (if requested) all pass.
        CommandSucceeded = 1,
        /// Build error and/or test failure.
        CommandFailed = 2,
        /// Unable to obtain a result due to input provided by the user.
        UserError = 3,
        /// Unable to obtain a result due to a failure within the build system.
        SystemError = 4,
        /// Build required too many resources, such as build tool RAM.
        ResourceExhausted = 5,
        /// An invocation attempt time exceeded its deadline.
        InvocationDeadlineExceeded = 6,
        /// Build request time exceeded the request_deadline
        RequestDeadlineExceeded = 8,
        /// The build was cancelled by a call to CancelBuild.
        Cancelled = 7,
    }
}
/// An event representing some state change that occurred in the build. This
/// message does not include field for uniquely identifying an event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildEvent {
    /// The timestamp of this event.
    #[prost(message, optional, tag = "1")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// //////////////////////////////////////////////////////////////////////////
    /// Events that indicate a state change of a build request in the build
    /// queue.
    #[prost(
        oneof = "build_event::Event",
        tags = "51, 52, 53, 55, 56, 59, 60, 61, 62"
    )]
    pub event: ::core::option::Option<build_event::Event>,
}
/// Nested message and enum types in `BuildEvent`.
pub mod build_event {
    /// Notification that the build system has attempted to run the build tool.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InvocationAttemptStarted {
        /// The number of the invocation attempt, starting at 1 and increasing by 1
        /// for each new attempt. Can be used to determine if there is a later
        /// invocation attempt replacing the current one a client is processing.
        #[prost(int64, tag = "1")]
        pub attempt_number: i64,
        /// Arbitrary details about the invocation attempt.
        #[prost(message, optional, tag = "2")]
        pub details: ::core::option::Option<::prost_types::Any>,
    }
    /// Notification that an invocation attempt has finished.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InvocationAttemptFinished {
        /// Final status of the invocation.
        #[prost(message, optional, tag = "3")]
        pub invocation_status: ::core::option::Option<super::BuildStatus>,
        /// Arbitrary details about the invocation attempt.
        #[prost(message, optional, tag = "4")]
        pub details: ::core::option::Option<::prost_types::Any>,
    }
    /// Notification that the build request is enqueued.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BuildEnqueued {
        /// Additional details about the Build.
        #[prost(message, optional, tag = "1")]
        pub details: ::core::option::Option<::prost_types::Any>,
    }
    /// Notification that the build request has finished, and no further
    /// invocations will occur.  Note that this applies to the entire Build.
    /// Individual invocations trigger InvocationFinished when they finish.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BuildFinished {
        /// Final status of the build.
        #[prost(message, optional, tag = "1")]
        pub status: ::core::option::Option<super::BuildStatus>,
        /// Additional details about the Build.
        #[prost(message, optional, tag = "2")]
        pub details: ::core::option::Option<::prost_types::Any>,
    }
    /// Textual output written to standard output or standard error.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConsoleOutput {
        /// The output stream type.
        #[prost(enumeration = "super::ConsoleOutputStream", tag = "1")]
        pub r#type: i32,
        /// The output stream content.
        #[prost(oneof = "console_output::Output", tags = "2, 3")]
        pub output: ::core::option::Option<console_output::Output>,
    }
    /// Nested message and enum types in `ConsoleOutput`.
    pub mod console_output {
        /// The output stream content.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Output {
            /// Regular UTF-8 output; normal text.
            #[prost(string, tag = "2")]
            TextOutput(::prost::alloc::string::String),
            /// Used if the output is not UTF-8 text (for example, a binary proto).
            #[prost(bytes, tag = "3")]
            BinaryOutput(::prost::alloc::vec::Vec<u8>),
        }
    }
    /// Notification of the end of a build event stream published by a build
    /// component other than CONTROLLER (See StreamId.BuildComponents).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BuildComponentStreamFinished {
        /// How the event stream finished.
        #[prost(enumeration = "build_component_stream_finished::FinishType", tag = "1")]
        pub r#type: i32,
    }
    /// Nested message and enum types in `BuildComponentStreamFinished`.
    pub mod build_component_stream_finished {
        /// How did the event stream finish.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum FinishType {
            /// Unknown or unspecified; callers should never set this value.
            Unspecified = 0,
            /// Set by the event publisher to indicate a build event stream is
            /// finished.
            Finished = 1,
            /// Set by the WatchBuild RPC server when the publisher of a build event
            /// stream stops publishing events without publishing a
            /// BuildComponentStreamFinished event whose type equals FINISHED.
            Expired = 2,
        }
    }
    /// //////////////////////////////////////////////////////////////////////////
    /// Events that indicate a state change of a build request in the build
    /// queue.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// An invocation attempt has started.
        #[prost(message, tag = "51")]
        InvocationAttemptStarted(InvocationAttemptStarted),
        /// An invocation attempt has finished.
        #[prost(message, tag = "52")]
        InvocationAttemptFinished(InvocationAttemptFinished),
        /// The build is enqueued.
        #[prost(message, tag = "53")]
        BuildEnqueued(BuildEnqueued),
        /// The build has finished. Set when the build is terminated.
        #[prost(message, tag = "55")]
        BuildFinished(BuildFinished),
        /// An event containing printed text.
        #[prost(message, tag = "56")]
        ConsoleOutput(ConsoleOutput),
        /// Indicates the end of a build event stream (with the same StreamId) from
        /// a build component executing the requested build task.
        /// *** This field does not indicate the WatchBuild RPC is finished. ***
        #[prost(message, tag = "59")]
        ComponentStreamFinished(BuildComponentStreamFinished),
        /// Structured build event generated by Bazel about its execution progress.
        #[prost(message, tag = "60")]
        BazelEvent(::prost_types::Any),
        /// An event that contains supplemental tool-specific information about
        /// build execution.
        #[prost(message, tag = "61")]
        BuildExecutionEvent(::prost_types::Any),
        /// An event that contains supplemental tool-specific information about
        /// source fetching.
        #[prost(message, tag = "62")]
        SourceFetchEvent(::prost_types::Any),
    }
}
/// Unique identifier for a build event stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamId {
    /// The id of a Build message.
    #[prost(string, tag = "1")]
    pub build_id: ::prost::alloc::string::String,
    /// The unique invocation ID within this build.
    /// It should be the same as {invocation} (below) during the migration.
    #[prost(string, tag = "6")]
    pub invocation_id: ::prost::alloc::string::String,
    /// The component that emitted this event.
    #[prost(enumeration = "stream_id::BuildComponent", tag = "3")]
    pub component: i32,
}
/// Nested message and enum types in `StreamId`.
pub mod stream_id {
    /// Which build component generates this event stream. Each build component
    /// may generate one event stream.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BuildComponent {
        /// Unknown or unspecified; callers should never set this value.
        UnknownComponent = 0,
        /// A component that coordinates builds.
        Controller = 1,
        /// A component that runs executables needed to complete a build.
        Worker = 2,
        /// A component that builds something.
        Tool = 3,
    }
}
/// The type of console output stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConsoleOutputStream {
    /// Unspecified or unknown.
    Unknown = 0,
    /// Normal output stream.
    Stdout = 1,
    /// Error output stream.
    Stderr = 2,
}
/// Publishes 'lifecycle events' that update the high-level state of a build:
/// - BuildEnqueued: When a build is scheduled.
/// - InvocationAttemptStarted: When work for a build starts; there can be
///     multiple invocations for a build (e.g. retries).
/// - InvocationAttemptCompleted: When work for a build finishes.
/// - BuildFinished: When a build is finished.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishLifecycleEventRequest {
    /// The interactivity of this build.
    #[prost(
        enumeration = "publish_lifecycle_event_request::ServiceLevel",
        tag = "1"
    )]
    pub service_level: i32,
    /// Required. The lifecycle build event. If this is a build tool event, the RPC
    /// will fail with INVALID_REQUEST.
    #[prost(message, optional, tag = "2")]
    pub build_event: ::core::option::Option<OrderedBuildEvent>,
    /// If the next event for this build or invocation (depending on the event
    /// type) hasn't been published after this duration from when {build_event}
    /// is written to BES, consider this stream expired. If this field is not set,
    /// BES backend will use its own default value.
    #[prost(message, optional, tag = "3")]
    pub stream_timeout: ::core::option::Option<::prost_types::Duration>,
    /// Additional information about a build request. These are define by the event
    /// publishers, and the Build Event Service does not validate or interpret
    /// them. They are used while notifying internal systems of new builds and
    /// invocations if the OrderedBuildEvent.event type is
    /// BuildEnqueued/InvocationAttemptStarted.
    #[prost(string, repeated, tag = "4")]
    pub notification_keywords: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The project this build is associated with.
    /// This should match the project used for the initial call to
    /// PublishLifecycleEvent (containing a BuildEnqueued message).
    #[prost(string, tag = "6")]
    pub project_id: ::prost::alloc::string::String,
    /// Whether to require a previously received matching parent lifecycle event
    /// for the current request's event before continuing processing.
    /// - InvocationAttemptStarted and BuildFinished events require a BuildEnqueued
    ///   parent event.
    /// - InvocationAttemptFinished events require an InvocationAttemptStarted
    ///   parent event.
    #[prost(bool, tag = "7")]
    pub check_preceding_lifecycle_events_present: bool,
}
/// Nested message and enum types in `PublishLifecycleEventRequest`.
pub mod publish_lifecycle_event_request {
    /// The service level of the build request. Backends only uses this value when
    /// the BuildEnqueued event is published to determine what level of service
    /// this build should receive.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ServiceLevel {
        /// Non-interactive builds can tolerate longer event latencies. This is the
        /// default ServiceLevel if callers do not specify one.
        Noninteractive = 0,
        /// The events of an interactive build should be delivered with low latency.
        Interactive = 1,
    }
}
/// States which event has been committed. Any failure to commit will cause
/// RPC errors, hence not recorded by this proto.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishBuildToolEventStreamResponse {
    /// The stream that contains this event.
    #[prost(message, optional, tag = "1")]
    pub stream_id: ::core::option::Option<StreamId>,
    /// The sequence number of this event that has been committed.
    #[prost(int64, tag = "2")]
    pub sequence_number: i64,
}
/// Build event with contextual information about the stream it belongs to and
/// its position in that stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderedBuildEvent {
    /// Which build event stream this event belongs to.
    #[prost(message, optional, tag = "1")]
    pub stream_id: ::core::option::Option<StreamId>,
    /// The position of this event in the stream. The sequence numbers for a build
    /// event stream should be a sequence of consecutive natural numbers starting
    /// from one. (1, 2, 3, ...)
    #[prost(int64, tag = "2")]
    pub sequence_number: i64,
    /// The actual event.
    #[prost(message, optional, tag = "3")]
    pub event: ::core::option::Option<BuildEvent>,
}
/// Streaming request message for PublishBuildToolEventStream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishBuildToolEventStreamRequest {
    /// Required. The build event with position info.
    /// New publishing clients should use this field rather than the 3 above.
    #[prost(message, optional, tag = "4")]
    pub ordered_build_event: ::core::option::Option<OrderedBuildEvent>,
    /// The keywords to be attached to the notification which notifies the start
    /// of a new build event stream. BES only reads this field when sequence_number
    /// or ordered_build_event.sequence_number is 1 in this message. If this field
    /// is empty, BES will not publish notification messages for this stream.
    #[prost(string, repeated, tag = "5")]
    pub notification_keywords: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The project this build is associated with.
    /// This should match the project used for the initial call to
    /// PublishLifecycleEvent (containing a BuildEnqueued message).
    #[prost(string, tag = "6")]
    pub project_id: ::prost::alloc::string::String,
    /// Whether to require a previously received matching InvocationAttemptStarted
    /// event before continuing event processing for the event in the current
    /// request. BES only performs this check for events with sequence_number 1
    /// i.e. the first event in the stream.
    #[prost(bool, tag = "7")]
    pub check_preceding_lifecycle_events_present: bool,
}
#[doc = r" Generated client implementations."]
pub mod publish_build_event_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A service for publishing BuildEvents. BuildEvents are generated by Build"]
    #[doc = " Systems to record actions taken during a Build. Events occur in streams,"]
    #[doc = " are identified by a StreamId, and ordered by sequence number in a stream."]
    #[doc = ""]
    #[doc = " A Build may contain several streams of BuildEvents, depending on the systems"]
    #[doc = " that are involved in the Build. Some BuildEvents are used to declare the"]
    #[doc = " beginning and end of major portions of a Build; these are called"]
    #[doc = " LifecycleEvents, and are used (for example) to indicate the beginning or end"]
    #[doc = " of a Build, and the beginning or end of an Invocation attempt (there can be"]
    #[doc = " more than 1 Invocation in a Build if, for example, a failure occurs somewhere"]
    #[doc = " and it needs to be retried)."]
    #[doc = ""]
    #[doc = " Other, build-tool events represent actions taken by the Build tool, such as"]
    #[doc = " target objects produced via compilation, tests run, et cetera. There could be"]
    #[doc = " more than one build tool stream for an invocation attempt of a build."]
    #[derive(Debug, Clone)]
    pub struct PublishBuildEventClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PublishBuildEventClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PublishBuildEventClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            PublishBuildEventClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Publish a build event stating the new state of a build (typically from the"]
        #[doc = " build queue). The BuildEnqueued event must be publishd before all other"]
        #[doc = " events for the same build ID."]
        #[doc = ""]
        #[doc = " The backend will persist the event and deliver it to registered frontend"]
        #[doc = " jobs immediately without batching."]
        #[doc = ""]
        #[doc = " The commit status of the request is reported by the RPC's util_status()"]
        #[doc = " function. The error code is the canoncial error code defined in"]
        #[doc = " //util/task/codes.proto."]
        pub async fn publish_lifecycle_event(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishLifecycleEventRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.build.v1.PublishBuildEvent/PublishLifecycleEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish build tool events belonging to the same stream to a backend job"]
        #[doc = " using bidirectional streaming."]
        pub async fn publish_build_tool_event_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::PublishBuildToolEventStreamRequest,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::PublishBuildToolEventStreamResponse>>,
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
                "/google.devtools.build.v1.PublishBuildEvent/PublishBuildToolEventStream",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
