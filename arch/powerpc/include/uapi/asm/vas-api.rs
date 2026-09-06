//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/vas-api.h
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
// Copyright 2019 IBM Corp.
//

// Flags to VAS TX open window ioctl
// To allocate a window with QoS credit, otherwise use default credit
pub const VAS_TX_WIN_FLAG_QOS_CREDIT: c_uint = 0x0000000000000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vas_tx_win_open_attr {
    pub version: __u32,
    pub /: *mut *mut __s16 vas_id; / specific instance of vas or -1 for default,
    pub reserved1: __u16,
    pub flags: __u64,
    pub reserved2: [__u64; 6],
}
