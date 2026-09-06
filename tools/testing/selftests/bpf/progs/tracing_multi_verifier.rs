//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tracing_multi_verifier.c
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
    SEC("fentry.multi/bpf_fentry_test1")
    __failure
    __msg("func 'bpf_multi_func' doesn't have 1-th argument")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry_direct_access, a: c_int) -> c_int {
    int BPF_PROG(fentry_direct_access, int a)
    {
    return a;
    }
    SEC("fexit.multi/bpf_fentry_test3")
    __failure
    __msg("invalid bpf_context access off=24 size=8")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fexit_direct_access, a: c_char, b: c_int, c: __u64, ret: c_int) -> c_int {
    int BPF_PROG(fexit_direct_access, char a, int b, __u64 c, int ret)
    {
    return ret;
    }
    SEC("fsession.multi/bpf_fentry_test4")
    __failure
    __msg("invalid bpf_context access off=16 size=8")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fsession_direct_access, a: *mut c_void, b: c_char, c: c_int, d: __u64, ret: c_int) -> c_int {
    int BPF_PROG(fsession_direct_access, void *a, char b, int c, __u64 d, int ret)
    {
    return c;
    }
