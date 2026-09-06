//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/btf_dump_test_case_ordering.c
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
// BTF-to-C dumper test for topological sorting of dependent structs.
//
// Copyright (c) 2019 Facebook
//
// ----- START-EXPECTED-OUTPUT -----
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s1 {
    pub s3: struct,
    pub s4: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s2 {
    pub s2: *mut s2,
    pub s3: *mut s3,
    pub s4: *mut s4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3 {
    pub s1: s1,
    pub s2: s2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s4 {
    pub s1: s1,
    pub s3: s3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct callback_head {
    pub next: *mut callback_head,
    pub func: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct root_struct {
    pub s4: s4,
    pub l: list_head,
    pub n: hlist_node,
    pub h: hlist_head,
    pub cb: callback_head,
}

// ------ END-EXPECTED-OUTPUT ------
#[no_mangle]
pub unsafe extern "C" fn f(root: *mut root_struct) -> c_int {
    int f(struct root_struct *root)
    {
    return 0;
    }
