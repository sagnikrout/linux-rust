//! Automatically rewritten from C to Rust
//! Source: scripts/gen_packed_field_checks.c
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
// Copyright (c) 2024, Intel Corporation

pub const MAX_PACKED_FIELD_SIZE: c_int = 50;
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
// The first macro doesn't need a 'do {} while(0)' loop
    printf("#define CHECK_PACKED_FIELDS_1(fields) \\\n");
    printf("\tCHECK_PACKED_FIELD(fields, 0)\n\n");
// Remaining macros require a do/while loop, and are implemented
// recursively by calling the previous iteration's macro.
//
    for (int i = 2; i <= MAX_PACKED_FIELD_SIZE; i++) {
    printf("#define CHECK_PACKED_FIELDS_%d(fields) do { \\\n", i);
    printf("\tCHECK_PACKED_FIELDS_%d(fields); \\\n", i - 1);
    printf("\tCHECK_PACKED_FIELD(fields, %d); \\\n", i - 1);
    printf("} while (0)\n\n");
    }
    printf("#define CHECK_PACKED_FIELDS(fields) \\\n");
    for (int i = 1; i <= MAX_PACKED_FIELD_SIZE; i++)
    printf("\t__builtin_choose_expr(ARRAY_SIZE(fields) == %d, ({ CHECK_PACKED_FIELDS_%d(fields); }), \\\n",
    i, i);
    printf("\t({ BUILD_BUG_ON_MSG(1, \"CHECK_PACKED_FIELDS() must be regenerated to support array sizes larger than %d.\"); }) \\\n",
    MAX_PACKED_FIELD_SIZE);
    for (int i = 1; i <= MAX_PACKED_FIELD_SIZE; i++)
    printf(")");
    printf("\n");
    }
