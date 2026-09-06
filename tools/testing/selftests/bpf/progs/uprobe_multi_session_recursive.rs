//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/uprobe_multi_session_recursive.c
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
    let mut pid: c_int = 0;
    let mut idx_entry: c_int = 0;
    let mut idx_return: c_int = 0;
    __u64 test_uprobe_cookie_entry[6];
    __u64 test_uprobe_cookie_return[3];
#[no_mangle]
unsafe extern "C" fn check_cookie(ctx: *mut pt_regs) -> c_int {
    static int check_cookie(struct pt_regs *ctx)
    {
    __u64 *cookie = bpf_session_cookie(ctx);
    if (bpf_session_is_return(ctx)) {
    if (idx_return >= ARRAY_SIZE(test_uprobe_cookie_return))
    return 1;
    test_uprobe_cookie_return[idx_return++] = *cookie;
    return 0;
    }
    if (idx_entry >= ARRAY_SIZE(test_uprobe_cookie_entry))
    return 1;
// cookie = test_uprobe_cookie_entry[idx_entry];
    return idx_entry++ % 2;
    }
    SEC("uprobe.session//proc/self/exe:uprobe_session_recursive")
#[no_mangle]
pub unsafe extern "C" fn uprobe_recursive(ctx: *mut pt_regs) -> c_int {
    int uprobe_recursive(struct pt_regs *ctx)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 1;
    return check_cookie(ctx);
    }
