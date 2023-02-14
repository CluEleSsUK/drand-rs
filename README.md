# 🎲🦀 drand-rs

A drand daemon and CLI implementation written in rust

## Packages
- cli

The CLI for running and interacting with your drand daemon
- daemon

The daemon which participates in the network

## TODO
- [x] generate a keypair
- [x] load the keypair on the daemon
- [ ] very rudimentary distributed key generation
- [ ] sending partial beacons
- [ ] aggregating partial beacons
- [ ] storing beacons
- [ ] HTTP API
- [ ] gossipsub API
- [ ] proper distributed key generation
- [ ] key resharing
- [ ] catchup of existing beacons
- [ ] metrics