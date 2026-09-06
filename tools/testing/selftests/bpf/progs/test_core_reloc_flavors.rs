//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_reloc_flavors.c
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
pub struct core_reloc_flavors {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
}

// local flavor with reversed layout
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_flavors___reversed {
    pub c: c_int,
    pub b: c_int,
    pub a: c_int,
}

// local flavor with nested/overlapping layout
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_flavors___weird {
    struct {
    pub b: c_int,
}

// a and c overlap in local flavor, but this should still work
// correctly with target original flavor
//
    union {
    int a;
    int c;
    };
    };

    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_core_flavors(ctx: *mut c_void) -> c_int {
    int test_core_flavors(void *ctx)
    {
    struct core_reloc_flavors *in_orig = (void *)&data.in;
    struct core_reloc_flavors___reversed *in_rev = (void *)&data.in;
    struct core_reloc_flavors___weird *in_weird = (void *)&data.in;
    struct core_reloc_flavors *out = (void *)&data.out;
// read a using weird layout
    if (CORE_READ(&out.a, &in_weird.a))
    return 1;
// read b using reversed layout
    if (CORE_READ(&out.b, &in_rev.b))
    return 1;
// read c using original layout
    if (CORE_READ(&out.c, &in_orig.c))
    return 1;
    return 0;
    }
