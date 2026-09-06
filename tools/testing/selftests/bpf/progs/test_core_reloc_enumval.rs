//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_reloc_enumval.c
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
    enum named_enum {
    NAMED_ENUM_VAL1 = 1,
    NAMED_ENUM_VAL2 = 2,
    NAMED_ENUM_VAL3 = 3,
    };
    typedef enum {
    ANON_ENUM_VAL1 = 0x10,
    ANON_ENUM_VAL2 = 0x20,
    ANON_ENUM_VAL3 = 0x30,
    } anon_enum;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_enumval_output {
    pub named_val1_exists: bool,
    pub named_val2_exists: bool,
    pub named_val3_exists: bool,
    pub anon_val1_exists: bool,
    pub anon_val2_exists: bool,
    pub anon_val3_exists: bool,
    pub named_val1: c_int,
    pub named_val2: c_int,
    pub anon_val1: c_int,
    pub anon_val2: c_int,
}

    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_core_enumval(ctx: *mut c_void) -> c_int {
    int test_core_enumval(void *ctx)
    {

    struct core_reloc_enumval_output *out = (void *)&data.out;
    let mut named: enum named_enum = 0;
    let mut anon: anon_enum = 0;
    out.named_val1_exists = bpf_core_enum_value_exists(named, NAMED_ENUM_VAL1);
    out.named_val2_exists = bpf_core_enum_value_exists(enum named_enum, NAMED_ENUM_VAL2);
    out.named_val3_exists = bpf_core_enum_value_exists(enum named_enum, NAMED_ENUM_VAL3);
    out.anon_val1_exists = bpf_core_enum_value_exists(anon, ANON_ENUM_VAL1);
    out.anon_val2_exists = bpf_core_enum_value_exists(anon_enum, ANON_ENUM_VAL2);
    out.anon_val3_exists = bpf_core_enum_value_exists(anon_enum, ANON_ENUM_VAL3);
    out.named_val1 = bpf_core_enum_value(named, NAMED_ENUM_VAL1);
    out.named_val2 = bpf_core_enum_value(named, NAMED_ENUM_VAL2);
// NAMED_ENUM_VAL3 value is optional
    out.anon_val1 = bpf_core_enum_value(anon, ANON_ENUM_VAL1);
    out.anon_val2 = bpf_core_enum_value(anon, ANON_ENUM_VAL2);
// ANON_ENUM_VAL3 value is optional

    data.skip = true;

    return 0;
    }
