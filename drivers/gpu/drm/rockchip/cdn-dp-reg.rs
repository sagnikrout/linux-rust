//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/rockchip/cdn-dp-reg.h
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
// Copyright (C) Rockchip Electronics Co., Ltd.
// Author: Chris Zhong <zyw@rock-chips.com>
//

pub const ADDR_IMEM: c_uint = 0x10000;
pub const ADDR_DMEM: c_uint = 0x20000;
// APB CFG addr
pub const APB_CTRL: c_int = 0;
pub const XT_INT_CTRL: c_uint = 0x04;
pub const MAILBOX_FULL_ADDR: c_uint = 0x08;
pub const MAILBOX_EMPTY_ADDR: c_uint = 0x0c;
pub const MAILBOX0_WR_DATA: c_uint = 0x10;
pub const MAILBOX0_RD_DATA: c_uint = 0x14;
pub const KEEP_ALIVE: c_uint = 0x18;
pub const VER_L: c_uint = 0x1c;
pub const VER_H: c_uint = 0x20;
pub const VER_LIB_L_ADDR: c_uint = 0x24;
pub const VER_LIB_H_ADDR: c_uint = 0x28;
pub const SW_DEBUG_L: c_uint = 0x2c;
pub const SW_DEBUG_H: c_uint = 0x30;
pub const MAILBOX_INT_MASK: c_uint = 0x34;
pub const MAILBOX_INT_STATUS: c_uint = 0x38;
pub const SW_CLK_L: c_uint = 0x3c;
pub const SW_CLK_H: c_uint = 0x40;
pub const SW_EVENTS0: c_uint = 0x44;
pub const SW_EVENTS1: c_uint = 0x48;
pub const SW_EVENTS2: c_uint = 0x4c;
pub const SW_EVENTS3: c_uint = 0x50;
pub const XT_OCD_CTRL: c_uint = 0x60;
pub const APB_INT_MASK: c_uint = 0x6c;
pub const APB_STATUS_MASK: c_uint = 0x70;
// audio decoder addr
pub const AUDIO_SRC_CNTL: c_uint = 0x30000;
pub const AUDIO_SRC_CNFG: c_uint = 0x30004;
pub const COM_CH_STTS_BITS: c_uint = 0x30008;

pub const SPDIF_CTRL_ADDR: c_uint = 0x3004c;
pub const SPDIF_CH1_CS_3100_ADDR: c_uint = 0x30050;
pub const SPDIF_CH1_CS_6332_ADDR: c_uint = 0x30054;
pub const SPDIF_CH1_CS_9564_ADDR: c_uint = 0x30058;
pub const SPDIF_CH1_CS_12796_ADDR: c_uint = 0x3005c;
pub const SPDIF_CH1_CS_159128_ADDR: c_uint = 0x30060;
pub const SPDIF_CH1_CS_191160_ADDR: c_uint = 0x30064;
pub const SPDIF_CH2_CS_3100_ADDR: c_uint = 0x30068;
pub const SPDIF_CH2_CS_6332_ADDR: c_uint = 0x3006c;
pub const SPDIF_CH2_CS_9564_ADDR: c_uint = 0x30070;
pub const SPDIF_CH2_CS_12796_ADDR: c_uint = 0x30074;
pub const SPDIF_CH2_CS_159128_ADDR: c_uint = 0x30078;
pub const SPDIF_CH2_CS_191160_ADDR: c_uint = 0x3007c;
pub const SMPL2PKT_CNTL: c_uint = 0x30080;
pub const SMPL2PKT_CNFG: c_uint = 0x30084;
pub const FIFO_CNTL: c_uint = 0x30088;
pub const FIFO_STTS: c_uint = 0x3008c;
// source pif addr
pub const SOURCE_PIF_WR_ADDR: c_uint = 0x30800;
pub const SOURCE_PIF_WR_REQ: c_uint = 0x30804;
pub const SOURCE_PIF_RD_ADDR: c_uint = 0x30808;
pub const SOURCE_PIF_RD_REQ: c_uint = 0x3080c;
pub const SOURCE_PIF_DATA_WR: c_uint = 0x30810;
pub const SOURCE_PIF_DATA_RD: c_uint = 0x30814;
pub const SOURCE_PIF_FIFO1_FLUSH: c_uint = 0x30818;
pub const SOURCE_PIF_FIFO2_FLUSH: c_uint = 0x3081c;
pub const SOURCE_PIF_STATUS: c_uint = 0x30820;
pub const SOURCE_PIF_INTERRUPT_SOURCE: c_uint = 0x30824;
pub const SOURCE_PIF_INTERRUPT_MASK: c_uint = 0x30828;
pub const SOURCE_PIF_PKT_ALLOC_REG: c_uint = 0x3082c;
pub const SOURCE_PIF_PKT_ALLOC_WR_EN: c_uint = 0x30830;
pub const SOURCE_PIF_SW_RESET: c_uint = 0x30834;
// below registers need access by mailbox
// source car addr
pub const SOURCE_HDTX_CAR: c_uint = 0x0900;
pub const SOURCE_DPTX_CAR: c_uint = 0x0904;
pub const SOURCE_PHY_CAR: c_uint = 0x0908;
pub const SOURCE_CEC_CAR: c_uint = 0x090c;
pub const SOURCE_CBUS_CAR: c_uint = 0x0910;
pub const SOURCE_PKT_CAR: c_uint = 0x0918;
pub const SOURCE_AIF_CAR: c_uint = 0x091c;
pub const SOURCE_CIPHER_CAR: c_uint = 0x0920;
pub const SOURCE_CRYPTO_CAR: c_uint = 0x0924;
// clock meters addr
pub const CM_CTRL: c_uint = 0x0a00;
pub const CM_I2S_CTRL: c_uint = 0x0a04;
pub const CM_SPDIF_CTRL: c_uint = 0x0a08;
pub const CM_VID_CTRL: c_uint = 0x0a0c;
pub const CM_LANE_CTRL: c_uint = 0x0a10;
pub const I2S_NM_STABLE: c_uint = 0x0a14;
pub const I2S_NCTS_STABLE: c_uint = 0x0a18;
pub const SPDIF_NM_STABLE: c_uint = 0x0a1c;
pub const SPDIF_NCTS_STABLE: c_uint = 0x0a20;
pub const NMVID_MEAS_STABLE: c_uint = 0x0a24;
pub const I2S_MEAS: c_uint = 0x0a40;
pub const SPDIF_MEAS: c_uint = 0x0a80;
pub const NMVID_MEAS: c_uint = 0x0ac0;
// source vif addr
pub const BND_HSYNC2VSYNC: c_uint = 0x0b00;
pub const HSYNC2VSYNC_F1_L1: c_uint = 0x0b04;
pub const HSYNC2VSYNC_F2_L1: c_uint = 0x0b08;
pub const HSYNC2VSYNC_STATUS: c_uint = 0x0b0c;
pub const HSYNC2VSYNC_POL_CTRL: c_uint = 0x0b10;
// dptx phy addr
pub const DP_TX_PHY_CONFIG_REG: c_uint = 0x2000;
pub const DP_TX_PHY_SW_RESET: c_uint = 0x2004;
pub const DP_TX_PHY_SCRAMBLER_SEED: c_uint = 0x2008;
pub const DP_TX_PHY_TRAINING_01_04: c_uint = 0x200c;
pub const DP_TX_PHY_TRAINING_05_08: c_uint = 0x2010;
pub const DP_TX_PHY_TRAINING_09_10: c_uint = 0x2014;
pub const TEST_COR: c_uint = 0x23fc;
// dptx hpd addr
pub const HPD_IRQ_DET_MIN_TIMER: c_uint = 0x2100;
pub const HPD_IRQ_DET_MAX_TIMER: c_uint = 0x2104;
pub const HPD_UNPLGED_DET_MIN_TIMER: c_uint = 0x2108;
pub const HPD_STABLE_TIMER: c_uint = 0x210c;
pub const HPD_FILTER_TIMER: c_uint = 0x2110;
pub const HPD_EVENT_MASK: c_uint = 0x211c;
pub const HPD_EVENT_DET: c_uint = 0x2120;
// dpyx framer addr
pub const DP_FRAMER_GLOBAL_CONFIG: c_uint = 0x2200;
pub const DP_SW_RESET: c_uint = 0x2204;
pub const DP_FRAMER_TU: c_uint = 0x2208;
pub const DP_FRAMER_PXL_REPR: c_uint = 0x220c;
pub const DP_FRAMER_SP: c_uint = 0x2210;
pub const AUDIO_PACK_CONTROL: c_uint = 0x2214;

pub const DP_VB_ID: c_uint = 0x2258;
pub const DP_MTPH_LVP_CONTROL: c_uint = 0x225c;
pub const DP_MTPH_SYMBOL_VALUES: c_uint = 0x2260;
pub const DP_MTPH_ECF_CONTROL: c_uint = 0x2264;
pub const DP_MTPH_ACT_CONTROL: c_uint = 0x2268;
pub const DP_MTPH_STATUS: c_uint = 0x226c;
pub const DP_INTERRUPT_SOURCE: c_uint = 0x2270;
pub const DP_INTERRUPT_MASK: c_uint = 0x2274;
pub const DP_FRONT_BACK_PORCH: c_uint = 0x2278;
pub const DP_BYTE_COUNT: c_uint = 0x227c;
// dptx stream addr
pub const MSA_HORIZONTAL_0: c_uint = 0x2280;
pub const MSA_HORIZONTAL_1: c_uint = 0x2284;
pub const MSA_VERTICAL_0: c_uint = 0x2288;
pub const MSA_VERTICAL_1: c_uint = 0x228c;
pub const MSA_MISC: c_uint = 0x2290;
pub const STREAM_CONFIG: c_uint = 0x2294;
pub const AUDIO_PACK_STATUS: c_uint = 0x2298;
pub const VIF_STATUS: c_uint = 0x229c;
pub const PCK_STUFF_STATUS_0: c_uint = 0x22a0;
pub const PCK_STUFF_STATUS_1: c_uint = 0x22a4;
pub const INFO_PACK_STATUS: c_uint = 0x22a8;
pub const RATE_GOVERNOR_STATUS: c_uint = 0x22ac;
pub const DP_HORIZONTAL: c_uint = 0x22b0;
pub const DP_VERTICAL_0: c_uint = 0x22b4;
pub const DP_VERTICAL_1: c_uint = 0x22b8;
pub const DP_BLOCK_SDP: c_uint = 0x22bc;
// dptx glbl addr
pub const DPTX_LANE_EN: c_uint = 0x2300;
pub const DPTX_ENHNCD: c_uint = 0x2304;
pub const DPTX_INT_MASK: c_uint = 0x2308;
pub const DPTX_INT_STATUS: c_uint = 0x230c;
// dp aux addr
pub const DP_AUX_HOST_CONTROL: c_uint = 0x2800;
pub const DP_AUX_INTERRUPT_SOURCE: c_uint = 0x2804;
pub const DP_AUX_INTERRUPT_MASK: c_uint = 0x2808;
pub const DP_AUX_SWAP_INVERSION_CONTROL: c_uint = 0x280c;
pub const DP_AUX_SEND_NACK_TRANSACTION: c_uint = 0x2810;
pub const DP_AUX_CLEAR_RX: c_uint = 0x2814;
pub const DP_AUX_CLEAR_TX: c_uint = 0x2818;
pub const DP_AUX_TIMER_STOP: c_uint = 0x281c;
pub const DP_AUX_TIMER_CLEAR: c_uint = 0x2820;
pub const DP_AUX_RESET_SW: c_uint = 0x2824;
pub const DP_AUX_DIVIDE_2M: c_uint = 0x2828;
pub const DP_AUX_TX_PREACHARGE_LENGTH: c_uint = 0x282c;
pub const DP_AUX_FREQUENCY_1M_MAX: c_uint = 0x2830;
pub const DP_AUX_FREQUENCY_1M_MIN: c_uint = 0x2834;
pub const DP_AUX_RX_PRE_MIN: c_uint = 0x2838;
pub const DP_AUX_RX_PRE_MAX: c_uint = 0x283c;
pub const DP_AUX_TIMER_PRESET: c_uint = 0x2840;
pub const DP_AUX_NACK_FORMAT: c_uint = 0x2844;
pub const DP_AUX_TX_DATA: c_uint = 0x2848;
pub const DP_AUX_RX_DATA: c_uint = 0x284c;
pub const DP_AUX_TX_STATUS: c_uint = 0x2850;
pub const DP_AUX_RX_STATUS: c_uint = 0x2854;
pub const DP_AUX_RX_CYCLE_COUNTER: c_uint = 0x2858;
pub const DP_AUX_MAIN_STATES: c_uint = 0x285c;
pub const DP_AUX_MAIN_TIMER: c_uint = 0x2860;
pub const DP_AUX_AFE_OUT: c_uint = 0x2864;
// crypto addr
pub const CRYPTO_HDCP_REVISION: c_uint = 0x5800;
pub const HDCP_CRYPTO_CONFIG: c_uint = 0x5804;
pub const CRYPTO_INTERRUPT_SOURCE: c_uint = 0x5808;
pub const CRYPTO_INTERRUPT_MASK: c_uint = 0x580c;
pub const CRYPTO22_CONFIG: c_uint = 0x5818;
pub const CRYPTO22_STATUS: c_uint = 0x581c;
pub const SHA_256_DATA_IN: c_uint = 0x583c;

pub const AES_32_DATA_IN: c_uint = 0x5880;

pub const CRYPTO14_CONFIG: c_uint = 0x58a0;
pub const CRYPTO14_STATUS: c_uint = 0x58a4;
pub const CRYPTO14_PRNM_OUT: c_uint = 0x58a8;
pub const CRYPTO14_KM_0: c_uint = 0x58ac;
pub const CRYPTO14_KM_1: c_uint = 0x58b0;
pub const CRYPTO14_AN_0: c_uint = 0x58b4;
pub const CRYPTO14_AN_1: c_uint = 0x58b8;
pub const CRYPTO14_YOUR_KSV_0: c_uint = 0x58bc;
pub const CRYPTO14_YOUR_KSV_1: c_uint = 0x58c0;
pub const CRYPTO14_MI_0: c_uint = 0x58c4;
pub const CRYPTO14_MI_1: c_uint = 0x58c8;
pub const CRYPTO14_TI_0: c_uint = 0x58cc;
pub const CRYPTO14_KI_0: c_uint = 0x58d0;
pub const CRYPTO14_KI_1: c_uint = 0x58d4;
pub const CRYPTO14_BLOCKS_NUM: c_uint = 0x58d8;
pub const CRYPTO14_KEY_MEM_DATA_0: c_uint = 0x58dc;
pub const CRYPTO14_KEY_MEM_DATA_1: c_uint = 0x58e0;
pub const CRYPTO14_SHA1_MSG_DATA: c_uint = 0x58e4;

pub const TRNG_CTRL: c_uint = 0x58fc;
pub const TRNG_DATA_RDY: c_uint = 0x5900;
pub const TRNG_DATA: c_uint = 0x5904;
// cipher addr
pub const HDCP_REVISION: c_uint = 0x60000;
pub const INTERRUPT_SOURCE: c_uint = 0x60004;
pub const INTERRUPT_MASK: c_uint = 0x60008;
pub const HDCP_CIPHER_CONFIG: c_uint = 0x6000c;
pub const AES_128_KEY_0: c_uint = 0x60010;
pub const AES_128_KEY_1: c_uint = 0x60014;
pub const AES_128_KEY_2: c_uint = 0x60018;
pub const AES_128_KEY_3: c_uint = 0x6001c;
pub const AES_128_RANDOM_0: c_uint = 0x60020;
pub const AES_128_RANDOM_1: c_uint = 0x60024;
pub const CIPHER14_KM_0: c_uint = 0x60028;
pub const CIPHER14_KM_1: c_uint = 0x6002c;
pub const CIPHER14_STATUS: c_uint = 0x60030;
pub const CIPHER14_RI_PJ_STATUS: c_uint = 0x60034;
pub const CIPHER_MODE: c_uint = 0x60038;
pub const CIPHER14_AN_0: c_uint = 0x6003c;
pub const CIPHER14_AN_1: c_uint = 0x60040;
pub const CIPHER22_AUTH: c_uint = 0x60044;
pub const CIPHER14_R0_DP_STATUS: c_uint = 0x60048;
pub const CIPHER14_BOOTSTRAP: c_uint = 0x6004c;

pub const ALL_INT_MASK: c_int = 3;
// mailbox
pub const MB_OPCODE_ID: c_int = 0;
pub const MB_MODULE_ID: c_int = 1;
pub const MB_SIZE_MSB_ID: c_int = 2;
pub const MB_SIZE_LSB_ID: c_int = 3;
pub const MB_DATA_ID: c_int = 4;
pub const MB_MODULE_ID_DP_TX: c_uint = 0x01;
pub const MB_MODULE_ID_HDCP_TX: c_uint = 0x07;
pub const MB_MODULE_ID_HDCP_RX: c_uint = 0x08;
pub const MB_MODULE_ID_HDCP_GENERAL: c_uint = 0x09;
pub const MB_MODULE_ID_GENERAL: c_uint = 0x0a;
// general opcode
pub const GENERAL_MAIN_CONTROL: c_uint = 0x01;
pub const GENERAL_TEST_ECHO: c_uint = 0x02;
pub const GENERAL_BUS_SETTINGS: c_uint = 0x03;
pub const GENERAL_TEST_ACCESS: c_uint = 0x04;
pub const DPTX_SET_POWER_MNG: c_uint = 0x00;
pub const DPTX_SET_HOST_CAPABILITIES: c_uint = 0x01;
pub const DPTX_GET_EDID: c_uint = 0x02;
pub const DPTX_READ_DPCD: c_uint = 0x03;
pub const DPTX_WRITE_DPCD: c_uint = 0x04;
pub const DPTX_ENABLE_EVENT: c_uint = 0x05;
pub const DPTX_WRITE_REGISTER: c_uint = 0x06;
pub const DPTX_READ_REGISTER: c_uint = 0x07;
pub const DPTX_WRITE_FIELD: c_uint = 0x08;
pub const DPTX_TRAINING_CONTROL: c_uint = 0x09;
pub const DPTX_READ_EVENT: c_uint = 0x0a;
pub const DPTX_READ_LINK_STAT: c_uint = 0x0b;
pub const DPTX_SET_VIDEO: c_uint = 0x0c;
pub const DPTX_SET_AUDIO: c_uint = 0x0d;
pub const DPTX_GET_LAST_AUX_STAUS: c_uint = 0x0e;
pub const DPTX_SET_LINK_BREAK_POINT: c_uint = 0x0f;
pub const DPTX_FORCE_LANES: c_uint = 0x10;
pub const DPTX_HPD_STATE: c_uint = 0x11;
pub const FW_STANDBY: c_int = 0;
pub const FW_ACTIVE: c_int = 1;

pub const LINK_TRAINING_NOT_ACTIVE: c_int = 0;
pub const LINK_TRAINING_RUN: c_int = 1;
pub const LINK_TRAINING_RESTART: c_int = 2;
pub const CONTROL_VIDEO_IDLE: c_int = 0;
pub const CONTROL_VIDEO_VALID: c_int = 1;

pub const INTERLACE_DTCT_WIN: c_uint = 0x20;

// capability
pub const AUX_HOST_INVERT: c_int = 3;
pub const FAST_LT_SUPPORT: c_int = 1;
pub const FAST_LT_NOT_SUPPORT: c_int = 0;
pub const LANE_MAPPING_NORMAL: c_uint = 0x1b;
pub const LANE_MAPPING_FLIPPED: c_uint = 0xe4;
pub const ENHANCED: c_int = 1;

pub const TU_SIZE: c_int = 30;

// audio

pub const TRANS_SMPL_WIDTH_16: c_int = 0;

// Reference cycles when using lane clock as reference
pub const LANE_REF_CYC: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum voltage_swing_level {
    VOLTAGE_LEVEL_0,
    VOLTAGE_LEVEL_1,
    VOLTAGE_LEVEL_2,
    VOLTAGE_LEVEL_3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pre_emphasis_level {
    PRE_EMPHASIS_LEVEL_0,
    PRE_EMPHASIS_LEVEL_1,
    PRE_EMPHASIS_LEVEL_2,
    PRE_EMPHASIS_LEVEL_3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pattern_set {
    PTS1		= BIT(0),
    PTS2		= BIT(1),
    PTS3		= BIT(2),
    PTS4		= BIT(3),
    DP_NONE		= BIT(4)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vic_color_depth {
    BCS_6 = 0x1,
    BCS_8 = 0x2,
    BCS_10 = 0x4,
    BCS_12 = 0x8,
    BCS_16 = 0x10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vic_bt_type {
    BT_601 = 0x0,
    BT_709 = 0x1,
}

extern "C" {
    pub fn cdn_dp_clock_reset(dp: *mut cdn_dp_device);
}
extern "C" {
    pub fn cdn_dp_set_fw_clk(dp: *mut cdn_dp_device, clk: c_ulong);
}
extern "C" {
    pub fn cdn_dp_set_firmware_active(dp: *mut cdn_dp_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn cdn_dp_set_host_cap(dp: *mut cdn_dp_device, lanes: u8, flip: bool) -> c_int;
}
extern "C" {
    pub fn cdn_dp_event_config(dp: *mut cdn_dp_device) -> c_int;
}
extern "C" {
    pub fn cdn_dp_get_event(dp: *mut cdn_dp_device) -> u32;
}
extern "C" {
    pub fn cdn_dp_get_hpd_status(dp: *mut cdn_dp_device) -> c_int;
}
extern "C" {
    pub fn cdn_dp_dpcd_write(dp: *mut cdn_dp_device, addr: u32, value: u8) -> c_int;
}
extern "C" {
    pub fn cdn_dp_dpcd_read(dp: *mut cdn_dp_device, addr: u32, data: *mut u8, len: u16) -> c_int;
}
extern "C" {
    pub fn cdn_dp_train_link(dp: *mut cdn_dp_device) -> c_int;
}
extern "C" {
    pub fn cdn_dp_set_video_status(dp: *mut cdn_dp_device, active: c_int) -> c_int;
}
extern "C" {
    pub fn cdn_dp_config_video(dp: *mut cdn_dp_device) -> c_int;
}
extern "C" {
    pub fn cdn_dp_audio_stop(dp: *mut cdn_dp_device, audio: *mut audio_info) -> c_int;
}
extern "C" {
    pub fn cdn_dp_audio_mute(dp: *mut cdn_dp_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn cdn_dp_audio_config(dp: *mut cdn_dp_device, audio: *mut audio_info) -> c_int;
}
