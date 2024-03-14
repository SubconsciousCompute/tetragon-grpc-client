//! Mimics `tetra --server-address unix:///var/run/tetragon/tetragon.sock getevents`
//!
//! Author: Aditeya V. Govind <aditeya.vg@subcom.tech>

use tokio::net::UnixStream;
use tonic::transport::{Channel, Endpoint, Uri};
use tower::service_fn;
use tracing_subscriber::FmtSubscriber;

use tetragon_grpc::fine_guidance_sensors_client::FineGuidanceSensorsClient;
use tetragon_grpc::{Filter, GetEventsRequest};

/// Http client
pub async fn http_client(
    uri: &'static str,
) -> Result<FineGuidanceSensorsClient<Channel>, Box<dyn std::error::Error>> {
    tracing::info!("Creating HTTP channel");
    let channel = Endpoint::try_from(uri)?;

    tracing::info!("Creating Client");
    let client = FineGuidanceSensorsClient::connect(channel).await?;
    Ok(client)
}

/// Unix domain socket client
pub async fn socket_client(
    path: &'static str,
) -> Result<FineGuidanceSensorsClient<Channel>, Box<dyn std::error::Error>> {
    tracing::info!("Creating socket channel");
    // NOTE: We will ignore this uri because uds do not use it
    // if your connector does use the uri it will be provided
    // as the request to the `MakeConnection`.
    // The url is the default address used for tetragon gRPC.
    let channel = Endpoint::try_from("http://0.0.0.0:54321")?
        .connect_with_connector(service_fn(move |_: Uri| {
            // Connect to a Uds socket
            UnixStream::connect(path)
        }))
        .await?;

    tracing::info!("Creating socket client");
    let client = FineGuidanceSensorsClient::new(channel);
    Ok(client)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(my_subscriber).expect("setting tracing default failed");

    // NOTE: get root
    sudo::escalate_if_needed()?;

    // NOTE: client via http
    // let mut client = socket_client("http://0.0.0.0:54321").await?;
    // NOTE: client via socket
    let mut client = socket_client("/var/run/tetragon/tetragon.sock").await?;

    tracing::info!("Creating Request");
    // NOTE: creating a request for getevents
    let request = tonic::Request::new(GetEventsRequest {
        allow_list: vec![Filter::default()],
        ..Default::default()
    });

    tracing::info!("Sending Request");
    // NOTE: running the request
    let mut response = client.get_events(request).await?;

    tracing::info!("Success! Printing Response:");
    // NOTE: reading the stream for events
    while let Ok(Some(t)) = response.get_mut().message().await {
        tracing::info!("{t:?}");
    }

    Ok(())
}
