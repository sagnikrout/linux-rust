//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/c2port.h
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
// Silicon Labs C2 port Linux support
//
// Copyright (c) 2007 Rodolfo Giometti <giometti@linux.it>
// Copyright (c) 2007 Eurotech S.p.A. <info@eurotech.it>
//
pub const C2PORT_NAME_LEN: c_int = 32;
//
// C2 port basic structs
//
// Main struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c2port_device {
    pub access:1: c_uint,
    pub flash_access:1: c_uint,
    pub id: c_int,
    pub name: [c_char; C2PORT_NAME_LEN],
    pub ops: *mut c2port_ops,
    pub /: *mut *mut mutex mutex; / prevent races during read/write,
    pub dev: *mut device,
    pub private_data: *mut c_void,
}

// Basic operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c2port_ops {
// Flash layout
    pub /: *mut *mut unsigned short block_size; / flash block size in bytes,
    pub /: *mut *mut unsigned short blocks_num; / flash blocks number,
// Enable or disable the access to C2 port
    pub status): *mut *mut *mut void (access)(struct c2port_device dev, int,
// Set C2D data line as input/output
    pub dir): *mut *mut *mut void (c2d_dir)(struct c2port_device dev, int,
// Read/write C2D data line
    pub dev): *mut *mut int (c2d_get)(struct c2port_device,
    pub status): *mut *mut *mut void (c2d_set)(struct c2port_device dev, int,
// Write C2CK clock line
    pub status): *mut *mut *mut void (c2ck_set)(struct c2port_device dev, int,
}

//
// Exported functions
//
extern "C" {
    pub fn c2port_device_unregister(dev: *mut c2port_device);
}
