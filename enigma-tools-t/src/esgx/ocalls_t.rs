use crate::common::errors_t::EnclaveError;
use enigma_types::traits::SliceCPtr;
use sgx_types::sgx_status_t;
use std::{path::PathBuf, str};

const PATH_MAX: usize = 4096; // linux/limits.h - this depends on the FS.

pub fn get_home_path(ocall: unsafe extern "C" fn(*mut u8, *mut u32) -> sgx_status_t) -> Result<PathBuf, EnclaveError> {
    // Get Home path via Ocall
    let mut home_slice: [u8; PATH_MAX] = [0; PATH_MAX];
    let mut result_len = 0;
    unsafe { ocall(home_slice.as_mut_ptr(), &mut result_len); }
    let home_str = str::from_utf8(&home_slice[..result_len as usize])?;
    debug_println!("Back from Ocall: {}", &home_str);

    Ok(PathBuf::from(home_str))
}


// TODO: Replace u64 with *const u8, and pass it via the ocall using *const *const u8
pub fn save_to_untrusted_memory(data: &[u8], ocall: unsafe extern "C" fn(*mut u64, *const u8, usize) -> sgx_status_t) -> Result<u64, EnclaveError> {
    let mut ptr = 0u64;
    match unsafe { ocall(&mut ptr as *mut u64, data.as_c_ptr(), data.len()) } {
        sgx_status_t::SGX_SUCCESS => Ok(ptr),
        e => Err(e.into()),
    }
}
