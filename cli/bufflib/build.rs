fn main() {
  let proto_root = "../../protobuffers";
  println!("cargo:rerun-if-changed={}", proto_root);
  protoc_grpcio::compile_grpc_protos(
    &["../../protobuffers/buff.proto"],
    &[proto_root],
    &"src/protobuffers",
    None,
  )
  .expect("Failed to compile gRPC definitions!");
}
