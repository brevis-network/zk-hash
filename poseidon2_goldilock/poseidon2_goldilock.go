package poseidon2_goldilock

/*
#cgo LDFLAGS: -L. -lp3_poseidon2_golidlock_lib
#include <stdint.h>
#include <stdlib.h>

// Forward declarations of the Rust functions
uint64_t* process_and_return_u64_array(const uint64_t* input_ptr, size_t input_len, size_t* output_len);
void free_u64_array(uint64_t* ptr, size_t len);
*/
import "C"
import (
	"unsafe"
)

func ComputePoseidon2GoldilockHash(input [8]uint64) [8]uint64 {
	// Example Go array

	// Get a pointer to the first element of the Go array
	inputPtr := (*C.uint64_t)(unsafe.Pointer(&input[0]))

	// Prepare a variable to hold the length of the output array
	var outputLength C.size_t

	// Call the Rust function
	outputPtr := C.process_and_return_u64_array(inputPtr, C.size_t(8), &outputLength)

	// Convert the returned C array to a Go slice
	outputSlice := (*[1 << 30]C.uint64_t)(unsafe.Pointer(outputPtr))[:outputLength:outputLength]

	res := [8]uint64{uint64(outputSlice[0]),
		uint64(outputSlice[1]),
		uint64(outputSlice[2]),
		uint64(outputSlice[3]),
		uint64(outputSlice[4]),
		uint64(outputSlice[5]),
		uint64(outputSlice[6]),
		uint64(outputSlice[7]),
	}
	// Free the output array
	C.free_u64_array(outputPtr, outputLength)

	return res
}
