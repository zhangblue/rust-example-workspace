fn main() {
    // 处理hello world的 protobuf格式
    tonic_build::configure()
        // .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["proto/helloworld.proto"], &["proto"])
        .unwrap();

    // 处理streaming的 protobuf格式
    tonic_build::configure()
        .compile(&["proto/streaming.proto"], &["proto"]).unwrap()
}