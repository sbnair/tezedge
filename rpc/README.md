# RPC

This project contains implementation of Tezos RPC endpoints and Tezedge development endpoints.

#### Integration test
Tezedge CI runs big integration test for RPC's on Drone server, which verifies compatibility Tezedge RPC's with original Tezos Ocaml RPC'c.
- run test and check headers in interval (FROM_BLOCK_HEADER, TO_BLOCK_HEADER>
- FROM_BLOCK_HEADER / TO_BLOCK_HEADER are passed as environment variables
- OCAML_NODE_CONTEXT_ROOT (default: http://ocaml-node-run:8732) - env variable where is Tezos Ocaml node running
- TEZEDGE_NODE_CONTEXT_ROOT (default: http://tezedge-node-run:18732) - env variable where is Tezedge Rust node running
- TODO: added more env vars, add desc. here
```
OCAML_NODE_RPC_CONTEXT_ROOT=http://ocaml-node-run:8732 \
TEZEDGE_NODE_RPC_CONTEXT_ROOT=http://tezedge-node-run:18732 \
FROM_BLOCK_HEADER=1 \
TO_BLOCK_HEADER=5000 \
cargo test --verbose -- --nocapture --ignored test_rpc_compare
```