//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/alive.h
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
// Copyright (C) 2012-2014, 2018, 2020-2021, 2024-2025 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_alive_h__
// alive response is_valid values

// alive response ver_type values
// alive response ver_subtype values
pub const IWL_ALIVE_STATUS_ERR: c_uint = 0xDEAD;
pub const IWL_ALIVE_STATUS_OK: c_uint = 0xCAFE;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_lmac_debug_addrs {
    pub /: *mut *mut __le32 error_event_table_ptr; / SRAM address for error log,
    pub /: *mut *mut __le32 log_event_table_ptr; / SRAM address for LMAC event log,
    pub cpu_register_ptr: __le32,
    pub dbgm_config_ptr: __le32,
    pub alive_counter_ptr: __le32,
    pub /: *mut *mut __le32 scd_base_ptr; / SRAM address for SCD,
    pub /: *mut *mut __le32 st_fwrd_addr; / pointer to Store and forward,
    pub st_fwrd_size: __le32,
    pub /: *mut *mut } __packed; / UCODE_DEBUG_ADDRS_API_S_VER_2,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_lmac_alive {
    pub ucode_major: __le32,
    pub ucode_minor: __le32,
    pub ver_subtype: u8,
    pub ver_type: u8,
    pub mac: u8,
    pub opt: u8,
    pub timestamp: __le32,
    pub dbg_ptrs: iwl_lmac_debug_addrs,
    pub /: *mut *mut } __packed; / UCODE_ALIVE_NTFY_API_S_VER_3,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_umac_debug_addrs {
    pub /: *mut *mut __le32 error_info_addr; / SRAM address for UMAC error log,
    pub dbg_print_buff_addr: __le32,
    pub /: *mut *mut } __packed; / UMAC_DEBUG_ADDRS_API_S_VER_1,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_umac_alive {
    pub /: *mut *mut __le32 umac_major; / UMAC version: major,
    pub /: *mut *mut __le32 umac_minor; / UMAC version: minor,
    pub dbg_ptrs: iwl_umac_debug_addrs,
    pub /: *mut *mut } __packed; / UMAC_ALIVE_DATA_API_S_VER_2,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_sku_id {
    pub data: [__le32; 3],
    pub /: *mut *mut } __packed; / SKU_ID_API_S_VER_1,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_alive_ntf_v3 {
    pub status: __le16,
    pub flags: __le16,
    pub lmac_data: iwl_lmac_alive,
    pub umac_data: iwl_umac_alive,
    pub /: *mut *mut } __packed; / UCODE_ALIVE_NTFY_API_S_VER_3,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_imr_alive_info {
    pub base_addr: __le64,
    pub size: __le32,
    pub enabled: __le32,
    pub /: *mut *mut } __packed; / IMR_ALIVE_INFO_API_S_VER_1,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_alive_ntf_v7 {
    pub status: __le16,
    pub flags: __le16,
    pub lmac_data: [iwl_lmac_alive; 2],
    pub umac_data: iwl_umac_alive,
    pub sku_id: iwl_sku_id,
    pub imr: iwl_imr_alive_info,
    pub /: *mut *mut } __packed; / UCODE_ALIVE_NTFY_API_S_VER_6,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_alive_ntf {
    pub status: __le16,
    pub flags: __le16,
    pub lmac_data: [iwl_lmac_alive; 2],
    pub umac_data: iwl_umac_alive,
    pub sku_id: iwl_sku_id,
    pub imr: iwl_imr_alive_info,
    pub platform_id: __le64,
    pub /: *mut *mut } __packed; / UCODE_ALIVE_NTFY_API_S_VER_8,
//
// enum iwl_extended_cfg_flags - commands driver may send before
// finishing init flow
// @IWL_INIT_DEBUG_CFG: driver is going to send debug config command
// @IWL_INIT_NVM: driver is going to send NVM_ACCESS commands
// @IWL_INIT_PHY: driver is going to send the PHY_CONFIGURATION_CMD
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_extended_cfg_flags {
    IWL_INIT_DEBUG_CFG,
    IWL_INIT_NVM,
    IWL_INIT_PHY,
}

//
// struct iwl_init_extended_cfg_cmd - mark what commands ucode should wait for
// before finishing init flows
// @init_flags: values from iwl_extended_cfg_flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_init_extended_cfg_cmd {
    pub init_flags: __le32,
    pub /: *mut *mut } __packed; / INIT_EXTENDED_CFG_CMD_API_S_VER_1,
//
// struct iwl_radio_version_notif - information on the radio version
// ( RADIO_VERSION_NOTIFICATION = 0x68 )
// @radio_flavor: radio flavor
// @radio_step: radio version step
// @radio_dash: radio version dash
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_radio_version_notif {
    pub radio_flavor: __le32,
    pub radio_step: __le32,
    pub radio_dash: __le32,
    pub /: *mut *mut } __packed; / RADIO_VERSION_NOTOFICATION_S_VER_1,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_card_state_flags {
    CARD_ENABLED		= 0x00,
    HW_CARD_DISABLED	= 0x01,
    SW_CARD_DISABLED	= 0x02,
    CT_KILL_CARD_DISABLED	= 0x04,
    HALT_CARD_DISABLED	= 0x08,
    CARD_DISABLED_MSK	= 0x0f,
    CARD_IS_RX_ON		= 0x10,
}

//
// enum iwl_error_recovery_flags - flags for error recovery cmd
// @ERROR_RECOVERY_UPDATE_DB: update db from blob sent
// @ERROR_RECOVERY_END_OF_RECOVERY: end of recovery
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_error_recovery_flags {
    ERROR_RECOVERY_UPDATE_DB = BIT(0),
    ERROR_RECOVERY_END_OF_RECOVERY = BIT(1),
}

//
// struct iwl_fw_error_recovery_cmd - recovery cmd sent upon assert
// @flags: &enum iwl_error_recovery_flags
// @buf_size: db buffer size in bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_error_recovery_cmd {
    pub flags: __le32,
    pub buf_size: __le32,
    pub /: *mut *mut } __packed; / ERROR_RECOVERY_CMD_HDR_API_S_VER_1,
