//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wcd938x.h
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

pub const WCD938X_REGULATOR_MODE_CLASS_AB: c_int = 1;

pub const WCD938X_MBHC_MECH_DETECT_TYPE_INS: c_int = 1;

pub const WCD938X_MBHC_HPHL_PLUG_TYPE_NO: c_int = 1;

pub const WCD938X_MBHC_GND_PLUG_TYPE_NO: c_int = 1;

pub const WCD938X_ANA_MBHC_BD_ISRC_OFF: c_int = 0;

pub const WCD938X_MICB_DISABLE: c_int = 0;
pub const WCD938X_MICB_ENABLE: c_int = 1;
pub const WCD938X_MICB_PULL_UP: c_int = 2;
pub const WCD938X_MICB_PULL_DOWN: c_int = 3;

pub const WCD938X_GAIN_SRC_SEL_REGISTER: c_int = 1;

pub const WCD938X_MBHC_BTN_DBNC_T_16_MS: c_uint = 0x2;

pub const WCD938X_MBHC_HS_VREF_1P5_V: c_uint = 0x1;

pub const WCD938X_MBHC_DBNC_TIMER_INSREM_DBNC_T_96_MS: c_uint = 0x6;

pub const WCD938X_AMIC1_IN_SEL_DMIC: c_int = 0;
pub const WCD938X_AMIC1_IN_SEL_AMIC: c_int = 0;

pub const WCD938X_DMIC4_RATE_2P4MHZ: c_int = 3;

pub const WCD938X_MAX_SWR_CH_IDS: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd938x_tx_sdw_ports {
    WCD938X_ADC_1_2_PORT = 1,
    WCD938X_ADC_3_4_PORT,
// DMIC0_0, DMIC0_1, DMIC1_0, DMIC1_1
    WCD938X_DMIC_0_3_MBHC_PORT,
    WCD938X_DMIC_4_7_PORT,
    WCD938X_MAX_TX_SWR_PORTS = WCD938X_DMIC_4_7_PORT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd938x_tx_sdw_channels {
    WCD938X_ADC1,
    WCD938X_ADC2,
    WCD938X_ADC3,
    WCD938X_ADC4,
    WCD938X_DMIC0,
    WCD938X_DMIC1,
    WCD938X_MBHC,
    WCD938X_DMIC2,
    WCD938X_DMIC3,
    WCD938X_DMIC4,
    WCD938X_DMIC5,
    WCD938X_DMIC6,
    WCD938X_DMIC7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd938x_rx_sdw_ports {
    WCD938X_HPH_PORT = 1,
    WCD938X_CLSH_PORT,
    WCD938X_COMP_PORT,
    WCD938X_LO_PORT,
    WCD938X_DSD_PORT,
    WCD938X_MAX_SWR_PORTS = WCD938X_DSD_PORT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd938x_rx_sdw_channels {
    WCD938X_HPH_L,
    WCD938X_HPH_R,
    WCD938X_CLSH,
    WCD938X_COMP_L,
    WCD938X_COMP_R,
    WCD938X_LO,
    WCD938X_DSD_R,
    WCD938X_DSD_L,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcd938x_sdw_priv {
    pub sdev: *mut sdw_slave,
    pub sconfig: sdw_stream_config,
    pub sruntime: *mut sdw_stream_runtime,
    pub port_config: [sdw_port_config; WCD938X_MAX_SWR_PORTS],
    pub ch_info: *const wcd_sdw_ch_info,
    pub port_enable: [bool; WCD938X_MAX_SWR_CH_IDS],
    pub active_ports: c_int,
    pub is_tx: bool,
    pub wcd938x: *mut wcd938x_priv,
    pub slave_irq: *mut irq_domain,
    pub regmap: *mut regmap,
}

