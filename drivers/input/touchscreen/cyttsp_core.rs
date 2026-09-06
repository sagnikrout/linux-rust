//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/touchscreen/cyttsp_core.h
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
// Header file for:
// Cypress TrueTouch(TM) Standard Product (TTSP) touchscreen drivers.
// For use with Cypress Txx3xx parts.
// Supported parts include:
// CY8CTST341
// CY8CTMA340
//
// Copyright (C) 2009, 2010, 2011 Cypress Semiconductor, Inc.
// Copyright (C) 2012 Javier Martinez Canillas <javier@dowhile0.org>
//
// Contact Cypress Semiconductor at www.cypress.com <kev@cypress.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyttsp_tch {
    pub y: __be16 x,,
    pub z: u8,
    pub __packed: },
// TrueTouch Standard Product Gen3 interface definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyttsp_xydata {
    pub hst_mode: u8,
    pub tt_mode: u8,
    pub tt_stat: u8,
    pub tch1: cyttsp_tch,
    pub touch12_id: u8,
    pub tch2: cyttsp_tch,
    pub gest_cnt: u8,
    pub gest_id: u8,
    pub tch3: cyttsp_tch,
    pub touch34_id: u8,
    pub tch4: cyttsp_tch,
    pub tt_undef: [u8; 3],
    pub act_dist: u8,
    pub tt_reserved: u8,
    pub __packed: },
// TTSP System Information interface definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyttsp_sysinfo_data {
    pub hst_mode: u8,
    pub mfg_stat: u8,
    pub mfg_cmd: u8,
    pub cid: [u8; 3],
    pub tt_undef1: u8,
    pub uid: [u8; 8],
    pub bl_verh: u8,
    pub bl_verl: u8,
    pub tts_verh: u8,
    pub tts_verl: u8,
    pub app_idh: u8,
    pub app_idl: u8,
    pub app_verh: u8,
    pub app_verl: u8,
    pub tt_undef: [u8; 5],
    pub scn_typ: u8,
    pub act_intrvl: u8,
    pub tch_tmout: u8,
    pub lp_intrvl: u8,
}

// TTSP Bootloader Register Map interface definition
pub const CY_BL_CHKSUM_OK: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyttsp_bootloader_data {
    pub bl_file: u8,
    pub bl_status: u8,
    pub bl_error: u8,
    pub blver_hi: u8,
    pub blver_lo: u8,
    pub bld_blver_hi: u8,
    pub bld_blver_lo: u8,
    pub ttspver_hi: u8,
    pub ttspver_lo: u8,
    pub appid_hi: u8,
    pub appid_lo: u8,
    pub appver_hi: u8,
    pub appver_lo: u8,
    pub cid_0: u8,
    pub cid_1: u8,
    pub cid_2: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyttsp_bus_ops {
    pub bustype: u16,
    pub values): *const c_void,
    pub values): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cyttsp_state {
    CY_IDLE_STATE,
    CY_ACTIVE_STATE,
    CY_BL_STATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyttsp {
    pub dev: *mut device,
    pub irq: c_int,
    pub input: *mut input_dev,
    pub bus_ops: *const cyttsp_bus_ops,
    pub bl_data: cyttsp_bootloader_data,
    pub sysinfo_data: cyttsp_sysinfo_data,
    pub xy_data: cyttsp_xydata,
    pub bl_ready: completion,
    pub state: cyttsp_state,
    pub suspended: bool,
    pub reset_gpio: *mut gpio_desc,
    pub use_hndshk: bool,
    pub act_dist: u8,
    pub act_intrvl: u8,
    pub tch_tmout: u8,
    pub lp_intrvl: u8,
    pub bl_keys: *mut u8,
    pub ____cacheline_aligned: u8 xfer_buf[],
}
