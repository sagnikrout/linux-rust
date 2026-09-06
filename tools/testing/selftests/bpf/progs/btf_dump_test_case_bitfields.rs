//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/btf_dump_test_case_bitfields.c
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// BTF-to-C dumper tests for bitfield.
//
// Copyright (c) 2019 Facebook
//

// ----- START-EXPECTED-OUTPUT -----
//
// struct bitfields_only_mixed_types {
// int a: 3;
// long b: 2;
// _Bool c: 1;
// enum {
// A = 0,
// B = 1,
// } d: 1;
// short e: 5;
// int: 20;
// unsigned int f: 30;
// };
//
// ------ END-EXPECTED-OUTPUT ------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitfields_only_mixed_types {
    pub 3: int a:,
    pub 2: long b:,
    pub /: *mut *mut bool c: 1; / it's really a _Bool type,
    enum {
    A, /* A = 0, dumper is very explicit */
    B, /* B = 1, same */
    pub 1: } d:,
    pub 5: short e:,
// 20-bit padding here
    pub /: *mut *mut unsigned f: 30; / this gets aligned on 4-byte boundary,
}

// ----- START-EXPECTED-OUTPUT -----
//
// struct bitfield_mixed_with_others {
// char: 4;
// int a: 4;
// short b;
// long c;
// long d: 8;
// int e;
// int f;
// };
//
// ------ END-EXPECTED-OUTPUT ------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitfield_mixed_with_others {
    pub /: *mut *mut char: 4; / char is enough as a backing field,
    pub 4: int a:,
// 8-bit implicit padding
    pub /: *mut *mut short b; / combined with previous bitfield,
// 4 more bytes of implicit padding
    pub c: c_long,
    pub 8: long d:,
// 24 bits implicit padding
    pub /: *mut *mut int e; / combined with previous bitfield,
    pub f: c_int,
// 4 bytes of padding
}

// ----- START-EXPECTED-OUTPUT -----
//
// struct bitfield_flushed {
// int a: 4;
// long: 60;
// long b: 16;
// };
//
// ------ END-EXPECTED-OUTPUT ------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitfield_flushed {
    pub 4: int a:,
    pub /: *mut *mut long: 0; / flush until next natural alignment boundary,
    pub 16: long b:,
}

    int f(struct {
    struct bitfields_only_mixed_types _1;
    struct bitfield_mixed_with_others _2;
    struct bitfield_flushed _3;
    } *_)
    {
    return 0;
    }
