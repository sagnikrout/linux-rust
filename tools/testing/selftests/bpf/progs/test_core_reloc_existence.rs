//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_reloc_existence.c
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
pub struct core_reloc_existence_output {
    pub a_exists: c_int,
    pub a_value: c_int,
    pub b_exists: c_int,
    pub b_value: c_int,
    pub c_exists: c_int,
    pub c_value: c_int,
    pub arr_exists: c_int,
    pub arr_value: c_int,
    pub s_exists: c_int,
    pub s_value: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_existence {
    struct {
    pub x: c_int,
    pub s: },
    pub arr: [c_int; 1],
    pub a: c_int,
    struct {
    pub b: c_int,
}

    int c;
    };
    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_core_existence(ctx: *mut c_void) -> c_int {
    int test_core_existence(void *ctx)
    {
    struct core_reloc_existence *in = (void *)&data.in;
    struct core_reloc_existence_output *out = (void *)&data.out;
    out.a_exists = bpf_core_field_exists(in.a);
    if (bpf_core_field_exists(struct core_reloc_existence, a))
    out.a_value = BPF_CORE_READ(in, a);
    else
    out.a_value = 0xff000001u;
    out.b_exists = bpf_core_field_exists(in.b);
    if (bpf_core_field_exists(struct core_reloc_existence, b))
    out.b_value = BPF_CORE_READ(in, b);
    else
    out.b_value = 0xff000002u;
    out.c_exists = bpf_core_field_exists(in.c);
    if (bpf_core_field_exists(struct core_reloc_existence, c))
    out.c_value = BPF_CORE_READ(in, c);
    else
    out.c_value = 0xff000003u;
    out.arr_exists = bpf_core_field_exists(in.arr);
    if (bpf_core_field_exists(struct core_reloc_existence, arr))
    out.arr_value = BPF_CORE_READ(in, arr[0]);
    else
    out.arr_value = 0xff000004u;
    out.s_exists = bpf_core_field_exists(in.s);
    if (bpf_core_field_exists(struct core_reloc_existence, s))
    out.s_value = BPF_CORE_READ(in, s.x);
    else
    out.s_value = 0xff000005u;
    return 0;
    }
