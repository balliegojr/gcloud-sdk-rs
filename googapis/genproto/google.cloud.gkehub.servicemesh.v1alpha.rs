/// **Service Mesh**: State for the whole Hub, as analyzed by the Service Mesh
/// Hub Controller.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureState {
    /// Output only. Results of running Service Mesh analyzers.
    #[prost(message, repeated, tag = "1")]
    pub analysis_messages: ::prost::alloc::vec::Vec<AnalysisMessage>,
}
/// **Service Mesh**: State for a single Membership, as analyzed by the Service
/// Mesh Hub Controller.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipState {
    /// Output only. Results of running Service Mesh analyzers.
    #[prost(message, repeated, tag = "1")]
    pub analysis_messages: ::prost::alloc::vec::Vec<AnalysisMessage>,
}
/// AnalysisMessageBase describes some common information that is
/// needed for all messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalysisMessageBase {
    /// Represents the specific type of a message.
    #[prost(message, optional, tag = "1")]
    pub r#type: ::core::option::Option<analysis_message_base::Type>,
    /// Represents how severe a message is.
    #[prost(enumeration = "analysis_message_base::Level", tag = "2")]
    pub level: i32,
    /// A url pointing to the Service Mesh or Istio documentation for this specific
    /// error type.
    #[prost(string, tag = "3")]
    pub documentation_url: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AnalysisMessageBase`.
pub mod analysis_message_base {
    /// A unique identifier for the type of message. Display_name is intended to be
    /// human-readable, code is intended to be machine readable. There should be a
    /// one-to-one mapping between display_name and code. (i.e. do not re-use
    /// display_names or codes between message types.)
    /// See istio.analysis.v1alpha1.AnalysisMessageBase.Type
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Type {
        /// A human-readable name for the message type. e.g. "InternalError",
        /// "PodMissingProxy". This should be the same for all messages of the same
        /// type. (This corresponds to the `name` field in open-source Istio.)
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// A 7 character code matching `^IST[0-9]{4}$` or `^ASM[0-9]{4}$`, intended
        /// to uniquely identify the message type. (e.g. "IST0001" is mapped to the
        /// "InternalError" message type.)
        #[prost(string, tag = "2")]
        pub code: ::prost::alloc::string::String,
    }
    /// The values here are chosen so that more severe messages get sorted higher,
    /// as well as leaving space in between to add more later
    /// See istio.analysis.v1alpha1.AnalysisMessageBase.Level
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Level {
        /// Illegal. Same istio.analysis.v1alpha1.AnalysisMessageBase.Level.UNKNOWN.
        Unspecified = 0,
        /// ERROR represents a misconfiguration that must be fixed.
        Error = 3,
        /// WARNING represents a misconfiguration that should be fixed.
        Warning = 8,
        /// INFO represents an informational finding.
        Info = 12,
    }
}
/// AnalysisMessage is a single message produced by an analyzer, and
/// it used to communicate to the end user about the state of their Service
/// Mesh configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalysisMessage {
    /// Details common to all types of Istio and ServiceMesh analysis messages.
    #[prost(message, optional, tag = "1")]
    pub message_base: ::core::option::Option<AnalysisMessageBase>,
    /// A human readable description of what the error means. It is suitable for
    /// non-internationalize display purposes.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// A list of strings specifying the resource identifiers that were the cause
    /// of message generation.
    /// A "path" here may be:
    /// * MEMBERSHIP_ID if the cause is a specific member cluster
    /// * MEMBERSHIP_ID/(NAMESPACE\/)?RESOURCETYPE/NAME if the cause is a resource
    /// in a cluster
    #[prost(string, repeated, tag = "3")]
    pub resource_paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A UI can combine these args with a template (based on message_base.type)
    /// to produce an internationalized message.
    #[prost(message, optional, tag = "4")]
    pub args: ::core::option::Option<::prost_types::Struct>,
}