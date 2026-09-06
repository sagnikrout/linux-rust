//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_reloc_mods.c
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
pub struct core_reloc_mods_output {
    pub h: int a, b, c, d, e, f, g,,
}

    typedef const int int_t;
    typedef const char *char_ptr_t;
    typedef const int arr_t[7];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_mods_substruct {
    pub x: c_int,
    pub y: c_int,
}

    typedef struct {
    int x;
    int y;
    } core_reloc_mods_substruct_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_mods {
    pub a: c_int,
    pub b: int_t,
    pub c: *mut c_char,
    pub d: char_ptr_t,
    pub e: [c_int; 3],
    pub f: arr_t,
    pub g: core_reloc_mods_substruct,
    pub h: core_reloc_mods_substruct_t,
}

    int __sz = sizeof(*(dst)) < sizeof(*(src)) ? sizeof(*(dst)) : \
    sizeof(*(src)); \
    bpf_core_read((char *)(dst) + sizeof(*(dst)) - __sz, __sz, \
    (const char *)(src) + sizeof(*(src)) - __sz); \
    })

    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_core_mods(ctx: *mut c_void) -> c_int {
    int test_core_mods(void *ctx)
    {
    struct core_reloc_mods *in = (void *)&data.in;
    struct core_reloc_mods_output *out = (void *)&data.out;
    if (CORE_READ(&out.a, &in.a) ||
    CORE_READ(&out.b, &in.b) ||
    CORE_READ(&out.c, &in.c) ||
    CORE_READ(&out.d, &in.d) ||
    CORE_READ(&out.e, &in.e[2]) ||
    CORE_READ(&out.f, &in.f[1]) ||
    CORE_READ(&out.g, &in.g.x) ||
    CORE_READ(&out.h, &in.h.y))
    return 1;
    return 0;
    }
