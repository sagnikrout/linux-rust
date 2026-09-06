//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/i2c-mux.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// i2c-mux.h - functions for the i2c-bus mux support
//
// Copyright (c) 2008-2009 Rodolfo Giometti <giometti@linux.it>
// Copyright (c) 2008-2009 Eurotech S.p.A. <info@eurotech.it>
// Michael Lawnick <michael.lawnick.ext@nsn.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_mux_core {
    pub parent: *mut i2c_adapter,
    pub dev: *mut device,
    pub mux_locked:1: c_uint,
    pub arbitrator:1: c_uint,
    pub gate:1: c_uint,
    pub priv: *mut c_void,
    pub chan_id): *mut *mut *mut int (select)(struct i2c_mux_core , u32,
    pub chan_id): *mut *mut *mut int (deselect)(struct i2c_mux_core , u32,
    pub num_adapters: c_int,
    pub max_adapters: c_int,
    pub adapter: [*mut i2c_adapter; ],
}

// flags for i2c_mux_alloc

//
// Called to create an i2c bus on a multiplexed bus segment.
// The chan_id parameter is passed to the select and deselect
// callback functions to perform hardware-specific mux control.
//
extern "C" {
    pub fn i2c_mux_del_adapters(muxc: *mut i2c_mux_core);
}

