fn main() -> Result<(), Box<dyn std::error::Error>> {
    match tonic_build::configure().build_server(false).compile(
        &[
            "../proto_files/capabilities.proto",
            "../proto_files/tetragon.proto",
            "../proto_files/stack.proto",
            "../proto_files/events.proto",
            "../proto_files/sensors.proto",
        ],
        &["../proto_files"],
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            // NOTE: print errors in a readable formab
            eprintln!("{}", e);
            Err("Failed".into())
        }
    }
}
