//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sysrq.h
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
// -*- linux-c -*-
//
// $Id: sysrq.h,v 1.3 1997/07/17 11:54:33 mj Exp $
//
// Linux Magic System Request Key Hacks
//
// (c) 1997 Martin Mares <mj@atrey.karlin.mff.cuni.cz>
//
// (c) 2000 Crutcher Dunnavant <crutcher+kernel@datastacks.com>
// overhauled to use key registration
// based upon discusions in irc://irc.openprojects.net/#kernelnewbies
//

// Possible values of bitmask for enabling sysrq functions
// 0x0001 is reserved for enable everything
pub const SYSRQ_ENABLE_LOG: c_uint = 0x0002;
pub const SYSRQ_ENABLE_KEYBOARD: c_uint = 0x0004;
pub const SYSRQ_ENABLE_DUMP: c_uint = 0x0008;
pub const SYSRQ_ENABLE_SYNC: c_uint = 0x0010;
pub const SYSRQ_ENABLE_REMOUNT: c_uint = 0x0020;
pub const SYSRQ_ENABLE_SIGNAL: c_uint = 0x0040;
pub const SYSRQ_ENABLE_BOOT: c_uint = 0x0080;
pub const SYSRQ_ENABLE_RTNICE: c_uint = 0x0100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysrq_key_op {
    pub handler)(u8): *const *const void (,
    pub help_msg: *const *const c_char,
    pub action_msg: *const *const c_char,
    pub enable_mask: c_int,
}

// Generic SysRq interface -- you may call it from any device driver, supplying
// ASCII code of the key, pointer to registers and kbd/tty structs (if they
// are available -- else NULL's).
//
extern "C" {
    pub fn handle_sysrq(key: u8);
}
extern "C" {
    pub fn __handle_sysrq(key: u8, check_mask: bool);
}
extern "C" {
    pub fn register_sysrq_key(key: u8, op: *const sysrq_key_op) -> c_int;
}
extern "C" {
    pub fn unregister_sysrq_key(key: u8, op: *const sysrq_key_op) -> c_int;
}
extern "C" {
    pub fn sysrq_toggle_support(enable_mask: c_int) -> c_int;
}
extern "C" {
    pub fn sysrq_mask() -> c_int;
}

// Magic SysRq disabled mask

