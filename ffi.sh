# export OCAML_LOG_ENABLED=true
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/developer/tezedge/tezos/interop/lib_tezos/artifacts
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/developer/tezedge/target/debug
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/developer/.rustup/toolchains/nightly-2020-02-04-x86_64-unknown-linux-gnu/lib/
#./target/debug/deps/test_bytes_roundtrips-74542cf6c12397a5 --nocapture --test-threads=1
valgrind --show-leak-kinds=all ./target/debug/deps/test_bytes_roundtrips-74542cf6c12397a5  --nocapture 