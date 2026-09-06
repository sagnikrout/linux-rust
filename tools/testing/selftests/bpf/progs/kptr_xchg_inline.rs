//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/kptr_xchg_inline.c
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
// Copyright (C) 2023. Huawei Technologies Co., Ltd

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bin_data {
    pub blob: [c_char; 32],
}

    private(kptr) struct bin_data __kptr * ptr;
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn kptr_xchg_inline() -> __naked int {
    __naked int kptr_xchg_inline(void)
    {
    asm volatile (
    "r1 = %[ptr] ll;"
    "r2 = 0;"
    "call %[bpf_kptr_xchg];"
    "if r0 == 0 goto 1f;"
    "r1 = r0;"
    "r2 = 0;"
    "call %[bpf_obj_drop];"
    "1:"
    "r0 = 0;"
    "exit;"
    :
    : __imm_addr(ptr),
    __imm(bpf_kptr_xchg),
    __imm(bpf_obj_drop)
    : __clobber_all
    );
    }
// BTF FUNC records are not generated for kfuncs referenced
// from inline assembly. These records are necessary for
// libbpf to link the program. The function below is a hack
// to ensure that BTF FUNC records are generated.
//
#[no_mangle]
pub unsafe extern "C" fn __btf_root() {
    void __btf_root(void)
    {
    bpf_obj_drop(core::ptr::null_mut());
    }
