//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/syslog.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Syslog internals
//
// Copyright 2010 Canonical, Ltd.
// Author: Kees Cook <kees.cook@canonical.com>
//

// Close the log.  Currently a NOP.
pub const SYSLOG_ACTION_CLOSE: c_int = 0;
// Open the log. Currently a NOP.
pub const SYSLOG_ACTION_OPEN: c_int = 1;
// Read from the log.
pub const SYSLOG_ACTION_READ: c_int = 2;
// Read all messages remaining in the ring buffer.
pub const SYSLOG_ACTION_READ_ALL: c_int = 3;
// Read and clear all messages remaining in the ring buffer
pub const SYSLOG_ACTION_READ_CLEAR: c_int = 4;
// Clear ring buffer.
pub const SYSLOG_ACTION_CLEAR: c_int = 5;
// Disable printk's to console
pub const SYSLOG_ACTION_CONSOLE_OFF: c_int = 6;
// Enable printk's to console
pub const SYSLOG_ACTION_CONSOLE_ON: c_int = 7;
// Set level of messages printed to console
pub const SYSLOG_ACTION_CONSOLE_LEVEL: c_int = 8;
// Return number of unread characters in the log buffer
pub const SYSLOG_ACTION_SIZE_UNREAD: c_int = 9;
// Return size of the log buffer
pub const SYSLOG_ACTION_SIZE_BUFFER: c_int = 10;
pub const SYSLOG_FROM_READER: c_int = 0;
pub const SYSLOG_FROM_PROC: c_int = 1;
extern "C" {
    pub fn do_syslog(type: c_int, buf: *mut char __user, count: c_int, source: c_int) -> c_int;
}
