//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_default_trusted_ptr.c
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
//
// Copyright 2026 Google LLC.
//

    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_default_trusted_ptr(ctx: *mut c_void) -> c_int {
    int test_default_trusted_ptr(void *ctx)
    {
    struct prog_test_member *trusted_ptr;
    trusted_ptr = bpf_kfunc_get_default_trusted_ptr_test();
//
// Test BPF kfunc bpf_get_default_trusted_ptr_test() returns a
// PTR_TO_BTF_ID | PTR_TRUSTED, therefore it should be accepted when
// passed to a BPF kfunc only accepting KF_TRUSTED_ARGS.
//
    bpf_kfunc_put_default_trusted_ptr_test(trusted_ptr);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
