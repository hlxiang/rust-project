
fn main() {
    // prost_build 作用：把abi.proto编译到src/pb目录下 
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}