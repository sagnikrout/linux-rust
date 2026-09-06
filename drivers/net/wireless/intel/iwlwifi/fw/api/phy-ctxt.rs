//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/phy-ctxt.h
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
// Copyright (C) 2012-2014, 2018, 2020-2025 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_phy_ctxt_h__
// Supported bands

// Supported channel width, vary if there is VHT support
pub const IWL_PHY_CHANNEL_MODE20: c_uint = 0x0;
pub const IWL_PHY_CHANNEL_MODE40: c_uint = 0x1;
pub const IWL_PHY_CHANNEL_MODE80: c_uint = 0x2;
pub const IWL_PHY_CHANNEL_MODE160: c_uint = 0x3;
// and 320 MHz for EHT
pub const IWL_PHY_CHANNEL_MODE320: c_uint = 0x4;
//
// Control channel position:
// For legacy set bit means upper channel, otherwise lower.
// For VHT - bit-2 marks if the control is lower/upper relative to center-freq
// bits-1:0 mark the distance from the center freq. for 20Mhz, offset is 0.
// For EHT - bit-3 is used for extended distance
// center_freq
// |
// 40Mhz                                     |____|____|
// 80Mhz                                |____|____|____|____|
// 160Mhz                     |____|____|____|____|____|____|____|____|
// 320MHz |____|____|____|____|____|____|____|____|____|____|____|____|____|____|____|____|
// code    1011 1010 1001 1000 0011 0010 0001 0000 0100 0101 0110 0111 1100 1101 1110 1111
//
pub const IWL_PHY_CTRL_POS_ABOVE: c_uint = 0x4;
pub const IWL_PHY_CTRL_POS_OFFS_EXT: c_uint = 0x8;
pub const IWL_PHY_CTRL_POS_OFFS_MSK: c_uint = 0x3;
//
// struct iwl_fw_channel_info_v1 - channel information
//
// @band: PHY_BAND_
// @channel: channel number
// @width: PHY_[VHT|LEGACY]_CHANNEL_
// @ctrl channel: PHY_[VHT|LEGACY]_CTRL_
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_channel_info_v1 {
    pub band: u8,
    pub channel: u8,
    pub width: u8,
    pub ctrl_pos: u8,
    pub /: *mut *mut } __packed; / CHANNEL_CONFIG_API_S_VER_1,
//
// struct iwl_fw_channel_info - channel information
//
// @channel: channel number
// @band: PHY_BAND_
// @width: PHY_[VHT|LEGACY]_CHANNEL_
// @ctrl channel: PHY_[VHT|LEGACY]_CTRL_
// @reserved: for future use and alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_channel_info {
    pub channel: __le32,
    pub band: u8,
    pub width: u8,
    pub ctrl_pos: u8,
    pub reserved: u8,
    pub /: *mut *mut } __packed; /CHANNEL_CONFIG_API_S_VER_2,

// TODO: fix the value, make it depend on firmware at runtime?
pub const NUM_PHY_CTX: c_int = 3;
// TODO: complete missing documentation
//
// struct iwl_phy_context_cmd_tail - tail of iwl_phy_ctx_cmd for alignment with
// various channel structures.
//
// @txchain_info: ???
// @rxchain_info: ???
// @acquisition_data: ???
// @dsp_cfg_flags: set to 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_phy_context_cmd_tail {
    pub txchain_info: __le32,
    pub rxchain_info: __le32,
    pub acquisition_data: __le32,
    pub dsp_cfg_flags: __le32,
    pub __packed: },
//
// struct iwl_phy_context_cmd_v1 - config of the PHY context
// ( PHY_CONTEXT_CMD = 0x8 )
// @id_and_color: ID and color of the relevant Binding
// @action: action to perform, see &enum iwl_ctxt_action
// @apply_time: 0 means immediate apply and context switch.
// other value means apply new params after X usecs
// @tx_param_color: ???
// @ci: channel info
// @tail: command tail
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_phy_context_cmd_v1 {
// COMMON_INDEX_HDR_API_S_VER_1
    pub id_and_color: __le32,
    pub action: __le32,
// PHY_CONTEXT_DATA_API_S_VER_3
    pub apply_time: __le32,
    pub tx_param_color: __le32,
    pub ci: iwl_fw_channel_info,
    pub tail: iwl_phy_context_cmd_tail,
    pub /: *mut *mut } __packed; / PHY_CONTEXT_CMD_API_VER_1,
//
// struct iwl_phy_context_cmd - config of the PHY context
// ( PHY_CONTEXT_CMD = 0x8 )
// @id_and_color: ID and color of the relevant Binding
// @action: action to perform, see &enum iwl_ctxt_action
// @lmac_id: the lmac id the phy context belongs to
// @ci: channel info
// @rxchain_info: ???
// @sbb_bandwidth: 0 disabled, 1 - 40Mhz ... 4 - 320MHz
// @sbb_ctrl_channel_loc: location of the control channel
// @puncture_mask: bitmap of punctured subchannels
// @dsp_cfg_flags: set to 0
// @secondary_ctrl_chnl_loc: location of secondary control channel
// @reserved: reserved to align to 64 bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_phy_context_cmd {
// COMMON_INDEX_HDR_API_S_VER_1
    pub id_and_color: __le32,
    pub action: __le32,
// PHY_CONTEXT_DATA_API_S_VER_3, PHY_CONTEXT_DATA_API_S_VER_4
    pub ci: iwl_fw_channel_info,
    pub lmac_id: __le32,
    pub /: *mut *mut __le32 rxchain_info; / reserved in _VER_4,
    pub sbb_bandwidth: u8,
    pub sbb_ctrl_channel_loc: u8,
    pub /: *mut *mut __le16 puncture_mask; / added in VER_6,
}

// PHY_CONTEXT_CMD_API_VER_4,
// PHY_CONTEXT_CMD_API_VER_5,
// PHY_CONTEXT_CMD_API_VER_6,
// PHY_CONTEXT_CMD_API_S_VER_7
//
