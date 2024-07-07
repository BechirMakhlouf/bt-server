use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // tonic_build::configure();
    // .file_descriptor_set_path(out_dir.join("grpc.health.v1_descriptor.bin"))
    // .compile(&[""], &["proto"])?;

    // tonic_build::compile_protos("proto/check_health.proto")?;
    Ok(())
}
