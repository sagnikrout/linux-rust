//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/wait.h
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
pub const WNOHANG: c_uint = 0x00000001;
pub const WUNTRACED: c_uint = 0x00000002;

pub const WEXITED: c_uint = 0x00000004;
pub const WCONTINUED: c_uint = 0x00000008;
pub const WNOWAIT: c_uint = 0x01000000	/* Don't reap, just poll status.  */;
pub const __WNOTHREAD: c_uint = 0x20000000	/* Don't wait on children of other threads in this group */;
pub const __WALL: c_uint = 0x40000000	/* Wait on all children, regardless of type */;
pub const __WCLONE: c_uint = 0x80000000	/* Wait only on non-SIGCHLD children */;
// First argument to waitid:
pub const P_ALL: c_int = 0;
pub const P_PID: c_int = 1;
pub const P_PGID: c_int = 2;
pub const P_PIDFD: c_int = 3;
