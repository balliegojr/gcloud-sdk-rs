use crate::source::{BoxSource, Token};
use chrono::prelude::*;
use std::convert::TryFrom;
use tonic::metadata::Ascii;
use tonic::service::Interceptor;
use tonic::{Request, Status};

#[derive(Clone)]
pub struct GoogleConnectorInterceptor {
    token: Token,
    token_value_meta: tonic::metadata::MetadataValue<Ascii>,
    cloud_resource_prefix_meta: Option<tonic::metadata::MetadataValue<Ascii>>,
}

impl GoogleConnectorInterceptor {
    pub fn init_google_services_channel_tls_config(
        domain_name: String,
    ) -> tonic::transport::ClientTlsConfig {
        tonic::transport::ClientTlsConfig::new()
            .ca_certificate(tonic::transport::Certificate::from_pem(
                crate::apis::CERTIFICATES,
            ))
            .domain_name(domain_name)
    }

    pub async fn init_google_services_channel(
        api_url: &'static str,
        google_services_tls_config: &tonic::transport::ClientTlsConfig,
    ) -> Result<tonic::transport::Channel, crate::error::Error> {
        Ok(tonic::transport::Channel::from_static(api_url)
            .tls_config(google_services_tls_config.clone())?
            .connect()
            .await?)
    }

    pub async fn new(
        token_source: &BoxSource,
        cloud_resource_prefix: Option<String>,
    ) -> crate::error::Result<Self> {
        let token = token_source.token().await?;

        let cloud_resource_prefix_meta =
            cloud_resource_prefix.and_then(|cloud_resource_prefix_value| {
                tonic::metadata::MetadataValue::try_from(cloud_resource_prefix_value.as_str()).ok()
            });

        let token_value_meta = tonic::metadata::MetadataValue::try_from(token.header_value())?;

        Ok(Self {
            token,
            token_value_meta,
            cloud_resource_prefix_meta,
        })
    }

    pub fn expiry_date(&self) -> DateTime<Utc> {
        self.token.expiry
    }

    fn google_interceptor_fn(
        &self,
        mut req: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        let metadata = req.metadata_mut();
        metadata.insert("authorization", self.token_value_meta.clone());
        if let Some(ref cloud_resource_prefix_meta) = self.cloud_resource_prefix_meta {
            metadata.insert(
                "google-cloud-resource-prefix",
                cloud_resource_prefix_meta.clone(),
            );
        }
        Ok(req)
    }
}

impl Interceptor for GoogleConnectorInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        self.google_interceptor_fn(request)
    }
}