fn main() {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(true)
        .compile(
            &[
                "proto/source.proto",
                "proto/sourcetransform.proto",
                "proto/map.proto",
                "proto/reduce.proto",
                "proto/sink.proto",
                "proto/sideinput.proto",
                "proto/batchmap.proto",
            ],
            &["proto"],
        )
        .unwrap_or_else(|e| panic!("failed to compile the proto, {:?}", e))
}
