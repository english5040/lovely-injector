use std::ffi::{c_void, CStr};
use std::ptr;

mod fishhook_bindings;

// Make the symbol old_name refer to new_fn.
// Returns the original function (as a pointer).
pub fn rebind_symbol(old_name: &'static CStr, new_fn: *const ()) -> *const () {
    let mut old_fn: *const () = ptr::null();
    let mut rebindings = fishhook_bindings::rebinding {
        name: old_name.as_ptr(),
        replacement: new_fn as *mut c_void,
        replaced: ptr::from_mut(&mut old_fn) as *mut *mut c_void,
    };
    unsafe {
        fishhook_bindings::rebind_symbols(ptr::from_mut(&mut rebindings), 1);
    }
    old_fn
}
