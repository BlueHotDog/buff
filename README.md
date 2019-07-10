[![CircleCI](https://circleci.com/gh/BlueHotDog/buff.svg?style=svg)](https://circleci.com/gh/BlueHotDog/buff)
[![codecov](https://codecov.io/gh/BlueHotDog/buff/branch/master/graph/badge.svg)](https://codecov.io/gh/BlueHotDog/buff)

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

# Getting started

- Install [Minikube](https://kubernetes.io/docs/tasks/tools/install-minikube/)


# MVP

- [ ] Auth works - Danni
  - [ ] Maybe SSO
  - [ ] Maybe 2FA
- [ ] Login/Logout/Signup - Danni
- [ ] 100% E2E tests
- [ ] 100% test coverage
- [ ] Awesome CI/CD pipeline including releases etc - Danni
- [ ] Good CLI error messages - Itay
- [ ] Exception/Error monitoring
- [ ] Backwards comp check
- [ ] History - Itay
- [ ] Frontend
  - [ ] Documentation
    - [ ] Onboarding(create S3 bucket etc, DB)
    - [ ] Helm chart?
  - [ ] Getting Started
  - [ ] TOS
  - [ ] COC
  - [ ] Search
  - [ ] Package view
    - [ ] Versions
    - [ ] GRPC Explorer interface - allow users to make calls to the GRPC service
  - [ ] Package readme
