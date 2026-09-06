//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wlcore/ini.h
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
// This file is part of wl1271
//
// Copyright (C) 2010 Nokia Corporation
//
// Contact: Luciano Coelho <luciano.coelho@nokia.com>
//
pub const GENERAL_SETTINGS_DRPW_LPD: c_uint = 0xc0;

pub const WL1271_INI_MAX_SMART_REFLEX_PARAM: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_ini_general_params {
    pub ref_clock: u8,
    pub settling_time: u8,
    pub clk_valid_on_wakeup: u8,
    pub dc2dc_mode: u8,
    pub dual_mode_select: u8,
    pub tx_bip_fem_auto_detect: u8,
    pub tx_bip_fem_manufacturer: u8,
    pub general_settings: u8,
    pub sr_state: u8,
    pub srf1: [u8; WL1271_INI_MAX_SMART_REFLEX_PARAM],
    pub srf2: [u8; WL1271_INI_MAX_SMART_REFLEX_PARAM],
    pub srf3: [u8; WL1271_INI_MAX_SMART_REFLEX_PARAM],
    pub __packed: },
pub const WL128X_INI_MAX_SETTINGS_PARAM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl128x_ini_general_params {
    pub ref_clock: u8,
    pub settling_time: u8,
    pub clk_valid_on_wakeup: u8,
    pub tcxo_ref_clock: u8,
    pub tcxo_settling_time: u8,
    pub tcxo_valid_on_wakeup: u8,
    pub tcxo_ldo_voltage: u8,
    pub xtal_itrim_val: u8,
    pub platform_conf: u8,
    pub dual_mode_select: u8,
    pub tx_bip_fem_auto_detect: u8,
    pub tx_bip_fem_manufacturer: u8,
    pub general_settings: [u8; WL128X_INI_MAX_SETTINGS_PARAM],
    pub sr_state: u8,
    pub srf1: [u8; WL1271_INI_MAX_SMART_REFLEX_PARAM],
    pub srf2: [u8; WL1271_INI_MAX_SMART_REFLEX_PARAM],
    pub srf3: [u8; WL1271_INI_MAX_SMART_REFLEX_PARAM],
    pub __packed: },
pub const WL1271_INI_RSSI_PROCESS_COMPENS_SIZE: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_ini_band_params_2 {
    pub rx_trace_insertion_loss: u8,
    pub tx_trace_loss: u8,
    pub rx_rssi_process_compens: [u8; WL1271_INI_RSSI_PROCESS_COMPENS_SIZE],
    pub __packed: },
pub const WL1271_INI_CHANNEL_COUNT_2: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl128x_ini_band_params_2 {
    pub rx_trace_insertion_loss: u8,
    pub tx_trace_loss: [u8; WL1271_INI_CHANNEL_COUNT_2],
    pub rx_rssi_process_compens: [u8; WL1271_INI_RSSI_PROCESS_COMPENS_SIZE],
    pub __packed: },
pub const WL1271_INI_RATE_GROUP_COUNT: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_ini_fem_params_2 {
    pub tx_bip_ref_pd_voltage: __le16,
    pub tx_bip_ref_power: u8,
    pub tx_bip_ref_offset: u8,
    pub tx_per_rate_pwr_limits_normal: [u8; WL1271_INI_RATE_GROUP_COUNT],
    pub tx_per_rate_pwr_limits_degraded: [u8; WL1271_INI_RATE_GROUP_COUNT],
    pub tx_per_rate_pwr_limits_extreme: [u8; WL1271_INI_RATE_GROUP_COUNT],
    pub tx_per_chan_pwr_limits_11b: [u8; WL1271_INI_CHANNEL_COUNT_2],
    pub tx_per_chan_pwr_limits_ofdm: [u8; WL1271_INI_CHANNEL_COUNT_2],
    pub tx_pd_vs_rate_offsets: [u8; WL1271_INI_RATE_GROUP_COUNT],
    pub tx_ibias: [u8; WL1271_INI_RATE_GROUP_COUNT],
    pub rx_fem_insertion_loss: u8,
    pub degraded_low_to_normal_thr: u8,
    pub normal_to_degraded_high_thr: u8,
    pub __packed: },
pub const WL128X_INI_RATE_GROUP_COUNT: c_int = 7;
// low and high temperatures
pub const WL128X_INI_PD_VS_TEMPERATURE_RANGES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl128x_ini_fem_params_2 {
    pub tx_bip_ref_pd_voltage: __le16,
    pub tx_bip_ref_power: u8,
    pub tx_bip_ref_offset: u8,
    pub tx_per_rate_pwr_limits_normal: [u8; WL128X_INI_RATE_GROUP_COUNT],
    pub tx_per_rate_pwr_limits_degraded: [u8; WL128X_INI_RATE_GROUP_COUNT],
    pub tx_per_rate_pwr_limits_extreme: [u8; WL128X_INI_RATE_GROUP_COUNT],
    pub tx_per_chan_pwr_limits_11b: [u8; WL1271_INI_CHANNEL_COUNT_2],
    pub tx_per_chan_pwr_limits_ofdm: [u8; WL1271_INI_CHANNEL_COUNT_2],
    pub tx_pd_vs_rate_offsets: [u8; WL128X_INI_RATE_GROUP_COUNT],
    pub 1]: u8 tx_ibias[WL128X_INI_RATE_GROUP_COUNT +,
    pub tx_pd_vs_chan_offsets: [u8; WL1271_INI_CHANNEL_COUNT_2],
    pub tx_pd_vs_temperature: [u8; WL128X_INI_PD_VS_TEMPERATURE_RANGES],
    pub rx_fem_insertion_loss: u8,
    pub degraded_low_to_normal_thr: u8,
    pub normal_to_degraded_high_thr: u8,
    pub __packed: },
pub const WL1271_INI_CHANNEL_COUNT_5: c_int = 35;
pub const WL1271_INI_SUB_BAND_COUNT_5: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_ini_band_params_5 {
    pub rx_trace_insertion_loss: [u8; WL1271_INI_SUB_BAND_COUNT_5],
    pub tx_trace_loss: [u8; WL1271_INI_SUB_BAND_COUNT_5],
    pub rx_rssi_process_compens: [u8; WL1271_INI_RSSI_PROCESS_COMPENS_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl128x_ini_band_params_5 {
    pub rx_trace_insertion_loss: [u8; WL1271_INI_SUB_BAND_COUNT_5],
    pub tx_trace_loss: [u8; WL1271_INI_CHANNEL_COUNT_5],
    pub rx_rssi_process_compens: [u8; WL1271_INI_RSSI_PROCESS_COMPENS_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_ini_fem_params_5 {
    pub tx_bip_ref_pd_voltage: [__le16; WL1271_INI_SUB_BAND_COUNT_5],
    pub tx_bip_ref_power: [u8; WL1271_INI_SUB_BAND_COUNT_5],
    pub tx_bip_ref_offset: [u8; WL1271_INI_SUB_BAND_COUNT_5],
    pub tx_per_rate_pwr_limits_normal: [u8; WL1271_INI_RATE_GROUP_COUNT],
    pub tx_per_rate_pwr_limits_degraded: [u8; WL1271_INI_RATE_GROUP_COUNT],
    pub tx_per_rate_pwr_limits_extreme: [u8; WL1271_INI_RATE_GROUP_COUNT],
    pub tx_per_chan_pwr_limits_ofdm: [u8; WL1271_INI_CHANNEL_COUNT_5],
    pub tx_pd_vs_rate_offsets: [u8; WL1271_INI_RATE_GROUP_COUNT],
    pub tx_ibias: [u8; WL1271_INI_RATE_GROUP_COUNT],
    pub rx_fem_insertion_loss: [u8; WL1271_INI_SUB_BAND_COUNT_5],
    pub degraded_low_to_normal_thr: u8,
    pub normal_to_degraded_high_thr: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl128x_ini_fem_params_5 {
    pub tx_bip_ref_pd_voltage: [__le16; WL1271_INI_SUB_BAND_COUNT_5],
    pub tx_bip_ref_power: [u8; WL1271_INI_SUB_BAND_COUNT_5],
    pub tx_bip_ref_offset: [u8; WL1271_INI_SUB_BAND_COUNT_5],
    pub tx_per_rate_pwr_limits_normal: [u8; WL128X_INI_RATE_GROUP_COUNT],
    pub tx_per_rate_pwr_limits_degraded: [u8; WL128X_INI_RATE_GROUP_COUNT],
    pub tx_per_rate_pwr_limits_extreme: [u8; WL128X_INI_RATE_GROUP_COUNT],
    pub tx_per_chan_pwr_limits_ofdm: [u8; WL1271_INI_CHANNEL_COUNT_5],
    pub tx_pd_vs_rate_offsets: [u8; WL128X_INI_RATE_GROUP_COUNT],
    pub tx_ibias: [u8; WL128X_INI_RATE_GROUP_COUNT],
    pub tx_pd_vs_chan_offsets: [u8; WL1271_INI_CHANNEL_COUNT_5],
    pub rx_fem_insertion_loss: [u8; WL1271_INI_SUB_BAND_COUNT_5],
    pub degraded_low_to_normal_thr: u8,
    pub normal_to_degraded_high_thr: u8,
    pub __packed: },
// NVS data structure
pub const WL1271_INI_NVS_SECTION_SIZE: c_int = 468;
// We have four FEM module types: 0-RFMD, 1-TQS, 2-SKW, 3-TQS_HP
pub const WL1271_INI_FEM_MODULE_COUNT: c_int = 4;
//
// In NVS we only store two FEM module entries -
// FEM modules 0,2,3 are stored in entry 0
// FEM module 1 is stored in entry 1
//
pub const WL12XX_NVS_FEM_MODULE_COUNT: c_int = 2;

pub const WL1271_INI_LEGACY_NVS_FILE_SIZE: c_int = 800;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_nvs_file {
// NVS section - must be first!
    pub nvs: [u8; WL1271_INI_NVS_SECTION_SIZE],
// INI section
    pub general_params: wl1271_ini_general_params,
    pub padding1: u8,
    pub stat_radio_params_2: wl1271_ini_band_params_2,
    pub padding2: u8,
    pub params: wl1271_ini_fem_params_2,
    pub padding: u8,
    pub dyn_radio_params_2: [}; WL12XX_NVS_FEM_MODULE_COUNT],
    pub stat_radio_params_5: wl1271_ini_band_params_5,
    pub padding3: u8,
    pub params: wl1271_ini_fem_params_5,
    pub padding: u8,
    pub dyn_radio_params_5: [}; WL12XX_NVS_FEM_MODULE_COUNT],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl128x_nvs_file {
// NVS section - must be first!
    pub nvs: [u8; WL1271_INI_NVS_SECTION_SIZE],
// INI section
    pub general_params: wl128x_ini_general_params,
    pub fem_vendor_and_options: u8,
    pub stat_radio_params_2: wl128x_ini_band_params_2,
    pub padding2: u8,
    pub params: wl128x_ini_fem_params_2,
    pub padding: u8,
    pub dyn_radio_params_2: [}; WL12XX_NVS_FEM_MODULE_COUNT],
    pub stat_radio_params_5: wl128x_ini_band_params_5,
    pub padding3: u8,
    pub params: wl128x_ini_fem_params_5,
    pub padding: u8,
    pub dyn_radio_params_5: [}; WL12XX_NVS_FEM_MODULE_COUNT],
    pub __packed: },
