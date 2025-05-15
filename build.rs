fn main() {
    tonic_build::compile_protos("proto/coffee_maker.proto").unwrap();
}
