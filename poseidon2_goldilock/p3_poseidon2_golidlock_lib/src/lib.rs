use p3_field::{AbstractField, PrimeField64};
use p3_goldilocks::{DiffusionMatrixGoldilocks, Goldilocks, HL_GOLDILOCKS_8_EXTERNAL_ROUND_CONSTANTS, HL_GOLDILOCKS_8_INTERNAL_ROUND_CONSTANTS};
use p3_poseidon2::{Poseidon2, Poseidon2ExternalMatrixHL};
use p3_symmetric::Permutation;
#[no_mangle]
pub extern "C" fn process_and_return_u64_array(
    input_ptr: *const u64, input_len: usize, output_len: *mut usize
) -> *mut u64 {
    if input_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let input_slice = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };

    let input_array: [u64; 8] = [
        input_slice[0],
        input_slice[1],
        input_slice[2],
        input_slice[3],
        input_slice[4],
        input_slice[5],
        input_slice[6],
        input_slice[7]
    ];

    let hash = hl_poseidon2_all(input_array);

    // Example operation: create a new array with doubled values
    let output_vec: Vec<u64> = hash.to_vec();
    let len = output_vec.len();

    unsafe {
        *output_len = len;
    }

    let ptr = output_vec.as_ptr() as *mut u64;
    std::mem::forget(output_vec);  // Prevent Rust from deallocating the array
    ptr
}

#[no_mangle]
pub extern "C" fn free_u64_array(ptr: *mut u64, len: usize) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Vec::from_raw_parts(ptr, len, len);
    }
}

pub fn hl_poseidon2_all(inputs: [u64; 8]) -> [u64; 8] {
    let mut inp: [Goldilocks; 8] = inputs.map(Goldilocks::from_wrapped_u64);
    hl_poseidon2_goldilocks_width_8(&mut inp);
    let hash = inp.map(|i| Goldilocks::as_canonical_u64(&i));
    hash
}

pub fn hl_poseidon2(inputs: [u64; 8]) -> [u64; 4] {
    let mut inp: [Goldilocks; 8] = inputs.map(Goldilocks::from_wrapped_u64);
    hl_poseidon2_goldilocks_width_8(&mut inp);
    let hash = inp.map(|i| Goldilocks::as_canonical_u64(&i));
    let mut output:[u64; 4] = [0; 4];
    for i in 0..4 {
        output[i] = hash[i]
    }
    output
}

// from plonky3
fn hl_poseidon2_goldilocks_width_8(input: &mut [Goldilocks; 8]) {
    const WIDTH: usize = 8;
    const D: u64 = 7;
    const ROUNDS_F: usize = 8;
    const ROUNDS_P: usize = 22;

    let poseidon2: Poseidon2<
        Goldilocks,
        Poseidon2ExternalMatrixHL,
        DiffusionMatrixGoldilocks,
        WIDTH,
        D,
    >  = Poseidon2::new(
        ROUNDS_F,
        HL_GOLDILOCKS_8_EXTERNAL_ROUND_CONSTANTS
            .map(to_goldilocks_array)
            .to_vec(),
        Poseidon2ExternalMatrixHL,
        ROUNDS_P,
        to_goldilocks_array(HL_GOLDILOCKS_8_INTERNAL_ROUND_CONSTANTS).to_vec(),
        DiffusionMatrixGoldilocks,
    );

    poseidon2.permute_mut(input);
}


#[inline]
#[must_use]
fn to_goldilocks_array<const N: usize>(input: [u64; N]) -> [Goldilocks; N] {
    let mut output = [Goldilocks::zero(); N];
    let mut i = 0;
    loop {
        if i == N {
            break;
        }
        output[i] = Goldilocks::from_canonical_u64(input[i]);
        i += 1;
    }
    output
}
