///  A data exchange is a container that lets you share data. Along with the
///  descriptive information about the data exchange, it contains listings that
///  reference shared datasets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataExchange {
    ///  Output only. The resource name of the data exchange.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Required. Human-readable display name of the data exchange. The display name must
    ///  contain only Unicode letters, numbers (0-9), underscores (_), dashes (-),
    ///  spaces ( ), ampersands (&) and must not start or end with spaces.
    ///  Default value is an empty string.
    ///  Max length: 63 bytes.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    ///  Optional. Description of the data exchange. The description must not contain Unicode
    ///  non-characters as well as C0 and C1 control codes except tabs (HT),
    ///  new lines (LF), carriage returns (CR), and page breaks (FF).
    ///  Default value is an empty string.
    ///  Max length: 2000 bytes.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    ///  Optional. Email or URL of the primary point of contact of the data exchange.
    ///  Max Length: 1000 bytes.
    #[prost(string, tag="4")]
    pub primary_contact: ::prost::alloc::string::String,
    ///  Optional. Documentation describing the data exchange.
    #[prost(string, tag="5")]
    pub documentation: ::prost::alloc::string::String,
    ///  Output only. Number of listings contained in the data exchange.
    #[prost(int32, tag="6")]
    pub listing_count: i32,
    ///  Optional. Base64 encoded image representing the data exchange. Max Size: 3.0MiB
    ///  Expected image dimensions are 512x512 pixels, however the API only
    ///  performs validation on size of the encoded data.
    ///  Note: For byte fields, the content of the fields are base64-encoded (which
    ///  increases the size of the data by 33-36%) when using JSON on the wire.
    #[prost(bytes="vec", tag="7")]
    pub icon: ::prost::alloc::vec::Vec<u8>,
}
///  Contains details of the data provider.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataProvider {
    ///  Optional. Name of the data provider.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Optional. Email or URL of the data provider.
    ///  Max Length: 1000 bytes.
    #[prost(string, tag="2")]
    pub primary_contact: ::prost::alloc::string::String,
}
///  Contains details of the listing publisher.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Publisher {
    ///  Optional. Name of the listing publisher.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Optional. Email or URL of the listing publisher.
    ///  Max Length: 1000 bytes.
    #[prost(string, tag="2")]
    pub primary_contact: ::prost::alloc::string::String,
}
///  Contains the reference that identifies a destination bigquery dataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationDatasetReference {
    ///  Required. A unique ID for this dataset, without the project name. The ID
    ///  must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_).
    ///  The maximum length is 1,024 characters.
    #[prost(string, tag="1")]
    pub dataset_id: ::prost::alloc::string::String,
    ///  Required. The ID of the project containing this dataset.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
}
///  Defines the destination bigquery dataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationDataset {
    ///  Required. A reference that identifies the destination dataset.
    #[prost(message, optional, tag="1")]
    pub dataset_reference: ::core::option::Option<DestinationDatasetReference>,
    ///  Optional. A descriptive name for the dataset.
    #[prost(message, optional, tag="2")]
    pub friendly_name: ::core::option::Option<::prost::alloc::string::String>,
    ///  Optional. A user-friendly description of the dataset.
    #[prost(message, optional, tag="3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    ///  Optional. The labels associated with this dataset. You can use these
    ///  to organize and group your datasets.
    ///  You can set this property when inserting or updating a dataset.
    ///  See <https://cloud.google.com/resource-manager/docs/creating-managing-labels>
    ///  for more information.
    #[prost(map="string, string", tag="4")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  Required. The geographic location where the dataset should reside. See
    ///  <https://cloud.google.com/bigquery/docs/locations> for supported
    ///  locations.
    #[prost(string, tag="5")]
    pub location: ::prost::alloc::string::String,
}
///  A listing is what gets published into a data exchange that a subscriber can
///  subscribe to. It contains a reference to the data source along with
///  descriptive information that will help subscribers find and subscribe the
///  data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Listing {
    ///  Output only. The resource name of the listing.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123/listings/456`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Required. Human-readable display name of the listing. The display name must contain
    ///  only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces
    ///  ( ), ampersands (&) and can't start or end with spaces.
    ///  Default value is an empty string.
    ///  Max length: 63 bytes.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    ///  Optional. Short description of the listing. The description must not contain
    ///  Unicode non-characters and C0 and C1 control codes except tabs (HT),
    ///  new lines (LF), carriage returns (CR), and page breaks (FF).
    ///  Default value is an empty string.
    ///  Max length: 2000 bytes.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    ///  Optional. Email or URL of the primary point of contact of the listing.
    ///  Max Length: 1000 bytes.
    #[prost(string, tag="4")]
    pub primary_contact: ::prost::alloc::string::String,
    ///  Optional. Documentation describing the listing.
    #[prost(string, tag="5")]
    pub documentation: ::prost::alloc::string::String,
    ///  Output only. Current state of the listing.
    #[prost(enumeration="listing::State", tag="7")]
    pub state: i32,
    ///  Optional. Base64 encoded image representing the listing. Max Size: 3.0MiB
    ///  Expected image dimensions are 512x512 pixels, however the API only
    ///  performs validation on size of the encoded data.
    ///  Note: For byte fields, the contents of the field are base64-encoded (which
    ///  increases the size of the data by 33-36%) when using JSON on the wire.
    #[prost(bytes="vec", tag="8")]
    pub icon: ::prost::alloc::vec::Vec<u8>,
    ///  Optional. Details of the data provider who owns the source data.
    #[prost(message, optional, tag="9")]
    pub data_provider: ::core::option::Option<DataProvider>,
    ///  Optional. Categories of the listing. Up to two categories are allowed.
    #[prost(enumeration="listing::Category", repeated, packed="false", tag="10")]
    pub categories: ::prost::alloc::vec::Vec<i32>,
    ///  Optional. Details of the publisher who owns the listing and who can share
    ///  the source data.
    #[prost(message, optional, tag="11")]
    pub publisher: ::core::option::Option<Publisher>,
    ///  Optional. Email or URL of the request access of the listing.
    ///  Subscribers can use this reference to request access.
    ///  Max Length: 1000 bytes.
    #[prost(string, tag="12")]
    pub request_access: ::prost::alloc::string::String,
    ///  Listing source.
    #[prost(oneof="listing::Source", tags="6")]
    pub source: ::core::option::Option<listing::Source>,
}
/// Nested message and enum types in `Listing`.
pub mod listing {
    ///  A reference to a shared dataset. It is an existing BigQuery dataset with a
    ///  collection of objects such as tables and views that you want to share
    ///  with subscribers.
    ///  When subscriber's subscribe to a listing, Analytics Hub creates a linked
    ///  dataset in
    ///  the subscriber's project. A Linked dataset is an opaque, read-only BigQuery
    ///  dataset that serves as a _symbolic link_ to a shared dataset.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQueryDatasetSource {
        ///  Resource name of the dataset source for this listing.
        ///  e.g. `projects/myproject/datasets/123`
        #[prost(string, tag="1")]
        pub dataset: ::prost::alloc::string::String,
    }
    ///  State of the listing.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        ///  Default value. This value is unused.
        Unspecified = 0,
        ///  Subscribable state. Users with dataexchange.listings.subscribe permission
        ///  can subscribe to this listing.
        Active = 1,
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
            }
        }
    }
    ///  Listing categories.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Category {
        Unspecified = 0,
        Others = 1,
        AdvertisingAndMarketing = 2,
        Commerce = 3,
        ClimateAndEnvironment = 4,
        Demographics = 5,
        Economics = 6,
        Education = 7,
        Energy = 8,
        Financial = 9,
        Gaming = 10,
        Geospatial = 11,
        HealthcareAndLifeScience = 12,
        Media = 13,
        PublicSector = 14,
        Retail = 15,
        Sports = 16,
        ScienceAndResearch = 17,
        TransportationAndLogistics = 18,
        TravelAndTourism = 19,
    }
    impl Category {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Category::Unspecified => "CATEGORY_UNSPECIFIED",
                Category::Others => "CATEGORY_OTHERS",
                Category::AdvertisingAndMarketing => "CATEGORY_ADVERTISING_AND_MARKETING",
                Category::Commerce => "CATEGORY_COMMERCE",
                Category::ClimateAndEnvironment => "CATEGORY_CLIMATE_AND_ENVIRONMENT",
                Category::Demographics => "CATEGORY_DEMOGRAPHICS",
                Category::Economics => "CATEGORY_ECONOMICS",
                Category::Education => "CATEGORY_EDUCATION",
                Category::Energy => "CATEGORY_ENERGY",
                Category::Financial => "CATEGORY_FINANCIAL",
                Category::Gaming => "CATEGORY_GAMING",
                Category::Geospatial => "CATEGORY_GEOSPATIAL",
                Category::HealthcareAndLifeScience => "CATEGORY_HEALTHCARE_AND_LIFE_SCIENCE",
                Category::Media => "CATEGORY_MEDIA",
                Category::PublicSector => "CATEGORY_PUBLIC_SECTOR",
                Category::Retail => "CATEGORY_RETAIL",
                Category::Sports => "CATEGORY_SPORTS",
                Category::ScienceAndResearch => "CATEGORY_SCIENCE_AND_RESEARCH",
                Category::TransportationAndLogistics => "CATEGORY_TRANSPORTATION_AND_LOGISTICS",
                Category::TravelAndTourism => "CATEGORY_TRAVEL_AND_TOURISM",
            }
        }
    }
    ///  Listing source.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        ///  Required. Shared dataset i.e. BigQuery dataset source.
        #[prost(message, tag="6")]
        BigqueryDataset(BigQueryDatasetSource),
    }
}
///  Message for requesting the list of data exchanges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataExchangesRequest {
    ///  Required. The parent resource path of the data exchanges.
    ///  e.g. `projects/myproject/locations/US`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The maximum number of results to return in a single response page. Leverage
    ///  the page tokens to iterate through the entire collection.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  Page token, returned by a previous call, to request the next page of
    ///  results.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
///  Message for response to the list of data exchanges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataExchangesResponse {
    ///  The list of data exchanges.
    #[prost(message, repeated, tag="1")]
    pub data_exchanges: ::prost::alloc::vec::Vec<DataExchange>,
    ///  A token to request the next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Message for requesting the list of data exchanges from projects in an
///  organization and location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgDataExchangesRequest {
    ///  Required. The organization resource path of the projects containing DataExchanges.
    ///  e.g. `organizations/myorg/locations/US`.
    #[prost(string, tag="1")]
    pub organization: ::prost::alloc::string::String,
    ///  The maximum number of results to return in a single response page. Leverage
    ///  the page tokens to iterate through the entire collection.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  Page token, returned by a previous call, to request the next page of
    ///  results.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
///  Message for response to listing data exchanges in an organization and
///  location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgDataExchangesResponse {
    ///  The list of data exchanges.
    #[prost(message, repeated, tag="1")]
    pub data_exchanges: ::prost::alloc::vec::Vec<DataExchange>,
    ///  A token to request the next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Message for getting a data exchange.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataExchangeRequest {
    ///  Required. The resource name of the data exchange.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Message for creating a data exchange.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataExchangeRequest {
    ///  Required. The parent resource path of the data exchange.
    ///  e.g. `projects/myproject/locations/US`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The ID of the data exchange.
    ///  Must contain only Unicode letters, numbers (0-9), underscores (_).
    ///  Should not use characters that require URL-escaping, or characters
    ///  outside of ASCII, spaces.
    ///  Max length: 100 bytes.
    #[prost(string, tag="2")]
    pub data_exchange_id: ::prost::alloc::string::String,
    ///  Required. The data exchange to create.
    #[prost(message, optional, tag="3")]
    pub data_exchange: ::core::option::Option<DataExchange>,
}
///  Message for updating a data exchange.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataExchangeRequest {
    ///  Required. Field mask specifies the fields to update in the data exchange
    ///  resource. The fields specified in the
    ///  `updateMask` are relative to the resource and are not a full request.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    ///  Required. The data exchange to update.
    #[prost(message, optional, tag="2")]
    pub data_exchange: ::core::option::Option<DataExchange>,
}
///  Message for deleting a data exchange.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataExchangeRequest {
    ///  Required. The full name of the data exchange resource that you want to delete.
    ///  For example, `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Message for requesting the list of listings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListListingsRequest {
    ///  Required. The parent resource path of the listing.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The maximum number of results to return in a single response page. Leverage
    ///  the page tokens to iterate through the entire collection.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  Page token, returned by a previous call, to request the next page of
    ///  results.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
///  Message for response to the list of Listings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListListingsResponse {
    ///  The list of Listing.
    #[prost(message, repeated, tag="1")]
    pub listings: ::prost::alloc::vec::Vec<Listing>,
    ///  A token to request the next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Message for getting a listing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetListingRequest {
    ///  Required. The resource name of the listing.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123/listings/456`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Message for creating a listing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateListingRequest {
    ///  Required. The parent resource path of the listing.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The ID of the listing to create.
    ///  Must contain only Unicode letters, numbers (0-9), underscores (_).
    ///  Should not use characters that require URL-escaping, or characters
    ///  outside of ASCII, spaces.
    ///  Max length: 100 bytes.
    #[prost(string, tag="2")]
    pub listing_id: ::prost::alloc::string::String,
    ///  Required. The listing to create.
    #[prost(message, optional, tag="3")]
    pub listing: ::core::option::Option<Listing>,
}
///  Message for updating a Listing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateListingRequest {
    ///  Required. Field mask specifies the fields to update in the listing resource. The
    ///  fields specified in the `updateMask` are relative to the resource and are
    ///  not a full request.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    ///  Required. The listing to update.
    #[prost(message, optional, tag="2")]
    pub listing: ::core::option::Option<Listing>,
}
///  Message for deleting a listing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteListingRequest {
    ///  Required. Resource name of the listing to delete.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123/listings/456`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Message for subscribing to a listing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeListingRequest {
    ///  Required. Resource name of the listing that you want to subscribe to.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123/listings/456`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Resulting destination of the listing that you subscribed to.
    #[prost(oneof="subscribe_listing_request::Destination", tags="3")]
    pub destination: ::core::option::Option<subscribe_listing_request::Destination>,
}
/// Nested message and enum types in `SubscribeListingRequest`.
pub mod subscribe_listing_request {
    ///  Resulting destination of the listing that you subscribed to.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        ///  BigQuery destination dataset to create for the subscriber.
        #[prost(message, tag="3")]
        DestinationDataset(super::DestinationDataset),
    }
}
///  Message for response when you subscribe to a listing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeListingResponse {
}
/// Generated client implementations.
pub mod analytics_hub_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The `AnalyticsHubService` API facilitates data sharing within and across
    /// organizations. It allows data providers to publish listings that reference
    /// shared datasets. With Analytics Hub, users can discover and search for
    /// listings that they have access to. Subscribers can view and subscribe to
    /// listings. When you subscribe to a listing, Analytics Hub creates a linked
    /// dataset in your project.
    #[derive(Debug, Clone)]
    pub struct AnalyticsHubServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AnalyticsHubServiceClient<tonic::transport::Channel> {
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
    impl<T> AnalyticsHubServiceClient<T>
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
        ) -> AnalyticsHubServiceClient<InterceptedService<T, F>>
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
            AnalyticsHubServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all data exchanges in a given project and location.
        pub async fn list_data_exchanges(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDataExchangesRequest>,
        ) -> Result<tonic::Response<super::ListDataExchangesResponse>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/ListDataExchanges",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all data exchanges from projects in a given organization and
        /// location.
        pub async fn list_org_data_exchanges(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrgDataExchangesRequest>,
        ) -> Result<
            tonic::Response<super::ListOrgDataExchangesResponse>,
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/ListOrgDataExchanges",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the details of a data exchange.
        pub async fn get_data_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataExchangeRequest>,
        ) -> Result<tonic::Response<super::DataExchange>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/GetDataExchange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new data exchange.
        pub async fn create_data_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDataExchangeRequest>,
        ) -> Result<tonic::Response<super::DataExchange>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/CreateDataExchange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an existing data exchange.
        pub async fn update_data_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDataExchangeRequest>,
        ) -> Result<tonic::Response<super::DataExchange>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/UpdateDataExchange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an existing data exchange.
        pub async fn delete_data_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDataExchangeRequest>,
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/DeleteDataExchange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all listings in a given project and location.
        pub async fn list_listings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListListingsRequest>,
        ) -> Result<tonic::Response<super::ListListingsResponse>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/ListListings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the details of a listing.
        pub async fn get_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::GetListingRequest>,
        ) -> Result<tonic::Response<super::Listing>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/GetListing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new listing.
        pub async fn create_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateListingRequest>,
        ) -> Result<tonic::Response<super::Listing>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/CreateListing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an existing listing.
        pub async fn update_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateListingRequest>,
        ) -> Result<tonic::Response<super::Listing>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/UpdateListing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a listing.
        pub async fn delete_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteListingRequest>,
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/DeleteListing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Subscribes to a listing.
        ///
        /// Currently, with Analytics Hub, you can create listings that
        /// reference only BigQuery datasets.
        /// Upon subscription to a listing for a BigQuery dataset, Analytics Hub
        /// creates a linked dataset in the subscriber's project.
        pub async fn subscribe_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeListingRequest>,
        ) -> Result<tonic::Response<super::SubscribeListingResponse>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/SubscribeListing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the IAM policy.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the IAM policy.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the permissions that a caller has.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}