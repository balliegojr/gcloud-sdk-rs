///  An insight along with the information used to derive the insight. The insight
///  may have associated recomendations as well.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Insight {
    ///  Name of the insight.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Free-form human readable summary in English. The maximum length is 500
    ///  characters.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    ///  Fully qualified resource names that this insight is targeting.
    #[prost(string, repeated, tag="9")]
    pub target_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  Insight subtype. Insight content schema will be stable for a given subtype.
    #[prost(string, tag="10")]
    pub insight_subtype: ::prost::alloc::string::String,
    ///  A struct of custom fields to explain the insight.
    ///  Example: "grantedPermissionsCount": "1000"
    #[prost(message, optional, tag="3")]
    pub content: ::core::option::Option<::prost_types::Struct>,
    ///  Timestamp of the latest data used to generate the insight.
    #[prost(message, optional, tag="4")]
    pub last_refresh_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Observation period that led to the insight. The source data used to
    ///  generate the insight ends at last_refresh_time and begins at
    ///  (last_refresh_time - observation_period).
    #[prost(message, optional, tag="5")]
    pub observation_period: ::core::option::Option<::prost_types::Duration>,
    ///  Information state and metadata.
    #[prost(message, optional, tag="6")]
    pub state_info: ::core::option::Option<InsightStateInfo>,
    ///  Category being targeted by the insight.
    #[prost(enumeration="insight::Category", tag="7")]
    pub category: i32,
    ///  Insight's severity.
    #[prost(enumeration="insight::Severity", tag="15")]
    pub severity: i32,
    ///  Fingerprint of the Insight. Provides optimistic locking when updating
    ///  states.
    #[prost(string, tag="11")]
    pub etag: ::prost::alloc::string::String,
    ///  Recommendations derived from this insight.
    #[prost(message, repeated, tag="8")]
    pub associated_recommendations: ::prost::alloc::vec::Vec<insight::RecommendationReference>,
}
/// Nested message and enum types in `Insight`.
pub mod insight {
    ///  Reference to an associated recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RecommendationReference {
        ///  Recommendation resource name, e.g.
        ///  projects/\[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/recommendations/[RECOMMENDATION_ID\]
        #[prost(string, tag="1")]
        pub recommendation: ::prost::alloc::string::String,
    }
    ///  Insight category.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Category {
        ///  Unspecified category.
        Unspecified = 0,
        ///  The insight is related to cost.
        Cost = 1,
        ///  The insight is related to security.
        Security = 2,
        ///  The insight is related to performance.
        Performance = 3,
        ///  This insight is related to manageability.
        Manageability = 4,
    }
    impl Category {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Category::Unspecified => "CATEGORY_UNSPECIFIED",
                Category::Cost => "COST",
                Category::Security => "SECURITY",
                Category::Performance => "PERFORMANCE",
                Category::Manageability => "MANAGEABILITY",
            }
        }
    }
    ///  Insight severity levels.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        ///  Insight has unspecified severity.
        Unspecified = 0,
        ///  Insight has low severity.
        Low = 1,
        ///  Insight has medium severity.
        Medium = 2,
        ///  Insight has high severity.
        High = 3,
        ///  Insight has critical severity.
        Critical = 4,
    }
    impl Severity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Severity::Unspecified => "SEVERITY_UNSPECIFIED",
                Severity::Low => "LOW",
                Severity::Medium => "MEDIUM",
                Severity::High => "HIGH",
                Severity::Critical => "CRITICAL",
            }
        }
    }
}
///  Information related to insight state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsightStateInfo {
    ///  Insight state.
    #[prost(enumeration="insight_state_info::State", tag="1")]
    pub state: i32,
    ///  A map of metadata for the state, provided by user or automations systems.
    #[prost(map="string, string", tag="2")]
    pub state_metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Nested message and enum types in `InsightStateInfo`.
pub mod insight_state_info {
    ///  Represents insight state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        ///  Unspecified state.
        Unspecified = 0,
        ///  Insight is active. Content for ACTIVE insights can be updated by Google.
        ///  ACTIVE insights can be marked DISMISSED OR ACCEPTED.
        Active = 1,
        ///  Some action has been taken based on this insight. Insights become
        ///  accepted when a recommendation derived from the insight has been marked
        ///  CLAIMED, SUCCEEDED, or FAILED. ACTIVE insights can also be marked
        ///  ACCEPTED explicitly. Content for ACCEPTED insights is immutable. ACCEPTED
        ///  insights can only be marked ACCEPTED (which may update state metadata).
        Accepted = 2,
        ///  Insight is dismissed. Content for DISMISSED insights can be updated by
        ///  Google. DISMISSED insights can be marked as ACTIVE.
        Dismissed = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "ACTIVE",
                State::Accepted => "ACCEPTED",
                State::Dismissed => "DISMISSED",
            }
        }
    }
}
///  A recommendation along with a suggested action. E.g., a rightsizing
///  recommendation for an underutilized VM, IAM role recommendations, etc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Recommendation {
    ///  Name of recommendation.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Free-form human readable summary in English. The maximum length is 500
    ///  characters.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    ///  Contains an identifier for a subtype of recommendations produced for the
    ///  same recommender. Subtype is a function of content and impact, meaning a
    ///  new subtype might be added when significant changes to `content` or
    ///  `primary_impact.category` are introduced. See the Recommenders section
    ///  to see a list of subtypes for a given Recommender.
    ///
    ///  Examples:
    ///    For recommender = "google.iam.policy.Recommender",
    ///    recommender_subtype can be one of "REMOVE_ROLE"/"REPLACE_ROLE"
    #[prost(string, tag="12")]
    pub recommender_subtype: ::prost::alloc::string::String,
    ///  Last time this recommendation was refreshed by the system that created it
    ///  in the first place.
    #[prost(message, optional, tag="4")]
    pub last_refresh_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The primary impact that this recommendation can have while trying to
    ///  optimize for one category.
    #[prost(message, optional, tag="5")]
    pub primary_impact: ::core::option::Option<Impact>,
    ///  Optional set of additional impact that this recommendation may have when
    ///  trying to optimize for the primary category. These may be positive
    ///  or negative.
    #[prost(message, repeated, tag="6")]
    pub additional_impact: ::prost::alloc::vec::Vec<Impact>,
    ///  Recommendation's priority.
    #[prost(enumeration="recommendation::Priority", tag="17")]
    pub priority: i32,
    ///  Content of the recommendation describing recommended changes to resources.
    #[prost(message, optional, tag="7")]
    pub content: ::core::option::Option<RecommendationContent>,
    ///  Information for state. Contains state and metadata.
    #[prost(message, optional, tag="10")]
    pub state_info: ::core::option::Option<RecommendationStateInfo>,
    ///  Fingerprint of the Recommendation. Provides optimistic locking when
    ///  updating states.
    #[prost(string, tag="11")]
    pub etag: ::prost::alloc::string::String,
    ///  Insights that led to this recommendation.
    #[prost(message, repeated, tag="14")]
    pub associated_insights: ::prost::alloc::vec::Vec<recommendation::InsightReference>,
    ///  Corresponds to a mutually exclusive group ID within a recommender.
    ///  A non-empty ID indicates that the recommendation belongs to a mutually
    ///  exclusive group. This means that only one recommendation within the group
    ///  is suggested to be applied.
    #[prost(string, tag="18")]
    pub xor_group_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Recommendation`.
pub mod recommendation {
    ///  Reference to an associated insight.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InsightReference {
        ///  Insight resource name, e.g.
        ///  projects/\[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/insights/[INSIGHT_ID\]
        #[prost(string, tag="1")]
        pub insight: ::prost::alloc::string::String,
    }
    ///  Recommendation priority levels.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Priority {
        ///  Recommendation has unspecified priority.
        Unspecified = 0,
        ///  Recommendation has P4 priority (lowest priority).
        P4 = 1,
        ///  Recommendation has P3 priority (second lowest priority).
        P3 = 2,
        ///  Recommendation has P2 priority (second highest priority).
        P2 = 3,
        ///  Recommendation has P1 priority (highest priority).
        P1 = 4,
    }
    impl Priority {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Priority::Unspecified => "PRIORITY_UNSPECIFIED",
                Priority::P4 => "P4",
                Priority::P3 => "P3",
                Priority::P2 => "P2",
                Priority::P1 => "P1",
            }
        }
    }
}
///  Contains what resources are changing and how they are changing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendationContent {
    ///  Operations to one or more Google Cloud resources grouped in such a way
    ///  that, all operations within one group are expected to be performed
    ///  atomically and in an order.
    #[prost(message, repeated, tag="2")]
    pub operation_groups: ::prost::alloc::vec::Vec<OperationGroup>,
    ///  Condensed overview information about the recommendation.
    #[prost(message, optional, tag="3")]
    pub overview: ::core::option::Option<::prost_types::Struct>,
}
///  Group of operations that need to be performed atomically.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationGroup {
    ///  List of operations across one or more resources that belong to this group.
    ///  Loosely based on RFC6902 and should be performed in the order they appear.
    #[prost(message, repeated, tag="1")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
}
///  Contains an operation for a resource loosely based on the JSON-PATCH format
///  with support for:
///
///  * Custom filters for describing partial array patch.
///  * Extended path values for describing nested arrays.
///  * Custom fields for describing the resource for which the operation is being
///    described.
///  * Allows extension to custom operations not natively supported by RFC6902.
///  See <https://tools.ietf.org/html/rfc6902> for details on the original RFC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    ///  Type of this operation. Contains one of 'add', 'remove', 'replace', 'move',
    ///  'copy', 'test' and custom operations. This field is case-insensitive and
    ///  always populated.
    #[prost(string, tag="1")]
    pub action: ::prost::alloc::string::String,
    ///  Type of GCP resource being modified/tested. This field is always populated.
    ///  Example: cloudresourcemanager.googleapis.com/Project,
    ///  compute.googleapis.com/Instance
    #[prost(string, tag="2")]
    pub resource_type: ::prost::alloc::string::String,
    ///  Contains the fully qualified resource name. This field is always populated.
    ///  ex: //cloudresourcemanager.googleapis.com/projects/foo.
    #[prost(string, tag="3")]
    pub resource: ::prost::alloc::string::String,
    ///  Path to the target field being operated on. If the operation is at the
    ///  resource level, then path should be "/". This field is always populated.
    #[prost(string, tag="4")]
    pub path: ::prost::alloc::string::String,
    ///  Can be set with action 'copy' to copy resource configuration across
    ///  different resources of the same type. Example: A resource clone can be
    ///  done via action = 'copy', path = "/", from = "/",
    ///  source_resource = <source> and resource_name = <target>.
    ///  This field is empty for all other values of `action`.
    #[prost(string, tag="5")]
    pub source_resource: ::prost::alloc::string::String,
    ///  Can be set with action 'copy' or 'move' to indicate the source field within
    ///  resource or source_resource, ignored if provided for other operation types.
    #[prost(string, tag="6")]
    pub source_path: ::prost::alloc::string::String,
    ///  Set of filters to apply if `path` refers to array elements or nested array
    ///  elements in order to narrow down to a single unique element that is being
    ///  tested/modified.
    ///  This is intended to be an exact match per filter. To perform advanced
    ///  matching, use path_value_matchers.
    ///
    ///  * Example:
    ///  ```
    ///  {
    ///    "/versions/*/name" : "it-123"
    ///    "/versions/*/targetSize/percent": 20
    ///  }
    ///  ```
    ///  * Example:
    ///  ```
    ///  {
    ///    "/bindings/*/role": "roles/owner"
    ///    "/bindings/*/condition" : null
    ///  }
    ///  ```
    ///  * Example:
    ///  ```
    ///  {
    ///    "/bindings/*/role": "roles/owner"
    ///    "/bindings/*/members/*" : ["x@example.com", "y@example.com"]
    ///  }
    ///  ```
    ///  When both path_filters and path_value_matchers are set, an implicit AND
    ///  must be performed.
    #[prost(map="string, message", tag="8")]
    pub path_filters: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
    ///  Similar to path_filters, this contains set of filters to apply if `path`
    ///  field refers to array elements. This is meant to support value matching
    ///  beyond exact match. To perform exact match, use path_filters.
    ///  When both path_filters and path_value_matchers are set, an implicit AND
    ///  must be performed.
    #[prost(map="string, message", tag="11")]
    pub path_value_matchers: ::std::collections::HashMap<::prost::alloc::string::String, ValueMatcher>,
    ///  One of the fields in the following block will be set and intend to
    ///  describe a value for 'path' field.
    #[prost(oneof="operation::PathValue", tags="7, 10")]
    pub path_value: ::core::option::Option<operation::PathValue>,
}
/// Nested message and enum types in `Operation`.
pub mod operation {
    ///  One of the fields in the following block will be set and intend to
    ///  describe a value for 'path' field.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PathValue {
        ///  Value for the `path` field. Will be set for actions:'add'/'replace'.
        ///  Maybe set for action: 'test'. Either this or `value_matcher` will be set
        ///  for 'test' operation. An exact match must be performed.
        #[prost(message, tag="7")]
        Value(::prost_types::Value),
        ///  Can be set for action 'test' for advanced matching for the value of
        ///  'path' field. Either this or `value` will be set for 'test' operation.
        #[prost(message, tag="10")]
        ValueMatcher(super::ValueMatcher),
    }
}
///  Contains various matching options for values for a GCP resource field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueMatcher {
    #[prost(oneof="value_matcher::MatchVariant", tags="1")]
    pub match_variant: ::core::option::Option<value_matcher::MatchVariant>,
}
/// Nested message and enum types in `ValueMatcher`.
pub mod value_matcher {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchVariant {
        ///  To be used for full regex matching. The regular expression is using the
        ///  Google RE2 syntax (<https://github.com/google/re2/wiki/Syntax>), so to be
        ///  used with RE2::FullMatch
        #[prost(string, tag="1")]
        MatchesPattern(::prost::alloc::string::String),
    }
}
///  Contains metadata about how much money a recommendation can save or incur.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CostProjection {
    ///  An approximate projection on amount saved or amount incurred. Negative cost
    ///  units indicate cost savings and positive cost units indicate increase.
    ///  See google.type.Money documentation for positive/negative units.
    ///
    ///  A user's permissions may affect whether the cost is computed using list
    ///  prices or custom contract prices.
    #[prost(message, optional, tag="1")]
    pub cost: ::core::option::Option<super::super::super::r#type::Money>,
    ///  Duration for which this cost applies.
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
///  Contains various ways of describing the impact on Security.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityProjection {
    ///  Additional security impact details that is provided by the recommender.
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<::prost_types::Struct>,
}
///  Contains the impact a recommendation can have for a given category.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Impact {
    ///  Category that is being targeted.
    #[prost(enumeration="impact::Category", tag="1")]
    pub category: i32,
    ///  Contains projections (if any) for this category.
    #[prost(oneof="impact::Projection", tags="100, 101")]
    pub projection: ::core::option::Option<impact::Projection>,
}
/// Nested message and enum types in `Impact`.
pub mod impact {
    ///  The category of the impact.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Category {
        ///  Default unspecified category. Don't use directly.
        Unspecified = 0,
        ///  Indicates a potential increase or decrease in cost.
        Cost = 1,
        ///  Indicates a potential increase or decrease in security.
        Security = 2,
        ///  Indicates a potential increase or decrease in performance.
        Performance = 3,
        ///  Indicates a potential increase or decrease in manageability.
        Manageability = 4,
    }
    impl Category {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Category::Unspecified => "CATEGORY_UNSPECIFIED",
                Category::Cost => "COST",
                Category::Security => "SECURITY",
                Category::Performance => "PERFORMANCE",
                Category::Manageability => "MANAGEABILITY",
            }
        }
    }
    ///  Contains projections (if any) for this category.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Projection {
        ///  Use with CategoryType.COST
        #[prost(message, tag="100")]
        CostProjection(super::CostProjection),
        ///  Use with CategoryType.SECURITY
        #[prost(message, tag="101")]
        SecurityProjection(super::SecurityProjection),
    }
}
///  Information for state. Contains state and metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendationStateInfo {
    ///  The state of the recommendation, Eg ACTIVE, SUCCEEDED, FAILED.
    #[prost(enumeration="recommendation_state_info::State", tag="1")]
    pub state: i32,
    ///  A map of metadata for the state, provided by user or automations systems.
    #[prost(map="string, string", tag="2")]
    pub state_metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Nested message and enum types in `RecommendationStateInfo`.
pub mod recommendation_state_info {
    ///  Represents Recommendation State.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        ///  Default state. Don't use directly.
        Unspecified = 0,
        ///  Recommendation is active and can be applied. Recommendations content can
        ///  be updated by Google.
        ///
        ///  ACTIVE recommendations can be marked as CLAIMED, SUCCEEDED, or FAILED.
        Active = 1,
        ///  Recommendation is in claimed state. Recommendations content is
        ///  immutable and cannot be updated by Google.
        ///
        ///  CLAIMED recommendations can be marked as CLAIMED, SUCCEEDED, or FAILED.
        Claimed = 6,
        ///  Recommendation is in succeeded state. Recommendations content is
        ///  immutable and cannot be updated by Google.
        ///
        ///  SUCCEEDED recommendations can be marked as SUCCEEDED, or FAILED.
        Succeeded = 3,
        ///  Recommendation is in failed state. Recommendations content is immutable
        ///  and cannot be updated by Google.
        ///
        ///  FAILED recommendations can be marked as SUCCEEDED, or FAILED.
        Failed = 4,
        ///  Recommendation is in dismissed state. Recommendation content can be
        ///  updated by Google.
        ///
        ///  DISMISSED recommendations can be marked as ACTIVE.
        Dismissed = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "ACTIVE",
                State::Claimed => "CLAIMED",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::Dismissed => "DISMISSED",
            }
        }
    }
}
///  Configuration for an InsightType.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsightTypeConfig {
    ///  Name of insight type config.
    ///  Eg,
    ///  projects/\[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID\]/config
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  InsightTypeGenerationConfig which configures the generation of
    ///  insights for this insight type.
    #[prost(message, optional, tag="2")]
    pub insight_type_generation_config: ::core::option::Option<InsightTypeGenerationConfig>,
    ///  Fingerprint of the InsightTypeConfig. Provides optimistic locking when
    ///  updating.
    #[prost(string, tag="3")]
    pub etag: ::prost::alloc::string::String,
    ///  Last time when the config was updated.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. Immutable. The revision ID of the config.
    ///  A new revision is committed whenever the config is changed in any way.
    ///  The format is an 8-character hexadecimal string.
    #[prost(string, tag="5")]
    pub revision_id: ::prost::alloc::string::String,
    ///  Allows clients to store small amounts of arbitrary data. Annotations must
    ///  follow the Kubernetes syntax.
    ///  The total size of all keys and values combined is limited to 256k.
    ///  Key can have 2 segments: prefix (optional) and name (required),
    ///  separated by a slash (/).
    ///  Prefix must be a DNS subdomain.
    ///  Name must be 63 characters or less, begin and end with alphanumerics,
    ///  with dashes (-), underscores (_), dots (.), and alphanumerics between.
    #[prost(map="string, string", tag="6")]
    pub annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  A user-settable field to provide a human-readable name to be used in user
    ///  interfaces.
    #[prost(string, tag="7")]
    pub display_name: ::prost::alloc::string::String,
}
///  A configuration to customize the generation of insights.
///  Eg, customizing the lookback period considered when generating a
///  insight.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsightTypeGenerationConfig {
    ///  Parameters for this InsightTypeGenerationConfig. These configs can be used
    ///  by or are applied to all subtypes.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<::prost_types::Struct>,
}
///  Configuration for a Recommender.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommenderConfig {
    ///  Name of recommender config.
    ///  Eg,
    ///  projects/\[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID\]/config
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  RecommenderGenerationConfig which configures the Generation of
    ///  recommendations for this recommender.
    #[prost(message, optional, tag="2")]
    pub recommender_generation_config: ::core::option::Option<RecommenderGenerationConfig>,
    ///  Fingerprint of the RecommenderConfig. Provides optimistic locking when
    ///  updating.
    #[prost(string, tag="3")]
    pub etag: ::prost::alloc::string::String,
    ///  Last time when the config was updated.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. Immutable. The revision ID of the config.
    ///  A new revision is committed whenever the config is changed in any way.
    ///  The format is an 8-character hexadecimal string.
    #[prost(string, tag="5")]
    pub revision_id: ::prost::alloc::string::String,
    ///  Allows clients to store small amounts of arbitrary data. Annotations must
    ///  follow the Kubernetes syntax.
    ///  The total size of all keys and values combined is limited to 256k.
    ///  Key can have 2 segments: prefix (optional) and name (required),
    ///  separated by a slash (/).
    ///  Prefix must be a DNS subdomain.
    ///  Name must be 63 characters or less, begin and end with alphanumerics,
    ///  with dashes (-), underscores (_), dots (.), and alphanumerics between.
    #[prost(map="string, string", tag="6")]
    pub annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  A user-settable field to provide a human-readable name to be used in user
    ///  interfaces.
    #[prost(string, tag="7")]
    pub display_name: ::prost::alloc::string::String,
}
///  A Configuration to customize the generation of recommendations.
///  Eg, customizing the lookback period considered when generating a
///  recommendation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommenderGenerationConfig {
    ///  Parameters for this RecommenderGenerationConfig. These configs can be used
    ///  by or are applied to all subtypes.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<::prost_types::Struct>,
}
///  Request for the `ListInsights` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInsightsRequest {
    ///  Required. The container resource on which to execute the request.
    ///  Acceptable formats:
    ///
    ///  * `projects/\[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID\]`
    ///
    ///  * `projects/\[PROJECT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID\]`
    ///
    ///  * `billingAccounts/\[BILLING_ACCOUNT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID\]`
    ///
    ///  * `folders/\[FOLDER_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID\]`
    ///
    ///  * `organizations/\[ORGANIZATION_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID\]`
    ///
    ///  LOCATION here refers to GCP Locations:
    ///  <https://cloud.google.com/about/locations/>
    ///  INSIGHT_TYPE_ID refers to supported insight types:
    ///  <https://cloud.google.com/recommender/docs/insights/insight-types.>
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Optional. The maximum number of results to return from this request.
    ///  Non-positive values are ignored. If not specified, the server will
    ///  determine the number of results to return.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  Optional. If present, retrieves the next batch of results from the
    ///  preceding call to this method. `page_token` must be the value of
    ///  `next_page_token` from the previous response. The values of other method
    ///  parameters must be identical to those in the previous call.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    ///  Optional. Filter expression to restrict the insights returned. Supported
    ///  filter fields:
    ///
    ///  * `stateInfo.state`
    ///
    ///  * `insightSubtype`
    ///
    ///  * `severity`
    ///
    ///  Examples:
    ///
    ///  * `stateInfo.state = ACTIVE OR stateInfo.state = DISMISSED`
    ///
    ///  * `insightSubtype = PERMISSIONS_USAGE`
    ///
    ///  * `severity = CRITICAL OR severity = HIGH`
    ///
    ///  * `stateInfo.state = ACTIVE AND (severity = CRITICAL OR severity = HIGH)`
    ///
    ///  (These expressions are based on the filter language described at
    ///  <https://google.aip.dev/160>)
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
///  Response to the `ListInsights` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInsightsResponse {
    ///  The set of insights for the `parent` resource.
    #[prost(message, repeated, tag="1")]
    pub insights: ::prost::alloc::vec::Vec<Insight>,
    ///  A token that can be used to request the next page of results. This field is
    ///  empty if there are no additional results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Request to the `GetInsight` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInsightRequest {
    ///  Required. Name of the insight.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Request for the `MarkInsightAccepted` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkInsightAcceptedRequest {
    ///  Required. Name of the insight.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Optional. State properties user wish to include with this state.  Full
    ///  replace of the current state_metadata.
    #[prost(map="string, string", tag="2")]
    pub state_metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  Required. Fingerprint of the Insight. Provides optimistic locking.
    #[prost(string, tag="3")]
    pub etag: ::prost::alloc::string::String,
}
///  Request for the `ListRecommendations` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRecommendationsRequest {
    ///  Required. The container resource on which to execute the request.
    ///  Acceptable formats:
    ///
    ///  * `projects/\[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID\]`
    ///
    ///  * `projects/\[PROJECT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID\]`
    ///
    ///  * `billingAccounts/\[BILLING_ACCOUNT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID\]`
    ///
    ///  * `folders/\[FOLDER_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID\]`
    ///
    ///  * `organizations/\[ORGANIZATION_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID\]`
    ///
    ///  LOCATION here refers to GCP Locations:
    ///  <https://cloud.google.com/about/locations/>
    ///  RECOMMENDER_ID refers to supported recommenders:
    ///  <https://cloud.google.com/recommender/docs/recommenders.>
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Optional. The maximum number of results to return from this request.
    ///  Non-positive values are ignored. If not specified, the server will
    ///  determine the number of results to return.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  Optional. If present, retrieves the next batch of results from the
    ///  preceding call to this method. `page_token` must be the value of
    ///  `next_page_token` from the previous response. The values of other method
    ///  parameters must be identical to those in the previous call.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    ///  Filter expression to restrict the recommendations returned. Supported
    ///  filter fields:
    ///
    ///  * `state_info.state`
    ///
    ///  * `recommenderSubtype`
    ///
    ///  * `priority`
    ///
    ///  Examples:
    ///
    ///  * `stateInfo.state = ACTIVE OR stateInfo.state = DISMISSED`
    ///
    ///  * `recommenderSubtype = REMOVE_ROLE OR recommenderSubtype = REPLACE_ROLE`
    ///
    ///  * `priority = P1 OR priority = P2`
    ///
    ///  * `stateInfo.state = ACTIVE AND (priority = P1 OR priority = P2)`
    ///
    ///  (These expressions are based on the filter language described at
    ///  <https://google.aip.dev/160>)
    #[prost(string, tag="5")]
    pub filter: ::prost::alloc::string::String,
}
///  Response to the `ListRecommendations` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRecommendationsResponse {
    ///  The set of recommendations for the `parent` resource.
    #[prost(message, repeated, tag="1")]
    pub recommendations: ::prost::alloc::vec::Vec<Recommendation>,
    ///  A token that can be used to request the next page of results. This field is
    ///  empty if there are no additional results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Request to the `GetRecommendation` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecommendationRequest {
    ///  Required. Name of the recommendation.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Request for the `MarkRecommendationClaimed` Method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkRecommendationClaimedRequest {
    ///  Required. Name of the recommendation.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  State properties to include with this state. Overwrites any existing
    ///  `state_metadata`.
    ///  Keys must match the regex `/^\[a-z0-9][a-z0-9_.-\]{0,62}$/`.
    ///  Values must match the regex `/^\[a-zA-Z0-9_./-\]{0,255}$/`.
    #[prost(map="string, string", tag="2")]
    pub state_metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  Required. Fingerprint of the Recommendation. Provides optimistic locking.
    #[prost(string, tag="3")]
    pub etag: ::prost::alloc::string::String,
}
///  Request for the `MarkRecommendationSucceeded` Method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkRecommendationSucceededRequest {
    ///  Required. Name of the recommendation.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  State properties to include with this state. Overwrites any existing
    ///  `state_metadata`.
    ///  Keys must match the regex `/^\[a-z0-9][a-z0-9_.-\]{0,62}$/`.
    ///  Values must match the regex `/^\[a-zA-Z0-9_./-\]{0,255}$/`.
    #[prost(map="string, string", tag="2")]
    pub state_metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  Required. Fingerprint of the Recommendation. Provides optimistic locking.
    #[prost(string, tag="3")]
    pub etag: ::prost::alloc::string::String,
}
///  Request for the `MarkRecommendationFailed` Method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkRecommendationFailedRequest {
    ///  Required. Name of the recommendation.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  State properties to include with this state. Overwrites any existing
    ///  `state_metadata`.
    ///  Keys must match the regex `/^\[a-z0-9][a-z0-9_.-\]{0,62}$/`.
    ///  Values must match the regex `/^\[a-zA-Z0-9_./-\]{0,255}$/`.
    #[prost(map="string, string", tag="2")]
    pub state_metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  Required. Fingerprint of the Recommendation. Provides optimistic locking.
    #[prost(string, tag="3")]
    pub etag: ::prost::alloc::string::String,
}
///  Request for the GetRecommenderConfig` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecommenderConfigRequest {
    ///  Required. Name of the Recommendation Config to get.
    ///
    ///  Acceptable formats:
    ///
    ///  * `projects/\[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID\]/config`
    ///
    ///  * `projects/\[PROJECT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID\]/config`
    ///
    ///  * `organizations/\[ORGANIZATION_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID\]/config`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Request for the `UpdateRecommenderConfig` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRecommenderConfigRequest {
    ///  Required. The RecommenderConfig to update.
    #[prost(message, optional, tag="1")]
    pub recommender_config: ::core::option::Option<RecommenderConfig>,
    ///  The list of fields to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    ///  If true, validate the request and preview the change, but do not actually
    ///  update it.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
}
///  Request for the GetInsightTypeConfig` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInsightTypeConfigRequest {
    ///  Required. Name of the InsightTypeConfig to get.
    ///
    ///  Acceptable formats:
    ///
    ///  * `projects/\[PROJECT_NUMBER]/locations/global/recommenders/[INSIGHT_TYPE_ID\]/config`
    ///
    ///  * `projects/\[PROJECT_ID]/locations/global/recommenders/[INSIGHT_TYPE_ID\]/config`
    ///
    ///  * `organizations/\[ORGANIZATION_ID]/locations/global/recommenders/[INSIGHT_TYPE_ID\]/config`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Request for the `UpdateInsightTypeConfig` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInsightTypeConfigRequest {
    ///  Required. The InsightTypeConfig to update.
    #[prost(message, optional, tag="1")]
    pub insight_type_config: ::core::option::Option<InsightTypeConfig>,
    ///  The list of fields to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    ///  If true, validate the request and preview the change, but do not actually
    ///  update it.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
}
/// Generated client implementations.
pub mod recommender_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides insights and recommendations for cloud customers for various
    /// categories like performance optimization, cost savings, reliability, feature
    /// discovery, etc. Insights and recommendations are generated automatically
    /// based on analysis of user resources, configuration and monitoring metrics.
    #[derive(Debug, Clone)]
    pub struct RecommenderClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RecommenderClient<tonic::transport::Channel> {
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
    impl<T> RecommenderClient<T>
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
        ) -> RecommenderClient<InterceptedService<T, F>>
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
            RecommenderClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists insights for the specified Cloud Resource. Requires the
        /// recommender.*.list IAM permission for the specified insight type.
        pub async fn list_insights(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInsightsRequest>,
        ) -> Result<tonic::Response<super::ListInsightsResponse>, tonic::Status> {
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
                "/google.cloud.recommender.v1.Recommender/ListInsights",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the requested insight. Requires the recommender.*.get IAM permission
        /// for the specified insight type.
        pub async fn get_insight(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInsightRequest>,
        ) -> Result<tonic::Response<super::Insight>, tonic::Status> {
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
                "/google.cloud.recommender.v1.Recommender/GetInsight",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Marks the Insight State as Accepted. Users can use this method to
        /// indicate to the Recommender API that they have applied some action based
        /// on the insight. This stops the insight content from being updated.
        ///
        /// MarkInsightAccepted can be applied to insights in ACTIVE state. Requires
        /// the recommender.*.update IAM permission for the specified insight.
        pub async fn mark_insight_accepted(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkInsightAcceptedRequest>,
        ) -> Result<tonic::Response<super::Insight>, tonic::Status> {
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
                "/google.cloud.recommender.v1.Recommender/MarkInsightAccepted",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists recommendations for the specified Cloud Resource. Requires the
        /// recommender.*.list IAM permission for the specified recommender.
        pub async fn list_recommendations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRecommendationsRequest>,
        ) -> Result<tonic::Response<super::ListRecommendationsResponse>, tonic::Status> {
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
                "/google.cloud.recommender.v1.Recommender/ListRecommendations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the requested recommendation. Requires the recommender.*.get
        /// IAM permission for the specified recommender.
        pub async fn get_recommendation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRecommendationRequest>,
        ) -> Result<tonic::Response<super::Recommendation>, tonic::Status> {
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
                "/google.cloud.recommender.v1.Recommender/GetRecommendation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Marks the Recommendation State as Claimed. Users can use this method to
        /// indicate to the Recommender API that they are starting to apply the
        /// recommendation themselves. This stops the recommendation content from being
        /// updated. Associated insights are frozen and placed in the ACCEPTED state.
        ///
        /// MarkRecommendationClaimed can be applied to recommendations in CLAIMED,
        /// SUCCEEDED, FAILED, or ACTIVE state.
        ///
        /// Requires the recommender.*.update IAM permission for the specified
        /// recommender.
        pub async fn mark_recommendation_claimed(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkRecommendationClaimedRequest>,
        ) -> Result<tonic::Response<super::Recommendation>, tonic::Status> {
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
                "/google.cloud.recommender.v1.Recommender/MarkRecommendationClaimed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Marks the Recommendation State as Succeeded. Users can use this method to
        /// indicate to the Recommender API that they have applied the recommendation
        /// themselves, and the operation was successful. This stops the recommendation
        /// content from being updated. Associated insights are frozen and placed in
        /// the ACCEPTED state.
        ///
        /// MarkRecommendationSucceeded can be applied to recommendations in ACTIVE,
        /// CLAIMED, SUCCEEDED, or FAILED state.
        ///
        /// Requires the recommender.*.update IAM permission for the specified
        /// recommender.
        pub async fn mark_recommendation_succeeded(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkRecommendationSucceededRequest>,
        ) -> Result<tonic::Response<super::Recommendation>, tonic::Status> {
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
                "/google.cloud.recommender.v1.Recommender/MarkRecommendationSucceeded",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Marks the Recommendation State as Failed. Users can use this method to
        /// indicate to the Recommender API that they have applied the recommendation
        /// themselves, and the operation failed. This stops the recommendation content
        /// from being updated. Associated insights are frozen and placed in the
        /// ACCEPTED state.
        ///
        /// MarkRecommendationFailed can be applied to recommendations in ACTIVE,
        /// CLAIMED, SUCCEEDED, or FAILED state.
        ///
        /// Requires the recommender.*.update IAM permission for the specified
        /// recommender.
        pub async fn mark_recommendation_failed(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkRecommendationFailedRequest>,
        ) -> Result<tonic::Response<super::Recommendation>, tonic::Status> {
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
                "/google.cloud.recommender.v1.Recommender/MarkRecommendationFailed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the requested Recommender Config. There is only one instance of the
        /// config for each Recommender.
        pub async fn get_recommender_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRecommenderConfigRequest>,
        ) -> Result<tonic::Response<super::RecommenderConfig>, tonic::Status> {
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
                "/google.cloud.recommender.v1.Recommender/GetRecommenderConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a Recommender Config. This will create a new revision of the
        /// config.
        pub async fn update_recommender_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRecommenderConfigRequest>,
        ) -> Result<tonic::Response<super::RecommenderConfig>, tonic::Status> {
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
                "/google.cloud.recommender.v1.Recommender/UpdateRecommenderConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the requested InsightTypeConfig. There is only one instance of the
        /// config for each InsightType.
        pub async fn get_insight_type_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInsightTypeConfigRequest>,
        ) -> Result<tonic::Response<super::InsightTypeConfig>, tonic::Status> {
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
                "/google.cloud.recommender.v1.Recommender/GetInsightTypeConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an InsightTypeConfig change. This will create a new revision of the
        /// config.
        pub async fn update_insight_type_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInsightTypeConfigRequest>,
        ) -> Result<tonic::Response<super::InsightTypeConfig>, tonic::Status> {
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
                "/google.cloud.recommender.v1.Recommender/UpdateInsightTypeConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}