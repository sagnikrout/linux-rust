//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/trace_dummy_st_ops.c
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

    let mut val: c_int = 0;
    SEC("fentry/test_1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry_test_1, st_ops_ctx: *mut __u64) -> c_int {
    int BPF_PROG(fentry_test_1, __u64 *st_ops_ctx)
    {
    __u64 state;
// Read the traced st_ops arg1 which is a pointer
    bpf_probe_read_kernel(&state, sizeof(__u64), (void *)st_ops_ctx);
// Read state->val
    bpf_probe_read_kernel(&val, sizeof(__u32), (void *)state);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
