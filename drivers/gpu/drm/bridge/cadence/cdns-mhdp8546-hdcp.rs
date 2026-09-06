//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/cadence/cdns-mhdp8546-hdcp.h
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
// Cadence MHDP8546 DP bridge driver.
//
// Copyright (C) 2020 Cadence Design Systems, Inc.
//

pub const HDCP_MAX_RECEIVERS: c_int = 32;
pub const HDCP_RECEIVER_ID_SIZE_BYTES: c_int = 5;
pub const HDCP_STATUS_SIZE: c_uint = 0x5;
pub const HDCP_PORT_STS_AUTH: c_uint = 0x1;
pub const HDCP_PORT_STS_LAST_ERR_SHIFT: c_uint = 0x5;

// use All HDCP versions

pub const HDCP_CONFIG_NONE: c_int = 0;

pub const HDCP_PAIRING_R_ID: c_int = 5;
pub const HDCP_PAIRING_M_LEN: c_int = 16;
pub const HDCP_KM_LEN: c_int = 16;
pub const HDCP_PAIRING_M_EKH: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_hdcp_pairing_data {
    pub receiver_id: [u8; HDCP_PAIRING_R_ID],
    pub m: [u8; HDCP_PAIRING_M_LEN],
    pub km: [u8; HDCP_KM_LEN],
    pub ekh: [u8; HDCP_PAIRING_M_EKH],
}

pub const DLP_MODULUS_N: c_int = 384;
pub const DLP_E: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_hdcp_tx_public_key_param {
    pub N: [u8; DLP_MODULUS_N],
    pub E: [u8; DLP_E],
}

extern "C" {
    pub fn cdns_mhdp_hdcp_enable(mhdp: *mut cdns_mhdp_device, content_type: u8) -> c_int;
}
extern "C" {
    pub fn cdns_mhdp_hdcp_disable(mhdp: *mut cdns_mhdp_device) -> c_int;
}
extern "C" {
    pub fn cdns_mhdp_hdcp_init(mhdp: *mut cdns_mhdp_device);
}
