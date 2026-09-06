//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/g_zero.h
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


// SPDX-License-Identifier: GPL-2.0
//
// This header declares the utility functions used by "Gadget Zero", plus
// interfaces to its two single-configuration function drivers.
//
pub const GZERO_BULK_BUFLEN: c_int = 4096;
pub const GZERO_QLEN: c_int = 32;
pub const GZERO_ISOC_INTERVAL: c_int = 4;
pub const GZERO_ISOC_MAXPACKET: c_int = 1024;
pub const GZERO_SS_BULK_QLEN: c_int = 1;
pub const GZERO_SS_ISO_QLEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_zero_options {
    pub pattern: unsigned,
    pub isoc_interval: unsigned,
    pub isoc_maxpacket: unsigned,
    pub isoc_mult: unsigned,
    pub isoc_maxburst: unsigned,
    pub bulk_buflen: unsigned,
    pub qlen: unsigned,
    pub ss_bulk_qlen: unsigned,
    pub ss_iso_qlen: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_ss_opts {
    pub func_inst: usb_function_instance,
    pub pattern: unsigned,
    pub isoc_interval: unsigned,
    pub isoc_maxpacket: unsigned,
    pub isoc_mult: unsigned,
    pub isoc_maxburst: unsigned,
    pub bulk_buflen: unsigned,
    pub bulk_maxburst: unsigned,
    pub bulk_qlen: unsigned,
    pub iso_qlen: unsigned,
//
// Read/write access to configfs attributes is handled by configfs.
//
// This is to protect the data from concurrent access by read/write
// and create symlink/remove symlink.
//
    pub lock: mutex,
    pub refcnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_lb_opts {
    pub func_inst: usb_function_instance,
    pub bulk_buflen: unsigned,
    pub qlen: unsigned,
//
// Read/write access to configfs attributes is handled by configfs.
//
// This is to protect the data from concurrent access by read/write
// and create symlink/remove symlink.
//
    pub lock: mutex,
    pub refcnt: c_int,
}

extern "C" {
    pub fn lb_modexit();
}
extern "C" {
    pub fn lb_modinit() -> c_int;
}
// common utilities
