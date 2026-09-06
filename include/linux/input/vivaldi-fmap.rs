//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/input/vivaldi-fmap.h
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

pub const VIVALDI_MAX_FUNCTION_ROW_KEYS: c_int = 24;
//
// struct vivaldi_data - Function row map data for ChromeOS Vivaldi keyboards
// @function_row_physmap: An array of scancodes or their equivalent (HID usage
// codes, encoded rows/columns, etc) for the top
// row function keys, in an order from left to right
// @num_function_row_keys: The number of top row keys in a custom keyboard
//
// This structure is supposed to be used by ChromeOS keyboards using
// the Vivaldi keyboard function row design.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vivaldi_data {
    pub function_row_physmap: [u32; VIVALDI_MAX_FUNCTION_ROW_KEYS],
    pub num_function_row_keys: c_uint,
}
