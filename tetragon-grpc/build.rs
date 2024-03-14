fn main() -> Result<(), Box<dyn std::error::Error>> {
    match tonic_build::configure().build_server(false).compile(
        &[
            "../proto_files/tetragon/capabilities.proto",
            "../proto_files/tetragon/tetragon.proto",
            "../proto_files/tetragon/stack.proto",
            "../proto_files/tetragon/events.proto",
            "../proto_files/tetragon/sensors.proto",
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
