//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/posix_types.h
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
// This allows for 1024 file descriptors: if NR_OPEN is ever grown
// beyond that you'll have to change this too. But 1024 fd's seem to be
// enough even for such "real" unices like OSF/1, so hopefully this is
// one limit that doesn't have to be changed [again].
//
// Note that POSIX wants the FD_CLEAR(fd,fdsetp) defines to be in
// <sys/time.h> (and thus <linux/time.h>) - but this is a more logical
// place for them. Solved by having dummy defines in <sys/time.h>.
//
// This macro may have been defined in <gnu/types.h>. But we always
// use the one here.
//

pub const __FD_SETSIZE: c_int = 1024;
// Type of a signal handler.
extern "C" {
    pub fn void(_arg: *mut __kernel_sighandler_t)(int) -> typedef;
}
// Type of a SYSV IPC key.
pub type __kernel_key_t = c_int;
pub type __kernel_mqd_t = c_int;

