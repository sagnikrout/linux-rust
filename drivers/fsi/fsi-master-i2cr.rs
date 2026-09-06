//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/fsi/fsi-master-i2cr.h
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
// Copyright (C) IBM Corporation 2023

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsi_master_i2cr {
    pub master: fsi_master,
    pub /: *mut *mut mutex lock; / protect HW access,
    pub client: *mut i2c_client,
}

extern "C" {
    pub fn fsi_master_i2cr_read(i2cr: *mut fsi_master_i2cr, addr: u32, data: *mut u64) -> c_int;
}
extern "C" {
    pub fn fsi_master_i2cr_write(i2cr: *mut fsi_master_i2cr, addr: u32, data: u64) -> c_int;
}
