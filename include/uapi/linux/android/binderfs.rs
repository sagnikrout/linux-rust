//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/android/binderfs.h
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
// Copyright (C) 2018 Canonical Ltd.
//

pub const BINDERFS_MAX_NAME: c_int = 255;
//
// struct binderfs_device - retrieve information about a new binder device
// @name:   the name to use for the new binderfs binder device
// @major:  major number allocated for binderfs binder devices
// @minor:  minor number allocated for the new binderfs binder device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binderfs_device {
    pub 1]: char name[BINDERFS_MAX_NAME +,
    pub major: __u32,
    pub minor: __u32,
}

//
// Allocate a new binder device.
//

