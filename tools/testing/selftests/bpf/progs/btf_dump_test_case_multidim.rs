//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/btf_dump_test_case_multidim.c
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
// BTF-to-C dumper test for multi-dimensional array output.
//
// Copyright (c) 2019 Facebook
//
// ----- START-EXPECTED-OUTPUT -----
    typedef int arr_t[2];
    typedef int multiarr_t[3][4][5];
    typedef int *ptr_arr_t[6];
    typedef int *ptr_multiarr_t[7][8][9][10];
    typedef int * (*fn_ptr_arr_t[11])(void);
    typedef int * (*fn_ptr_multiarr_t[12][13])(void);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct root_struct {
    pub _1: arr_t,
    pub _2: multiarr_t,
    pub _3: ptr_arr_t,
    pub _4: ptr_multiarr_t,
    pub _5: fn_ptr_arr_t,
    pub _6: fn_ptr_multiarr_t,
}

// ------ END-EXPECTED-OUTPUT ------
#[no_mangle]
pub unsafe extern "C" fn f(s: *mut root_struct) -> c_int {
    int f(struct root_struct *s)
    {
    return 0;
    }
