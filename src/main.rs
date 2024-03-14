//! Mimics `tetra --server-address unix:///var/run/tetragon/tetragon.sock getevents`
//!
//! Author: Aditeya V. Govind <aditeya.vg@subcom.tech>

use tracing_subscriber::FmtSubscriber;

fn main() -> anyhow::Result<()> {
    let my_subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(my_subscriber).expect("setting tracing default failed");

    // NOTE: get root
    sudo::escalate_if_needed().expect("sudo is needed");
    
    let (tx, rx) = crossbeam_channel::bounded(100);
    let mut subscriber = tetragon_grpc::EventsSubscriber::new(None, tx);

    std::thread::spawn(move || {
        subscriber.run().expect("failed to launch");
    });

    // NOTE: reading the stream for events
    for t in rx.iter() {
        tracing::info!("{t:?}");
    }

    Ok(())
}
