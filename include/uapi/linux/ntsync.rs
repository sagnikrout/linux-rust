//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ntsync.h
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
// Kernel support for NT synchronization primitive emulation
//
// Copyright (C) 2021-2022 Elizabeth Figura <zfigura@codeweavers.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntsync_sem_args {
    pub count: __u32,
    pub max: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntsync_mutex_args {
    pub owner: __u32,
    pub count: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntsync_event_args {
    pub manual: __u32,
    pub signaled: __u32,
}

pub const NTSYNC_WAIT_REALTIME: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntsync_wait_args {
    pub timeout: __u64,
    pub objs: __u64,
    pub count: __u32,
    pub index: __u32,
    pub flags: __u32,
    pub owner: __u32,
    pub alert: __u32,
    pub pad: __u32,
}

pub const NTSYNC_MAX_WAIT_COUNT: c_int = 64;

