//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/btf_dump_test_case_padding.c
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
// BTF-to-C dumper tests for implicit and explicit padding between fields and
// at the end of a struct.
//
// Copyright (c) 2019 Facebook
//
// ----- START-EXPECTED-OUTPUT -----
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padded_implicitly {
    pub a: c_int,
    pub b: c_long,
    pub c: c_char,
}

// ------ END-EXPECTED-OUTPUT ------
// ----- START-EXPECTED-OUTPUT -----
//
// struct padded_explicitly {
// int a;
// long: 0;
// int b;
// };
//
// ------ END-EXPECTED-OUTPUT ------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padded_explicitly {
    pub a: c_int,
    pub /: *mut *mut int: 1; / algo will emit aligning `long: 0;` here,
    pub b: c_int,
}

// ----- START-EXPECTED-OUTPUT -----
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padded_a_lot {
    pub a: c_int,
    pub 64: long:,
    pub 64: long:,
    pub b: c_int,
}

// ------ END-EXPECTED-OUTPUT ------
// ----- START-EXPECTED-OUTPUT -----
//
// struct padded_cache_line {
// int a;
// long: 64;
// int b;
// long: 64;
// };
//
// ------ END-EXPECTED-OUTPUT ------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padded_cache_line {
    pub a: c_int,
    pub __attribute__((aligned(32))): int b,
}

// ----- START-EXPECTED-OUTPUT -----
//
// struct zone_padding {
// char x[0];
// };
//
// struct zone {
// int a;
// short b;
// long: 0;
// struct zone_padding __pad__;
// };
//
// ------ END-EXPECTED-OUTPUT ------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zone_padding {
    pub x: [c_char; 0],
    pub __attribute__((__aligned__(8))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zone {
    pub a: c_int,
    pub b: c_short,
    pub __pad__: zone_padding,
}

// ----- START-EXPECTED-OUTPUT -----
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padding_wo_named_members {
    pub 64: long:,
    pub 64: long:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct padding_weird_1 {
    pub a: c_int,
    pub 64: long:,
    pub 16: short:,
    pub b: c_short,
}

// ------ END-EXPECTED-OUTPUT ------
// ----- START-EXPECTED-OUTPUT -----
//
// struct padding_weird_2 {
// long: 56;
// char a;
// long: 56;
// char b;
// char: 8;
// };
//
// ------ END-EXPECTED-OUTPUT ------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padding_weird_2 {
    pub /: *mut *mut int: 32; / these paddings will be collapsed into `long: 56;`,
    pub 16: short:,
    pub 8: char:,
    pub a: c_char,
    pub /: *mut *mut int: 32; / these paddings will be collapsed into `long: 56;`,
    pub 16: short:,
    pub 8: char:,
    pub b: c_char,
    pub 8: char:,
}

// ----- START-EXPECTED-OUTPUT -----
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exact_1byte {
    pub x: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct padded_1byte {
    pub 8: char:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exact_2bytes {
    pub x: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct padded_2bytes {
    pub 16: short:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exact_4bytes {
    pub x: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct padded_4bytes {
    pub 32: int:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exact_8bytes {
    pub x: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct padded_8bytes {
    pub 64: long:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff_periodic_effect {
    pub 32: int:,
    pub magnitude: c_short,
    pub 0: long:,
    pub phase: c_short,
    pub 0: long:,
    pub 32: int:,
    pub custom_len: c_int,
    pub custom_data: *mut c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_wc {
    pub 64: long:,
    pub 64: long:,
    pub 32: int:,
    pub byte_len: c_int,
    pub qp: *mut c_void,
    pub ex: union {},
    pub 64: long:,
    pub slid: c_int,
    pub wc_flags: c_int,
    pub 64: long:,
    pub smac: [c_char; 6],
    pub 0: long:,
    pub network_hdr_type: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_method {
    pub 64: long:,
    pub 8: char:,
    pub type: c_char,
    pub reference_count: c_short,
    pub flags: c_char,
    pub 0: short:,
    pub 8: char:,
    pub sync_level: c_char,
    pub 64: long:,
    pub node: *mut c_void,
    pub aml_start: *mut c_void,
    pub dispatch: union {},
    pub 64: long:,
    pub aml_length: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nested_unpacked {
    pub x: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nested_packed {
    pub a: nested_unpacked,
    pub c: c_char,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct outer_mixed_but_unpacked {
    pub b1: nested_packed,
    pub a1: c_short,
    pub b2: nested_packed,
}

// ------ END-EXPECTED-OUTPUT ------
    int f(struct {
    struct padded_implicitly _1;
    struct padded_explicitly _2;
    struct padded_a_lot _3;
    struct padded_cache_line _4;
    struct zone _5;
    struct padding_wo_named_members _6;
    struct padding_weird_1 _7;
    struct padding_weird_2 _8;
    struct exact_1byte _100;
    struct padded_1byte _101;
    struct exact_2bytes _102;
    struct padded_2bytes _103;
    struct exact_4bytes _104;
    struct padded_4bytes _105;
    struct exact_8bytes _106;
    struct padded_8bytes _107;
    struct ff_periodic_effect _200;
    struct ib_wc _201;
    struct acpi_object_method _202;
    struct outer_mixed_but_unpacked _203;
    } *_)
    {
    return 0;
    }
