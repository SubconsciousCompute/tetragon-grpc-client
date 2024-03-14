//! getevents.

use futures::executor::block_on;
use tokio::net::UnixStream;
use crossbeam_channel::Sender;
use std::path::{PathBuf};
use tonic::transport::{Channel, Endpoint, Uri};
use tower::service_fn;
use crate::fine_guidance_sensors_client::FineGuidanceSensorsClient;
use crate::{Filter, GetEventsRequest, GetEventsResponse};

/// Subscribe to tetragon events.
pub struct EventsSubscriber {
    socket_path: PathBuf,
    tx: Sender<GetEventsResponse>,
}


impl EventsSubscriber {
    /// Create a new EventsSubscriber  that writes events to tx.
    pub fn new(socketpath: Option<PathBuf>, tx: Sender<GetEventsResponse>) -> Self {
        let socket_path = socketpath.unwrap_or("/var/run/tetragon/tetragon.sock".into());
        Self { socket_path, tx }
    }

    /// Run the subscriber.
    pub fn run(&mut self) -> anyhow::Result<()> {
        let mut client = block_on(Self::socket_client(self.socket_path.clone()))?;
        // NOTE: creating a request for getevents
        let request = tonic::Request::new(GetEventsRequest {
            allow_list: vec![Filter::default()],
            ..Default::default()
        });

        tracing::info!("Sending Request");
        let mut response = block_on(client.get_events(request))?;

        while let Ok(Some(t)) = block_on(response.get_mut().message()) {
            if let Err(e) = self.tx.try_send(t) {
                tracing::info!("failed to send event: {e}");
            }
        }

        Ok(())
    }

    /// Unix domain socket client
    pub async fn socket_client(path: PathBuf) -> anyhow::Result<FineGuidanceSensorsClient<Channel>> {
        tracing::info!("Creating socket channel");

        // NOTE: We will ignore this uri because uds do not use it
        // if your connector does use the uri it will be provided
        // as the request to the `MakeConnection`.
        let channel = Endpoint::try_from("http://0.0.0.0:9999")?
            .connect_with_connector(service_fn(move |_: Uri| {
                // Connect to a Uds socket
                UnixStream::connect(path.clone())
            }))
        .await?;

        tracing::info!("Creating socket client");
        let client = FineGuidanceSensorsClient::new(channel);
        Ok(client)
    }


}
