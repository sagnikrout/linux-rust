//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/char/keyboard.h
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
// ebcdic keycode functions for s390 console drivers
//
// Copyright IBM Corp. 2003
// Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com),
//

pub const NR_FN_HANDLER: c_int = 20;
extern "C" {
    pub fn void(: *mut fn_handler_fn)(struct kbd_data) -> typedef;
}
//
// FIXME: explain key_maps tricks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kbd_data {
    pub port: *mut tty_port,
    pub key_maps: *mut c_ushort,
    pub func_table: *mut c_char,
    pub fn_handler: *mut fn_handler_fn,
    pub accent_table: *mut kbdiacruc,
    pub accent_table_size: c_uint,
    pub diacr: c_uint,
    pub sysrq: c_ushort,
}

extern "C" {
    pub fn kbd_free(: *mut kbd_data);
}
extern "C" {
    pub fn kbd_ascebc(: *mut kbd_data, : *mut c_uchar);
}
extern "C" {
    pub fn kbd_keycode(: *mut kbd_data, int: unsigned);
}
extern "C" {
    pub fn kbd_ioctl(: *mut kbd_data, int: unsigned, long: unsigned) -> c_int;
}
//
// Helper Functions.
//
