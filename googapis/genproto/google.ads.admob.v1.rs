/// A publisher account contains information relevant to the use of this API,
/// such as the time zone used for the reports.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublisherAccount {
    /// Resource name of this account.
    /// Format is accounts/{publisher_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The unique ID by which this publisher account can be identified
    /// in the API requests (for example, pub-1234567890).
    #[prost(string, tag = "2")]
    pub publisher_id: ::prost::alloc::string::String,
    /// The time zone that is used in reports that are generated for this account.
    /// The value is a time-zone ID as specified by the CLDR project,
    /// for example, "America/Los_Angeles".
    #[prost(string, tag = "3")]
    pub reporting_time_zone: ::prost::alloc::string::String,
    /// Currency code of the earning-related metrics, which is the 3-letter code
    /// defined in ISO 4217. The daily average rate is used for the currency
    /// conversion.
    #[prost(string, tag = "4")]
    pub currency_code: ::prost::alloc::string::String,
}
/// The specification for generating an AdMob Network report.
/// For example, the specification to get clicks and estimated earnings for only
/// the 'US' and 'CN' countries can look like the following example:
///
///     {
///       'date_range': {
///         'start_date': {'year': 2018, 'month': 9, 'day': 1},
///         'end_date': {'year': 2018, 'month': 9, 'day': 30}
///       },
///       'dimensions': ['DATE', 'APP', 'COUNTRY'],
///       'metrics': ['CLICKS', 'ESTIMATED_EARNINGS'],
///       'dimension_filters': [
///         {
///           'dimension': 'COUNTRY',
///           'matches_any': {'values': [{'value': 'US', 'value': 'CN'}]}
///         }
///       ],
///       'sort_conditions': [
///         {'dimension':'APP', order: 'ASCENDING'},
///         {'metric':'CLICKS', order: 'DESCENDING'}
///       ],
///       'localization_settings': {
///         'currency_code': 'USD',
///         'language_code': 'en-US'
///       }
///     }
///
/// For a better understanding, you can treat the preceding specification like
/// the following pseudo SQL:
///
///     SELECT DATE, APP, COUNTRY, CLICKS, ESTIMATED_EARNINGS
///     FROM NETWORK_REPORT
///     WHERE DATE >= '2018-09-01' AND DATE <= '2018-09-30'
///         AND COUNTRY IN ('US', 'CN')
///     GROUP BY DATE, APP, COUNTRY
///     ORDER BY APP ASC, CLICKS DESC;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkReportSpec {
    /// The date range for which the report is generated.
    #[prost(message, optional, tag = "1")]
    pub date_range: ::core::option::Option<DateRange>,
    /// List of dimensions of the report. The value combination of these dimensions
    /// determines the row of the report. If no dimensions are specified, the
    /// report returns a single row of requested metrics for the entire account.
    #[prost(enumeration = "network_report_spec::Dimension", repeated, tag = "2")]
    pub dimensions: ::prost::alloc::vec::Vec<i32>,
    /// List of metrics of the report. A report must specify at least one metric.
    #[prost(enumeration = "network_report_spec::Metric", repeated, tag = "3")]
    pub metrics: ::prost::alloc::vec::Vec<i32>,
    /// Describes which report rows to match based on their dimension values.
    #[prost(message, repeated, tag = "4")]
    pub dimension_filters: ::prost::alloc::vec::Vec<network_report_spec::DimensionFilter>,
    /// Describes the sorting of report rows. The order of the condition in the
    /// list defines its precedence; the earlier the condition, the higher its
    /// precedence. If no sort conditions are specified, the row ordering is
    /// undefined.
    #[prost(message, repeated, tag = "5")]
    pub sort_conditions: ::prost::alloc::vec::Vec<network_report_spec::SortCondition>,
    /// Localization settings of the report.
    #[prost(message, optional, tag = "6")]
    pub localization_settings: ::core::option::Option<LocalizationSettings>,
    /// Maximum number of report data rows to return. If the value is not set, the
    /// API returns as many rows as possible, up to 100000. Acceptable values are
    /// 1-100000, inclusive. Any other values are treated as 100000.
    #[prost(int32, tag = "7")]
    pub max_report_rows: i32,
    /// A report time zone. Accepts an IANA TZ name values, such as
    /// "America/Los_Angeles."  If no time zone is defined, the account default
    /// takes effect. Check default value by the get account action.
    ///
    /// **Warning:** The "America/Los_Angeles" is the only supported value at
    /// the moment.
    #[prost(string, tag = "8")]
    pub time_zone: ::prost::alloc::string::String,
}
/// Nested message and enum types in `NetworkReportSpec`.
pub mod network_report_spec {
    /// Describes which report rows to match based on their dimension values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DimensionFilter {
        /// Applies the filter criterion to the specified dimension.
        #[prost(enumeration = "Dimension", tag = "1")]
        pub dimension: i32,
        /// Filter operator to be applied.
        #[prost(oneof = "dimension_filter::Operator", tags = "2")]
        pub operator: ::core::option::Option<dimension_filter::Operator>,
    }
    /// Nested message and enum types in `DimensionFilter`.
    pub mod dimension_filter {
        /// Filter operator to be applied.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Operator {
            /// Matches a row if its value for the specified dimension is in one of the
            /// values specified in this condition.
            #[prost(message, tag = "2")]
            MatchesAny(super::super::StringList),
        }
    }
    /// Sorting direction to be applied on a dimension or a metric.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SortCondition {
        /// Sorting order of the dimension or metric.
        #[prost(enumeration = "super::SortOrder", tag = "3")]
        pub order: i32,
        /// Identifies which values to sort on.
        #[prost(oneof = "sort_condition::SortOn", tags = "1, 2")]
        pub sort_on: ::core::option::Option<sort_condition::SortOn>,
    }
    /// Nested message and enum types in `SortCondition`.
    pub mod sort_condition {
        /// Identifies which values to sort on.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum SortOn {
            /// Sort by the specified dimension.
            #[prost(enumeration = "super::Dimension", tag = "1")]
            Dimension(i32),
            /// Sort by the specified metric.
            #[prost(enumeration = "super::Metric", tag = "2")]
            Metric(i32),
        }
    }
    /// The dimensions of the network report. Dimensions are data attributes to
    /// break down or refine the quantitative measurements (metrics) by certain
    /// attributes, such as the ad format or the platform an ad was viewed on.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Dimension {
        /// Default value for an unset field. Do not use.
        Unspecified = 0,
        /// A date in the YYYY-MM-DD format (for example, "2018-12-21"). Requests can
        /// specify at most one time dimension.
        Date = 1,
        /// A month in the YYYY-MM format (for example, "2018-12"). Requests can
        /// specify at most one time dimension.
        Month = 2,
        /// The date of the first day of a week in the YYYY-MM-DD format
        /// (for example, "2018-12-21"). Requests can specify at most one time
        /// dimension.
        Week = 3,
        /// The unique ID of the ad unit (for example, "ca-app-pub-1234/1234").
        /// If AD_UNIT dimension is specified, then APP is included automatically.
        AdUnit = 4,
        /// The unique ID of the mobile application (for example,
        /// "ca-app-pub-1234~1234").
        App = 5,
        /// Type of the ad (for example, "text" or "image"), an ad delivery
        /// dimension.
        ///
        /// **Warning:** The dimension is incompatible with
        /// [AD_REQUESTS](#Metric.ENUM_VALUES.AD_REQUESTS),
        /// [MATCH_RATE](#Metric.ENUM_VALUES.MATCH_RATE) and
        /// [IMPRESSION_RPM](#Metric.ENUM_VALUES.IMPRESSION_RPM) metrics.
        AdType = 6,
        /// CLDR country code of the place where the ad views/clicks occur (for
        /// example, "US" or "FR"). This is a geography dimension.
        Country = 7,
        /// Format of the ad unit (for example, "banner", "native"), an ad delivery
        /// dimension.
        Format = 8,
        /// Mobile OS platform of the app (for example, "Android" or "iOS").
        Platform = 9,
    }
    /// The metrics of the network report. Metrics are quantitative measurements
    /// indicating how the publisher business is performing. They are aggregated
    /// from the individual ad events and grouped by the report dimensions. The
    /// metric value is either integer, or decimal (without rounding).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Metric {
        /// Default value for an unset field. Do not use.
        Unspecified = 0,
        /// The number of ad requests. The value is an integer.
        ///
        /// **Warning:** The metric is incompatible with
        /// [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension.
        AdRequests = 1,
        /// The number of times a user clicks an ad. The value is an integer.
        Clicks = 2,
        /// The estimated earnings of the AdMob publisher. The currency unit (USD,
        /// EUR, or other) of the earning metrics are determined by the localization
        /// setting for currency. The amount is in micros. For example, $6.50 would
        /// be represented as 6500000.
        EstimatedEarnings = 3,
        /// The total number of ads shown to users. The value is an integer.
        Impressions = 4,
        /// The ratio of clicks over impressions. The value is a double precision
        /// (approximate) decimal value.
        ImpressionCtr = 5,
        /// The estimated earnings per thousand ad impressions. The value is in
        /// micros. For example, $1.03 would be represented as 1030000.
        ///
        /// **Warning:** The metric is incompatible with
        /// [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension.
        ImpressionRpm = 6,
        /// The number of times ads are returned in response to a request. The value
        /// is an integer.
        MatchedRequests = 7,
        /// The ratio of matched ad requests over the total ad requests. The value is
        /// a double precision (approximate) decimal value.
        ///
        /// **Warning:** The metric is incompatible with
        /// [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension.
        MatchRate = 8,
        /// The ratio of ads that are displayed over ads that are returned, defined
        /// as impressions / matched requests. The value is a double precision
        /// (approximate) decimal value.
        ShowRate = 9,
    }
}
/// The specification for generating an AdMob Mediation report.
/// For example, the specification to get observed ECPM sliced by ad source and
/// app for the 'US' and 'CN' countries can look like the following example:
///
///     {
///       "date_range": {
///         "start_date": {"year": 2018, "month": 9, "day": 1},
///         "end_date": {"year": 2018, "month": 9, "day": 30}
///       },
///       "dimensions": ["AD_SOURCE", "APP", "COUNTRY"],
///       "metrics": ["OBSERVED_ECPM"],
///       "dimension_filters": [
///         {
///           "dimension": "COUNTRY",
///           "matches_any": {"values": [{"value": "US", "value": "CN"}]}
///         }
///       ],
///       "sort_conditions": [
///         {"dimension":"APP", order: "ASCENDING"}
///       ],
///       "localization_settings": {
///         "currency_code": "USD",
///         "language_code": "en-US"
///       }
///     }
///
/// For a better understanding, you can treat the preceding specification like
/// the following pseudo SQL:
///
///     SELECT AD_SOURCE, APP, COUNTRY, OBSERVED_ECPM
///     FROM MEDIATION_REPORT
///     WHERE DATE >= '2018-09-01' AND DATE <= '2018-09-30'
///         AND COUNTRY IN ('US', 'CN')
///     GROUP BY AD_SOURCE, APP, COUNTRY
///     ORDER BY APP ASC;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediationReportSpec {
    /// The date range for which the report is generated.
    #[prost(message, optional, tag = "1")]
    pub date_range: ::core::option::Option<DateRange>,
    /// List of dimensions of the report. The value combination of these dimensions
    /// determines the row of the report. If no dimensions are specified, the
    /// report returns a single row of requested metrics for the entire account.
    #[prost(enumeration = "mediation_report_spec::Dimension", repeated, tag = "2")]
    pub dimensions: ::prost::alloc::vec::Vec<i32>,
    /// List of metrics of the report. A report must specify at least one metric.
    #[prost(enumeration = "mediation_report_spec::Metric", repeated, tag = "3")]
    pub metrics: ::prost::alloc::vec::Vec<i32>,
    /// Describes which report rows to match based on their dimension values.
    #[prost(message, repeated, tag = "4")]
    pub dimension_filters: ::prost::alloc::vec::Vec<mediation_report_spec::DimensionFilter>,
    /// Describes the sorting of report rows. The order of the condition in the
    /// list defines its precedence; the earlier the condition, the higher its
    /// precedence. If no sort conditions are specified, the row ordering is
    /// undefined.
    #[prost(message, repeated, tag = "5")]
    pub sort_conditions: ::prost::alloc::vec::Vec<mediation_report_spec::SortCondition>,
    /// Localization settings of the report.
    #[prost(message, optional, tag = "6")]
    pub localization_settings: ::core::option::Option<LocalizationSettings>,
    /// Maximum number of report data rows to return. If the value is not set, the
    /// API returns as many rows as possible, up to 100000. Acceptable values are
    /// 1-100000, inclusive. Any other values are treated as 100000.
    #[prost(int32, tag = "7")]
    pub max_report_rows: i32,
    /// A report time zone. Accepts an IANA TZ name values, such as
    /// "America/Los_Angeles."  If no time zone is defined, the account default
    /// takes effect. Check default value by the get account action.
    ///
    /// **Warning:** The "America/Los_Angeles" is the only supported value at
    /// the moment.
    #[prost(string, tag = "8")]
    pub time_zone: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MediationReportSpec`.
pub mod mediation_report_spec {
    /// Describes which report rows to match based on their dimension values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DimensionFilter {
        /// Applies the filter criterion to the specified dimension.
        #[prost(enumeration = "Dimension", tag = "1")]
        pub dimension: i32,
        /// Filter operator to be applied.
        #[prost(oneof = "dimension_filter::Operator", tags = "2")]
        pub operator: ::core::option::Option<dimension_filter::Operator>,
    }
    /// Nested message and enum types in `DimensionFilter`.
    pub mod dimension_filter {
        /// Filter operator to be applied.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Operator {
            /// Matches a row if its value for the specified dimension is in one of the
            /// values specified in this condition.
            #[prost(message, tag = "2")]
            MatchesAny(super::super::StringList),
        }
    }
    /// Sorting direction to be applied on a dimension or a metric.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SortCondition {
        /// Sorting order of the dimension or metric.
        #[prost(enumeration = "super::SortOrder", tag = "3")]
        pub order: i32,
        /// Identifies which values to sort on.
        #[prost(oneof = "sort_condition::SortOn", tags = "1, 2")]
        pub sort_on: ::core::option::Option<sort_condition::SortOn>,
    }
    /// Nested message and enum types in `SortCondition`.
    pub mod sort_condition {
        /// Identifies which values to sort on.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum SortOn {
            /// Sort by the specified dimension.
            #[prost(enumeration = "super::Dimension", tag = "1")]
            Dimension(i32),
            /// Sort by the specified metric.
            #[prost(enumeration = "super::Metric", tag = "2")]
            Metric(i32),
        }
    }
    /// The dimensions of the mediation report. Dimensions are data attributes to
    /// break down or refine the quantitative measurements (metrics) by certain
    /// attributes, such as the ad format or the platform an ad was viewed on.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Dimension {
        /// Default value for an unset field. Do not use.
        Unspecified = 0,
        /// A date in the YYYY-MM-DD format (for example, "2018-12-21"). Requests can
        /// specify at most one time dimension.
        Date = 1,
        /// A month in the YYYY-MM format (for example, "2018-12"). Requests can
        /// specify at most one time dimension.
        Month = 2,
        /// The date of the first day of a week in the YYYY-MM-DD format
        /// (for example, "2018-12-21"). Requests can specify at most one time
        /// dimension.
        Week = 3,
        /// The [unique ID of the ad source](/admob/api/v1/ad_sources) (for example,
        /// "5450213213286189855" and "AdMob Network" as label value).
        AdSource = 4,
        /// The unique ID of the ad source instance (for example,
        /// "ca-app-pub-1234#5678" and "AdMob (default)" as label value).
        AdSourceInstance = 5,
        /// The unique ID of the ad unit (for example, "ca-app-pub-1234/8790").
        /// If AD_UNIT dimension is specified, then APP is included automatically.
        AdUnit = 6,
        /// The unique ID of the mobile application (for example,
        /// "ca-app-pub-1234~1234").
        App = 7,
        /// The unique ID of the mediation group (for example,
        /// "ca-app-pub-1234:mg:1234" and "AdMob (default)" as label value).
        MediationGroup = 11,
        /// CLDR country code of the place where the ad views/clicks occur (for
        /// example, "US" or "FR"). This is a geography dimension.
        Country = 8,
        /// Format of the ad unit (for example, "banner", "native"), an ad delivery
        /// dimension.
        Format = 9,
        /// Mobile OS platform of the app (for example, "Android" or "iOS").
        Platform = 10,
    }
    /// The metrics of the mediation report. Metrics are quantitative measurements
    /// indicating how the publisher business is performing. They are aggregated
    /// from the individual ad events and grouped by the report dimensions. The
    /// metric value is either integer, or decimal (without rounding).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Metric {
        /// Default value for an unset field. Do not use.
        Unspecified = 0,
        /// The number of requests. The value is an integer.
        AdRequests = 1,
        /// The number of times a user clicks an ad. The value is an integer.
        Clicks = 2,
        /// The estimated earnings of the AdMob publisher. The currency unit (USD,
        /// EUR, or other) of the earning metrics are determined by the localization
        /// setting for currency. The amount is in micros. For example, $6.50 would
        /// be represented as 6500000.
        ///
        /// Estimated earnings per mediation group and per ad source instance level
        /// is supported dating back to October 20, 2019. Third-party estimated
        /// earnings will show 0 for dates prior to October 20, 2019.
        EstimatedEarnings = 3,
        /// The total number of ads shown to users. The value is an integer.
        Impressions = 4,
        /// The ratio of clicks over impressions. The value is a double precision
        /// (approximate) decimal value.
        ImpressionCtr = 5,
        /// The number of times ads are returned in response to a request. The value
        /// is an integer.
        MatchedRequests = 6,
        /// The ratio of matched ad requests over the total ad requests. The value is
        /// a double precision (approximate) decimal value.
        MatchRate = 7,
        /// The third-party ad network's estimated average eCPM. The currency unit
        /// (USD, EUR, or other) of the earning metrics are determined by the
        /// localization setting for currency. The amount is in micros. For example,
        /// $2.30 would be represented as 2300000.
        ///
        /// The estimated average eCPM per mediation group and per ad source instance
        /// level is supported dating back to October 20, 2019. Third-party estimated
        /// average eCPM will show 0 for dates prior to October 20, 2019.
        ObservedEcpm = 8,
    }
}
/// A row of the returning report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportRow {
    /// Map of dimension values in a row, with keys as enum name of the dimensions.
    #[prost(map = "string, message", tag = "1")]
    pub dimension_values:
        ::std::collections::HashMap<::prost::alloc::string::String, report_row::DimensionValue>,
    /// Map of metric values in a row, with keys as enum name of the metrics. If
    /// a metric being requested has no value returned, the map will not include
    /// it.
    #[prost(map = "string, message", tag = "2")]
    pub metric_values:
        ::std::collections::HashMap<::prost::alloc::string::String, report_row::MetricValue>,
}
/// Nested message and enum types in `ReportRow`.
pub mod report_row {
    /// Representation of a dimension value.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DimensionValue {
        /// Dimension value in the format specified in the report's spec Dimension
        /// enum.
        #[prost(string, tag = "1")]
        pub value: ::prost::alloc::string::String,
        /// The localized string representation of the value. If unspecified, the
        /// display label should be derived from the value.
        #[prost(string, tag = "2")]
        pub display_label: ::prost::alloc::string::String,
    }
    /// Representation of a metric value.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetricValue {
        /// Metric value in the format specified in the report's spec Metric enum
        /// name.
        #[prost(oneof = "metric_value::Value", tags = "1, 2, 3")]
        pub value: ::core::option::Option<metric_value::Value>,
    }
    /// Nested message and enum types in `MetricValue`.
    pub mod metric_value {
        /// Metric value in the format specified in the report's spec Metric enum
        /// name.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            /// Metric integer value.
            #[prost(int64, tag = "1")]
            IntegerValue(i64),
            /// Double precision (approximate) decimal values. Rates are from 0 to 1.
            #[prost(double, tag = "2")]
            DoubleValue(f64),
            /// Amount in micros. One million is equivalent to one unit. Currency value
            /// is in the unit (USD, EUR or other) specified by the request.
            /// For example, $6.50 whould be represented as 6500000 micros.
            #[prost(int64, tag = "3")]
            MicrosValue(i64),
        }
    }
}
/// Warnings associated with generation of the report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportWarning {
    /// Type of the warning.
    #[prost(enumeration = "report_warning::Type", tag = "1")]
    pub r#type: i32,
    /// Describes the details of the warning message, in English.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ReportWarning`.
pub mod report_warning {
    /// Warning type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default value for an unset field. Do not use.
        Unspecified = 0,
        /// Some data in this report is aggregated based on a time zone different
        /// from the requested time zone. This could happen if a local time-zone
        /// report has the start time before the last time this time zone changed.
        /// The description field will contain the date of the last time zone
        /// change.
        DataBeforeAccountTimezoneChange = 1,
        /// There is an unusual delay in processing the source data for the
        /// requested date range. The report results might be less up to date than
        /// usual. AdMob is aware of the issue and is actively working to resolve
        /// it.
        DataDelayed = 2,
        /// Warnings that are exposed without a specific type. Useful when new
        /// warning types are added but the API is not changed yet.
        Other = 3,
        /// The currency being requested is not the account currency. The earning
        /// metrics will be based on the requested currency, and thus not a good
        /// estimation of the final payment anymore, due to the currency rate
        /// fluctuation.
        ReportCurrencyNotAccountCurrency = 4,
    }
}
/// Groups data helps to treat the generated report. Always sent as a first
/// message in the stream response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportHeader {
    /// The date range for which the report is generated. This is identical to the
    /// range specified in the report request.
    #[prost(message, optional, tag = "1")]
    pub date_range: ::core::option::Option<DateRange>,
    /// Localization settings of the report. This is identical to the settings
    /// in the report request.
    #[prost(message, optional, tag = "2")]
    pub localization_settings: ::core::option::Option<LocalizationSettings>,
    /// The report time zone. The value is a time-zone ID as specified by the CLDR
    /// project, for example, "America/Los_Angeles".
    #[prost(string, tag = "3")]
    pub reporting_time_zone: ::prost::alloc::string::String,
}
/// Groups data available after report generation, for example, warnings and row
/// counts. Always sent as the last message in the stream response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportFooter {
    /// Warnings associated with generation of the report.
    #[prost(message, repeated, tag = "1")]
    pub warnings: ::prost::alloc::vec::Vec<ReportWarning>,
    /// Total number of rows that matched the request.
    ///
    /// Warning: This count does NOT always match the number of rows in the
    /// response. Do not make that assumption when processing the response.
    #[prost(int64, tag = "2")]
    pub matching_row_count: i64,
}
/// Specification of a single date range. Both dates are inclusive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateRange {
    /// Start date of the date range, inclusive. Must be less than or equal to the
    /// end date.
    #[prost(message, optional, tag = "1")]
    pub start_date: ::core::option::Option<super::super::super::r#type::Date>,
    /// End date of the date range, inclusive. Must be greater than or equal to the
    /// start date.
    #[prost(message, optional, tag = "2")]
    pub end_date: ::core::option::Option<super::super::super::r#type::Date>,
}
/// Localization settings for reports, such as currency and language. It affects
/// how metrics are calculated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalizationSettings {
    /// Currency code of the earning related metrics, which is the 3-letter code
    /// defined in ISO 4217. The daily average rate is used for the currency
    /// conversion. Defaults to the account currency code if unspecified.
    #[prost(string, tag = "1")]
    pub currency_code: ::prost::alloc::string::String,
    /// Language used for any localized text, such as some dimension value display
    /// labels. The language tag defined in the IETF BCP47. Defaults to 'en-US' if
    /// unspecified.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
}
/// List of string values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringList {
    /// The string values.
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The sorting order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SortOrder {
    /// Default value for an unset field. Do not use.
    Unspecified = 0,
    /// Sort dimension value or metric value in ascending order.
    Ascending = 1,
    /// Sort dimension value or metric value in descending order.
    Descending = 2,
}
/// Request to retrieve the specified publisher account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPublisherAccountRequest {
    /// Resource name of the publisher account to retrieve.
    /// Example: accounts/pub-9876543210987654
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to retrieve the AdMob publisher account accessible with the client
/// credential. Currently all credentials have access to at most 1 account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPublisherAccountsRequest {
    /// Maximum number of accounts to return.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// The value returned by the last `ListPublisherAccountsResponse`; indicates
    /// that this is a continuation of a prior `ListPublisherAccounts` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for the publisher account list request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPublisherAccountsResponse {
    /// Publisher that the client credentials can access.
    #[prost(message, repeated, tag = "1")]
    pub account: ::prost::alloc::vec::Vec<PublisherAccount>,
    /// If not empty, indicates that there might be more accounts for the request;
    /// you must pass this value in a new `ListPublisherAccountsRequest`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to generate an AdMob Mediation report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateMediationReportRequest {
    /// Resource name of the account to generate the report for.
    /// Example: accounts/pub-9876543210987654
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Network report specification.
    #[prost(message, optional, tag = "2")]
    pub report_spec: ::core::option::Option<MediationReportSpec>,
}
/// The streaming response for the AdMob Mediation report where the first
/// response contains the report header, then a stream of row responses, and
/// finally a footer as the last response message.
///
/// For example:
///
///     [{
///       "header": {
///         "date_range": {
///           "start_date": {"year": 2018, "month": 9, "day": 1},
///           "end_date": {"year": 2018, "month": 9, "day": 1}
///         },
///         "localization_settings": {
///           "currency_code": "USD",
///           "language_code": "en-US"
///         }
///       }
///     },
///     {
///       "row": {
///         "dimension_values": {
///           "DATE": {"value": "20180918"},
///           "APP": {
///             "value": "ca-app-pub-8123415297019784~1001342552",
///              "display_label": "My app name!"
///           }
///         },
///         "metric_values": {
///           "ESTIMATED_EARNINGS": {"decimal_value": "1324746"}
///         }
///       }
///     },
///     {
///       "footer": {"matching_row_count": 1}
///     }]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateMediationReportResponse {
    /// Each stream response message contains one type of payload.
    #[prost(
        oneof = "generate_mediation_report_response::Payload",
        tags = "1, 2, 3"
    )]
    pub payload: ::core::option::Option<generate_mediation_report_response::Payload>,
}
/// Nested message and enum types in `GenerateMediationReportResponse`.
pub mod generate_mediation_report_response {
    /// Each stream response message contains one type of payload.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Report generation settings that describes the report contents, such as
        /// the report date range and localization settings.
        #[prost(message, tag = "1")]
        Header(super::ReportHeader),
        /// Actual report data.
        #[prost(message, tag = "2")]
        Row(super::ReportRow),
        /// Additional information about the generated report, such as warnings about
        /// the data.
        #[prost(message, tag = "3")]
        Footer(super::ReportFooter),
    }
}
/// Request to generate an AdMob Network report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateNetworkReportRequest {
    /// Resource name of the account to generate the report for.
    /// Example: accounts/pub-9876543210987654
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Network report specification.
    #[prost(message, optional, tag = "2")]
    pub report_spec: ::core::option::Option<NetworkReportSpec>,
}
/// The streaming response for the AdMob Network report where the first response
/// contains the report header, then a stream of row responses, and finally a
/// footer as the last response message.
///
/// For example:
///
///     [{
///       "header": {
///         "dateRange": {
///           "startDate": {"year": 2018, "month": 9, "day": 1},
///           "endDate": {"year": 2018, "month": 9, "day": 1}
///         },
///         "localizationSettings": {
///           "currencyCode": "USD",
///           "languageCode": "en-US"
///         }
///       }
///     },
///     {
///       "row": {
///         "dimensionValues": {
///           "DATE": {"value": "20180918"},
///           "APP": {
///             "value": "ca-app-pub-8123415297019784~1001342552",
///              displayLabel: "My app name!"
///           }
///         },
///         "metricValues": {
///           "ESTIMATED_EARNINGS": {"microsValue": 6500000}
///         }
///       }
///     },
///     {
///       "footer": {"matchingRowCount": 1}
///     }]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateNetworkReportResponse {
    /// Each stream response message contains one type of payload.
    #[prost(oneof = "generate_network_report_response::Payload", tags = "1, 2, 3")]
    pub payload: ::core::option::Option<generate_network_report_response::Payload>,
}
/// Nested message and enum types in `GenerateNetworkReportResponse`.
pub mod generate_network_report_response {
    /// Each stream response message contains one type of payload.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Report generation settings that describes the report contents, such as
        /// the report date range and localization settings.
        #[prost(message, tag = "1")]
        Header(super::ReportHeader),
        /// Actual report data.
        #[prost(message, tag = "2")]
        Row(super::ReportRow),
        /// Additional information about the generated report, such as warnings about
        /// the data.
        #[prost(message, tag = "3")]
        Footer(super::ReportFooter),
    }
}
#[doc = r" Generated client implementations."]
pub mod ad_mob_api_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The AdMob API allows AdMob publishers programmatically get information about"]
    #[doc = " their AdMob account."]
    pub struct AdMobApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdMobApiClient<T>
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
        #[doc = " Gets information about the specified AdMob publisher account."]
        pub async fn get_publisher_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPublisherAccountRequest>,
        ) -> Result<tonic::Response<super::PublisherAccount>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.admob.v1.AdMobApi/GetPublisherAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the AdMob publisher account accessible with the client credential."]
        #[doc = " Currently, all credentials have access to at most one AdMob account."]
        pub async fn list_publisher_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPublisherAccountsRequest>,
        ) -> Result<tonic::Response<super::ListPublisherAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.admob.v1.AdMobApi/ListPublisherAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generates an AdMob Network report based on the provided report"]
        #[doc = " specification."]
        pub async fn generate_network_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateNetworkReportRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GenerateNetworkReportResponse>>,
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
                "/google.ads.admob.v1.AdMobApi/GenerateNetworkReport",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Generates an AdMob Mediation report based on the provided report"]
        #[doc = " specification."]
        pub async fn generate_mediation_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateMediationReportRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GenerateMediationReportResponse>>,
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
                "/google.ads.admob.v1.AdMobApi/GenerateMediationReport",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for AdMobApiClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdMobApiClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdMobApiClient {{ ... }}")
        }
    }
}
