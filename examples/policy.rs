//! Policy Manager
//! 
//! Policy Manager can list add & delte policies
//! This example is based of [this tetragon guide](https://tetragon.io/docs/concepts/tracing-policy/example/).


use std::env::args;

use tokio::net::UnixStream;
use tonic::transport::{Channel, Endpoint, Uri};
use tower::service_fn;
use tracing_subscriber::FmtSubscriber;

use tetragon_grpc::fine_guidance_sensors_client::FineGuidanceSensorsClient;
use tetragon_grpc::{
    AddTracingPolicyRequest, DeleteTracingPolicyRequest, ListTracingPoliciesRequest,
};

pub async fn http_client(
    uri: &'static str,
) -> Result<FineGuidanceSensorsClient<Channel>, Box<dyn std::error::Error>> {
    tracing::info!("Creating Channel");
    let channel = Endpoint::try_from(uri)?;

    tracing::info!("Creating Client");
    let client = FineGuidanceSensorsClient::connect(channel).await?;
    Ok(client)
}

pub async fn socket_client(
    path: &'static str,
) -> Result<FineGuidanceSensorsClient<Channel>, Box<dyn std::error::Error>> {
    tracing::info!("Creating Channel");
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

    tracing::info!("Creating Client");
    let client = FineGuidanceSensorsClient::new(channel);
    Ok(client)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(my_subscriber).expect("setting tracing default failed");

    // NOTE: get root
    sudo::escalate_if_needed()?;
    let mut client = socket_client("/var/run/tetragon/tetragon.sock").await?;

    match args().nth(1).as_deref() {
        Some("add") => add_policy(&mut client).await?,
        Some("remove") => remove_policy(&mut client).await?,
        Some("list") => list_policies(&mut client).await?,
        _ => {
            tracing::error!("Parameter required\nOptions:\n- list\n- add\n- remove");
            return Ok(());
        }
    }

    Ok(())
}

async fn add_policy(
    client: &mut FineGuidanceSensorsClient<Channel>,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Creating Policy");
    // NOTE: creating a request for getevents
    let request = tonic::Request::new(AddTracingPolicyRequest {
        yaml: include_str!("../resources/fd-install.yaml").to_string(),
    });

    tracing::info!("Adding Policy");
    // NOTE: running the request
    let response = client.add_tracing_policy(request).await?;

    tracing::info!("Success! Printing Response:");
    tracing::info!("{response:#?}");
    Ok(())
}

async fn remove_policy(
    client: &mut FineGuidanceSensorsClient<Channel>,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Creating Request");
    // NOTE: creating a request for getevents
    let request = tonic::Request::new(DeleteTracingPolicyRequest {
        name: "fd-install".into(),
    });

    tracing::info!("Deleteing Policy");
    // NOTE: running the request
    let response = client.delete_tracing_policy(request).await?;

    tracing::info!("Success! Printing Response:");
    tracing::info!("{response:#?}");
    Ok(())
}

async fn list_policies(
    client: &mut FineGuidanceSensorsClient<Channel>,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Creating Request");
    // NOTE: creating a request for getevents
    let request = tonic::Request::new(ListTracingPoliciesRequest {});

    tracing::info!("Deleteing Policy");
    // NOTE: running the request
    let response = client.list_tracing_policies(request).await?;
    tracing::info!("Success! Printing Response:");
    for policy in response.into_inner().policies {
        tracing::info!("{policy:#?}");
    }

    Ok(())
}
