/// A topic resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Topic {
    /// Name of the topic.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A message data and its attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubsubMessage {
    /// The message payload. For JSON requests, the value of this field must be
    /// base64-encoded.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Optional attributes for this message.
    #[prost(map = "string, string", tag = "2")]
    pub attributes:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// ID of this message assigned by the server at publication time. Guaranteed
    /// to be unique within the topic. This value may be read by a subscriber
    /// that receives a PubsubMessage via a Pull call or a push delivery. It must
    /// not be populated by a publisher in a Publish call.
    #[prost(string, tag = "3")]
    pub message_id: ::prost::alloc::string::String,
}
/// Request for the GetTopic method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicRequest {
    /// The name of the topic to get.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
}
/// Request for the Publish method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishRequest {
    /// The messages in the request will be published on this topic.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    /// The messages to publish.
    #[prost(message, repeated, tag = "2")]
    pub messages: ::prost::alloc::vec::Vec<PubsubMessage>,
}
/// Response for the Publish method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishResponse {
    /// The server-assigned ID of each published message, in the same order as
    /// the messages in the request. IDs are guaranteed to be unique within
    /// the topic.
    #[prost(string, repeated, tag = "1")]
    pub message_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for the ListTopics method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicsRequest {
    /// The name of the cloud project that topics belong to.
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    /// Maximum number of topics to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last ListTopicsResponse; indicates that this is
    /// a continuation of a prior ListTopics call, and that the system should
    /// return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for the ListTopics method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicsResponse {
    /// The resulting topics.
    #[prost(message, repeated, tag = "1")]
    pub topics: ::prost::alloc::vec::Vec<Topic>,
    /// If not empty, indicates that there may be more topics that match the
    /// request; this value should be passed in a new ListTopicsRequest.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the ListTopicSubscriptions method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicSubscriptionsRequest {
    /// The name of the topic that subscriptions are attached to.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    /// Maximum number of subscription names to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last ListTopicSubscriptionsResponse; indicates
    /// that this is a continuation of a prior ListTopicSubscriptions call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for the ListTopicSubscriptions method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicSubscriptionsResponse {
    /// The names of the subscriptions that match the request.
    #[prost(string, repeated, tag = "1")]
    pub subscriptions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If not empty, indicates that there may be more subscriptions that match
    /// the request; this value should be passed in a new
    /// ListTopicSubscriptionsRequest to get more subscriptions.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the DeleteTopic method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTopicRequest {
    /// Name of the topic to delete.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
}
/// A subscription resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscription {
    /// Name of the subscription.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the topic from which this subscription is receiving messages.
    /// This will be present if and only if the subscription has not been detached
    /// from its topic.
    #[prost(string, tag = "2")]
    pub topic: ::prost::alloc::string::String,
    /// If push delivery is used with this subscription, this field is
    /// used to configure it. An empty pushConfig signifies that the subscriber
    /// will pull and ack messages using API methods.
    #[prost(message, optional, tag = "4")]
    pub push_config: ::core::option::Option<PushConfig>,
    /// This value is the maximum time after a subscriber receives a message
    /// before the subscriber should acknowledge the message. After message
    /// delivery but before the ack deadline expires and before the message is
    /// acknowledged, it is an outstanding message and will not be delivered
    /// again during that time (on a best-effort basis).
    ///
    /// For pull delivery this value
    /// is used as the initial value for the ack deadline. It may be overridden
    /// for a specific message by calling ModifyAckDeadline.
    ///
    /// For push delivery, this value is also used to set the request timeout for
    /// the call to the push endpoint.
    ///
    /// If the subscriber never acknowledges the message, the Pub/Sub
    /// system will eventually redeliver the message.
    #[prost(int32, tag = "5")]
    pub ack_deadline_seconds: i32,
}
/// Configuration for a push delivery endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushConfig {
    /// A URL locating the endpoint to which messages should be pushed.
    /// For example, a Webhook endpoint might use "https://example.com/push".
    #[prost(string, tag = "1")]
    pub push_endpoint: ::prost::alloc::string::String,
    /// Endpoint configuration attributes.
    ///
    /// Every endpoint has a set of API supported attributes that can be used to
    /// control different aspects of the message delivery.
    ///
    /// The currently supported attribute is `x-goog-version`, which you can
    /// use to change the format of the push message. This attribute
    /// indicates the version of the data expected by the endpoint. This
    /// controls the shape of the envelope (i.e. its fields and metadata).
    /// The endpoint version is based on the version of the Pub/Sub
    /// API.
    ///
    /// If not present during the CreateSubscription call, it will default to
    /// the version of the API used to make such call. If not present during a
    /// ModifyPushConfig call, its value will not be changed. GetSubscription
    /// calls will always return a valid version, even if the subscription was
    /// created without this attribute.
    ///
    /// The possible values for this attribute are:
    ///
    /// * `v1beta1`: uses the push format defined in the v1beta1 Pub/Sub API.
    /// * `v1beta2`: uses the push format defined in the v1beta2 Pub/Sub API.
    ///
    #[prost(map = "string, string", tag = "2")]
    pub attributes:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// A message and its corresponding acknowledgment ID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceivedMessage {
    /// This ID can be used to acknowledge the received message.
    #[prost(string, tag = "1")]
    pub ack_id: ::prost::alloc::string::String,
    /// The message.
    #[prost(message, optional, tag = "2")]
    pub message: ::core::option::Option<PubsubMessage>,
}
/// Request for the GetSubscription method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSubscriptionRequest {
    /// The name of the subscription to get.
    #[prost(string, tag = "1")]
    pub subscription: ::prost::alloc::string::String,
}
/// Request for the ListSubscriptions method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubscriptionsRequest {
    /// The name of the cloud project that subscriptions belong to.
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    /// Maximum number of subscriptions to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last ListSubscriptionsResponse; indicates that
    /// this is a continuation of a prior ListSubscriptions call, and that the
    /// system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for the ListSubscriptions method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubscriptionsResponse {
    /// The subscriptions that match the request.
    #[prost(message, repeated, tag = "1")]
    pub subscriptions: ::prost::alloc::vec::Vec<Subscription>,
    /// If not empty, indicates that there may be more subscriptions that match
    /// the request; this value should be passed in a new ListSubscriptionsRequest
    /// to get more subscriptions.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the DeleteSubscription method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSubscriptionRequest {
    /// The subscription to delete.
    #[prost(string, tag = "1")]
    pub subscription: ::prost::alloc::string::String,
}
/// Request for the ModifyPushConfig method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyPushConfigRequest {
    /// The name of the subscription.
    #[prost(string, tag = "1")]
    pub subscription: ::prost::alloc::string::String,
    /// The push configuration for future deliveries.
    ///
    /// An empty pushConfig indicates that the Pub/Sub system should
    /// stop pushing messages from the given subscription and allow
    /// messages to be pulled and acknowledged - effectively pausing
    /// the subscription if Pull is not called.
    #[prost(message, optional, tag = "2")]
    pub push_config: ::core::option::Option<PushConfig>,
}
/// Request for the Pull method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullRequest {
    /// The subscription from which messages should be pulled.
    #[prost(string, tag = "1")]
    pub subscription: ::prost::alloc::string::String,
    /// If this is specified as true the system will respond immediately even if
    /// it is not able to return a message in the Pull response. Otherwise the
    /// system is allowed to wait until at least one message is available rather
    /// than returning no messages. The client may cancel the request if it does
    /// not wish to wait any longer for the response.
    #[prost(bool, tag = "2")]
    pub return_immediately: bool,
    /// The maximum number of messages returned for this request. The Pub/Sub
    /// system may return fewer than the number specified.
    #[prost(int32, tag = "3")]
    pub max_messages: i32,
}
/// Response for the Pull method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullResponse {
    /// Received Pub/Sub messages. The Pub/Sub system will return zero messages if
    /// there are no more available in the backlog. The Pub/Sub system may return
    /// fewer than the maxMessages requested even if there are more messages
    /// available in the backlog.
    #[prost(message, repeated, tag = "1")]
    pub received_messages: ::prost::alloc::vec::Vec<ReceivedMessage>,
}
/// Request for the ModifyAckDeadline method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyAckDeadlineRequest {
    /// The name of the subscription.
    #[prost(string, tag = "1")]
    pub subscription: ::prost::alloc::string::String,
    /// The acknowledgment ID.
    #[prost(string, tag = "2")]
    pub ack_id: ::prost::alloc::string::String,
    /// The new ack deadline with respect to the time this request was sent to the
    /// Pub/Sub system. Must be >= 0. For example, if the value is 10, the new ack
    /// deadline will expire 10 seconds after the ModifyAckDeadline call was made.
    /// Specifying zero may immediately make the message available for another pull
    /// request.
    #[prost(int32, tag = "3")]
    pub ack_deadline_seconds: i32,
}
/// Request for the Acknowledge method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcknowledgeRequest {
    /// The subscription whose message is being acknowledged.
    #[prost(string, tag = "1")]
    pub subscription: ::prost::alloc::string::String,
    /// The acknowledgment ID for the messages being acknowledged that was returned
    /// by the Pub/Sub system in the Pull response. Must not be empty.
    #[prost(string, repeated, tag = "2")]
    pub ack_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod subscriber_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The service that an application uses to manipulate subscriptions and to"]
    #[doc = " consume messages from a subscription via the Pull method."]
    pub struct SubscriberClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SubscriberClient<T>
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
        #[doc = " Creates a subscription to a given topic for a given subscriber."]
        #[doc = " If the subscription already exists, returns ALREADY_EXISTS."]
        #[doc = " If the corresponding topic doesn't exist, returns NOT_FOUND."]
        #[doc = ""]
        #[doc = " If the name is not provided in the request, the server will assign a random"]
        #[doc = " name for this subscription on the same project as the topic."]
        pub async fn create_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::Subscription>,
        ) -> Result<tonic::Response<super::Subscription>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1beta2.Subscriber/CreateSubscription",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the configuration details of a subscription."]
        pub async fn get_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSubscriptionRequest>,
        ) -> Result<tonic::Response<super::Subscription>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1beta2.Subscriber/GetSubscription",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists matching subscriptions."]
        pub async fn list_subscriptions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSubscriptionsRequest>,
        ) -> Result<tonic::Response<super::ListSubscriptionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1beta2.Subscriber/ListSubscriptions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an existing subscription. All pending messages in the subscription"]
        #[doc = " are immediately dropped. Calls to Pull after deletion will return"]
        #[doc = " NOT_FOUND. After a subscription is deleted, a new one may be created with"]
        #[doc = " the same name, but the new one has no association with the old"]
        #[doc = " subscription, or its topic unless the same topic is specified."]
        pub async fn delete_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSubscriptionRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1beta2.Subscriber/DeleteSubscription",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Modifies the ack deadline for a specific message. This method is useful to"]
        #[doc = " indicate that more time is needed to process a message by the subscriber,"]
        #[doc = " or to make the message available for redelivery if the processing was"]
        #[doc = " interrupted."]
        pub async fn modify_ack_deadline(
            &mut self,
            request: impl tonic::IntoRequest<super::ModifyAckDeadlineRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1beta2.Subscriber/ModifyAckDeadline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Acknowledges the messages associated with the ack tokens in the"]
        #[doc = " AcknowledgeRequest. The Pub/Sub system can remove the relevant messages"]
        #[doc = " from the subscription."]
        #[doc = ""]
        #[doc = " Acknowledging a message whose ack deadline has expired may succeed,"]
        #[doc = " but such a message may be redelivered later. Acknowledging a message more"]
        #[doc = " than once will not result in an error."]
        pub async fn acknowledge(
            &mut self,
            request: impl tonic::IntoRequest<super::AcknowledgeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1beta2.Subscriber/Acknowledge",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Pulls messages from the server. Returns an empty list if there are no"]
        #[doc = " messages available in the backlog. The server may return UNAVAILABLE if"]
        #[doc = " there are too many concurrent pull requests pending for the given"]
        #[doc = " subscription."]
        pub async fn pull(
            &mut self,
            request: impl tonic::IntoRequest<super::PullRequest>,
        ) -> Result<tonic::Response<super::PullResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1beta2.Subscriber/Pull");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Modifies the PushConfig for a specified subscription."]
        #[doc = ""]
        #[doc = " This may be used to change a push subscription to a pull one (signified"]
        #[doc = " by an empty PushConfig) or vice versa, or change the endpoint URL and other"]
        #[doc = " attributes of a push subscription. Messages will accumulate for"]
        #[doc = " delivery continuously through the call regardless of changes to the"]
        #[doc = " PushConfig."]
        pub async fn modify_push_config(
            &mut self,
            request: impl tonic::IntoRequest<super::ModifyPushConfigRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1beta2.Subscriber/ModifyPushConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SubscriberClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SubscriberClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SubscriberClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod publisher_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The service that an application uses to manipulate topics, and to send"]
    #[doc = " messages to a topic."]
    pub struct PublisherClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PublisherClient<T>
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
        #[doc = " Creates the given topic with the given name."]
        pub async fn create_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::Topic>,
        ) -> Result<tonic::Response<super::Topic>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1beta2.Publisher/CreateTopic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds one or more messages to the topic. Returns NOT_FOUND if the topic does"]
        #[doc = " not exist."]
        pub async fn publish(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishRequest>,
        ) -> Result<tonic::Response<super::PublishResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1beta2.Publisher/Publish");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the configuration of a topic."]
        pub async fn get_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTopicRequest>,
        ) -> Result<tonic::Response<super::Topic>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1beta2.Publisher/GetTopic");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists matching topics."]
        pub async fn list_topics(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTopicsRequest>,
        ) -> Result<tonic::Response<super::ListTopicsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1beta2.Publisher/ListTopics");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the name of the subscriptions for this topic."]
        pub async fn list_topic_subscriptions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTopicSubscriptionsRequest>,
        ) -> Result<tonic::Response<super::ListTopicSubscriptionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1beta2.Publisher/ListTopicSubscriptions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the topic with the given name. Returns NOT_FOUND if the topic does"]
        #[doc = " not exist. After a topic is deleted, a new topic may be created with the"]
        #[doc = " same name; this is an entirely new topic with none of the old"]
        #[doc = " configuration or subscriptions. Existing subscriptions to this topic are"]
        #[doc = " not deleted."]
        pub async fn delete_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTopicRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1beta2.Publisher/DeleteTopic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PublisherClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PublisherClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PublisherClient {{ ... }}")
        }
    }
}
