extern crate embed_resource;

fn main() {
    #[cfg(windows)]
    embed_resource::compile("../build_resources/windows/res.rc");
}
