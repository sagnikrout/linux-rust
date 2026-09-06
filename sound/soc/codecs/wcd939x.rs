//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wcd939x.h
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
// Copyright (c) 2018-2021, The Linux Foundation. All rights reserved.
// Copyright (c) 2022 Qualcomm Innovation Center, Inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd939x_tx_sdw_ports {
    WCD939X_ADC_1_4_PORT = 1,
    WCD939X_ADC_DMIC_1_2_PORT,
    WCD939X_DMIC_0_3_MBHC_PORT,
    WCD939X_DMIC_3_7_PORT,
    WCD939X_MAX_TX_SWR_PORTS = WCD939X_DMIC_3_7_PORT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd939x_tx_sdw_channels {
    WCD939X_ADC1,
    WCD939X_ADC2,
    WCD939X_ADC3,
    WCD939X_ADC4,
    WCD939X_DMIC0,
    WCD939X_DMIC1,
    WCD939X_MBHC,
    WCD939X_DMIC2,
    WCD939X_DMIC3,
    WCD939X_DMIC4,
    WCD939X_DMIC5,
    WCD939X_DMIC6,
    WCD939X_DMIC7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd939x_rx_sdw_ports {
    WCD939X_HPH_PORT = 1,
    WCD939X_CLSH_PORT,
    WCD939X_COMP_PORT,
    WCD939X_LO_PORT,
    WCD939X_DSD_PORT,
    WCD939X_HIFI_PCM_PORT,
    WCD939X_MAX_RX_SWR_PORTS = WCD939X_HIFI_PCM_PORT,
    WCD939X_MAX_SWR_PORTS = WCD939X_MAX_RX_SWR_PORTS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd939x_rx_sdw_channels {
    WCD939X_HPH_L,
    WCD939X_HPH_R,
    WCD939X_CLSH,
    WCD939X_COMP_L,
    WCD939X_COMP_R,
    WCD939X_LO,
    WCD939X_DSD_L,
    WCD939X_DSD_R,
    WCD939X_HIFI_PCM_L,
    WCD939X_HIFI_PCM_R,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcd939x_sdw_priv {
    pub sdev: *mut sdw_slave,
    pub sconfig: sdw_stream_config,
    pub sruntime: *mut sdw_stream_runtime,
    pub port_config: [sdw_port_config; WCD939X_MAX_SWR_PORTS],
    pub ch_info: *const wcd_sdw_ch_info,
    pub port_enable: [bool; WCD939X_MAX_SWR_CH_IDS],
    pub active_ports: c_int,
    pub is_tx: bool,
    pub wcd939x: *mut wcd939x_priv,
    pub slave_irq: *mut irq_domain,
    pub regmap: *mut regmap,
}

