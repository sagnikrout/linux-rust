//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/btf_dump_test_case_packing.c
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
// BTF-to-C dumper tests for struct packing determination.
//
// Copyright (c) 2019 Facebook
//
// ----- START-EXPECTED-OUTPUT -----
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_trailing_space {
    pub a: c_int,
    pub b: c_short,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct non_packed_trailing_space {
    pub a: c_int,
    pub b: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packed_fields {
    pub a: c_short,
    pub b: c_int,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct non_packed_fields {
    pub a: c_short,
    pub b: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nested_packed {
    pub 4: char:,
    pub 4: int a:,
    pub b: c_long,
    struct {
    pub c: c_char,
    pub d: c_int,
// C attribute field omitted
    pub __attribute__((packed)): },
    union union_is_never_packed {
    pub 4: int a:,
    pub b: c_char,
    pub 1: char c:,
}

    union union_does_not_need_packing {
    struct {
    long a;
    int b;
    } __attribute__((packed));
    int c;
    };
    union jump_code_union {
    char code[5];
    struct {
    char jump;
    int offset;
    } __attribute__((packed));
    };
// ----- START-EXPECTED-OUTPUT -----
//
// struct nested_packed_but_aligned_struct {
// int x1;
// int x2;
// };
//
// struct outer_implicitly_packed_struct {
// char y1;
// struct nested_packed_but_aligned_struct y2;
// } __attribute__((packed));
//
// ------ END-EXPECTED-OUTPUT ------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nested_packed_but_aligned_struct {
    pub x1: c_int,
    pub x2: c_int,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct outer_implicitly_packed_struct {
    pub y1: c_char,
    pub y2: nested_packed_but_aligned_struct,
}

// ----- START-EXPECTED-OUTPUT -----
//
// struct usb_ss_ep_comp_descriptor {
// char: 8;
// char bDescriptorType;
// char bMaxBurst;
// short wBytesPerInterval;
// };
//
// struct usb_host_endpoint {
// long: 64;
// char: 8;
// struct usb_ss_ep_comp_descriptor ss_ep_comp;
// long: 0;
// } __attribute__((packed));
//
// ------ END-EXPECTED-OUTPUT ------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ss_ep_comp_descriptor {
    pub 8: char:,
    pub bDescriptorType: c_char,
    pub bMaxBurst: c_char,
    pub 0: int:,
    pub wBytesPerInterval: c_short,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_host_endpoint {
    pub 64: long:,
    pub 8: char:,
    pub ss_ep_comp: usb_ss_ep_comp_descriptor,
    pub 0: long:,
}

// ----- START-EXPECTED-OUTPUT -----
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nested_packed_struct {
    pub a: c_int,
    pub b: c_char,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct outer_nonpacked_struct {
    pub a: c_short,
    pub b: nested_packed_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct outer_packed_struct {
    pub a: c_short,
    pub b: nested_packed_struct,
    pub __attribute__((packed)): },
// ------ END-EXPECTED-OUTPUT ------
    int f(struct {
    pub _1: packed_trailing_space,
    pub _2: non_packed_trailing_space,
    pub _3: packed_fields,
    pub _4: non_packed_fields,
    pub _5: nested_packed,
    pub _6: union union_is_never_packed,
    pub _7: union union_does_not_need_packing,
    pub _8: union jump_code_union,
    pub _9: outer_implicitly_packed_struct,
    pub _10: usb_host_endpoint,
    pub _11: outer_nonpacked_struct,
    pub _12: outer_packed_struct,
    } *_)
    {
    pub 0: return,
    }
