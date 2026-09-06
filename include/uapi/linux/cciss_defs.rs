//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/cciss_defs.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// general boundary definitions

// Command Status value
pub const CMD_SUCCESS: c_uint = 0x0000;
pub const CMD_TARGET_STATUS: c_uint = 0x0001;
pub const CMD_DATA_UNDERRUN: c_uint = 0x0002;
pub const CMD_DATA_OVERRUN: c_uint = 0x0003;
pub const CMD_INVALID: c_uint = 0x0004;
pub const CMD_PROTOCOL_ERR: c_uint = 0x0005;
pub const CMD_HARDWARE_ERR: c_uint = 0x0006;
pub const CMD_CONNECTION_LOST: c_uint = 0x0007;
pub const CMD_ABORTED: c_uint = 0x0008;
pub const CMD_ABORT_FAILED: c_uint = 0x0009;
pub const CMD_UNSOLICITED_ABORT: c_uint = 0x000A;
pub const CMD_TIMEOUT: c_uint = 0x000B;
pub const CMD_UNABORTABLE: c_uint = 0x000C;
// transfer direction
pub const XFER_NONE: c_uint = 0x00;
pub const XFER_WRITE: c_uint = 0x01;
pub const XFER_READ: c_uint = 0x02;
pub const XFER_RSVD: c_uint = 0x03;
// task attribute
pub const ATTR_UNTAGGED: c_uint = 0x00;
pub const ATTR_SIMPLE: c_uint = 0x04;
pub const ATTR_HEADOFQUEUE: c_uint = 0x05;
pub const ATTR_ORDERED: c_uint = 0x06;
pub const ATTR_ACA: c_uint = 0x07;
// cdb type
pub const TYPE_CMD: c_uint = 0x00;
pub const TYPE_MSG: c_uint = 0x01;
// Type defs used in the following structs

pub const CISS_MAX_LUN: c_int = 1024;

pub const LEVEL3LUN: c_int = 0;

// Command List Structure

