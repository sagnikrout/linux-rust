//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/btf_data.c
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct S {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
}

    union U {
    int	a;
    int	b;
    int	c;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct S1 {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
}

    union U1 {
    int	a;
    int	b;
    int	c;
    };
    typedef int T;
    typedef int S;
    typedef int U;
    typedef int T1;
    typedef int S1;
    typedef int U1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct root_struct {
    pub m_1: S,
    pub m_2: T,
    pub m_3: U,
    pub m_4: S1,
    pub m_5: T1,
    pub m_6: U1,
    pub m_7: S,
    pub m_8: S1,
    pub m_9: union U,
    pub m_10: union U1,
}

#[no_mangle]
pub unsafe extern "C" fn func(root: *mut root_struct) -> c_int {
    int func(struct root_struct *root)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kfunc_a(root: *mut root_struct) -> c_int {
    int kfunc_a(struct root_struct *root)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kfunc_b(root: *mut root_struct) -> c_int {
    int kfunc_b(struct root_struct *root)
    {
    return 0;
    }
    struct root_struct *kfunc_c(struct root_struct *a, struct root_struct *b)
    {
    return a;
    }
#[no_mangle]
pub unsafe extern "C" fn kfunc_d(a: *mut root_struct, b: *mut root_struct) -> c_int {
    int kfunc_d(struct root_struct *a, struct root_struct *b)
    {
    return 0;
    }
    int kfunc_e(struct root_struct *a__arena,
    struct root_struct *b__arena__nullable,
    struct root_struct *c__arena,
    struct root_struct *d__arena__nullable,
    struct root_struct *e__arena)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kfunc_f(a: *mut root_struct, b__arena: *mut root_struct, flags: c_int) -> c_int {
    int kfunc_f(struct root_struct *a, struct root_struct *b__arena, int flags)
    {
    return 0;
    }
    struct root_struct *kfunc_g(struct root_struct *a__arena,
    struct root_struct *b__arena__nullable)
    {
    return a__arena;
    }
