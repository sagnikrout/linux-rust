//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/i8042.h
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
// Standard commands.
//
pub const I8042_CMD_CTL_RCTR: c_uint = 0x0120;
pub const I8042_CMD_CTL_WCTR: c_uint = 0x1060;
pub const I8042_CMD_CTL_TEST: c_uint = 0x01aa;
pub const I8042_CMD_KBD_DISABLE: c_uint = 0x00ad;
pub const I8042_CMD_KBD_ENABLE: c_uint = 0x00ae;
pub const I8042_CMD_KBD_TEST: c_uint = 0x01ab;
pub const I8042_CMD_KBD_LOOP: c_uint = 0x11d2;
pub const I8042_CMD_AUX_DISABLE: c_uint = 0x00a7;
pub const I8042_CMD_AUX_ENABLE: c_uint = 0x00a8;
pub const I8042_CMD_AUX_TEST: c_uint = 0x01a9;
pub const I8042_CMD_AUX_SEND: c_uint = 0x10d4;
pub const I8042_CMD_AUX_LOOP: c_uint = 0x11d3;
pub const I8042_CMD_MUX_PFX: c_uint = 0x0090;
pub const I8042_CMD_MUX_SEND: c_uint = 0x1090;
//
// Status register bits.
//
pub const I8042_STR_PARITY: c_uint = 0x80;
pub const I8042_STR_TIMEOUT: c_uint = 0x40;
pub const I8042_STR_AUXDATA: c_uint = 0x20;
pub const I8042_STR_KEYLOCK: c_uint = 0x10;
pub const I8042_STR_CMDDAT: c_uint = 0x08;
pub const I8042_STR_MUXERR: c_uint = 0x04;
pub const I8042_STR_IBF: c_uint = 0x02;
pub const I8042_STR_OBF: c_uint = 0x01;
//
// Control register bits.
//
pub const I8042_CTR_KBDINT: c_uint = 0x01;
pub const I8042_CTR_AUXINT: c_uint = 0x02;
pub const I8042_CTR_IGNKEYLOCK: c_uint = 0x08;
pub const I8042_CTR_KBDDIS: c_uint = 0x10;
pub const I8042_CTR_AUXDIS: c_uint = 0x20;
pub const I8042_CTR_XLATE: c_uint = 0x40;
//
// typedef i8042_filter_t - i8042 filter callback
// @data: Data received by the i8042 controller
// @str: Status register of the i8042 controller
// @serio: Serio of the i8042 controller
// @context: Context pointer associated with this callback
//
// This represents a i8042 filter callback which can be used with i8042_install_filter()
// and i8042_remove_filter() to filter the i8042 input for platform-specific key codes.
//
// Context: Interrupt context.
// Returns: true if the data should be filtered out, false if otherwise.
//

extern "C" {
    pub fn i8042_lock_chip();
}
extern "C" {
    pub fn i8042_unlock_chip();
}
extern "C" {
    pub fn i8042_command(param: *mut c_uchar, command: c_int) -> c_int;
}
extern "C" {
    pub fn i8042_install_filter(filter: i8042_filter_t, context: *mut c_void) -> c_int;
}
extern "C" {
    pub fn i8042_remove_filter(filter: i8042_filter_t) -> c_int;
}

