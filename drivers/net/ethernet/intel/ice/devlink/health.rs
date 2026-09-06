//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/devlink/health.h
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
// Copyright (c) 2024, Intel Corporation.

//
// DOC: health.h
//
// This header file stores everything that is needed for broadly understood
// devlink health mechanism for ice driver.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_mdd_src {
    ICE_MDD_SRC_TX_PQM,
    ICE_MDD_SRC_TX_TCLAN,
    ICE_MDD_SRC_TX_TDPU,
    ICE_MDD_SRC_RX,
}

//
// struct ice_health - stores ice devlink health reporters and accompanied data
// @fw: devlink health reporter for FW Health Status events
// @mdd: devlink health reporter for MDD detection event
// @port: devlink health reporter for Port Health Status events
// @tx_hang: devlink health reporter for tx_hang event
// @tx_hang_buf: pre-allocated place to put info for Tx hang reporter from
// non-sleeping context
// @tx_ring: ring that the hang occurred on
// @head: descriptor head
// @intr: interrupt register value
// @vsi_num: VSI owning the queue that the hang occurred on
// @fw_status: buffer for last received FW Status event
// @port_status: buffer for last received Port Status event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_health {
    pub fw: *mut devlink_health_reporter,
    pub mdd: *mut devlink_health_reporter,
    pub port: *mut devlink_health_reporter,
    pub tx_hang: *mut devlink_health_reporter,
    pub tx_ring: *mut ice_tx_ring,
    pub head: u32,
    pub intr: u32,
    pub vsi_num: u16,
    pub fw_status: ice_aqc_health_status_elem,
    pub port_status: ice_aqc_health_status_elem,
}

extern "C" {
    pub fn ice_health_init(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_health_deinit(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_health_clear(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_report_tx_hang(pf: *mut ice_pf);
}
