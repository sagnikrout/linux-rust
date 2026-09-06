//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/nospec-branch.h
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

extern "C" {
    pub fn nospec_init_branches();
}
extern "C" {
    pub fn nospec_auto_detect();
}
extern "C" {
    pub fn nospec_revert(start: *mut i32, end: *mut i32);
}
extern "C" {
    pub fn __s390_indirect_jump_r1();
}
extern "C" {
    pub fn __s390_indirect_jump_r2();
}
extern "C" {
    pub fn __s390_indirect_jump_r3();
}
extern "C" {
    pub fn __s390_indirect_jump_r4();
}
extern "C" {
    pub fn __s390_indirect_jump_r5();
}
extern "C" {
    pub fn __s390_indirect_jump_r6();
}
extern "C" {
    pub fn __s390_indirect_jump_r7();
}
extern "C" {
    pub fn __s390_indirect_jump_r8();
}
extern "C" {
    pub fn __s390_indirect_jump_r9();
}
extern "C" {
    pub fn __s390_indirect_jump_r10();
}
extern "C" {
    pub fn __s390_indirect_jump_r11();
}
extern "C" {
    pub fn __s390_indirect_jump_r12();
}
extern "C" {
    pub fn __s390_indirect_jump_r13();
}
extern "C" {
    pub fn __s390_indirect_jump_r14();
}
extern "C" {
    pub fn __s390_indirect_jump_r15();
}

