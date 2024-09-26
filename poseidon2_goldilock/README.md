cd p3_poseidon2_golidlock_lib

cargo build --release

(1) copy ./target/release/libp3_poseidon2_golidlock_lib.dylib to /usr/lib/
(2) export CGO_LDFLAGS="-L./target/release"