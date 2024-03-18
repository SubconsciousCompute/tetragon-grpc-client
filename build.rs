//! Custom Build Config
//! Custom build to create the `tetragon.rs` file from the proto files.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match tonic_build::configure().build_server(false).compile(
        &[
            "./proto_v1/tetragon/capabilities.proto",
            "./proto_v1/tetragon/tetragon.proto",
            "./proto_v1/tetragon/stack.proto",
            "./proto_v1/tetragon/events.proto",
            "./proto_v1/tetragon/sensors.proto",
        ],
        &["./proto_v1"],
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            // NOTE: print errors in a readable formab
            eprintln!("{}", e);
            Err("Failed".into())
        }
    }
}
