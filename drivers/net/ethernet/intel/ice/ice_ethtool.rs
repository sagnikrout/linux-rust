//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_ethtool.h
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
// Copyright (C) 2023 Intel Corporation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_phy_type_to_ethtool {
    pub aq_link_speed: u64,
    pub link_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_serdes_equalization_to_ethtool {
    pub rx_equ_pre2: c_int,
    pub rx_equ_pre1: c_int,
    pub rx_equ_post1: c_int,
    pub rx_equ_bflf: c_int,
    pub rx_equ_bfhf: c_int,
    pub rx_equ_ctle_gainhf: c_int,
    pub rx_equ_ctle_gainlf: c_int,
    pub rx_equ_ctle_gaindc: c_int,
    pub rx_equ_ctle_bw: c_int,
    pub rx_equ_dfe_gain: c_int,
    pub rx_equ_dfe_gain_2: c_int,
    pub rx_equ_dfe_2: c_int,
    pub rx_equ_dfe_3: c_int,
    pub rx_equ_dfe_4: c_int,
    pub rx_equ_dfe_5: c_int,
    pub rx_equ_dfe_6: c_int,
    pub rx_equ_dfe_7: c_int,
    pub rx_equ_dfe_8: c_int,
    pub rx_equ_dfe_9: c_int,
    pub rx_equ_dfe_10: c_int,
    pub rx_equ_dfe_11: c_int,
    pub rx_equ_dfe_12: c_int,
    pub tx_equ_pre1: c_int,
    pub tx_equ_pre3: c_int,
    pub tx_equ_atten: c_int,
    pub tx_equ_post1: c_int,
    pub tx_equ_pre2: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_regdump_to_ethtool {
// A multilane port can have max 4 serdes
    pub equalization: [ice_serdes_equalization_to_ethtool; 4],
}

// Port topology from lport i.e.
// serdes mapping, pcsquad, macport, cage etc...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_port_topology {
    pub pcs_port: u16,
    pub primary_serdes_lane: u16,
    pub serdes_lane_count: u16,
    pub pcs_quad_select: u16,
}

// Macro to make PHY type to Ethtool link mode table entry.
// The index is the PHY type.
//

// Lookup table mapping PHY type low to link speed and Ethtool link modes.
// Array index corresponds to HW PHY type bit, see
// ice_adminq_cmd.h:ICE_PHY_TYPE_LOW_*.
//
// Lookup table mapping PHY type high to link speed and Ethtool link modes.
// Array index corresponds to HW PHY type bit, see
// ice_adminq_cmd.h:ICE_PHY_TYPE_HIGH_
//
