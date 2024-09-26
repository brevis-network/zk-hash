cd p3_poseidon2_golidlock_lib

cargo build --release

(1) 

for ub ubuntu:

export CGO_LDFLAGS="-L/root/zk-hash/poseidon2_goldilock/p3_poseidon2_golidlock_lib/target/release"

for mac, also can use

export LD_LIBRARY_PATH=/zk-hash/poseidon2_goldilock/p3_poseidon2_golidlock_lib/target/release:$LD_LIBRARY_PATH

or

(2) copy libp3_poseidon2_golidlock_lib.dylib to/usr/lib