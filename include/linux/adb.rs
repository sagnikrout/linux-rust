//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/adb.h
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
// Definitions for ADB (Apple Desktop Bus) support.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adb_request {
    pub data: [c_uchar; 32],
    pub nbytes: c_int,
    pub reply: [c_uchar; 32],
    pub reply_len: c_int,
    pub reply_expected: c_uchar,
    pub sent: c_uchar,
    pub complete: c_uchar,
    pub ): *mut *mut void (done)(struct adb_request,
    pub arg: *mut c_void,
    pub next: *mut adb_request,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adb_ids {
    pub nids: c_int,
    pub id: [c_uchar; 16],
}

// Structure which encapsulates a low-level ADB driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adb_driver {
    pub name: [c_char; 16],
    pub (*probe)(void): *mut c_int,
    pub (*init)(void): *mut c_int,
    pub sync): *mut *mut *mut int (send_request)(struct adb_request req, int,
    pub devs): *mut *mut int (autopoll)(int,
    pub (*poll)(void): *mut c_void,
    pub (*reset_bus)(void): *mut c_int,
}

// Values for adb_request flags

// Messages sent thru the client_list notifier. You should NOT stop
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adb_message {
    ADB_MSG_POWERDOWN,	/* Currently called before sleep only */
    ADB_MSG_PRE_RESET,	/* Called before resetting the bus */
    ADB_MSG_POST_RESET	/* Called after resetting the bus (re-do init & register) */
}

extern "C" {
    pub fn adb_unregister(index: c_int) -> c_int;
}
extern "C" {
    pub fn adb_poll();
}
extern "C" {
    pub fn adb_input(: *mut c_uchar, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn adb_reset_bus() -> c_int;
}
extern "C" {
    pub fn adb_try_handler_change(address: c_int, new_id: c_int) -> c_int;
}
extern "C" {
    pub fn adb_get_infos(address: c_int, original_address: *mut c_int, handler_id: *mut c_int) -> c_int;
}
