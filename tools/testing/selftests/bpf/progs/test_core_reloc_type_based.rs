//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_reloc_type_based.c
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
// Copyright (c) 2020 Facebook

    char _license[] SEC("license") = "GPL";
    struct {
    char in[256];
    char out[256];
    bool skip;
    } data = {};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_struct {
    pub x: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_complex_struct {
    union {
    pub a: *mut a_struct,
    pub b: *mut c_void,
    pub x: },
    pub y: volatile long,
}

    union a_union {
    int y;
    int z;
    };
    typedef struct a_struct named_struct_typedef;
    typedef struct { int x, y, z; } anon_struct_typedef;
    typedef struct {
    int a, b, c;
    } *struct_ptr_typedef;
    enum an_enum {
    AN_ENUM_VAL1 = 1,
    AN_ENUM_VAL2 = 2,
    AN_ENUM_VAL3 = 3,
    };
    typedef int int_typedef;
    typedef enum { TYPEDEF_ENUM_VAL1, TYPEDEF_ENUM_VAL2 } enum_typedef;
    typedef void *void_ptr_typedef;
    typedef int *restrict restrict_ptr_typedef;
    typedef int (*func_proto_typedef)(long);
    typedef char arr_typedef[20];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_type_based_output {
    pub struct_exists: bool,
    pub complex_struct_exists: bool,
    pub union_exists: bool,
    pub enum_exists: bool,
    pub typedef_named_struct_exists: bool,
    pub typedef_anon_struct_exists: bool,
    pub typedef_struct_ptr_exists: bool,
    pub typedef_int_exists: bool,
    pub typedef_enum_exists: bool,
    pub typedef_void_ptr_exists: bool,
    pub typedef_restrict_ptr_exists: bool,
    pub typedef_func_proto_exists: bool,
    pub typedef_arr_exists: bool,
    pub struct_matches: bool,
    pub complex_struct_matches: bool,
    pub union_matches: bool,
    pub enum_matches: bool,
    pub typedef_named_struct_matches: bool,
    pub typedef_anon_struct_matches: bool,
    pub typedef_struct_ptr_matches: bool,
    pub typedef_int_matches: bool,
    pub typedef_enum_matches: bool,
    pub typedef_void_ptr_matches: bool,
    pub typedef_restrict_ptr_matches: bool,
    pub typedef_func_proto_matches: bool,
    pub typedef_arr_matches: bool,
    pub struct_sz: c_int,
    pub union_sz: c_int,
    pub enum_sz: c_int,
    pub typedef_named_struct_sz: c_int,
    pub typedef_anon_struct_sz: c_int,
    pub typedef_struct_ptr_sz: c_int,
    pub typedef_int_sz: c_int,
    pub typedef_enum_sz: c_int,
    pub typedef_void_ptr_sz: c_int,
    pub typedef_func_proto_sz: c_int,
    pub typedef_arr_sz: c_int,
}

    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_core_type_based(ctx: *mut c_void) -> c_int {
    int test_core_type_based(void *ctx)
    {
// Support for the BPF_TYPE_MATCHES argument to the
// __builtin_preserve_type_info builtin was added at some point during
// development of clang 15 and it's what we require for this test. Part of it
// could run with merely __builtin_preserve_type_info (which could be checked
// separately), but we have to find an upper bound.
//

    struct core_reloc_type_based_output *out = (void *)&data.out;
    out.struct_exists = bpf_core_type_exists(struct a_struct);
    out.complex_struct_exists = bpf_core_type_exists(struct a_complex_struct);
    out.union_exists = bpf_core_type_exists(union a_union);
    out.enum_exists = bpf_core_type_exists(enum an_enum);
    out.typedef_named_struct_exists = bpf_core_type_exists(named_struct_typedef);
    out.typedef_anon_struct_exists = bpf_core_type_exists(anon_struct_typedef);
    out.typedef_struct_ptr_exists = bpf_core_type_exists(struct_ptr_typedef);
    out.typedef_int_exists = bpf_core_type_exists(int_typedef);
    out.typedef_enum_exists = bpf_core_type_exists(enum_typedef);
    out.typedef_void_ptr_exists = bpf_core_type_exists(void_ptr_typedef);
    out.typedef_restrict_ptr_exists = bpf_core_type_exists(restrict_ptr_typedef);
    out.typedef_func_proto_exists = bpf_core_type_exists(func_proto_typedef);
    out.typedef_arr_exists = bpf_core_type_exists(arr_typedef);
    out.struct_matches = bpf_core_type_matches(struct a_struct);
    out.complex_struct_matches = bpf_core_type_matches(struct a_complex_struct);
    out.union_matches = bpf_core_type_matches(union a_union);
    out.enum_matches = bpf_core_type_matches(enum an_enum);
    out.typedef_named_struct_matches = bpf_core_type_matches(named_struct_typedef);
    out.typedef_anon_struct_matches = bpf_core_type_matches(anon_struct_typedef);
    out.typedef_struct_ptr_matches = bpf_core_type_matches(struct_ptr_typedef);
    out.typedef_int_matches = bpf_core_type_matches(int_typedef);
    out.typedef_enum_matches = bpf_core_type_matches(enum_typedef);
    out.typedef_void_ptr_matches = bpf_core_type_matches(void_ptr_typedef);
    out.typedef_restrict_ptr_matches = bpf_core_type_matches(restrict_ptr_typedef);
    out.typedef_func_proto_matches = bpf_core_type_matches(func_proto_typedef);
    out.typedef_arr_matches = bpf_core_type_matches(arr_typedef);
    out.struct_sz = bpf_core_type_size(struct a_struct);
    out.union_sz = bpf_core_type_size(union a_union);
    out.enum_sz = bpf_core_type_size(enum an_enum);
    out.typedef_named_struct_sz = bpf_core_type_size(named_struct_typedef);
    out.typedef_anon_struct_sz = bpf_core_type_size(anon_struct_typedef);
    out.typedef_struct_ptr_sz = bpf_core_type_size(struct_ptr_typedef);
    out.typedef_int_sz = bpf_core_type_size(int_typedef);
    out.typedef_enum_sz = bpf_core_type_size(enum_typedef);
    out.typedef_void_ptr_sz = bpf_core_type_size(void_ptr_typedef);
    out.typedef_func_proto_sz = bpf_core_type_size(func_proto_typedef);
    out.typedef_arr_sz = bpf_core_type_size(arr_typedef);

    data.skip = true;

    return 0;
    }
