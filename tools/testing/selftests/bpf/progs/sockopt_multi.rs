//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/sockopt_multi.c
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
    let mut page_size: __s32 = 0;
    SEC("cgroup/getsockopt")
#[no_mangle]
pub unsafe extern "C" fn _getsockopt_child(ctx: *mut bpf_sockopt) -> c_int {
    int _getsockopt_child(struct bpf_sockopt *ctx)
    {
    __u8 *optval_end = ctx.optval_end;
    __u8 *optval = ctx.optval;
    if (ctx.level != SOL_IP || ctx.optname != IP_TOS)
    goto out;
    if (optval + 1 > optval_end)
    return 0; /* EPERM, bounds check */
    if (optval[0] != 0x80)
    return 0; /* EPERM, unexpected optval from the kernel */
    ctx.retval = 0; /* Reset system call return value to zero */
    optval[0] = 0x90;
    ctx.optlen = 1;
    return 1;
    out:
// optval larger than PAGE_SIZE use kernel's buffer.
    if (ctx.optlen > page_size)
    ctx.optlen = 0;
    return 1;
    }
    SEC("cgroup/getsockopt")
#[no_mangle]
pub unsafe extern "C" fn _getsockopt_parent(ctx: *mut bpf_sockopt) -> c_int {
    int _getsockopt_parent(struct bpf_sockopt *ctx)
    {
    __u8 *optval_end = ctx.optval_end;
    __u8 *optval = ctx.optval;
    if (ctx.level != SOL_IP || ctx.optname != IP_TOS)
    goto out;
    if (optval + 1 > optval_end)
    return 0; /* EPERM, bounds check */
    if (optval[0] != 0x90)
    return 0; /* EPERM, unexpected optval from the kernel */
    ctx.retval = 0; /* Reset system call return value to zero */
    optval[0] = 0xA0;
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
    __u8 *optval = ctx.optval;
    if (ctx.level != SOL_IP || ctx.optname != IP_TOS)
    goto out;
    if (optval + 1 > optval_end)
    return 0; /* EPERM, bounds check */
    optval[0] += 0x10;
    ctx.optlen = 1;
    return 1;
    out:
// optval larger than PAGE_SIZE use kernel's buffer.
    if (ctx.optlen > page_size)
    ctx.optlen = 0;
    return 1;
    }
