//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_reloc_ints.c
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
// Copyright (c) 2019 Facebook

    char _license[] SEC("license") = "GPL";
    struct {
    char in[256];
    char out[256];
    } data = {};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_ints {
    pub u8_field: u8,
    pub s8_field: i8,
    pub u16_field: u16,
    pub s16_field: i16,
    pub u32_field: u32,
    pub s32_field: i32,
    pub u64_field: u64,
    pub s64_field: i64,
}

    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_core_ints(ctx: *mut c_void) -> c_int {
    int test_core_ints(void *ctx)
    {
    struct core_reloc_ints *in = (void *)&data.in;
    struct core_reloc_ints *out = (void *)&data.out;
    if (CORE_READ(&out.u8_field, &in.u8_field) ||
    CORE_READ(&out.s8_field, &in.s8_field) ||
    CORE_READ(&out.u16_field, &in.u16_field) ||
    CORE_READ(&out.s16_field, &in.s16_field) ||
    CORE_READ(&out.u32_field, &in.u32_field) ||
    CORE_READ(&out.s32_field, &in.s32_field) ||
    CORE_READ(&out.u64_field, &in.u64_field) ||
    CORE_READ(&out.s64_field, &in.s64_field))
    return 1;
    return 0;
    }
