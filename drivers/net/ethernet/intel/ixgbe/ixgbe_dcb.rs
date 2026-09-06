//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_dcb.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.

// DCB data structures
pub const IXGBE_MAX_PACKET_BUFFERS: c_int = 8;
pub const MAX_USER_PRIORITY: c_int = 8;
pub const MAX_BW_GROUP: c_int = 8;
pub const BW_PERCENT: c_int = 100;
pub const DCB_TX_CONFIG: c_int = 0;
pub const DCB_RX_CONFIG: c_int = 1;
// DCB error Codes
pub const DCB_SUCCESS: c_int = 0;

// Transmit and receive Errors
// Error in bandwidth group allocation

// Error in traffic class bandwidth allocation

// Traffic class has both link strict and group strict enabled

// Link strict traffic class has non zero bandwidth

// Link strict bandwidth group has non zero bandwidth

// Traffic class has zero bandwidth

pub const DCB_NOT_IMPLEMENTED: c_uint = 0x7FFFFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcb_pfc_tc_debug {
    pub tc: u8,
    pub pause_status: u8,
    pub pause_quanta: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum strict_prio_type {
    prio_none = 0,
    prio_group,
    prio_link
}

// DCB capability definitions
pub const IXGBE_DCB_PG_SUPPORT: c_uint = 0x00000001;
pub const IXGBE_DCB_PFC_SUPPORT: c_uint = 0x00000002;
pub const IXGBE_DCB_BCN_SUPPORT: c_uint = 0x00000004;
pub const IXGBE_DCB_UP2TC_SUPPORT: c_uint = 0x00000008;
pub const IXGBE_DCB_GSP_SUPPORT: c_uint = 0x00000010;
pub const IXGBE_DCB_8_TC_SUPPORT: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcb_support {
// DCB capabilities
    pub capabilities: u32,
// Each bit represents a number of TCs configurable in the hw.
// If 8 traffic classes can be configured, the value is 0x80.
//
    pub traffic_classes: u8,
    pub pfc_traffic_classes: u8,
}

// Traffic class bandwidth allocation per direction
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_bw_alloc {
    pub /: *mut *mut u8 bwg_id; / Bandwidth Group (BWG) ID,
    pub /: *mut *mut u8 bwg_percent; / % of BWG's bandwidth,
    pub /: *mut *mut u8 link_percent; / % of link bandwidth,
    pub /: *mut *mut u8 up_to_tc_bitmap; / User Priority to Traffic Class mapping,
    pub /: *mut *mut u16 data_credits_refill; / Credit refill amount in 64B granularity,
    pub buffer: *mut *mut u16 data_credits_max; / Max credits for a configured packet,
// in 64B granularity.
    pub /: *mut *mut strict_prio_type prio_type; / Link or Group Strict Priority,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcb_pfc_type {
    pfc_disabled = 0,
    pfc_enabled_full,
    pfc_enabled_tx,
    pfc_enabled_rx
}

// Traffic class configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_configuration {
    pub /: *mut *mut tc_bw_alloc path[2]; / One each for Tx/Rx,
    pub /: *mut *mut dcb_pfc_type dcb_pfc; / Class based flow control setting,
    pub /: *mut *mut u16 desc_credits_max; / For Tx Descriptor arbitration,
    pub /: *mut *mut u8 tc; / Traffic class (TC),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcb_num_tcs {
    pub pg_tcs: u8,
    pub pfc_tcs: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_dcb_config {
    pub support: dcb_support,
    pub num_tcs: dcb_num_tcs,
    pub tc_config: [tc_configuration; MAX_TRAFFIC_CLASS],
    pub /: *mut *mut u8 bw_percentage[2][MAX_BW_GROUP]; / One each for Tx/Rx,
    pub pfc_mode_enable: bool,
    pub /: *mut *mut u32 dcb_cfg_version; / Not used...OS-specific?,
    pub /: *mut *mut u32 link_speed; / For bandwidth allocation validation purpose,
}

// DCB driver APIs
extern "C" {
    pub fn ixgbe_dcb_unpack_pfc(cfg: *mut ixgbe_dcb_config, pfc_en: *mut u8);
}
extern "C" {
    pub fn ixgbe_dcb_unpack_refill(: *mut ixgbe_dcb_config, _arg: c_int, : *mut u16);
}
extern "C" {
    pub fn ixgbe_dcb_unpack_max(: *mut ixgbe_dcb_config, : *mut u16);
}
extern "C" {
    pub fn ixgbe_dcb_unpack_bwgid(: *mut ixgbe_dcb_config, _arg: c_int, : *mut u8);
}
extern "C" {
    pub fn ixgbe_dcb_unpack_prio(: *mut ixgbe_dcb_config, _arg: c_int, : *mut u8);
}
extern "C" {
    pub fn ixgbe_dcb_unpack_map(: *mut ixgbe_dcb_config, _arg: c_int, : *mut u8);
}
extern "C" {
    pub fn ixgbe_dcb_get_tc_from_up(: *mut ixgbe_dcb_config, _arg: c_int, _arg: u8) -> u8;
}
// DCB credits calculation
// DCB hw initialization
extern "C" {
    pub fn ixgbe_dcb_hw_ets(hw: *mut ixgbe_hw, ets: *mut ieee_ets, max: c_int) -> c_int;
}
extern "C" {
    pub fn ixgbe_dcb_hw_pfc_config(hw: *mut ixgbe_hw, pfc_en: u8, tc_prio: *mut u8) -> c_int;
}
extern "C" {
    pub fn ixgbe_dcb_hw_config(: *mut ixgbe_hw, : *mut ixgbe_dcb_config) -> c_int;
}
extern "C" {
    pub fn ixgbe_dcb_read_rtrup2tc(hw: *mut ixgbe_hw, map: *mut u8);
}
// DCB definitions for credit calculation

