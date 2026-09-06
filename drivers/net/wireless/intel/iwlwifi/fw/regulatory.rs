//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/regulatory.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2023-2026 Intel Corporation
//

// Macro flag: #define __fw_regulatory_h__

pub const BIOS_SAR_MAX_PROFILE_NUM: c_int = 4;
//
// Each SAR profile has (up to, depends on the table revision) 4 chains:
// chain A, chain B, chain A when in CDB, chain B when in CDB
//
pub const BIOS_SAR_MAX_CHAINS_PER_PROFILE: c_int = 4;
pub const BIOS_SAR_NUM_CHAINS: c_int = 2;
pub const BIOS_SAR_MAX_SUB_BANDS_NUM: c_int = 12;
pub const BIOS_PPAG_MAX_SUB_BANDS_NUM: c_int = 12;
pub const BIOS_GEO_NUM_CHAINS: c_int = 2;
pub const BIOS_GEO_MAX_NUM_BANDS: c_int = 4;
pub const BIOS_GEO_MAX_PROFILE_NUM: c_int = 8;
pub const BIOS_GEO_MIN_PROFILE_NUM: c_int = 3;

pub const IWL_REDUCE_POWER_FLAGS_POS: c_int = 1;
// PPAG gain value bounds in 1/8 dBm

pub const IWL_PPAG_MAX_LB: c_int = 24;

pub const IWL_PPAG_MAX_HB: c_int = 40;
pub const IWL_PPAG_ETSI_CHINA_MASK: c_int = 3;
pub const IWL_PPAG_REV3_MASK: c_uint = 0x7FF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tas_selection_data {
}

pub const BIOS_MCC_CHINA: c_uint = 0x434e;
//
// The profile for revision 2 is a superset of revision 1, which is in
// turn a superset of revision 0.  So we can store all revisions
// inside revision 2, which is what we represent here.
//
// struct iwl_sar_profile_chain - per-chain values of a SAR profile
// @subbands: the SAR value for each subband
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_sar_profile_chain {
    pub subbands: [u8; BIOS_SAR_MAX_SUB_BANDS_NUM],
}

//
// struct iwl_sar_profile - SAR profile from SAR tables
// @enabled: whether the profile is enabled or not
// @chains: per-chain SAR values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_sar_profile {
    pub enabled: bool,
    pub chains: [iwl_sar_profile_chain; BIOS_SAR_MAX_CHAINS_PER_PROFILE],
}

// Same thing as with SAR, all revisions fit in revision 2
//
// struct iwl_geo_profile_band - per-band geo SAR offsets
// @max: the max tx power allowed for the band
// @chains: SAR offsets values for each chain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_geo_profile_band {
    pub max: u8,
    pub chains: [u8; BIOS_GEO_NUM_CHAINS],
}

//
// struct iwl_geo_profile - geo profile
// @bands: per-band table of the SAR offsets
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_geo_profile {
    pub bands: [iwl_geo_profile_band; BIOS_GEO_MAX_NUM_BANDS],
}

// Same thing as with SAR, all revisions fit in revision 2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ppag_chain {
    pub subbands: [i8; BIOS_PPAG_MAX_SUB_BANDS_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tas_data {
    pub block_list_size: u8,
    pub block_list_array: [u16; IWL_WTAS_BLACK_LIST_MAX],
    pub table_source: u8,
    pub table_revision: u8,
    pub tas_selection: u32,
}

// For DSM revision 0 and 4
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dsm_funcs {
    DSM_FUNC_QUERY = 0,
    DSM_FUNC_DISABLE_SRD = 1,
    DSM_FUNC_ENABLE_INDONESIA_5G2 = 2,
    DSM_FUNC_ENABLE_6E = 3,
    DSM_FUNC_REGULATORY_CONFIG = 4,
    DSM_FUNC_11AX_ENABLEMENT = 6,
    DSM_FUNC_ENABLE_UNII4_CHAN = 7,
    DSM_FUNC_ACTIVATE_CHANNEL = 8,
    DSM_FUNC_FORCE_DISABLE_CHANNELS = 9,
    DSM_FUNC_ENERGY_DETECTION_THRESHOLD = 10,
    DSM_FUNC_RFI_CONFIG = 11,
    DSM_FUNC_ENABLE_11BE = 12,
    DSM_FUNC_ENABLE_11BN = 13,
    DSM_FUNC_ENABLE_UNII_9 = 14,
    DSM_FUNC_NUM_FUNCS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dsm_values_srd {
    DSM_VALUE_SRD_ACTIVE,
    DSM_VALUE_SRD_PASSIVE,
    DSM_VALUE_SRD_DISABLE,
    DSM_VALUE_SRD_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dsm_values_indonesia {
    DSM_VALUE_INDONESIA_DISABLE,
    DSM_VALUE_INDONESIA_ENABLE,
    DSM_VALUE_INDONESIA_RESERVED,
    DSM_VALUE_INDONESIA_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dsm_unii4_bitmap {
    DSM_VALUE_UNII4_US_OVERRIDE_MSK		= BIT(0),
    DSM_VALUE_UNII4_US_EN_MSK		= BIT(1),
    DSM_VALUE_UNII4_ETSI_OVERRIDE_MSK	= BIT(2),
    DSM_VALUE_UNII4_ETSI_EN_MSK		= BIT(3),
    DSM_VALUE_UNII4_CANADA_OVERRIDE_MSK	= BIT(4),
    DSM_VALUE_UNII4_CANADA_EN_MSK		= BIT(5),
}

pub const DSM_11AX_ALLOW_BITMAP: c_uint = 0xF;
pub const DSM_EDT_ALLOWED_BITMAP: c_uint = 0x7ffff0;
pub const DSM_FORCE_DISABLE_CHANNELS_ALLOWED_BITMAP: c_uint = 0x7FF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dsm_values_rfi {
    DSM_VALUE_RFI_DLVR_DISABLE	= BIT(0),
    DSM_VALUE_RFI_DDR_DISABLE	= BIT(1),
}

extern "C" {
    pub fn iwl_rfi_is_enabled_in_bios(fwrt: *mut iwl_fw_runtime) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dsm_masks_reg {
    DSM_MASK_CHINA_22_REG = BIT(2)
}

// Print the PPAG table as read from BIOS
extern "C" {
    pub fn iwl_bios_print_ppag(fwrt: *mut iwl_fw_runtime, n_subbands: c_int);
}
extern "C" {
    pub fn iwl_sar_geo_support(fwrt: *mut iwl_fw_runtime) -> bool;
}
extern "C" {
    pub fn iwl_is_ppag_approved(fwrt: *mut iwl_fw_runtime) -> bool;
}
extern "C" {
    pub fn iwl_is_tas_approved() -> bool;
}
extern "C" {
    pub fn iwl_add_mcc_to_tas_block_list(list: *mut u16, size: *mut u8, mcc: u16) -> bool;
}
extern "C" {
    pub fn iwl_bios_get_wrds_table(fwrt: *mut iwl_fw_runtime) -> c_int;
}
extern "C" {
    pub fn iwl_bios_get_ewrd_table(fwrt: *mut iwl_fw_runtime) -> c_int;
}
extern "C" {
    pub fn iwl_bios_get_wgds_table(fwrt: *mut iwl_fw_runtime) -> c_int;
}
extern "C" {
    pub fn iwl_bios_get_ppag_table(fwrt: *mut iwl_fw_runtime) -> c_int;
}
extern "C" {
    pub fn iwl_bios_get_mcc(fwrt: *mut iwl_fw_runtime, mcc: *mut c_char) -> c_int;
}
extern "C" {
    pub fn iwl_bios_get_eckv(fwrt: *mut iwl_fw_runtime, ext_clk: *mut u32) -> c_int;
}
extern "C" {
    pub fn iwl_bios_get_wbem(fwrt: *mut iwl_fw_runtime, value: *mut u32) -> c_int;
}
// For revision 4 and above driver is pipe
extern "C" {
    pub fn iwl_puncturing_is_allowed_in_bios(puncturing: u32, mcc: u16) -> bool;
}

extern "C" {
    pub fn iwl_bios_get_dsbr(fwrt: *mut iwl_fw_runtime, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn iwl_bios_get_phy_filters(fwrt: *mut iwl_fw_runtime) -> c_int;
}
