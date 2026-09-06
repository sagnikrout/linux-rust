//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/sembuf.h
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
// The semid64_ds structure for x86 architecture.
// Note extra padding because this structure is passed back and forth
// between kernel and user space.
//
// Pad space is left for:
// - 2 miscellaneous 32-bit values
//
// x86_64 and x32 incorrectly added padding here, so the structures
// are still incompatible with the padding on x86.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct semid64_ds {
    pub /: *mut *mut ipc64_perm sem_perm; / permissions .. see ipc.h,

    pub /: *mut *mut unsigned long sem_otime; / last semop time,
    pub sem_otime_high: c_ulong,
    pub /: *mut *mut unsigned long sem_ctime; / last change time,
    pub sem_ctime_high: c_ulong,

    pub /: *mut *mut __kernel_long_t sem_otime; / last semop time,
    pub __unused1: __kernel_ulong_t,
    pub /: *mut *mut __kernel_long_t sem_ctime; / last change time,
    pub __unused2: __kernel_ulong_t,

    pub /: *mut *mut __kernel_ulong_t sem_nsems; / no. of semaphores in array,
    pub __unused3: __kernel_ulong_t,
    pub __unused4: __kernel_ulong_t,
}
