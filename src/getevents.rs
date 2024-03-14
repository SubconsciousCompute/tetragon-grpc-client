//! getevents.

use crate::fine_guidance_sensors_client::FineGuidanceSensorsClient;
use crate::{Filter, GetEventsRequest, GetEventsResponse};
use crossbeam_channel::Sender;
use futures::executor::block_on;
use std::path::PathBuf;
use tokio::net::UnixStream;
use tonic::transport::{Channel, Endpoint, Uri};
use tower::service_fn;

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
        let rt = tokio::runtime::Runtime::new().unwrap();

        let mut client = Self::socket_client(self.socket_path.clone(), rt.handle())?;
        // NOTE: creating a request for getevents
        let request = tonic::Request::new(GetEventsRequest {
            allow_list: vec![Filter::default()],
            ..Default::default()
        });

        tracing::info!("Sending Request");
        let mut response = rt.block_on(client.get_events(request))?;

        while let Ok(Some(t)) = rt.block_on(response.get_mut().message()) {
            if let Err(e) = self.tx.try_send(t) {
                tracing::info!("failed to send event: {e}");
            }
        }

        Ok(())
    }

    /// Unix domain socket client
    fn socket_client(
        path: PathBuf,
        rt: &tokio::runtime::Handle,
    ) -> anyhow::Result<FineGuidanceSensorsClient<Channel>> {
        tracing::info!("Creating socket channel");

        // NOTE: We will ignore this uri because uds do not use it
        // if your connector does use the uri it will be provided
        // as the request to the `MakeConnection`.
        let channel = rt.block_on(
            Endpoint::try_from("http://0.0.0.0:9999")?.connect_with_connector(service_fn(
                move |_: Uri| {
                    // Connect to a Uds socket
                    UnixStream::connect(path.clone())
                },
            )),
        )?;

        tracing::info!("Creating socket client");
        let client = FineGuidanceSensorsClient::new(channel);
        Ok(client)
    }
}
