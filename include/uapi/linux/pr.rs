//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/pr.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pr_status {
    PR_STS_SUCCESS			= 0x0,
//
// The following error codes are based on SCSI, because the interface
// was originally created for it and has existing users.
//
// Generic device failure.
    PR_STS_IOERR			= 0x2,
    PR_STS_RESERVATION_CONFLICT	= 0x18,
// Temporary path failure that can be retried.
    PR_STS_RETRY_PATH_FAILURE	= 0xe0000,
// The request was failed due to a fast failure timer.
    PR_STS_PATH_FAST_FAILED		= 0xf0000,
// The path cannot be reached and has been marked as failed.
    PR_STS_PATH_FAILED		= 0x10000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pr_type {
    PR_WRITE_EXCLUSIVE		= 1,
    PR_EXCLUSIVE_ACCESS		= 2,
    PR_WRITE_EXCLUSIVE_REG_ONLY	= 3,
    PR_EXCLUSIVE_ACCESS_REG_ONLY	= 4,
    PR_WRITE_EXCLUSIVE_ALL_REGS	= 5,
    PR_EXCLUSIVE_ACCESS_ALL_REGS	= 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_reservation {
    pub key: __u64,
    pub type: __u32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_registration {
    pub old_key: __u64,
    pub new_key: __u64,
    pub flags: __u32,
    pub __pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_preempt {
    pub old_key: __u64,
    pub new_key: __u64,
    pub type: __u32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_clear {
    pub key: __u64,
    pub flags: __u32,
    pub __pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_read_keys {
    pub generation: __u32,
    pub num_keys: __u32,
    pub keys_ptr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_read_reservation {
    pub key: __u64,
    pub generation: __u32,
    pub type: __u32,
}

