use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// SslCertsCreateEphemeralRequest : SslCerts create ephemeral certificate request.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SslCertsCreateEphemeralRequest {
    /// Access token to include in the signed certificate.
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// PEM encoded public key to include in the signed certificate.
    #[serde(rename = "public_key", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

impl SslCertsCreateEphemeralRequest {
    /// SslCerts create ephemeral certificate request.
    pub fn new() -> SslCertsCreateEphemeralRequest {
        SslCertsCreateEphemeralRequest {
            access_token: None,
            public_key: None,
        }
    }
}
