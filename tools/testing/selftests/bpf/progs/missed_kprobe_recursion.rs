//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/missed_kprobe_recursion.c
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
//
// No tests in here, just to trigger 'bpf_fentry_test*'
// through tracing test_run
//
    SEC("fentry/bpf_modify_return_test")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trigger) -> c_int {
    int BPF_PROG(trigger)
    {
    return 0;
    }
    SEC("kprobe.multi/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn test1(ctx: *mut pt_regs) -> c_int {
    int test1(struct pt_regs *ctx)
    {
    bpf_kfunc_common_test();
    return 0;
    }
    SEC("kprobe/bpf_kfunc_common_test")
#[no_mangle]
pub unsafe extern "C" fn test2(ctx: *mut pt_regs) -> c_int {
    int test2(struct pt_regs *ctx)
    {
    return 0;
    }
    SEC("kprobe/bpf_kfunc_common_test")
#[no_mangle]
pub unsafe extern "C" fn test3(ctx: *mut pt_regs) -> c_int {
    int test3(struct pt_regs *ctx)
    {
    return 0;
    }
    SEC("kprobe/bpf_kfunc_common_test")
#[no_mangle]
pub unsafe extern "C" fn test4(ctx: *mut pt_regs) -> c_int {
    int test4(struct pt_regs *ctx)
    {
    return 0;
    }
    SEC("kprobe.multi/bpf_kfunc_common_test")
#[no_mangle]
pub unsafe extern "C" fn test5(ctx: *mut pt_regs) -> c_int {
    int test5(struct pt_regs *ctx)
    {
    return 0;
    }
    SEC("kprobe.session/bpf_kfunc_common_test")
#[no_mangle]
pub unsafe extern "C" fn test6(ctx: *mut pt_regs) -> c_int {
    int test6(struct pt_regs *ctx)
    {
    return 0;
    }
