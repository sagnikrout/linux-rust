//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/elx/libefc/efc_els.h
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
// Copyright (C) 2021 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
//

pub const EFC_ELS_IO_POOL_SZ: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efc_els_io_req {
    pub list_entry: list_head,
    pub ref: kref,
    pub arg): *mut *mut void (release)(struct kref,
    pub node: *mut efc_node,
    pub cb: *mut c_void,
    pub els_retries_remaining: u32,
    pub els_req_free: bool,
    pub delay_timer: timer_list,
    pub display_name: *const c_char,
    pub io: efc_disc_io,
}

extern "C" {
    pub fn _efc_els_io_free(arg: *mut kref);
}
extern "C" {
    pub fn efc_els_io_free(els: *mut efc_els_io_req);
}
// ELS command send
// ELS acc send
// CT
