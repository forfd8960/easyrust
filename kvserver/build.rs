use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .bytes(&["."])
        .out_dir("src/pb")
        .type_attribute(".", "#[derive(PartialOrd)]")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
    Ok(())
}
