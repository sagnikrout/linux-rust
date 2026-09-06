//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/as102/as102_fw.h
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
pub const MAX_FW_PKT_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as10x_raw_fw_pkt {
    pub address: [c_uchar; 4],
    pub 6]: unsigned char data[MAX_FW_PKT_SIZE -,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as10x_fw_pkt_t {
    pub request: [c_uchar; 2],
    pub length: [c_uchar; 2],
    pub u: } __packed,
    pub raw: as10x_raw_fw_pkt,
    pub __packed: },

    pub bus_adap): *mut int as102_fw_upload(struct as10x_bus_adapter_t,
