//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_tp_btf_nullable.c
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

    SEC("tp_btf/bpf_testmod_test_nullable_bare_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('trusted_ptr_or_null_'": "R1 invalid mem access) -> __failure {
    __failure __msg("R1 invalid mem access 'trusted_ptr_or_null_'")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handle_tp_btf_nullable_bare1, nullable_ctx: *mut bpf_testmod_test_read_ctx) -> c_int {
    int BPF_PROG(handle_tp_btf_nullable_bare1, struct bpf_testmod_test_read_ctx *nullable_ctx)
    {
    return nullable_ctx.len;
    }
    SEC("tp_btf/bpf_testmod_test_nullable_bare_tp")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handle_tp_btf_nullable_bare2, nullable_ctx: *mut bpf_testmod_test_read_ctx) -> c_int {
    int BPF_PROG(handle_tp_btf_nullable_bare2, struct bpf_testmod_test_read_ctx *nullable_ctx)
    {
    if (nullable_ctx)
    return nullable_ctx.len;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
