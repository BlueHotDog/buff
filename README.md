# buff - protocolBuffer Registry
buff aims to provide an easy way to share, check and discover GRPC services.

# Problem
GRPC allows companies to build scalabale and backwards compatible services easily. Unfortunaly there isnt any good solution to search, package and check your services in an easy way.

# Project Goals

- [ ] CLI tool to package and publish GRPC definitions as packages.
- [ ] Define common protobuf that other services can use/depend on.
- [ ] Easy interface to search for services/packages
- [ ] Automatic backwards compatability validation
- [ ] Easy way to interact with services defined in packages.
- [ ] You should be easily host this on your own infrastracture. We want to explore Pub/Sub to allow companies to host their own but still be able to depend on packages in other comapnies registries.

# Project structure
The project is split into two parts CLI and Server, each containing their respective documentation.

