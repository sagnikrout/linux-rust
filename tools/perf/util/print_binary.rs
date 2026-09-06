//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/print_binary.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum binary_printer_ops {
    BINARY_PRINT_DATA_BEGIN,
    BINARY_PRINT_LINE_BEGIN,
    BINARY_PRINT_ADDR,
    BINARY_PRINT_NUM_DATA,
    BINARY_PRINT_NUM_PAD,
    BINARY_PRINT_SEP,
    BINARY_PRINT_CHAR_DATA,
    BINARY_PRINT_CHAR_PAD,
    BINARY_PRINT_LINE_END,
    BINARY_PRINT_DATA_END,
}

extern "C" {
    pub fn is_printable_array(p: *mut c_char, len: c_uint) -> c_int;
}
