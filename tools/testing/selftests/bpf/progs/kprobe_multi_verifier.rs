//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/kprobe_multi_verifier.c
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
    SEC("kprobe.session")
    __success
#[no_mangle]
pub unsafe extern "C" fn kprobe_session_return_0(ctx: *mut pt_regs) -> c_int {
    int kprobe_session_return_0(struct pt_regs *ctx)
    {
    return 0;
    }
    SEC("kprobe.session")
    __success
#[no_mangle]
pub unsafe extern "C" fn kprobe_session_return_1(ctx: *mut pt_regs) -> c_int {
    int kprobe_session_return_1(struct pt_regs *ctx)
    {
    return 1;
    }
    SEC("kprobe.session")
    __failure
    __msg("At program exit the register R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn kprobe_session_return_2(ctx: *mut pt_regs) -> c_int {
    int kprobe_session_return_2(struct pt_regs *ctx)
    {
    return 2;
    }
