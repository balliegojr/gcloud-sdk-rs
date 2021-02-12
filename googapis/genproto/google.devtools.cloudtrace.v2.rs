/// A span represents a single operation within a trace. Spans can be
/// nested to form a trace tree. Often, a trace contains a root span
/// that describes the end-to-end latency, and one or more subspans for
/// its sub-operations. A trace can also contain multiple root spans,
/// or none at all. Spans do not need to be contiguous&mdash;there may be
/// gaps or overlaps between spans in a trace.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Span {
    /// Required. The resource name of the span in the following format:
    ///
    ///     projects/[PROJECT_ID]/traces/[TRACE_ID]/spans/[SPAN_ID]
    ///
    /// [TRACE_ID] is a unique identifier for a trace within a project;
    /// it is a 32-character hexadecimal encoding of a 16-byte array.
    ///
    /// [SPAN_ID] is a unique identifier for a span within a trace; it
    /// is a 16-character hexadecimal encoding of an 8-byte array.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The [SPAN_ID] portion of the span's resource name.
    #[prost(string, tag = "2")]
    pub span_id: ::prost::alloc::string::String,
    /// The [SPAN_ID] of this span's parent span. If this is a root span,
    /// then this field must be empty.
    #[prost(string, tag = "3")]
    pub parent_span_id: ::prost::alloc::string::String,
    /// Required. A description of the span's operation (up to 128 bytes).
    /// Stackdriver Trace displays the description in the
    /// Google Cloud Platform Console.
    /// For example, the display name can be a qualified method name or a file name
    /// and a line number where the operation is called. A best practice is to use
    /// the same display name within an application and at the same call point.
    /// This makes it easier to correlate spans in different traces.
    #[prost(message, optional, tag = "4")]
    pub display_name: ::core::option::Option<TruncatableString>,
    /// Required. The start time of the span. On the client side, this is the time kept by
    /// the local machine where the span execution starts. On the server side, this
    /// is the time when the server's application handler starts running.
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The end time of the span. On the client side, this is the time kept by
    /// the local machine where the span execution ends. On the server side, this
    /// is the time when the server application handler stops running.
    #[prost(message, optional, tag = "6")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A set of attributes on the span. You can have up to 32 attributes per
    /// span.
    #[prost(message, optional, tag = "7")]
    pub attributes: ::core::option::Option<span::Attributes>,
    /// Stack trace captured at the start of the span.
    #[prost(message, optional, tag = "8")]
    pub stack_trace: ::core::option::Option<StackTrace>,
    /// A set of time events. You can have up to 32 annotations and 128 message
    /// events per span.
    #[prost(message, optional, tag = "9")]
    pub time_events: ::core::option::Option<span::TimeEvents>,
    /// Links associated with the span. You can have up to 128 links per Span.
    #[prost(message, optional, tag = "10")]
    pub links: ::core::option::Option<span::Links>,
    /// Optional. The final status for this span.
    #[prost(message, optional, tag = "11")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// Optional. Set this parameter to indicate whether this span is in
    /// the same process as its parent. If you do not set this parameter,
    /// Stackdriver Trace is unable to take advantage of this helpful
    /// information.
    #[prost(message, optional, tag = "12")]
    pub same_process_as_parent_span: ::core::option::Option<bool>,
    /// Optional. The number of child spans that were generated while this span
    /// was active. If set, allows implementation to detect missing child spans.
    #[prost(message, optional, tag = "13")]
    pub child_span_count: ::core::option::Option<i32>,
    /// Optional. Distinguishes between spans generated in a particular context. For example,
    /// two spans with the same name may be distinguished using `CLIENT` (caller)
    /// and `SERVER` (callee) to identify an RPC call.
    #[prost(enumeration = "span::SpanKind", tag = "14")]
    pub span_kind: i32,
}
/// Nested message and enum types in `Span`.
pub mod span {
    /// A set of attributes, each in the format `[KEY]:[VALUE]`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Attributes {
        /// The set of attributes. Each attribute's key can be up to 128 bytes
        /// long. The value can be a string up to 256 bytes, a signed 64-bit integer,
        /// or the Boolean values `true` and `false`. For example:
        ///
        ///     "/instance_id": { "string_value": { "value": "my-instance" } }
        ///     "/http/request_bytes": { "int_value": 300 }
        ///     "abc.com/myattribute": { "bool_value": false }
        #[prost(map = "string, message", tag = "1")]
        pub attribute_map:
            ::std::collections::HashMap<::prost::alloc::string::String, super::AttributeValue>,
        /// The number of attributes that were discarded. Attributes can be discarded
        /// because their keys are too long or because there are too many attributes.
        /// If this value is 0 then all attributes are valid.
        #[prost(int32, tag = "2")]
        pub dropped_attributes_count: i32,
    }
    /// A time-stamped annotation or message event in the Span.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimeEvent {
        /// The timestamp indicating the time the event occurred.
        #[prost(message, optional, tag = "1")]
        pub time: ::core::option::Option<::prost_types::Timestamp>,
        /// A `TimeEvent` can contain either an `Annotation` object or a
        /// `MessageEvent` object, but not both.
        #[prost(oneof = "time_event::Value", tags = "2, 3")]
        pub value: ::core::option::Option<time_event::Value>,
    }
    /// Nested message and enum types in `TimeEvent`.
    pub mod time_event {
        /// Text annotation with a set of attributes.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Annotation {
            /// A user-supplied message describing the event. The maximum length for
            /// the description is 256 bytes.
            #[prost(message, optional, tag = "1")]
            pub description: ::core::option::Option<super::super::TruncatableString>,
            /// A set of attributes on the annotation. You can have up to 4 attributes
            /// per Annotation.
            #[prost(message, optional, tag = "2")]
            pub attributes: ::core::option::Option<super::Attributes>,
        }
        /// An event describing a message sent/received between Spans.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MessageEvent {
            /// Type of MessageEvent. Indicates whether the message was sent or
            /// received.
            #[prost(enumeration = "message_event::Type", tag = "1")]
            pub r#type: i32,
            /// An identifier for the MessageEvent's message that can be used to match
            /// SENT and RECEIVED MessageEvents. It is recommended to be unique within
            /// a Span.
            #[prost(int64, tag = "2")]
            pub id: i64,
            /// The number of uncompressed bytes sent or received.
            #[prost(int64, tag = "3")]
            pub uncompressed_size_bytes: i64,
            /// The number of compressed bytes sent or received. If missing assumed to
            /// be the same size as uncompressed.
            #[prost(int64, tag = "4")]
            pub compressed_size_bytes: i64,
        }
        /// Nested message and enum types in `MessageEvent`.
        pub mod message_event {
            /// Indicates whether the message was sent or received.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum Type {
                /// Unknown event type.
                Unspecified = 0,
                /// Indicates a sent message.
                Sent = 1,
                /// Indicates a received message.
                Received = 2,
            }
        }
        /// A `TimeEvent` can contain either an `Annotation` object or a
        /// `MessageEvent` object, but not both.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            /// Text annotation with a set of attributes.
            #[prost(message, tag = "2")]
            Annotation(Annotation),
            /// An event describing a message sent/received between Spans.
            #[prost(message, tag = "3")]
            MessageEvent(MessageEvent),
        }
    }
    /// A collection of `TimeEvent`s. A `TimeEvent` is a time-stamped annotation
    /// on the span, consisting of either user-supplied key:value pairs, or
    /// details of a message sent/received between Spans.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimeEvents {
        /// A collection of `TimeEvent`s.
        #[prost(message, repeated, tag = "1")]
        pub time_event: ::prost::alloc::vec::Vec<TimeEvent>,
        /// The number of dropped annotations in all the included time events.
        /// If the value is 0, then no annotations were dropped.
        #[prost(int32, tag = "2")]
        pub dropped_annotations_count: i32,
        /// The number of dropped message events in all the included time events.
        /// If the value is 0, then no message events were dropped.
        #[prost(int32, tag = "3")]
        pub dropped_message_events_count: i32,
    }
    /// A pointer from the current span to another span in the same trace or in a
    /// different trace. For example, this can be used in batching operations,
    /// where a single batch handler processes multiple requests from different
    /// traces or when the handler receives a request from a different project.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Link {
        /// The [TRACE_ID] for a trace within a project.
        #[prost(string, tag = "1")]
        pub trace_id: ::prost::alloc::string::String,
        /// The [SPAN_ID] for a span within a trace.
        #[prost(string, tag = "2")]
        pub span_id: ::prost::alloc::string::String,
        /// The relationship of the current span relative to the linked span.
        #[prost(enumeration = "link::Type", tag = "3")]
        pub r#type: i32,
        /// A set of attributes on the link. You have have up to  32 attributes per
        /// link.
        #[prost(message, optional, tag = "4")]
        pub attributes: ::core::option::Option<Attributes>,
    }
    /// Nested message and enum types in `Link`.
    pub mod link {
        /// The relationship of the current span relative to the linked span: child,
        /// parent, or unspecified.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Type {
            /// The relationship of the two spans is unknown.
            Unspecified = 0,
            /// The linked span is a child of the current span.
            ChildLinkedSpan = 1,
            /// The linked span is a parent of the current span.
            ParentLinkedSpan = 2,
        }
    }
    /// A collection of links, which are references from this span to a span
    /// in the same or different trace.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Links {
        /// A collection of links.
        #[prost(message, repeated, tag = "1")]
        pub link: ::prost::alloc::vec::Vec<Link>,
        /// The number of dropped links after the maximum size was enforced. If
        /// this value is 0, then no links were dropped.
        #[prost(int32, tag = "2")]
        pub dropped_links_count: i32,
    }
    /// Type of span. Can be used to specify additional relationships between spans
    /// in addition to a parent/child relationship.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SpanKind {
        /// Unspecified. Do NOT use as default.
        /// Implementations MAY assume SpanKind.INTERNAL to be default.
        Unspecified = 0,
        /// Indicates that the span is used internally. Default value.
        Internal = 1,
        /// Indicates that the span covers server-side handling of an RPC or other
        /// remote network request.
        Server = 2,
        /// Indicates that the span covers the client-side wrapper around an RPC or
        /// other remote request.
        Client = 3,
        /// Indicates that the span describes producer sending a message to a broker.
        /// Unlike client and  server, there is no direct critical path latency
        /// relationship between producer and consumer spans (e.g. publishing a
        /// message to a pubsub service).
        Producer = 4,
        /// Indicates that the span describes consumer receiving a message from a
        /// broker. Unlike client and  server, there is no direct critical path
        /// latency relationship between producer and consumer spans (e.g. receiving
        /// a message from a pubsub service subscription).
        Consumer = 5,
    }
}
/// The allowed types for [VALUE] in a `[KEY]:[VALUE]` attribute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributeValue {
    /// The type of the value.
    #[prost(oneof = "attribute_value::Value", tags = "1, 2, 3")]
    pub value: ::core::option::Option<attribute_value::Value>,
}
/// Nested message and enum types in `AttributeValue`.
pub mod attribute_value {
    /// The type of the value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A string up to 256 bytes long.
        #[prost(message, tag = "1")]
        StringValue(super::TruncatableString),
        /// A 64-bit signed integer.
        #[prost(int64, tag = "2")]
        IntValue(i64),
        /// A Boolean value represented by `true` or `false`.
        #[prost(bool, tag = "3")]
        BoolValue(bool),
    }
}
/// A call stack appearing in a trace.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StackTrace {
    /// Stack frames in this stack trace. A maximum of 128 frames are allowed.
    #[prost(message, optional, tag = "1")]
    pub stack_frames: ::core::option::Option<stack_trace::StackFrames>,
    /// The hash ID is used to conserve network bandwidth for duplicate
    /// stack traces within a single trace.
    ///
    /// Often multiple spans will have identical stack traces.
    /// The first occurrence of a stack trace should contain both the
    /// `stackFrame` content and a value in `stackTraceHashId`.
    ///
    /// Subsequent spans within the same request can refer
    /// to that stack trace by only setting `stackTraceHashId`.
    #[prost(int64, tag = "2")]
    pub stack_trace_hash_id: i64,
}
/// Nested message and enum types in `StackTrace`.
pub mod stack_trace {
    /// Represents a single stack frame in a stack trace.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StackFrame {
        /// The fully-qualified name that uniquely identifies the function or
        /// method that is active in this frame (up to 1024 bytes).
        #[prost(message, optional, tag = "1")]
        pub function_name: ::core::option::Option<super::TruncatableString>,
        /// An un-mangled function name, if `function_name` is
        /// [mangled](http://www.avabodh.com/cxxin/namemangling.html). The name can
        /// be fully-qualified (up to 1024 bytes).
        #[prost(message, optional, tag = "2")]
        pub original_function_name: ::core::option::Option<super::TruncatableString>,
        /// The name of the source file where the function call appears (up to 256
        /// bytes).
        #[prost(message, optional, tag = "3")]
        pub file_name: ::core::option::Option<super::TruncatableString>,
        /// The line number in `file_name` where the function call appears.
        #[prost(int64, tag = "4")]
        pub line_number: i64,
        /// The column number where the function call appears, if available.
        /// This is important in JavaScript because of its anonymous functions.
        #[prost(int64, tag = "5")]
        pub column_number: i64,
        /// The binary module from where the code was loaded.
        #[prost(message, optional, tag = "6")]
        pub load_module: ::core::option::Option<super::Module>,
        /// The version of the deployed source code (up to 128 bytes).
        #[prost(message, optional, tag = "7")]
        pub source_version: ::core::option::Option<super::TruncatableString>,
    }
    /// A collection of stack frames, which can be truncated.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StackFrames {
        /// Stack frames in this call stack.
        #[prost(message, repeated, tag = "1")]
        pub frame: ::prost::alloc::vec::Vec<StackFrame>,
        /// The number of stack frames that were dropped because there
        /// were too many stack frames.
        /// If this value is 0, then no stack frames were dropped.
        #[prost(int32, tag = "2")]
        pub dropped_frames_count: i32,
    }
}
/// Binary module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// For example: main binary, kernel modules, and dynamic libraries
    /// such as libc.so, sharedlib.so (up to 256 bytes).
    #[prost(message, optional, tag = "1")]
    pub module: ::core::option::Option<TruncatableString>,
    /// A unique identifier for the module, usually a hash of its
    /// contents (up to 128 bytes).
    #[prost(message, optional, tag = "2")]
    pub build_id: ::core::option::Option<TruncatableString>,
}
/// Represents a string that might be shortened to a specified length.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TruncatableString {
    /// The shortened string. For example, if the original string is 500
    /// bytes long and the limit of the string is 128 bytes, then
    /// `value` contains the first 128 bytes of the 500-byte string.
    ///
    /// Truncation always happens on a UTF8 character boundary. If there
    /// are multi-byte characters in the string, then the length of the
    /// shortened string might be less than the size limit.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
    /// The number of bytes removed from the original string. If this
    /// value is 0, then the string was not shortened.
    #[prost(int32, tag = "2")]
    pub truncated_byte_count: i32,
}
/// The request message for the `BatchWriteSpans` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchWriteSpansRequest {
    /// Required. The name of the project where the spans belong. The format is
    /// `projects/[PROJECT_ID]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. A list of new spans. The span names must not match existing
    /// spans, or the results are undefined.
    #[prost(message, repeated, tag = "2")]
    pub spans: ::prost::alloc::vec::Vec<Span>,
}
#[doc = r" Generated client implementations."]
pub mod trace_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " This file describes an API for collecting and viewing traces and spans"]
    #[doc = " within a trace.  A Trace is a collection of spans corresponding to a single"]
    #[doc = " operation or set of operations for an application. A span is an individual"]
    #[doc = " timed event which forms a node of the trace tree. A single trace may"]
    #[doc = " contain span(s) from multiple services."]
    pub struct TraceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TraceServiceClient<T>
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
        #[doc = " Sends new spans to new or existing traces. You cannot update"]
        #[doc = " existing spans."]
        pub async fn batch_write_spans(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchWriteSpansRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudtrace.v2.TraceService/BatchWriteSpans",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new span."]
        pub async fn create_span(
            &mut self,
            request: impl tonic::IntoRequest<super::Span>,
        ) -> Result<tonic::Response<super::Span>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudtrace.v2.TraceService/CreateSpan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for TraceServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for TraceServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TraceServiceClient {{ ... }}")
        }
    }
}
