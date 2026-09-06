//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/signal_types.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Basic signal handling related data type definitions:
//

//
// Real Time signals may be queued.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigqueue {
    pub list: list_head,
    pub flags: c_int,
    pub info: kernel_siginfo_t,
    pub ucounts: *mut ucounts,
}

// flags values.
pub const SIGQUEUE_PREALLOC: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigpending {
    pub list: list_head,
    pub signal: sigset_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_flags: c_ulong,

    pub sa_flags: c_uint,
    pub sa_handler: __sighandler_t,

    pub sa_restorer: __sigrestore_t,

    pub /: *mut *mut sigset_t sa_mask; / mask last for extensibility,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k_sigaction {
    pub sa: sigaction,

    pub ka_restorer: __sigrestore_t,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct old_sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_mask: old_sigset_t,
    pub sa_flags: c_ulong,
    pub sa_restorer: __sigrestore_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksignal {
    pub ka: k_sigaction,
    pub info: kernel_siginfo_t,
    pub sig: c_int,
}

// Used to kill the race between sigaction and forced signals
pub const SA_IMMUTABLE: c_uint = 0x00800000;

pub const __ARCH_UAPI_SA_FLAGS: c_int = 0;

