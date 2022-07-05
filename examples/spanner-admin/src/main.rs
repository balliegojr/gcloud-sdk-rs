use gcloud_sdk::google::spanner::admin::database::v1::{
    database_admin_client::DatabaseAdminClient, ListDatabasesRequest,
};
use gcloud_sdk::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project = std::env::var("PROJECT")?;
    let instance = std::env::var("INSTANCE")?;

    let spanner_client: GoogleApiClientFn<DatabaseAdminClient<GoogleConnectorInterceptedService>> = GoogleApiClient::from_function(
        |channel, interceptor | DatabaseAdminClient::with_interceptor(channel, interceptor),
        "https://spanner.googleapis.com",
        chrono::Duration::minutes(15),
        None,
    )
        .await?;

    let response = spanner_client
        .get()
        .await?
        .list_databases(tonic::Request::new(ListDatabasesRequest {
            parent: format!("projects/{}/instances/{}", project, instance),
            page_size: 100,
            ..Default::default()
        }))
        .await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
