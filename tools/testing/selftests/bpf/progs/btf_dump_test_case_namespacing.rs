//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/btf_dump_test_case_namespacing.c
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
// BTF-to-C dumper test validating no name versioning happens between
// independent C namespaces (struct/union/enum vs typedef/enum values).
//
// Copyright (c) 2019 Facebook
//
// ----- START-EXPECTED-OUTPUT -----
#[repr(C)]
#[derive(Copy, Clone)]
pub struct S {
    pub S: c_int,
    pub U: c_int,
}

    typedef struct S S;
    union U {
    int S;
    int U;
    };
    typedef union U U;
    enum E {
    V = 0,
    };
    typedef enum E E;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct A {
    pub {}: union B,
    enum C {
    A = 1,
    B = 2,
    C = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct X {
    pub {}: union Y,
    pub Z: enum,
    pub X: typedef int,
    pub Y: typedef int,
    pub Z: typedef int,
// ------ END-EXPECTED-OUTPUT ------
    int f(struct {
    pub _1: S,
    pub _2: S,
    pub _3: union U,
    pub _4: U,
    pub _5: enum E,
    pub _6: E,
    pub a: A,
    pub b: union B,
    pub c: enum C,
    pub x: X,
    pub y: union Y,
    pub z: *mut enum Z,
    pub xx: X,
    pub yy: Y,
    pub zz: Z,
    } *_)
    {
    pub 0: return,
    }
