//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb/m920x.h
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

pub const M9206_CORE: c_uint = 0x22;
pub const M9206_RC_STATE: c_uint = 0xff51;
pub const M9206_RC_KEY: c_uint = 0xff52;
pub const M9206_RC_INIT1: c_uint = 0xff54;
pub const M9206_RC_INIT2: c_uint = 0xff55;
pub const M9206_FW_GO: c_uint = 0xff69;
pub const M9206_I2C: c_uint = 0x23;
pub const M9206_FILTER: c_uint = 0x25;
pub const M9206_FW: c_uint = 0x30;
pub const M9206_MAX_FILTERS: c_int = 8;
pub const M9206_MAX_ADAPTERS: c_int = 4;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m920x_state {
    pub filters: [u16; M9206_MAX_ADAPTERS][M9206_MAX_FILTERS],
    pub filtering_enabled: [c_int; M9206_MAX_ADAPTERS],
    pub rep_count: c_int,
}

// Initialisation data for the m920x
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m920x_inits {
    pub address: u16,
    pub data: u8,
}
