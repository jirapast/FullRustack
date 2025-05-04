
grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{\"name\": \"Tonic\"}' [::1]:50051 helloworld.Greeter/SayHello

