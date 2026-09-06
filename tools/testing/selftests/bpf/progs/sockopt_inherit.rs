//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/sockopt_inherit.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0

    char _license[] SEC("license") = "GPL";
pub const SOL_CUSTOM: c_uint = 0xdeadbeef;
pub const CUSTOM_INHERIT1: c_int = 0;
pub const CUSTOM_INHERIT2: c_int = 1;
pub const CUSTOM_LISTENER: c_int = 2;
    let mut page_size: __s32 = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockopt_inherit {
    pub val: __u8,
}

    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC | BPF_F_CLONE);
    __type(key, int);
    __type(value, struct sockopt_inherit);
    } cloned1_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC | BPF_F_CLONE);
    __type(key, int);
    __type(value, struct sockopt_inherit);
    } cloned2_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct sockopt_inherit);
    } listener_only_map SEC(".maps");
    static __inline struct sockopt_inherit *get_storage(struct bpf_sockopt *ctx)
    {
    if (ctx.optname == CUSTOM_INHERIT1)
    return bpf_sk_storage_get(&cloned1_map, ctx.sk, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
#[no_mangle]
pub unsafe extern "C" fn if(CUSTOM_INHERIT2: ctx->optname ==) -> else {
    else if (ctx.optname == CUSTOM_INHERIT2)
    return bpf_sk_storage_get(&cloned2_map, ctx.sk, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
    else
    return bpf_sk_storage_get(&listener_only_map, ctx.sk, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
    }
    SEC("cgroup/getsockopt")
#[no_mangle]
pub unsafe extern "C" fn _getsockopt(ctx: *mut bpf_sockopt) -> c_int {
    int _getsockopt(struct bpf_sockopt *ctx)
    {
    __u8 *optval_end = ctx.optval_end;
    struct sockopt_inherit *storage;
    __u8 *optval = ctx.optval;
    if (ctx.level != SOL_CUSTOM)
    goto out; /* only interested in SOL_CUSTOM */
    if (optval + 1 > optval_end)
    return 0; /* EPERM, bounds check */
    storage = get_storage(ctx);
    if (!storage)
    return 0; /* EPERM, couldn't get sk storage */
    ctx.retval = 0; /* Reset system call return value to zero */
    optval[0] = storage.val;
    ctx.optlen = 1;
    return 1;
    out:
// optval larger than PAGE_SIZE use kernel's buffer.
    if (ctx.optlen > page_size)
    ctx.optlen = 0;
    return 1;
    }
    SEC("cgroup/setsockopt")
#[no_mangle]
pub unsafe extern "C" fn _setsockopt(ctx: *mut bpf_sockopt) -> c_int {
    int _setsockopt(struct bpf_sockopt *ctx)
    {
    __u8 *optval_end = ctx.optval_end;
    struct sockopt_inherit *storage;
    __u8 *optval = ctx.optval;
    if (ctx.level != SOL_CUSTOM)
    goto out; /* only interested in SOL_CUSTOM */
    if (optval + 1 > optval_end)
    return 0; /* EPERM, bounds check */
    storage = get_storage(ctx);
    if (!storage)
    return 0; /* EPERM, couldn't get sk storage */
    storage.val = optval[0];
    ctx.optlen = -1;
    return 1;
    out:
// optval larger than PAGE_SIZE use kernel's buffer.
    if (ctx.optlen > page_size)
    ctx.optlen = 0;
    return 1;
    }
