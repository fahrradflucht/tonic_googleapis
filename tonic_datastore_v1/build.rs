fn main() -> Result<(), Box<dyn std::error::Error>> {
  let out_dir = std::env::var_os("OUT_DIR").unwrap();

  tonic_build::configure()
      .build_server(false)
      .out_dir(out_dir)
      .compile(
          &["../googleapis/google/datastore/v1/datastore.proto"],
          &["../googleapis/"],
      )?;

  Ok(())
}
