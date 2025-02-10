use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "./proto/reviews.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .file_descriptor_set_path(out_dir.join("reviews_descriptor.bin"))
        .out_dir("./src")
        .compile_protos(&[proto_file], &["proto"])?;

    Ok(())
}
