//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/stm.h
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
// System Trace Module (STM) userspace interfaces
// Copyright (c) 2014, Intel Corporation.
//
// STM class implements generic infrastructure for  System Trace Module devices
// as defined in MIPI STPv2 specification.
//

// Maximum allowed master and channel values
pub const STP_MASTER_MAX: c_uint = 0xffff;
pub const STP_CHANNEL_MAX: c_uint = 0xffff;
//
// struct stp_policy_id - identification for the STP policy
// @size:	size of the structure including real id[] length
// @master:	assigned master
// @channel:	first assigned channel
// @width:	number of requested channels
// @id:		identification string
//
// User must calculate the total size of the structure and put it into
// @size field, fill out the @id and desired @width. In return, kernel
// fills out @master, @channel and @width.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stp_policy_id {
    pub size: __u32,
    pub master: __u16,
    pub channel: __u16,
    pub width: __u16,
// padding
    pub __reserved_0: __u16,
    pub __reserved_1: __u32,
    pub id: [c_char; ],
}

