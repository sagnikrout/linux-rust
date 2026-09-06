//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/atomic.h
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
//
// Copyright IBM Corp. 1999, 2016
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>,
// Denis Joseph Barrow,
// Arnd Bergmann,
//

extern "C" {
    pub fn __atomic_read(_arg: &v->counter) -> return;
}

extern "C" {
    pub fn __atomic_add_barrier(_arg: i, _arg: &v->counter) -> return;
}

extern "C" {
    pub fn __atomic_add_and_test_barrier(_arg: -i, _arg: &v->counter) -> return;
}

extern "C" {
    pub fn __atomic_add_const_and_test_barrier(_arg: -1, _arg: &v->counter) -> return;
}

extern "C" {
    pub fn __atomic_add_const_and_test_barrier(_arg: 1, _arg: &v->counter) -> return;
}

extern "C" {
    pub fn arch_xchg(_arg: &v->counter, _arg: new) -> return;
}

extern "C" {
    pub fn arch_cmpxchg(_arg: &v->counter, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn arch_try_cmpxchg(_arg: &v->counter, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn __atomic64_read()&v->counter: *mut (long) -> return;
}

extern "C" {
    pub fn __atomic64_add_barrier(_arg: i, )&v->counter: *mut (long) -> return;
}

extern "C" {
    pub fn __atomic64_add_and_test_barrier(_arg: -i, )&v->counter: *mut (long) -> return;
}

extern "C" {
    pub fn __atomic64_add_const_and_test_barrier(_arg: -1, )&v->counter: *mut (long) -> return;
}

extern "C" {
    pub fn __atomic64_add_const_and_test_barrier(_arg: 1, )&v->counter: *mut (long) -> return;
}

extern "C" {
    pub fn arch_xchg(_arg: &v->counter, _arg: new) -> return;
}

extern "C" {
    pub fn arch_cmpxchg(_arg: &v->counter, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn arch_try_cmpxchg(_arg: &v->counter, _arg: old, _arg: new) -> return;
}

