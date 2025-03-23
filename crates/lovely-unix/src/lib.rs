use lovely_core::log::*;
use lovely_core::sys::{LuaState, LUA_LIB};
use std::{env, mem, panic, ptr::null};

use lovely_core::Lovely;
use once_cell::sync::{Lazy, OnceCell};

static RUNTIME: OnceCell<Lovely> = OnceCell::new();

static RECALL: Lazy<
    unsafe extern "C" fn(*mut LuaState, *const u8, isize, *const u8, *const u8) -> u32,
> = Lazy::new(|| unsafe {
    mem::transmute(fishhook::rebind_symbol(
        c"luaL_loadbufferx",
        luaL_loadbufferx_new as *const (),
    ))
});

#[allow(non_snake_case)]
unsafe extern "C" fn luaL_loadbuffer_new(
    state: *mut LuaState,
    buf_ptr: *const u8,
    size: isize,
    name_ptr: *const u8,
) -> u32 {
    let rt = RUNTIME.get_unchecked();
    rt.apply_buffer_patches(state, buf_ptr, size, name_ptr, null())
}

#[allow(non_snake_case)]
unsafe extern "C" fn luaL_loadbufferx_new(
    state: *mut LuaState,
    buf_ptr: *const u8,
    size: isize,
    name_ptr: *const u8,
    mode_ptr: *const u8,
) -> u32 {
    let rt = RUNTIME.get_unchecked();
    rt.apply_buffer_patches(state, buf_ptr, size, name_ptr, mode_ptr)
}

#[ctor::ctor]
unsafe fn construct() {
    panic::set_hook(Box::new(|x| {
        let message = format!("lovely-injector has crashed: \n{x}");
        error!("{message}");
    }));
    let args: Vec<_> = env::args().collect();
    let dump_all = args.contains(&"--dump-all".to_string());

    let _ = fishhook::rebind_symbol(c"luaL_loadbuffer", luaL_loadbuffer_new as *const ());

    let rt = Lovely::init(&|a, b, c, d, e| RECALL(a, b, c, d, e), dump_all);

    RUNTIME
        .set(rt)
        .unwrap_or_else(|_| panic!("Failed to instantiate runtime."));
}
