fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}

// Generating Server and Client code

// This tells tonic-build to compile your protobufs when you build your Rust project.

// .proto > build.rs > client, server code
// user > call client code > server code

// server side
//     grpc server
//         service greeter
//             idle > get incoming req > execute service > response

// client side
//     stub
//         idle > get incoming req from user > send req to server > get response from server
