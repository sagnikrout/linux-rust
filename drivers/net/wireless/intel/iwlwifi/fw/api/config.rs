//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/config.h
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
// Copyright (C) 2012-2014, 2018-2019, 2023-2024 Intel Corporation
// Copyright (C) 2026 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_config_h__
//
// struct iwl_dqa_enable_cmd
// @cmd_queue: the TXQ number of the command queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dqa_enable_cmd {
    pub cmd_queue: __le32,
    pub /: *mut *mut } __packed; / DQA_CONTROL_CMD_API_S_VER_1,
//
// struct iwl_tx_ant_cfg_cmd
// @valid: valid antenna configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tx_ant_cfg_cmd {
    pub valid: __le32,
    pub __packed: },
//
// struct iwl_calib_ctrl - Calibration control struct.
// Sent as part of the phy configuration command.
// @flow_trigger: bitmap for which calibrations to perform according to
// flow triggers, using &enum iwl_calib_cfg
// @event_trigger: bitmap for which calibrations to perform according to
// event triggers, using &enum iwl_calib_cfg
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_calib_ctrl {
    pub flow_trigger: __le32,
    pub event_trigger: __le32,
    pub __packed: },
// This enum defines the bitmap of various calibrations to enable in both
// init ucode and runtime ucode through CALIBRATION_CFG_CMD.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_calib_cfg {
    IWL_CALIB_CFG_XTAL_IDX			= BIT(0),
    IWL_CALIB_CFG_TEMPERATURE_IDX		= BIT(1),
    IWL_CALIB_CFG_VOLTAGE_READ_IDX		= BIT(2),
    IWL_CALIB_CFG_PAPD_IDX			= BIT(3),
    IWL_CALIB_CFG_TX_PWR_IDX		= BIT(4),
    IWL_CALIB_CFG_DC_IDX			= BIT(5),
    IWL_CALIB_CFG_BB_FILTER_IDX		= BIT(6),
    IWL_CALIB_CFG_LO_LEAKAGE_IDX		= BIT(7),
    IWL_CALIB_CFG_TX_IQ_IDX			= BIT(8),
    IWL_CALIB_CFG_TX_IQ_SKEW_IDX		= BIT(9),
    IWL_CALIB_CFG_RX_IQ_IDX			= BIT(10),
    IWL_CALIB_CFG_RX_IQ_SKEW_IDX		= BIT(11),
    IWL_CALIB_CFG_SENSITIVITY_IDX		= BIT(12),
    IWL_CALIB_CFG_CHAIN_NOISE_IDX		= BIT(13),
    IWL_CALIB_CFG_DISCONNECTED_ANT_IDX	= BIT(14),
    IWL_CALIB_CFG_ANT_COUPLING_IDX		= BIT(15),
    IWL_CALIB_CFG_DAC_IDX			= BIT(16),
    IWL_CALIB_CFG_ABS_IDX			= BIT(17),
    IWL_CALIB_CFG_AGC_IDX			= BIT(18),
}

//
// struct iwl_phy_specific_cfg - specific PHY filter configuration
//
// Sent as part of the phy configuration command (v3) to configure specific FW
// defined PHY filters that can be applied to each antenna.
//
// @filter_cfg_chains: filter config id for LMAC1 chain A, LMAC1 chain B,
// LMAC2 chain A, LMAC2 chain B (in that order)
// values: 0: no filter; 0xffffffff: reserved; otherwise: filter id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_phy_specific_cfg {
    pub filter_cfg_chains: [__le32; 4],
    pub PHY_SPECIFIC_CONFIGURATION_API_VER_1*/: *mut *mut } __packed; /,
//
// struct iwl_phy_cfg_cmd_v1 - Phy configuration command
//
// @phy_cfg: PHY configuration value, uses &enum iwl_fw_phy_cfg
// @calib_control: calibration control data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_phy_cfg_cmd_v1 {
    pub phy_cfg: __le32,
    pub calib_control: iwl_calib_ctrl,
    pub __packed: },
//
// struct iwl_phy_cfg_cmd_v3 - Phy configuration command (v3)
//
// @phy_cfg: PHY configuration value, uses &enum iwl_fw_phy_cfg
// @calib_control: calibration control data
// @phy_specific_cfg: configure predefined PHY filters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_phy_cfg_cmd_v3 {
    pub phy_cfg: __le32,
    pub calib_control: iwl_calib_ctrl,
    pub phy_specific_cfg: iwl_phy_specific_cfg,
    pub /: *mut *mut } __packed; / PHY_CONFIGURATION_CMD_API_S_VER_3,
