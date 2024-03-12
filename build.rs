fn main() -> Result<(), Box<dyn std::error::Error>> {
    match tonic_build::configure().build_server(false).compile(
        &[
            "proto/tetragon/capabilities.proto",
            "proto/tetragon/tetragon.proto",
            "proto/tetragon/stack.proto",
            "proto/tetragon/events.proto",
            "proto/tetragon/sensors.proto",
        ],
        &["./proto"],
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            // NOTE: print errors in a readable formab
            eprintln!("{}", e);
            Err("Failed".into())
        }
    }
}
