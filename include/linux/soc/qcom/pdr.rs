//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/qcom/pdr.h
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

pub const SERVREG_NAME_LENGTH: c_int = 64;
pub const SERVREG_PFR_LENGTH: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum servreg_service_state {
    SERVREG_LOCATOR_ERR = 0x1,
    SERVREG_SERVICE_STATE_DOWN = 0x0FFFFFFF,
    SERVREG_SERVICE_STATE_UP = 0x1FFFFFFF,
    SERVREG_SERVICE_STATE_EARLY_DOWN = 0x2FFFFFFF,
    SERVREG_SERVICE_STATE_UNINIT = 0x7FFFFFFF,
}

extern "C" {
    pub fn pdr_restart_pd(pdr: *mut pdr_handle, pds: *mut pdr_service) -> c_int;
}
extern "C" {
    pub fn pdr_handle_release(pdr: *mut pdr_handle);
}
