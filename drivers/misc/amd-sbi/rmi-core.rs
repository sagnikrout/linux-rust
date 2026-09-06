//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/amd-sbi/rmi-core.h
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
// Copyright (C) 2025 Advanced Micro Devices, Inc.
//

// SB-RMI registers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbrmi_reg {
    SBRMI_REV,
    SBRMI_CTRL,
    SBRMI_STATUS,
    SBRMI_OUTBNDMSG0	= 0x30,
    SBRMI_OUTBNDMSG1,
    SBRMI_OUTBNDMSG2,
    SBRMI_OUTBNDMSG3,
    SBRMI_OUTBNDMSG4,
    SBRMI_OUTBNDMSG5,
    SBRMI_OUTBNDMSG6,
    SBRMI_OUTBNDMSG7,
    SBRMI_INBNDMSG0,
    SBRMI_INBNDMSG1,
    SBRMI_INBNDMSG2,
    SBRMI_INBNDMSG3,
    SBRMI_INBNDMSG4,
    SBRMI_INBNDMSG5,
    SBRMI_INBNDMSG6,
    SBRMI_INBNDMSG7,
    SBRMI_SW_INTERRUPT,
    SBRMI_THREAD128CS	= 0x4b,
}

//
// SB-RMI supports soft mailbox service request to MP1 (power management
// firmware) through SBRMI inbound/outbound message registers.
// SB-RMI message IDs
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbrmi_msg_id {
    SBRMI_READ_PKG_PWR_CONSUMPTION = 0x1,
    SBRMI_WRITE_PKG_PWR_LIMIT,
    SBRMI_READ_PKG_PWR_LIMIT,
    SBRMI_READ_PKG_MAX_PWR_LIMIT,
}

// Each client has this additional data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbrmi_data {
    pub sbrmi_misc_dev: miscdevice,
    pub regmap: *mut regmap,
// Mutex locking
    pub lock: mutex,
    pub pwr_limit_max: u32,
    pub dev_static_addr: u8,
    pub rev: u8,
}

extern "C" {
    pub fn rmi_mailbox_xfer(data: *mut sbrmi_data, msg: *mut apml_mbox_msg) -> c_int;
}

extern "C" {
    pub fn create_hwmon_sensor_device(dev: *mut device, data: *mut sbrmi_data) -> c_int;
}

extern "C" {
    pub fn create_misc_rmi_device(data: *mut sbrmi_data, dev: *mut device) -> c_int;
}
