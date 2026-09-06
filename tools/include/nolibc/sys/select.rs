//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/sys/select.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT

// commonly an fd_set represents 256 FDs

pub const FD_SETSIZE: c_int = 256;

// for select()

//
// int select(int nfds, fd_set *read_fds, fd_set *write_fds,
// fd_set *except_fds, struct timeval *timeout);
//

extern "C" {
    pub fn __sysret(_arg: _sys_select(nfds, _arg: rfds, _arg: wfds, _arg: efds, _arg: timeout)) -> return;
}
