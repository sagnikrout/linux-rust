//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/signalfd.h
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
// include/linux/signalfd.h
//
// Copyright (C) 2007  Davide Libenzi <davidel@xmailserver.org>
//

// For O_CLOEXEC and O_NONBLOCK

// Flags for signalfd4.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct signalfd_siginfo {
    pub ssi_signo: __u32,
    pub ssi_errno: __s32,
    pub ssi_code: __s32,
    pub ssi_pid: __u32,
    pub ssi_uid: __u32,
    pub ssi_fd: __s32,
    pub ssi_tid: __u32,
    pub ssi_band: __u32,
    pub ssi_overrun: __u32,
    pub ssi_trapno: __u32,
    pub ssi_status: __s32,
    pub ssi_int: __s32,
    pub ssi_ptr: __u64,
    pub ssi_utime: __u64,
    pub ssi_stime: __u64,
    pub ssi_addr: __u64,
    pub ssi_addr_lsb: __u16,
    pub __pad2: __u16,
    pub ssi_syscall: __s32,
    pub ssi_call_addr: __u64,
    pub ssi_arch: __u32,
//
// Pad strcture to 128 bytes. Remember to update the
// pad size when you add new members. We use a fixed
// size structure to avoid compatibility problems with
// future versions, and we leave extra space for additional
// members. We use fixed size members because this strcture
// comes out of a read(2) and we really don't want to have
// a compat on read(2).
//
    pub __pad: [__u8; 28],
}
