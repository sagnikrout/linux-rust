//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_autocreate.c
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
    let mut test_1_result: c_int = 0;
    SEC("struct_ops/test_1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_1) -> c_int {
    int BPF_PROG(test_1)
    {
    test_1_result = 42;
    return 0;
    }
    SEC("struct_ops/test_1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_2) -> c_int {
    int BPF_PROG(test_2)
    {
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_ops___v1 {
    pub (*test_1)(void): *mut c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_ops___v2 {
    pub (*test_1)(void): *mut c_int,
    pub (*does_not_exist)(void): *mut c_int,
}

    SEC(".struct_ops.link")
    struct bpf_testmod_ops___v1 testmod_1 = {
    .test_1 = (void *)test_1
    };
    SEC(".struct_ops.link")
    struct bpf_testmod_ops___v2 testmod_2 = {
    .test_1 = (void *)test_1,
    .does_not_exist = (void *)test_2
    };
    SEC("?.struct_ops")
    struct bpf_testmod_ops___v1 optional_map = {
    .test_1 = (void *)test_1,
    };
    SEC("?.struct_ops.link")
    struct bpf_testmod_ops___v1 optional_map2 = {
    .test_1 = (void *)test_1,
    };
