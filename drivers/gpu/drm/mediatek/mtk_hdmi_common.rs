//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mediatek/mtk_hdmi_common.h
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
//
// Copyright (c) 2021 MediaTek Inc.
// Copyright (c) 2024 Collabora Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_aud_input_type {
    HDMI_AUD_INPUT_I2S = 0,
    HDMI_AUD_INPUT_SPDIF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_aud_i2s_fmt {
    HDMI_I2S_MODE_RJT_24BIT = 0,
    HDMI_I2S_MODE_RJT_16BIT,
    HDMI_I2S_MODE_LJT_24BIT,
    HDMI_I2S_MODE_LJT_16BIT,
    HDMI_I2S_MODE_I2S_24BIT,
    HDMI_I2S_MODE_I2S_16BIT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_aud_mclk {
    HDMI_AUD_MCLK_128FS,
    HDMI_AUD_MCLK_192FS,
    HDMI_AUD_MCLK_256FS,
    HDMI_AUD_MCLK_384FS,
    HDMI_AUD_MCLK_512FS,
    HDMI_AUD_MCLK_768FS,
    HDMI_AUD_MCLK_1152FS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_aud_channel_type {
    HDMI_AUD_CHAN_TYPE_1_0 = 0,
    HDMI_AUD_CHAN_TYPE_1_1,
    HDMI_AUD_CHAN_TYPE_2_0,
    HDMI_AUD_CHAN_TYPE_2_1,
    HDMI_AUD_CHAN_TYPE_3_0,
    HDMI_AUD_CHAN_TYPE_3_1,
    HDMI_AUD_CHAN_TYPE_4_0,
    HDMI_AUD_CHAN_TYPE_4_1,
    HDMI_AUD_CHAN_TYPE_5_0,
    HDMI_AUD_CHAN_TYPE_5_1,
    HDMI_AUD_CHAN_TYPE_6_0,
    HDMI_AUD_CHAN_TYPE_6_1,
    HDMI_AUD_CHAN_TYPE_7_0,
    HDMI_AUD_CHAN_TYPE_7_1,
    HDMI_AUD_CHAN_TYPE_3_0_LRS,
    HDMI_AUD_CHAN_TYPE_3_1_LRS,
    HDMI_AUD_CHAN_TYPE_4_0_CLRS,
    HDMI_AUD_CHAN_TYPE_4_1_CLRS,
    HDMI_AUD_CHAN_TYPE_6_1_CS,
    HDMI_AUD_CHAN_TYPE_6_1_CH,
    HDMI_AUD_CHAN_TYPE_6_1_OH,
    HDMI_AUD_CHAN_TYPE_6_1_CHR,
    HDMI_AUD_CHAN_TYPE_7_1_LH_RH,
    HDMI_AUD_CHAN_TYPE_7_1_LSR_RSR,
    HDMI_AUD_CHAN_TYPE_7_1_LC_RC,
    HDMI_AUD_CHAN_TYPE_7_1_LW_RW,
    HDMI_AUD_CHAN_TYPE_7_1_LSD_RSD,
    HDMI_AUD_CHAN_TYPE_7_1_LSS_RSS,
    HDMI_AUD_CHAN_TYPE_7_1_LHS_RHS,
    HDMI_AUD_CHAN_TYPE_7_1_CS_CH,
    HDMI_AUD_CHAN_TYPE_7_1_CS_OH,
    HDMI_AUD_CHAN_TYPE_7_1_CS_CHR,
    HDMI_AUD_CHAN_TYPE_7_1_CH_OH,
    HDMI_AUD_CHAN_TYPE_7_1_CH_CHR,
    HDMI_AUD_CHAN_TYPE_7_1_OH_CHR,
    HDMI_AUD_CHAN_TYPE_7_1_LSS_RSS_LSR_RSR,
    HDMI_AUD_CHAN_TYPE_6_0_CS,
    HDMI_AUD_CHAN_TYPE_6_0_CH,
    HDMI_AUD_CHAN_TYPE_6_0_OH,
    HDMI_AUD_CHAN_TYPE_6_0_CHR,
    HDMI_AUD_CHAN_TYPE_7_0_LH_RH,
    HDMI_AUD_CHAN_TYPE_7_0_LSR_RSR,
    HDMI_AUD_CHAN_TYPE_7_0_LC_RC,
    HDMI_AUD_CHAN_TYPE_7_0_LW_RW,
    HDMI_AUD_CHAN_TYPE_7_0_LSD_RSD,
    HDMI_AUD_CHAN_TYPE_7_0_LSS_RSS,
    HDMI_AUD_CHAN_TYPE_7_0_LHS_RHS,
    HDMI_AUD_CHAN_TYPE_7_0_CS_CH,
    HDMI_AUD_CHAN_TYPE_7_0_CS_OH,
    HDMI_AUD_CHAN_TYPE_7_0_CS_CHR,
    HDMI_AUD_CHAN_TYPE_7_0_CH_OH,
    HDMI_AUD_CHAN_TYPE_7_0_CH_CHR,
    HDMI_AUD_CHAN_TYPE_7_0_OH_CHR,
    HDMI_AUD_CHAN_TYPE_7_0_LSS_RSS_LSR_RSR,
    HDMI_AUD_CHAN_TYPE_8_0_LH_RH_CS,
    HDMI_AUD_CHAN_TYPE_UNKNOWN = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_aud_channel_swap_type {
    HDMI_AUD_SWAP_LR,
    HDMI_AUD_SWAP_LFE_CC,
    HDMI_AUD_SWAP_LSRS,
    HDMI_AUD_SWAP_RLS_RRS,
    HDMI_AUD_SWAP_LR_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_audio_param {
    pub aud_codec: hdmi_audio_coding_type,
    pub aud_sample_size: hdmi_audio_sample_size,
    pub aud_input_type: hdmi_aud_input_type,
    pub aud_i2s_fmt: hdmi_aud_i2s_fmt,
    pub aud_mclk: hdmi_aud_mclk,
    pub aud_input_chan_type: hdmi_aud_channel_type,
    pub codec_params: hdmi_codec_params,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_hpd_state {
    HDMI_PLUG_OUT = 0,
    HDMI_PLUG_IN_AND_SINK_POWER_ON,
    HDMI_PLUG_IN_ONLY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_hdmi_ver_conf {
    pub bridge_funcs: *const drm_bridge_funcs,
    pub codec_ops: *const hdmi_codec_ops,
    pub mtk_hdmi_clock_names: *const *const c_char,
    pub num_clocks: c_int,
    pub interlace_allowed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_hdmi_conf {
    pub ver_conf: *const mtk_hdmi_ver_conf,
    pub tz_disabled: bool,
    pub cea_modes_only: bool,
    pub max_mode_clock: c_ulong,
    pub reg_hdmi_tx_cfg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_hdmi {
    pub bridge: drm_bridge,
    pub /: *mut *mut *mut drm_connector curr_conn;/ current connector (only valid when 'enabled'),
    pub dev: *mut device,
    pub conf: *const mtk_hdmi_conf,
    pub phy: *mut phy,
    pub cec_dev: *mut device,
    pub ddc_adpt: *mut i2c_adapter,
    pub clk: *mut clk,
    pub mode: drm_display_mode,
    pub dvi_mode: bool,
    pub sys_regmap: *mut regmap,
    pub sys_offset: c_uint,
    pub regs: *mut regmap,
    pub audio_pdev: *mut platform_device,
    pub aud_param: hdmi_audio_param,
    pub audio_enable: bool,
    pub powered: bool,
    pub enabled: bool,
    pub irq: c_int,
    pub hpd: hdmi_hpd_state,
    pub plugged_cb: hdmi_codec_plugged_cb,
    pub codec_dev: *mut device,
    pub update_plugged_status_lock: mutex,
}

extern "C" {
    pub fn container_of(_arg: b, mtk_hdmi: struct, _arg: bridge) -> return;
}
extern "C" {
    pub fn mtk_hdmi_audio_get_eld(dev: *mut device, data: *mut c_void, buf: *mut u8, len: usize) -> c_int;
}
