//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_reloc_size.c
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
pub struct core_reloc_size_output {
    pub int_sz: c_int,
    pub int_off: c_int,
    pub struct_sz: c_int,
    pub struct_off: c_int,
    pub union_sz: c_int,
    pub union_off: c_int,
    pub arr_sz: c_int,
    pub arr_off: c_int,
    pub arr_elem_sz: c_int,
    pub arr_elem_off: c_int,
    pub ptr_sz: c_int,
    pub ptr_off: c_int,
    pub enum_sz: c_int,
    pub enum_off: c_int,
    pub float_sz: c_int,
    pub float_off: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_size {
    pub int_field: c_int,
    pub struct_field: { int x; },
    pub union_field: union { int x; },
    pub arr_field: [c_int; 4],
    pub ptr_field: *mut c_void,
    pub enum_field: enum { VALUE = 123 },
    pub float_field: float,
}

    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_core_size(ctx: *mut c_void) -> c_int {
    int test_core_size(void *ctx)
    {
    struct core_reloc_size *in = (void *)&data.in;
    struct core_reloc_size_output *out = (void *)&data.out;
    out.int_sz = bpf_core_field_size(in.int_field);
    out.int_off = bpf_core_field_offset(in.int_field);
    out.struct_sz = bpf_core_field_size(in.struct_field);
    out.struct_off = bpf_core_field_offset(in.struct_field);
    out.union_sz = bpf_core_field_size(in.union_field);
    out.union_off = bpf_core_field_offset(in.union_field);
    out.arr_sz = bpf_core_field_size(in.arr_field);
    out.arr_off = bpf_core_field_offset(in.arr_field);
    out.arr_elem_sz = bpf_core_field_size(struct core_reloc_size, arr_field[1]);
    out.arr_elem_off = bpf_core_field_offset(struct core_reloc_size, arr_field[1]);
    out.ptr_sz = bpf_core_field_size(struct core_reloc_size, ptr_field);
    out.ptr_off = bpf_core_field_offset(struct core_reloc_size, ptr_field);
    out.enum_sz = bpf_core_field_size(struct core_reloc_size, enum_field);
    out.enum_off = bpf_core_field_offset(struct core_reloc_size, enum_field);
    out.float_sz = bpf_core_field_size(struct core_reloc_size, float_field);
    out.float_off = bpf_core_field_offset(struct core_reloc_size, float_field);
    return 0;
    }
