//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunserialcore.h
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
// sunserialcore.h
//
// Generic SUN serial/kbd/ms layer.  Based entirely
// upon drivers/sbus/char/sunserial.h which is:
//
// Copyright (C) 1997  Eddie C. Dost  (ecd@skynet.be)
//
// Port to new UART layer is:
//
// Copyright (C) 2002 David S. Miller (davem@redhat.com)
//

// Serial keyboard defines for L1-A processing...
pub const SUNKBD_RESET: c_uint = 0xff;
pub const SUNKBD_L1: c_uint = 0x01;
pub const SUNKBD_UP: c_uint = 0x80;
pub const SUNKBD_A: c_uint = 0x4d;
extern "C" {
    pub fn suncore_mouse_baud_cflag_next(int: unsigned, : *mut c_int) -> c_uint;
}
extern "C" {
    pub fn suncore_mouse_baud_detection(char: unsigned, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn sunserial_register_minors(: *mut uart_driver, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn sunserial_unregister_minors(: *mut uart_driver, _arg: c_int);
}
