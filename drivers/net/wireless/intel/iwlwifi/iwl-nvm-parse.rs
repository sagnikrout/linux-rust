//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-nvm-parse.h
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
// Copyright (C) 2005-2015, 2018-2026 Intel Corporation
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_nvm_parse_h__

//
// enum iwl_nvm_sbands_flags - modification flags for the channel profiles
//
// @IWL_NVM_SBANDS_FLAGS_LAR: LAR is enabled
// @IWL_NVM_SBANDS_FLAGS_NO_WIDE_IN_5GHZ: disallow 40, 80 and 160MHz on 5GHz
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_nvm_sbands_flags {
    IWL_NVM_SBANDS_FLAGS_LAR		= BIT(0),
    IWL_NVM_SBANDS_FLAGS_NO_WIDE_IN_5GHZ	= BIT(1),
}

//
// enum iwl_puncturing_status - EHT puncturing status from MCC capabilities
// @IWL_PUNCTURING_STATUS_UNKNOWN: puncturing status is not provided by FW
// or is not initialized yet
// @IWL_PUNCTURING_STATUS_ENABLED: puncturing is enabled for the current MCC
// @IWL_PUNCTURING_STATUS_DISABLED: puncturing is disabled for the current MCC
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_puncturing_status {
    IWL_PUNCTURING_STATUS_UNKNOWN,
    IWL_PUNCTURING_STATUS_ENABLED,
    IWL_PUNCTURING_STATUS_DISABLED,
}

//
// struct iwl_reg_capa - struct for global regulatory capabilities, Used for
// handling the different APIs of reg_capa_flags.
//
// @allow_40mhz: 11n channel with a width of 40Mhz is allowed
// for this regulatory domain.
// @allow_80mhz: 11ac channel with a width of 80Mhz is allowed
// for this regulatory domain (valid only in 5 and 6 Ghz).
// @allow_160mhz: 11ac channel with a width of 160Mhz is allowed
// for this regulatory domain (valid only in 5 and 6 Ghz).
// @allow_320mhz: 11be channel with a width of 320Mhz is allowed
// for this regulatory domain (valid only in 6 Ghz).
// @disable_11ax: 11ax is forbidden for this regulatory domain.
// @disable_11be: 11be is forbidden for this regulatory domain.
// @disable_11bn: UHR/11bn is not allowed for this regulatory domain
// @puncturing_status: EHT puncturing status for the current MCC.
// See &enum iwl_puncturing_status.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_reg_capa {
    pub allow_40mhz: bool,
    pub allow_80mhz: bool,
    pub allow_160mhz: bool,
    pub allow_320mhz: bool,
    pub disable_11ax: bool,
    pub disable_11be: bool,
    pub disable_11bn: bool,
    pub puncturing_status: iwl_puncturing_status,
}

//
// enum iwl_nvm_channel_flags - channel flags in NVM
// @NVM_CHANNEL_VALID: channel is usable for this SKU/geo
// @NVM_CHANNEL_IBSS: usable as an IBSS channel and deprecated
// when %IWL_NVM_SBANDS_FLAGS_LAR enabled.
// @NVM_CHANNEL_ALLOW_20MHZ_ACTIVITY: active scanning allowed and
// AP allowed only in 20 MHz. Valid only
// when %IWL_NVM_SBANDS_FLAGS_LAR enabled.
// @NVM_CHANNEL_ACTIVE: active scanning allowed and allows IBSS
// when %IWL_NVM_SBANDS_FLAGS_LAR enabled.
// @NVM_CHANNEL_RADAR: radar detection required
// @NVM_CHANNEL_INDOOR_ONLY: only indoor use is allowed
// @NVM_CHANNEL_GO_CONCURRENT: GO operation is allowed when connected to BSS
// on same channel on 2.4 or same UNII band on 5.2
// @NVM_CHANNEL_UNIFORM: uniform spreading required
// @NVM_CHANNEL_20MHZ: 20 MHz channel okay
// @NVM_CHANNEL_40MHZ: 40 MHz channel okay
// @NVM_CHANNEL_80MHZ: 80 MHz channel okay
// @NVM_CHANNEL_160MHZ: 160 MHz channel okay
// @NVM_CHANNEL_DC_HIGH: DC HIGH required/allowed (?)
// @NVM_CHANNEL_VLP: client support connection to UHB VLP AP
// @NVM_CHANNEL_AFC: client support connection to UHB AFC AP
// @NVM_CHANNEL_VLP_AP_NOT_ALLOWED: UHB VLP AP not allowed,
// Valid only when %NVM_CHANNEL_VLP is enabled.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_nvm_channel_flags {
    NVM_CHANNEL_VALID			= BIT(0),
    NVM_CHANNEL_IBSS			= BIT(1),
    NVM_CHANNEL_ALLOW_20MHZ_ACTIVITY	= BIT(2),
    NVM_CHANNEL_ACTIVE			= BIT(3),
    NVM_CHANNEL_RADAR			= BIT(4),
    NVM_CHANNEL_INDOOR_ONLY			= BIT(5),
    NVM_CHANNEL_GO_CONCURRENT		= BIT(6),
    NVM_CHANNEL_UNIFORM			= BIT(7),
    NVM_CHANNEL_20MHZ			= BIT(8),
    NVM_CHANNEL_40MHZ			= BIT(9),
    NVM_CHANNEL_80MHZ			= BIT(10),
    NVM_CHANNEL_160MHZ			= BIT(11),
    NVM_CHANNEL_DC_HIGH			= BIT(12),
    NVM_CHANNEL_VLP				= BIT(13),
    NVM_CHANNEL_AFC				= BIT(14),
    NVM_CHANNEL_VLP_AP_NOT_ALLOWED		= BIT(15),
}

//
// iwl_parse_nvm_data - parse NVM data and return values
//
// This function parses all NVM values we need and then
// returns a (newly allocated) struct containing all the
// relevant values for driver use. The struct must be freed
// later with iwl_free_nvm_data().
//
// iwl_parse_nvm_mcc_info - parse MCC (mobile country code) info coming from FW
//
// This function parses the regulatory channel data received as a
// MCC_UPDATE_CMD command.
//
// Return: a newly allocation regulatory domain, to be given to the regulatory
// core. In case the geo_info is set handle accordingly. An ERR_PTR is
// returned on error. If not given to the regulatory core, the user is
// responsible for freeing the regdomain returned here with kfree().
//
// @trans: the transport
// @num_of_ch: the number of channels
// @channels: channel map
// @fw_mcc: firmware country code
// @geo_info: geo info value
// @cap: capability
// @resp_ver: FW response version
// @puncturing_status: FW puncturing status for current MCC, when available
//
// struct iwl_nvm_section - describes an NVM section in memory.
//
// This struct holds an NVM section read from the NIC using NVM_ACCESS_CMD,
// and saved for later use by the driver. Not all NVM sections are saved
// this way, only the needed ones.
// @length: length of the section
// @data: section data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_section {
    pub length: u16,
    pub data: *const u8,
}

//
// iwl_read_external_nvm - Reads external NVM from a file into nvm_sections
// @trans: the transport
// @nvm_file_name: the filename to request
// @nvm_sections: sections data to fill
// Return: 0 on success or an error code
//
// iwl_get_nvm - retrieve NVM data from firmware
//
// Allocates a new iwl_nvm_data structure, fills it with
// NVM data, and returns it to caller.
//
// iwl_parse_mei_nvm_data - parse the mei_nvm_data and get an iwl_nvm_data
//
// iwl_reinit_cab - to be called when the tx_chains or rx_chains are modified
//
