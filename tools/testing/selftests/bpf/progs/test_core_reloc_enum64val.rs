//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_reloc_enum64val.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    struct {
    char in[256];
    char out[256];
    bool skip;
    } data = {};
    enum named_unsigned_enum64 {
    UNSIGNED_ENUM64_VAL1 = 0x1ffffffffULL,
    UNSIGNED_ENUM64_VAL2 = 0x2ffffffffULL,
    UNSIGNED_ENUM64_VAL3 = 0x3ffffffffULL,
    };
    enum named_signed_enum64 {
    SIGNED_ENUM64_VAL1 = 0x1ffffffffLL,
    SIGNED_ENUM64_VAL2 = -2,
    SIGNED_ENUM64_VAL3 = 0x3ffffffffLL,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_enum64val_output {
    pub unsigned_val1_exists: bool,
    pub unsigned_val2_exists: bool,
    pub unsigned_val3_exists: bool,
    pub signed_val1_exists: bool,
    pub signed_val2_exists: bool,
    pub signed_val3_exists: bool,
    pub unsigned_val1: c_long,
    pub unsigned_val2: c_long,
    pub signed_val1: c_long,
    pub signed_val2: c_long,
}

    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_core_enum64val(ctx: *mut c_void) -> c_int {
    int test_core_enum64val(void *ctx)
    {

    struct core_reloc_enum64val_output *out = (void *)&data.out;
    let mut named_unsigned: enum named_unsigned_enum64 = 0;
    let mut named_signed: enum named_signed_enum64 = 0;
    out.unsigned_val1_exists = bpf_core_enum_value_exists(named_unsigned, UNSIGNED_ENUM64_VAL1);
    out.unsigned_val2_exists = bpf_core_enum_value_exists(enum named_unsigned_enum64, UNSIGNED_ENUM64_VAL2);
    out.unsigned_val3_exists = bpf_core_enum_value_exists(enum named_unsigned_enum64, UNSIGNED_ENUM64_VAL3);
    out.signed_val1_exists = bpf_core_enum_value_exists(named_signed, SIGNED_ENUM64_VAL1);
    out.signed_val2_exists = bpf_core_enum_value_exists(enum named_signed_enum64, SIGNED_ENUM64_VAL2);
    out.signed_val3_exists = bpf_core_enum_value_exists(enum named_signed_enum64, SIGNED_ENUM64_VAL3);
    out.unsigned_val1 = bpf_core_enum_value(named_unsigned, UNSIGNED_ENUM64_VAL1);
    out.unsigned_val2 = bpf_core_enum_value(named_unsigned, UNSIGNED_ENUM64_VAL2);
    out.signed_val1 = bpf_core_enum_value(named_signed, SIGNED_ENUM64_VAL1);
    out.signed_val2 = bpf_core_enum_value(named_signed, SIGNED_ENUM64_VAL2);
// NAMED_ENUM64_VAL3 value is optional

    data.skip = true;

    return 0;
    }
