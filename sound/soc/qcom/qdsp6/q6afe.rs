//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/qcom/qdsp6/q6afe.h
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

pub const MSM_AFE_PORT_TYPE_RX: c_int = 0;
pub const MSM_AFE_PORT_TYPE_TX: c_int = 1;

pub const Q6AFE_MAX_MI2S_LINES: c_int = 4;
pub const AFE_MAX_CHAN_COUNT: c_int = 8;
pub const AFE_PORT_MAX_AUDIO_CHAN_CNT: c_uint = 0x8;
pub const Q6AFE_LPASS_CLK_SRC_INTERNAL: c_int = 1;
pub const Q6AFE_LPASS_CLK_ROOT_DEFAULT: c_int = 0;
pub const LPAIF_DIG_CLK: c_int = 1;
pub const LPAIF_BIT_CLK: c_int = 2;
pub const LPAIF_OSR_CLK: c_int = 3;
// Clock ID for Primary I2S IBIT
pub const Q6AFE_LPASS_CLK_ID_PRI_MI2S_IBIT: c_uint = 0x100;
// Clock ID for Primary I2S EBIT
pub const Q6AFE_LPASS_CLK_ID_PRI_MI2S_EBIT: c_uint = 0x101;
// Clock ID for Secondary I2S IBIT
pub const Q6AFE_LPASS_CLK_ID_SEC_MI2S_IBIT: c_uint = 0x102;
// Clock ID for Secondary I2S EBIT
pub const Q6AFE_LPASS_CLK_ID_SEC_MI2S_EBIT: c_uint = 0x103;
// Clock ID for Tertiary I2S IBIT
pub const Q6AFE_LPASS_CLK_ID_TER_MI2S_IBIT: c_uint = 0x104;
// Clock ID for Tertiary I2S EBIT
pub const Q6AFE_LPASS_CLK_ID_TER_MI2S_EBIT: c_uint = 0x105;
// Clock ID for Quartnery I2S IBIT
pub const Q6AFE_LPASS_CLK_ID_QUAD_MI2S_IBIT: c_uint = 0x106;
// Clock ID for Quartnery I2S EBIT
pub const Q6AFE_LPASS_CLK_ID_QUAD_MI2S_EBIT: c_uint = 0x107;
// Clock ID for Speaker I2S IBIT
pub const Q6AFE_LPASS_CLK_ID_SPEAKER_I2S_IBIT: c_uint = 0x108;
// Clock ID for Speaker I2S EBIT
pub const Q6AFE_LPASS_CLK_ID_SPEAKER_I2S_EBIT: c_uint = 0x109;
// Clock ID for Speaker I2S OSR
pub const Q6AFE_LPASS_CLK_ID_SPEAKER_I2S_OSR: c_uint = 0x10A;
// Clock ID for QUINARY  I2S IBIT
pub const Q6AFE_LPASS_CLK_ID_QUI_MI2S_IBIT: c_uint = 0x10B;
// Clock ID for QUINARY  I2S EBIT
pub const Q6AFE_LPASS_CLK_ID_QUI_MI2S_EBIT: c_uint = 0x10C;
// Clock ID for SENARY  I2S IBIT
pub const Q6AFE_LPASS_CLK_ID_SEN_MI2S_IBIT: c_uint = 0x10D;
// Clock ID for SENARY  I2S EBIT
pub const Q6AFE_LPASS_CLK_ID_SEN_MI2S_EBIT: c_uint = 0x10E;
// Clock ID for INT0 I2S IBIT
pub const Q6AFE_LPASS_CLK_ID_INT0_MI2S_IBIT: c_uint = 0x10F;
// Clock ID for INT1 I2S IBIT
pub const Q6AFE_LPASS_CLK_ID_INT1_MI2S_IBIT: c_uint = 0x110;
// Clock ID for INT2 I2S IBIT
pub const Q6AFE_LPASS_CLK_ID_INT2_MI2S_IBIT: c_uint = 0x111;
// Clock ID for INT3 I2S IBIT
pub const Q6AFE_LPASS_CLK_ID_INT3_MI2S_IBIT: c_uint = 0x112;
// Clock ID for INT4 I2S IBIT
pub const Q6AFE_LPASS_CLK_ID_INT4_MI2S_IBIT: c_uint = 0x113;
// Clock ID for INT5 I2S IBIT
pub const Q6AFE_LPASS_CLK_ID_INT5_MI2S_IBIT: c_uint = 0x114;
// Clock ID for INT6 I2S IBIT
pub const Q6AFE_LPASS_CLK_ID_INT6_MI2S_IBIT: c_uint = 0x115;
// Clock ID for QUINARY MI2S OSR CLK
pub const Q6AFE_LPASS_CLK_ID_QUI_MI2S_OSR: c_uint = 0x116;
// Clock ID for Primary PCM IBIT
pub const Q6AFE_LPASS_CLK_ID_PRI_PCM_IBIT: c_uint = 0x200;
// Clock ID for Primary PCM EBIT
pub const Q6AFE_LPASS_CLK_ID_PRI_PCM_EBIT: c_uint = 0x201;
// Clock ID for Secondary PCM IBIT
pub const Q6AFE_LPASS_CLK_ID_SEC_PCM_IBIT: c_uint = 0x202;
// Clock ID for Secondary PCM EBIT
pub const Q6AFE_LPASS_CLK_ID_SEC_PCM_EBIT: c_uint = 0x203;
// Clock ID for Tertiary PCM IBIT
pub const Q6AFE_LPASS_CLK_ID_TER_PCM_IBIT: c_uint = 0x204;
// Clock ID for Tertiary PCM EBIT
pub const Q6AFE_LPASS_CLK_ID_TER_PCM_EBIT: c_uint = 0x205;
// Clock ID for Quartery PCM IBIT
pub const Q6AFE_LPASS_CLK_ID_QUAD_PCM_IBIT: c_uint = 0x206;
// Clock ID for Quartery PCM EBIT
pub const Q6AFE_LPASS_CLK_ID_QUAD_PCM_EBIT: c_uint = 0x207;
// Clock ID for Quinary PCM IBIT
pub const Q6AFE_LPASS_CLK_ID_QUIN_PCM_IBIT: c_uint = 0x208;
// Clock ID for Quinary PCM EBIT
pub const Q6AFE_LPASS_CLK_ID_QUIN_PCM_EBIT: c_uint = 0x209;
// Clock ID for QUINARY PCM OSR
pub const Q6AFE_LPASS_CLK_ID_QUI_PCM_OSR: c_uint = 0x20A;
// Clock ID for Primary TDM IBIT
pub const Q6AFE_LPASS_CLK_ID_PRI_TDM_IBIT: c_uint = 0x200;
// Clock ID for Primary TDM EBIT
pub const Q6AFE_LPASS_CLK_ID_PRI_TDM_EBIT: c_uint = 0x201;
// Clock ID for Secondary TDM IBIT
pub const Q6AFE_LPASS_CLK_ID_SEC_TDM_IBIT: c_uint = 0x202;
// Clock ID for Secondary TDM EBIT
pub const Q6AFE_LPASS_CLK_ID_SEC_TDM_EBIT: c_uint = 0x203;
// Clock ID for Tertiary TDM IBIT
pub const Q6AFE_LPASS_CLK_ID_TER_TDM_IBIT: c_uint = 0x204;
// Clock ID for Tertiary TDM EBIT
pub const Q6AFE_LPASS_CLK_ID_TER_TDM_EBIT: c_uint = 0x205;
// Clock ID for Quartery TDM IBIT
pub const Q6AFE_LPASS_CLK_ID_QUAD_TDM_IBIT: c_uint = 0x206;
// Clock ID for Quartery TDM EBIT
pub const Q6AFE_LPASS_CLK_ID_QUAD_TDM_EBIT: c_uint = 0x207;
// Clock ID for Quinary TDM IBIT
pub const Q6AFE_LPASS_CLK_ID_QUIN_TDM_IBIT: c_uint = 0x208;
// Clock ID for Quinary TDM EBIT
pub const Q6AFE_LPASS_CLK_ID_QUIN_TDM_EBIT: c_uint = 0x209;
// Clock ID for Quinary TDM OSR
pub const Q6AFE_LPASS_CLK_ID_QUIN_TDM_OSR: c_uint = 0x20A;
// Clock ID for MCLK1
pub const Q6AFE_LPASS_CLK_ID_MCLK_1: c_uint = 0x300;
// Clock ID for MCLK2
pub const Q6AFE_LPASS_CLK_ID_MCLK_2: c_uint = 0x301;
// Clock ID for MCLK3
pub const Q6AFE_LPASS_CLK_ID_MCLK_3: c_uint = 0x302;
// Clock ID for MCLK4
pub const Q6AFE_LPASS_CLK_ID_MCLK_4: c_uint = 0x304;
// Clock ID for Internal Digital Codec Core
pub const Q6AFE_LPASS_CLK_ID_INTERNAL_DIGITAL_CODEC_CORE: c_uint = 0x303;
// Clock ID for INT MCLK0
pub const Q6AFE_LPASS_CLK_ID_INT_MCLK_0: c_uint = 0x305;
// Clock ID for INT MCLK1
pub const Q6AFE_LPASS_CLK_ID_INT_MCLK_1: c_uint = 0x306;
pub const Q6AFE_LPASS_CLK_ID_WSA_CORE_MCLK: c_uint = 0x309;
pub const Q6AFE_LPASS_CLK_ID_WSA_CORE_NPL_MCLK: c_uint = 0x30a;
pub const Q6AFE_LPASS_CLK_ID_TX_CORE_MCLK: c_uint = 0x30c;
pub const Q6AFE_LPASS_CLK_ID_TX_CORE_NPL_MCLK: c_uint = 0x30d;
pub const Q6AFE_LPASS_CLK_ID_RX_CORE_MCLK: c_uint = 0x30e;
pub const Q6AFE_LPASS_CLK_ID_RX_CORE_NPL_MCLK: c_uint = 0x30f;
pub const Q6AFE_LPASS_CLK_ID_VA_CORE_MCLK: c_uint = 0x30b;
pub const Q6AFE_LPASS_CLK_ID_VA_CORE_2X_MCLK: c_uint = 0x310;
pub const Q6AFE_LPASS_CORE_AVTIMER_BLOCK: c_uint = 0x2;
pub const Q6AFE_LPASS_CORE_HW_MACRO_BLOCK: c_uint = 0x3;
pub const Q6AFE_LPASS_CORE_HW_DCODEC_BLOCK: c_uint = 0x4;
// Clock attribute for invalid use (reserved for internal usage)
pub const Q6AFE_LPASS_CLK_ATTRIBUTE_INVALID: c_uint = 0x0;
// Clock attribute for no couple case
pub const Q6AFE_LPASS_CLK_ATTRIBUTE_COUPLE_NO: c_uint = 0x1;
// Clock attribute for dividend couple case
pub const Q6AFE_LPASS_CLK_ATTRIBUTE_COUPLE_DIVIDEND: c_uint = 0x2;
// Clock attribute for divisor couple case
pub const Q6AFE_LPASS_CLK_ATTRIBUTE_COUPLE_DIVISOR: c_uint = 0x3;
// Clock attribute for invert and no couple case
pub const Q6AFE_LPASS_CLK_ATTRIBUTE_INVERT_COUPLE_NO: c_uint = 0x4;
pub const Q6AFE_CMAP_INVALID: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6afe_hdmi_cfg {
    pub datatype: u16,
    pub channel_allocation: u16,
    pub sample_rate: u32,
    pub bit_width: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6afe_slim_cfg {
    pub sample_rate: u32,
    pub bit_width: u16,
    pub data_format: u16,
    pub num_channels: u16,
    pub ch_mapping: [u8; AFE_MAX_CHAN_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6afe_i2s_cfg {
    pub sample_rate: u32,
    pub bit_width: u16,
    pub data_format: u16,
    pub num_channels: u16,
    pub sd_line_mask: u32,
    pub fmt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6afe_tdm_cfg {
    pub num_channels: u16,
    pub sample_rate: u32,
    pub bit_width: u16,
    pub data_format: u16,
    pub sync_mode: u16,
    pub sync_src: u16,
    pub nslots_per_frame: u16,
    pub slot_width: u16,
    pub slot_mask: u16,
    pub data_align_type: u32,
    pub ch_mapping: [u16; AFE_MAX_CHAN_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6afe_cdc_dma_cfg {
    pub sample_rate: u16,
    pub bit_width: u16,
    pub data_format: u16,
    pub num_channels: u16,
    pub active_channels_mask: u16,
}

//
// struct q6afe_usb_cfg
// @cfg_minor_version: Minor version used for tracking USB audio device
// configuration.
// Supported values:
// AFE_API_MINOR_VERSION_USB_AUDIO_CONFIG
// @sample_rate: Sampling rate of the port
// Supported values:
// AFE_PORT_SAMPLE_RATE_8K
// AFE_PORT_SAMPLE_RATE_11025
// AFE_PORT_SAMPLE_RATE_12K
// AFE_PORT_SAMPLE_RATE_16K
// AFE_PORT_SAMPLE_RATE_22050
// AFE_PORT_SAMPLE_RATE_24K
// AFE_PORT_SAMPLE_RATE_32K
// AFE_PORT_SAMPLE_RATE_44P1K
// AFE_PORT_SAMPLE_RATE_48K
// AFE_PORT_SAMPLE_RATE_96K
// AFE_PORT_SAMPLE_RATE_192K
// @bit_width: Bit width of the sample.
// Supported values: 16, 24
// @num_channels: Number of channels
// Supported values: 1, 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6afe_usb_cfg {
    pub cfg_minor_version: u32,
    pub sample_rate: u32,
    pub bit_width: u16,
    pub num_channels: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6afe_port_config {
    pub hdmi: q6afe_hdmi_cfg,
    pub slim: q6afe_slim_cfg,
    pub i2s_cfg: q6afe_i2s_cfg,
    pub tdm: q6afe_tdm_cfg,
    pub dma_cfg: q6afe_cdc_dma_cfg,
    pub usb_audio: q6afe_usb_cfg,
}

extern "C" {
    pub fn q6afe_port_start(port: *mut q6afe_port) -> c_int;
}
extern "C" {
    pub fn q6afe_port_stop(port: *mut q6afe_port) -> c_int;
}
extern "C" {
    pub fn q6afe_port_put(port: *mut q6afe_port);
}
extern "C" {
    pub fn q6afe_get_port_id(index: c_int) -> c_int;
}
extern "C" {
    pub fn q6afe_i2s_port_prepare(port: *mut q6afe_port, cfg: *mut q6afe_i2s_cfg) -> c_int;
}
extern "C" {
    pub fn q6afe_tdm_port_prepare(port: *mut q6afe_port, cfg: *mut q6afe_tdm_cfg);
}
extern "C" {
    pub fn afe_port_send_usb_dev_param(port: *mut q6afe_port, cardidx: c_int, pcmidx: c_int) -> c_int;
}
