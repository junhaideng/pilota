use pilota_build::plugin::SerdePlugin;

fn main() {
    let t = pilota_build::Builder::thrift()
        .ignore_unused(false)
        .only_types(true)
        .plugin(SerdePlugin);
    t.compile(&["idl/model.thrift"], "src/model.rs");
}
