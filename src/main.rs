use tokio::net::UnixStream;
use tonic::transport::{Channel, Endpoint, Uri};
use tower::service_fn;
use tracing_subscriber::FmtSubscriber;

use crate::tetragon::fine_guidance_sensors_client::FineGuidanceSensorsClient;
use crate::tetragon::{AggregationOptions, GetEventsRequest, GetEventsResponse};

pub mod tetragon {
    tonic::include_proto!("tetragon");
}

pub async fn http_client() -> Result<FineGuidanceSensorsClient<Channel>, Box<dyn std::error::Error>>
{
    tracing::info!("Creating Channel");
    let channel = Endpoint::try_from("http://0.0.0.0:54321")?;

    tracing::info!("Done\nCreating Client");
    let client = FineGuidanceSensorsClient::connect(channel).await?;
    Ok(client)
}

pub async fn socket_client(
) -> Result<FineGuidanceSensorsClient<Channel>, Box<dyn std::error::Error>> {
    tracing::info!("Creating Channel");
    // NOTE: We will ignore this uri because uds do not use it
    // if your connector does use the uri it will be provided
    // as the request to the `MakeConnection`.
    // The url is the default address used for tetragon gRPC.
    let channel = Endpoint::try_from("http://0.0.0.0:54321")?
        .connect_with_connector(service_fn(|_: Uri| {
            let path = "/var/run/tetragon/tetragon.sock";

            // Connect to a Uds socket
            UnixStream::connect(path)
        }))
        .await?;

    tracing::info!("Done\nCreating Client");
    let client = FineGuidanceSensorsClient::new(channel);
    Ok(client)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(my_subscriber)
        .expect("setting tracing default failed");

    tracing::info!("test");
    tracing::info!("Creating Client");
    // let mut client = socket_client().await?;
    let mut client = socket_client().await?;

    tracing::info!("Done\nCreating Request");
    let request = tonic::Request::new(GetEventsRequest {
        aggregation_options: Some(AggregationOptions {
            window_size: None,
            channel_buffer_size: u64::MAX,
        }),
        ..Default::default()
    });

    tracing::info!("Done\nSending Request");
    let mut response = client.get_events(request).await?;
    tracing::info!("Success!\nPrinting Response:\n");
    while let Ok(Some(t)) = response.get_mut().message().await {
        tracing::info!("{t:#?}");
    }

    Ok(())
}
