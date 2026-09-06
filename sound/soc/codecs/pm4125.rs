//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/pm4125.h
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
// Copyright (c) 2023-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

pub const PM4125_ANA_BASE_ADDR: c_uint = 0x3000;
pub const PM4125_DIG_BASE_ADDR: c_uint = 0x3400;

pub const PM4125_ANA_MICBIAS_MICB_PULL_ENABLE: c_int = 1;
pub const PM4125_ANA_MICBIAS_MICB_PULL_DISABLE: c_int = 0;

pub const PM4125_ANA_MBHC_ELECT_BIAS_ENABLE: c_int = 1;
pub const PM4125_ANA_MBHC_ELECT_BIAS_DISABLE: c_int = 0;

pub const PM4125_ANA_NCP_ENABLE: c_int = 1;
pub const PM4125_ANA_NCP_DISABLE: c_int = 0;

pub const PM4125_ANA_HPHPA_CNP_CTL_1_EN: c_int = 1;

pub const PM4125_ANA_HPHPA_CNP_OCP_ENABLE: c_int = 1;
pub const PM4125_ANA_HPHPA_CNP_OCP_DISABLE: c_int = 0;

pub const PM4125_ANA_HPHPA_FSM_CLK_DIV_ENABLE: c_int = 1;
pub const PM4125_ANA_HPHPA_FSM_CLK_DIV_DISABLE: c_int = 0;

pub const PM4125_ANA_SURGE_PROTECTION_ENABLE: c_int = 1;
pub const PM4125_ANA_SURGE_PROTECTION_DISABLE: c_int = 0;

pub const PM4125_ANA_COMBO_PA_SELECT_EAR: c_int = 0;
pub const PM4125_ANA_COMBO_PA_SELECT_LO: c_int = 1;

pub const PM4125_ANA_MBIAS_EN_ENABLE: c_int = 1;
pub const PM4125_ANA_MBIAS_EN_DISABLE: c_int = 0;

pub const PM4125_DIG_SWR_RX_CLK_ENABLE: c_int = 1;
pub const PM4125_DIG_SWR_RX_CLK_DISABLE: c_int = 0;

pub const PM4125_DIG_SWR_DSM_DITHER_DISABLE: c_int = 0;
pub const PM4125_DIG_SWR_DSM_DITHER_ENABLE: c_int = 1;

pub const PM4125_DIG_SWR_COMP_ENABLE: c_int = 1;
pub const PM4125_DIG_SWR_COMP_DISABLE: c_int = 0;

pub const PM4125_DIG_SWR_RX_INPUT_DISABLE: c_int = 0;
pub const PM4125_DIG_SWR_RX_INPUT_ENABLE: c_int = 1;

pub const PM4125_DIG_SWR_AMIC_SELECT_DMIC1: c_int = 0;
pub const PM4125_DIG_SWR_AMIC_SELECT_AMIC3: c_int = 1;

pub const PM4125_DIG_SWR_DMIC1_CLK_ENABLE: c_int = 1;
pub const PM4125_DIG_SWR_DMIC1_CLK_DISABLE: c_int = 0;

pub const PM4125_MAX_MICBIAS: c_int = 3;
pub const PM4125_MAX_SWR_CH_IDS: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm4125_tx_sdw_ports {
    PM4125_ADC_1_2_DMIC1L_BCS_PORT = 1,
    PM4125_DMIC_1L_1R_ADC1_BCS_PORT,
    PM4125_MAX_TX_SWR_PORTS = PM4125_DMIC_1L_1R_ADC1_BCS_PORT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm4125_rx_sdw_ports {
    PM4125_HPH_PORT = 1,
    PM4125_COMP_PORT,
    PM4125_MAX_SWR_PORTS = PM4125_COMP_PORT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm4125_sdw_priv {
    pub sdev: *mut sdw_slave,
    pub sconfig: sdw_stream_config,
    pub sruntime: *mut sdw_stream_runtime,
    pub port_config: [sdw_port_config; PM4125_MAX_SWR_PORTS],
    pub ch_info: *mut wcd_sdw_ch_info,
    pub port_enable: [bool; PM4125_MAX_SWR_CH_IDS],
    pub master_channel_map: [c_uint; SDW_MAX_PORTS],
    pub active_ports: c_int,
    pub num_ports: c_int,
    pub is_tx: bool,
    pub pm4125: *mut pm4125_priv,
    pub slave_irq: *mut irq_domain,
    pub regmap: *mut regmap,
}

// INTR_CTRL_INT_MASK_0
// INTR_CTRL_INT_MASK_1
// INTR_CTRL_INT_MASK_2
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm4125_tx_sdw_channels {
    PM4125_ADC1,
    PM4125_ADC2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm4125_rx_sdw_channels {
    PM4125_HPH_L,
    PM4125_HPH_R,
    PM4125_COMP_L,
    PM4125_COMP_R,
}
