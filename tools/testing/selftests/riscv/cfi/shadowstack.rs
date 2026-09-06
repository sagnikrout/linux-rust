//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/riscv/cfi/shadowstack.h
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


// SPDX-License-Identifier: GPL-2.0-only

//
// A CFI test returns true for success or false for fail.
// Takes a test number to index into array, and a void pointer.
//
extern "C" {
    pub fn bool(test_num: *mut *mut shstk_test_func)(unsigned long, : *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shadow_stack_tests {
    pub name: *mut c_char,
    pub t_func: shstk_test_func,
}

extern "C" {
    pub fn shadow_stack_fork_test(test_num: c_ulong, ctx: *mut c_void) -> bool;
}
extern "C" {
    pub fn shadow_stack_map_test(test_num: c_ulong, ctx: *mut c_void) -> bool;
}
extern "C" {
    pub fn shadow_stack_protection_test(test_num: c_ulong, ctx: *mut c_void) -> bool;
}
extern "C" {
    pub fn shadow_stack_gup_tests(test_num: c_ulong, ctx: *mut c_void) -> bool;
}
extern "C" {
    pub fn shadow_stack_signal_test(test_num: c_ulong, ctx: *mut c_void) -> bool;
}
extern "C" {
    pub fn execute_shadow_stack_tests() -> c_int;
}
