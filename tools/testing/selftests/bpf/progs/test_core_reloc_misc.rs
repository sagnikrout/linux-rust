//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_reloc_misc.c
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
pub struct core_reloc_misc_output {
    pub c: int a, b,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_misc___a {
    pub a1: c_int,
    pub a2: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_misc___b {
    pub b1: c_int,
    pub b2: c_int,
}

// fixed two first members, can be extended with new fields
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_misc_extensible {
    pub a: c_int,
    pub b: c_int,
}

    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_core_misc(ctx: *mut c_void) -> c_int {
    int test_core_misc(void *ctx)
    {
    struct core_reloc_misc___a *in_a = (void *)&data.in;
    struct core_reloc_misc___b *in_b = (void *)&data.in;
    struct core_reloc_misc_extensible *in_ext = (void *)&data.in;
    struct core_reloc_misc_output *out = (void *)&data.out;
// record two different relocations with the same accessor string
    if (CORE_READ(&out.a, &in_a.a1) ||		/* accessor: 0:0 */
    CORE_READ(&out.b, &in_b.b1))		/* accessor: 0:0 */
    return 1;
// Validate relocations capture array-only accesses for structs with
// fixed header, but with potentially extendable tail. This will read
// first 4 bytes of 2nd element of in_ext array of potentially
// variably sized struct core_reloc_misc_extensible.
    if (CORE_READ(&out.c, &in_ext[2]))		/* accessor: 2 */
    return 1;
    return 0;
    }
