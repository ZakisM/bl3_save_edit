use std::path::Path;

use protobuf_codegen_pure::{Codegen, Customize};

fn main() {
    let files = vec![
        "protobufs/oak_profile.proto",
        "protobufs/oak_save.proto",
        "protobufs/oak_shared.proto",
    ];

    if files
        .iter()
        .map(|f| {
            f.replace("protobufs/", "src/protos/")
                .replace(".proto", ".rs")
        })
        .any(|f| !Path::new(&f).exists())
    {
        Codegen::new()
            .out_dir("src/protos")
            .include("protobufs")
            .inputs(files)
            .customize(Customize {
                gen_mod_rs: Some(true),
                ..Default::default()
            })
            .run()
            .expect("Failed to generate protocol buffers");
    }
}
