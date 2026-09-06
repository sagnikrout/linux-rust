//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_reloc_type_id.c
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
// some types are shared with test_core_reloc_type_based.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_struct {
    pub x: c_int,
}

    union a_union {
    int y;
    int z;
    };
    enum an_enum {
    AN_ENUM_VAL1 = 1,
    AN_ENUM_VAL2 = 2,
    AN_ENUM_VAL3 = 3,
    };
    typedef struct a_struct named_struct_typedef;
    typedef int (*func_proto_typedef)(long);
    typedef char arr_typedef[20];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_type_id_output {
    pub local_anon_struct: c_int,
    pub local_anon_union: c_int,
    pub local_anon_enum: c_int,
    pub local_anon_func_proto_ptr: c_int,
    pub local_anon_void_ptr: c_int,
    pub local_anon_arr: c_int,
    pub local_struct: c_int,
    pub local_union: c_int,
    pub local_enum: c_int,
    pub local_int: c_int,
    pub local_struct_typedef: c_int,
    pub local_func_proto_typedef: c_int,
    pub local_arr_typedef: c_int,
    pub targ_struct: c_int,
    pub targ_union: c_int,
    pub targ_enum: c_int,
    pub targ_int: c_int,
    pub targ_struct_typedef: c_int,
    pub targ_func_proto_typedef: c_int,
    pub targ_arr_typedef: c_int,
}

// preserve types even if Clang doesn't support built-in
    let mut t1: a_struct = {};
    let mut t2: union a_union = {};
    let mut t3: enum an_enum = 0;
    let mut t4: named_struct_typedef = {};
    let mut t5: func_proto_typedef = 0;
    let mut t6: arr_typedef = {};
    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_core_type_id(ctx: *mut c_void) -> c_int {
    int test_core_type_id(void *ctx)
    {
// We use __builtin_btf_type_id() in this tests, but up until the time
// __builtin_preserve_type_info() was added it contained a bug that
// would make this test fail. The bug was fixed ([0]) with addition of
// __builtin_preserve_type_info(), though, so that's what we are using
// to detect whether this test has to be executed, however strange
// that might look like.
//
// [0] https://github.com/llvm/llvm-project/commit/00602ee7ef0bf6c68d690a2bd729c12b95c95c99
//

    struct core_reloc_type_id_output *out = (void *)&data.out;
    out.local_anon_struct = bpf_core_type_id_local(struct { int marker_field; });
    out.local_anon_union = bpf_core_type_id_local(union { int marker_field; });
    out.local_anon_enum = bpf_core_type_id_local(enum { MARKER_ENUM_VAL = 123 });
    out.local_anon_func_proto_ptr = bpf_core_type_id_local(_Bool(*)(int));
    out.local_anon_void_ptr = bpf_core_type_id_local(void *);
    out.local_anon_arr = bpf_core_type_id_local(_Bool[47]);
    out.local_struct = bpf_core_type_id_local(struct a_struct);
    out.local_union = bpf_core_type_id_local(union a_union);
    out.local_enum = bpf_core_type_id_local(enum an_enum);
    out.local_int = bpf_core_type_id_local(int);
    out.local_struct_typedef = bpf_core_type_id_local(named_struct_typedef);
    out.local_func_proto_typedef = bpf_core_type_id_local(func_proto_typedef);
    out.local_arr_typedef = bpf_core_type_id_local(arr_typedef);
    out.targ_struct = bpf_core_type_id_kernel(struct a_struct);
    out.targ_union = bpf_core_type_id_kernel(union a_union);
    out.targ_enum = bpf_core_type_id_kernel(enum an_enum);
    out.targ_int = bpf_core_type_id_kernel(int);
    out.targ_struct_typedef = bpf_core_type_id_kernel(named_struct_typedef);
    out.targ_func_proto_typedef = bpf_core_type_id_kernel(func_proto_typedef);
    out.targ_arr_typedef = bpf_core_type_id_kernel(arr_typedef);

    data.skip = true;

    return 0;
    }
