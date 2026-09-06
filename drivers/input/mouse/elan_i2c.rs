//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/mouse/elan_i2c.h
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
// Elan I2C/SMBus Touchpad driver
//
// Copyright (c) 2013 ELAN Microelectronics Corp.
//
// Author: 林政維 (Duson Lin) <dusonlin@emc.com.tw>
//
// Based on cyapa driver:
// copyright (c) 2011-2012 Cypress Semiconductor, Inc.
// copyright (c) 2011-2012 Google, Inc.
//
// Trademarks are the property of their respective owners.
//

pub const ETP_ENABLE_ABS: c_uint = 0x0001;
pub const ETP_ENABLE_CALIBRATE: c_uint = 0x0002;
pub const ETP_DISABLE_CALIBRATE: c_uint = 0x0000;
pub const ETP_DISABLE_POWER: c_uint = 0x0001;
pub const ETP_PRESSURE_OFFSET: c_int = 25;
pub const ETP_CALIBRATE_MAX_LEN: c_int = 3;

pub const ETP_REPORT_ID: c_uint = 0x5D;
pub const ETP_TP_REPORT_ID: c_uint = 0x5E;
pub const ETP_TP_REPORT_ID2: c_uint = 0x5F;
pub const ETP_REPORT_ID2: c_uint = 0x60	/* High precision report */;
pub const ETP_REPORT_ID_OFFSET: c_int = 2;
pub const ETP_TOUCH_INFO_OFFSET: c_int = 3;
pub const ETP_FINGER_DATA_OFFSET: c_int = 4;
pub const ETP_HOVER_INFO_OFFSET: c_int = 30;

pub const ETP_MAX_REPORT_LEN: c_int = 39;
pub const ETP_MAX_FINGERS: c_int = 5;
pub const ETP_FINGER_DATA_LEN: c_int = 5;
// IAP Firmware handling

pub const ETP_IAP_START_ADDR: c_uint = 0x0083;

pub const ETP_FW_PAGE_SIZE: c_int = 64;
pub const ETP_FW_PAGE_SIZE_128: c_int = 128;
pub const ETP_FW_PAGE_SIZE_512: c_int = 512;
pub const ETP_FW_SIGNATURE_SIZE: c_int = 6;
pub const ETP_PRODUCT_ID_WHITEBOX: c_uint = 0x00B8;
pub const ETP_PRODUCT_ID_VOXEL: c_uint = 0x00BF;
pub const ETP_PRODUCT_ID_DELBIN: c_uint = 0x00C2;
pub const ETP_PRODUCT_ID_MAGPIE: c_uint = 0x0120;
pub const ETP_PRODUCT_ID_BOBBA: c_uint = 0x0121;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tp_mode {
    IAP_MODE = 1,
    MAIN_MODE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elan_transport_ops {
    pub client): *mut *mut int (initialize)(struct i2c_client,
    pub sleep): *mut *mut *mut int (sleep_control)(struct i2c_client , bool,
    pub enable): *mut *mut *mut int (power_control)(struct i2c_client , bool,
    pub mode): *mut *mut *mut int (set_mode)(struct i2c_client client, u8,
    pub client): *mut *mut int (calibrate)(struct i2c_client,
    pub val): *mut *mut *mut int (calibrate_result)(struct i2c_client client, u8,
    pub value): *mut bool max_baseline, u8,
    pub version): *mut u8,
    pub clickpad): *mut *mut *mut u16 ic_type, u8 version, u8,
    pub csum): *mut *mut *mut int (get_checksum)(struct i2c_client client, bool iap, u16,
    pub id): *mut *mut *mut int (get_product_id)(struct i2c_client client, u16,
    pub max_y): *mut *mut unsigned int max_x, unsigned int,
    pub hw_res_y): *mut *mut u8 hw_res_x, u8,
    pub y_tracenum): *mut c_uint,
    pub mode): *mut *mut *mut int (iap_get_mode)(struct i2c_client client, enum tp_mode,
    pub client): *mut *mut int (iap_reset)(struct i2c_client,
    pub fw_page_size): u8 iap_version, u16,
    pub idx): c_int,
    pub reset_done): *mut completion,
    pub report_len): *mut c_uint,
    pub report_len): c_uint,
    pub adjustment): *mut c_int,
    pub pattern): *mut *mut *mut int (get_pattern)(struct i2c_client client, u8,
}
