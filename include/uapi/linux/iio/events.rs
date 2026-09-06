//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/iio/events.h
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
// The industrial I/O - event passing to userspace
//
// Copyright (c) 2008-2011 Jonathan Cameron
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published by
// the Free Software Foundation.
//

//
// struct iio_event_data - The actual event being pushed to userspace
// @id:		event identifier
// @timestamp:	best estimate of time of event occurrence (often from
// the interrupt handler)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_event_data {
    pub id: __u64,
    pub timestamp: __s64,
}

// Event code number extraction depends on which type of event we have.
// Perhaps review this function in the future

