//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/connect4_dropper.c
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

pub const VERDICT_REJECT: c_int = 0;
pub const VERDICT_PROCEED: c_int = 1;
    int port;
    SEC("cgroup/connect4")
#[no_mangle]
pub unsafe extern "C" fn connect_v4_dropper(ctx: *mut bpf_sock_addr) -> c_int {
    int connect_v4_dropper(struct bpf_sock_addr *ctx)
    {
    if (ctx.type != SOCK_STREAM)
    return VERDICT_PROCEED;
    if (ctx.user_port == bpf_htons(port))
    return VERDICT_REJECT;
    return VERDICT_PROCEED;
    }
    char _license[] SEC("license") = "GPL";
