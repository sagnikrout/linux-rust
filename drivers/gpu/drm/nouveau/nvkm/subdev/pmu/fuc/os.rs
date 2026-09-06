//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/pmu/fuc/os.h
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


// SPDX-License-Identifier: MIT
// Process names
pub const PROC_KERN: c_uint = 0x52544e49;
pub const PROC_IDLE: c_uint = 0x454c4449;
pub const PROC_HOST: c_uint = 0x54534f48;
pub const PROC_MEMX: c_uint = 0x584d454d;
pub const PROC_PERF: c_uint = 0x46524550;
pub const PROC_I2C_: c_uint = 0x5f433249;
pub const PROC_TEST: c_uint = 0x54534554;
// KERN: message identifiers
pub const KMSG_FIFO: c_uint = 0x00000000;
pub const KMSG_ALARM: c_uint = 0x00000001;
// MEMX: message identifiers
pub const MEMX_MSG_INFO: c_int = 0;
pub const MEMX_MSG_EXEC: c_int = 1;
// MEMX: info types
pub const MEMX_INFO_DATA: c_int = 0;
pub const MEMX_INFO_TRAIN: c_int = 1;
// MEMX: script opcode definitions
pub const MEMX_ENTER: c_int = 1;
pub const MEMX_LEAVE: c_int = 2;
pub const MEMX_WR32: c_int = 3;
pub const MEMX_WAIT: c_int = 4;
pub const MEMX_DELAY: c_int = 5;
pub const MEMX_VBLANK: c_int = 6;
pub const MEMX_TRAIN: c_int = 7;
// I2C_: message identifiers
pub const I2C__MSG_RD08: c_int = 0;
pub const I2C__MSG_WR08: c_int = 1;

