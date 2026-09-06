//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/offload/provider.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2024 Analog Devices Inc.
// Copyright (C) 2024 BayLibre, SAS
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_offload_trigger_ops {
    pub nargs): *mut *mut spi_offload_trigger_type type, u64 args, u32,
    pub nargs): *mut *mut spi_offload_trigger_type type, u64 args, u32,
    pub trigger): *mut *mut void (release)(struct spi_offload_trigger,
    pub config): *mut spi_offload_trigger_config,
    pub config): *mut spi_offload_trigger_config,
    pub trigger): *mut *mut void (disable)(struct spi_offload_trigger,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_offload_trigger_info {
// @fwnode: Provider fwnode, used to match to consumer.
    pub fwnode: *mut fwnode_handle,
// @ops: Provider-specific callbacks.
    pub ops: *const spi_offload_trigger_ops,
// Provider-specific state to be used in callbacks.
    pub priv: *mut c_void,
}
