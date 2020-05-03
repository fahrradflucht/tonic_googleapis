fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("tonic_datastore_v1/src/gen")
        .compile(
            &["googleapis/google/datastore/v1/datastore.proto"],
            &["googleapis/"],
        )?;

    Ok(())
}
