//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/as102/as10x_handle.h
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
// Abilis Systems Single DVB-T Receiver
// Copyright (C) 2008 Pierrick Hascoet <pierrick.hascoet@abilis.com>
//

// values for "mode" field
pub const REGMODE8: c_int = 8;
pub const REGMODE16: c_int = 16;
pub const REGMODE32: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as102_priv_ops_t {
    pub swap32): *mut *mut unsigned char buf, int buflen, int,
    pub buflen): *mut *mut unsigned char buf, int,
    pub recv_buf_len): *mut *mut unsigned char recv_buf, int,
    pub dev): *mut *mut int (start_stream)(struct as102_dev_t,
    pub dev): *mut *mut void (stop_stream)(struct as102_dev_t,
    pub bus_adap): *mut *mut int (reset_target)(struct as10x_bus_adapter_t,
    pub wr_len): uint32_t wr_addr, uint16_t,
    pub recv_buf_len): c_int,
}
