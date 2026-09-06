//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/tty.h
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
// 'tty.h' defines some structures used by tty_io.c and some defines.
//
// line disciplines
pub const N_TTY: c_int = 0;
pub const N_SLIP: c_int = 1;
pub const N_MOUSE: c_int = 2;
pub const N_PPP: c_int = 3;
pub const N_STRIP: c_int = 4;
pub const N_AX25: c_int = 5;

pub const N_6PACK: c_int = 7;

// cards about SMS messages

// Always the newest line discipline + 1
pub const NR_LDISCS: c_int = 31;
