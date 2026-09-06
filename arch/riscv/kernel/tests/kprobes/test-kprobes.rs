//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/kernel/tests/kprobes/test-kprobes.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// The magic value that all the functions in the test_kprobes_functions array return. The test
// installs kprobes into these functions, and verify that the functions still correctly return this
// value.
//
pub const KPROBE_TEST_MAGIC: c_uint = 0xcafebabe;
pub const KPROBE_TEST_MAGIC_LOWER: c_uint = 0x0000babe;
pub const KPROBE_TEST_MAGIC_UPPER: c_uint = 0xcafe0000;
// array of addresses to install kprobes
// array of functions that return KPROBE_TEST_MAGIC
extern "C" {
    pub fn long(_arg: *mut test_kprobes_functions[])(void) -> extern;
}

