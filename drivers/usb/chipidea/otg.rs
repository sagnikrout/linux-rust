//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/chipidea/otg.h
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
// Copyright (C) 2013-2014 Freescale Semiconductor, Inc.
//
// Author: Peter Chen
//
extern "C" {
    pub fn hw_read_otgsc(ci: *mut ci_hdrc, mask: u32) -> u32;
}
extern "C" {
    pub fn hw_write_otgsc(ci: *mut ci_hdrc, mask: u32, data: u32);
}
extern "C" {
    pub fn ci_hdrc_otg_init(ci: *mut ci_hdrc) -> c_int;
}
extern "C" {
    pub fn ci_hdrc_otg_destroy(ci: *mut ci_hdrc);
}
extern "C" {
    pub fn ci_otg_role(ci: *mut ci_hdrc) -> ci_role;
}
extern "C" {
    pub fn ci_handle_vbus_change(ci: *mut ci_hdrc);
}
extern "C" {
    pub fn ci_handle_id_switch(ci: *mut ci_hdrc);
}
