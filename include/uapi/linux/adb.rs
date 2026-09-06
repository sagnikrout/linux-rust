//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/adb.h
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
//
// Definitions for ADB (Apple Desktop Bus) support.
//
// ADB commands
pub const ADB_BUSRESET: c_int = 0;

// ADB default device IDs (upper 4 bits of ADB command byte)

pub const ADB_KEYBOARD: c_int = 2;
pub const ADB_MOUSE: c_int = 3;
pub const ADB_TABLET: c_int = 4;
pub const ADB_MODEM: c_int = 5;

pub const ADB_RET_OK: c_int = 0;
pub const ADB_RET_TIMEOUT: c_int = 3;
// The kind of ADB request. The controller may emulate some
pub const ADB_PACKET: c_int = 0;
pub const CUDA_PACKET: c_int = 1;
pub const ERROR_PACKET: c_int = 2;
pub const TIMER_PACKET: c_int = 3;
pub const POWER_PACKET: c_int = 4;
pub const MACIIC_PACKET: c_int = 5;
pub const PMU_PACKET: c_int = 6;
pub const ADB_QUERY: c_int = 7;
// ADB queries
// ADB_QUERY_GETDEVINFO
// Query ADB slot for device presence
// data[2] = id, rep[0] = orig addr, rep[1] = handler_id
//
pub const ADB_QUERY_GETDEVINFO: c_int = 1;
