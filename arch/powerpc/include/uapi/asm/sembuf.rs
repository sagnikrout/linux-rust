//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/sembuf.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note

//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//
// The semid64_ds structure for PPC architecture.
// Note extra padding because this structure is passed back and forth
// between kernel and user space.
//
// Pad space is left for:
// - 2 miscellaneous 32/64-bit values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct semid64_ds {
    pub /: *mut *mut ipc64_perm sem_perm; / permissions .. see ipc.h,

    pub sem_otime_high: c_ulong,
    pub /: *mut *mut unsigned long sem_otime; / last semop time,
    pub sem_ctime_high: c_ulong,
    pub /: *mut *mut unsigned long sem_ctime; / last change time,

    pub /: *mut *mut long sem_otime; / last semop time,
    pub /: *mut *mut long sem_ctime; / last change time,

    pub /: *mut *mut unsigned long sem_nsems; / no. of semaphores in array,
    pub __unused3: c_ulong,
    pub __unused4: c_ulong,
}
