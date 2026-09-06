//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_reloc_arrays.c
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
pub struct core_reloc_arrays_output {
    pub a2: c_int,
    pub a3: c_int,
    pub b123: c_char,
    pub c1c: c_int,
    pub d00d: c_int,
    pub f01c: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays_substruct {
    pub c: c_int,
    pub d: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays {
    pub a: [c_int; 5],
    pub b: [c_char; 2][3][4],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [core_reloc_arrays_substruct; 1][2],
    pub f: [core_reloc_arrays_substruct; ][2],
}

    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_core_arrays(ctx: *mut c_void) -> c_int {
    int test_core_arrays(void *ctx)
    {
    struct core_reloc_arrays *in = (void *)&data.in;
    struct core_reloc_arrays_output *out = (void *)&data.out;
    int *a;
    if (CORE_READ(&out.a2, &in.a[2]))
    return 1;
    if (CORE_READ(&out.b123, &in.b[1][2][3]))
    return 1;
    if (CORE_READ(&out.c1c, &in.c[1].c))
    return 1;
    if (CORE_READ(&out.d00d, &in.d[0][0].d))
    return 1;
    if (CORE_READ(&out.f01c, &in.f[0][1].c))
    return 1;
    a = __builtin_preserve_access_index(({ in.a; }));
    out.a3 = a[0] + a[1] + a[2] + a[3];
    return 0;
    }
