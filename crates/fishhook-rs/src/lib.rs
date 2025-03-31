use std::ffi::{c_void, CStr};
use std::ptr;
use std::sync::atomic::AtomicPtr;

mod fishhook_bindings;

// Make the symbol `old_name` refer to `new_fn`.
// This function can be called even if the library that defines `old_name`
// hasn't been loaded.
//
// The old function (as a pointer) will be written into `old_fn_out` if it's not None.
//
// Implementation safety notes:
// `old_fn_out` can be written to at an indefinite future time,
// namely when the dynamic library that defines `old_name` is loaded,
// so for safety, old_fn_out should have static lifetime.
// It should also be thread-safe (this might be overkill?).
pub fn rebind_symbol(
    old_name: &'static CStr,
    new_fn: *const c_void,
    old_fn_out: Option<&'static AtomicPtr<c_void>>,
) {
    let mut rebindings = fishhook_bindings::rebinding {
        name: old_name.as_ptr(),
        replacement: new_fn as *mut c_void,
        replaced: match old_fn_out {
            Some(x) => x.as_ptr(),
            None => ptr::null_mut(),
        },
    };
    unsafe {
        fishhook_bindings::rebind_symbols(ptr::from_mut(&mut rebindings), 1);
    }
}
