//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/adv7511/adv7511.h
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
// Analog Devices ADV7511 HDMI transmitter driver
//
// Copyright 2012 Analog Devices Inc.
//

pub const ADV7511_REG_CHIP_REVISION: c_uint = 0x00;
pub const ADV7511_REG_N0: c_uint = 0x01;
pub const ADV7511_REG_N1: c_uint = 0x02;
pub const ADV7511_REG_N2: c_uint = 0x03;
pub const ADV7511_REG_SPDIF_FREQ: c_uint = 0x04;
pub const ADV7511_REG_CTS_AUTOMATIC1: c_uint = 0x05;
pub const ADV7511_REG_CTS_AUTOMATIC2: c_uint = 0x06;
pub const ADV7511_REG_CTS_MANUAL0: c_uint = 0x07;
pub const ADV7511_REG_CTS_MANUAL1: c_uint = 0x08;
pub const ADV7511_REG_CTS_MANUAL2: c_uint = 0x09;
pub const ADV7511_REG_AUDIO_SOURCE: c_uint = 0x0a;
pub const ADV7511_REG_AUDIO_CONFIG: c_uint = 0x0b;
pub const ADV7511_REG_I2S_CONFIG: c_uint = 0x0c;
pub const ADV7511_REG_I2S_WIDTH: c_uint = 0x0d;
pub const ADV7511_REG_AUDIO_SUB_SRC0: c_uint = 0x0e;
pub const ADV7511_REG_AUDIO_SUB_SRC1: c_uint = 0x0f;
pub const ADV7511_REG_AUDIO_SUB_SRC2: c_uint = 0x10;
pub const ADV7511_REG_AUDIO_SUB_SRC3: c_uint = 0x11;
pub const ADV7511_REG_AUDIO_CFG1: c_uint = 0x12;
pub const ADV7511_REG_AUDIO_CFG2: c_uint = 0x13;
pub const ADV7511_REG_AUDIO_CFG3: c_uint = 0x14;
pub const ADV7511_REG_I2C_FREQ_ID_CFG: c_uint = 0x15;
pub const ADV7511_REG_VIDEO_INPUT_CFG1: c_uint = 0x16;

pub const ADV7511_REG_PIXEL_REPETITION: c_uint = 0x3b;
pub const ADV7511_REG_VIC_MANUAL: c_uint = 0x3c;
pub const ADV7511_REG_VIC_SEND: c_uint = 0x3d;
pub const ADV7511_REG_VIC_DETECTED: c_uint = 0x3e;
pub const ADV7511_REG_AUX_VIC_DETECTED: c_uint = 0x3f;
pub const ADV7511_REG_PACKET_ENABLE0: c_uint = 0x40;
pub const ADV7511_REG_POWER: c_uint = 0x41;
pub const ADV7511_REG_STATUS: c_uint = 0x42;
pub const ADV7511_REG_EDID_I2C_ADDR: c_uint = 0x43;
pub const ADV7511_REG_PACKET_ENABLE1: c_uint = 0x44;
pub const ADV7511_REG_PACKET_I2C_ADDR: c_uint = 0x45;
pub const ADV7511_REG_DSD_ENABLE: c_uint = 0x46;
pub const ADV7511_REG_VIDEO_INPUT_CFG2: c_uint = 0x48;
pub const ADV7511_REG_INFOFRAME_UPDATE: c_uint = 0x4a;

pub const ADV7511_REG_AVI_INFOFRAME_VERSION: c_uint = 0x52;
pub const ADV7511_REG_AVI_INFOFRAME_LENGTH: c_uint = 0x53;
pub const ADV7511_REG_AVI_INFOFRAME_CHECKSUM: c_uint = 0x54;

pub const ADV7511_REG_AUDIO_INFOFRAME_VERSION: c_uint = 0x70;
pub const ADV7511_REG_AUDIO_INFOFRAME_LENGTH: c_uint = 0x71;
pub const ADV7511_REG_AUDIO_INFOFRAME_CHECKSUM: c_uint = 0x72;

pub const ADV7511_REG_INPUT_CLK_DIV: c_uint = 0x9d;
pub const ADV7511_REG_PLL_STATUS: c_uint = 0x9e;
pub const ADV7511_REG_HDMI_POWER: c_uint = 0xa1;
pub const ADV7511_REG_HDCP_HDMI_CFG: c_uint = 0xaf;

pub const ADV7511_REG_HDCP_STATUS: c_uint = 0xb8;
pub const ADV7511_REG_BCAPS: c_uint = 0xbe;

pub const ADV7511_REG_EDID_SEGMENT: c_uint = 0xc4;
pub const ADV7511_REG_DDC_STATUS: c_uint = 0xc8;
pub const ADV7511_REG_EDID_READ_CTRL: c_uint = 0xc9;

pub const ADV7511_REG_TIMING_GEN_SEQ: c_uint = 0xd0;
pub const ADV7511_REG_POWER2: c_uint = 0xd6;
pub const ADV7511_REG_HSYNC_PLACEMENT_MSB: c_uint = 0xfa;

pub const ADV7511_REG_TMDS_CLOCK_INV: c_uint = 0xde;
pub const ADV7511_REG_ARC_CTRL: c_uint = 0xdf;
pub const ADV7511_REG_CEC_I2C_ADDR: c_uint = 0xe1;
pub const ADV7511_REG_CEC_CTRL: c_uint = 0xe2;
pub const ADV7511_REG_CHIP_ID_HIGH: c_uint = 0xf5;
pub const ADV7511_REG_CHIP_ID_LOW: c_uint = 0xf6;
// Hardware defined default addresses for I2C register maps
pub const ADV7511_CEC_I2C_ADDR_DEFAULT: c_uint = 0x3c;
pub const ADV7511_EDID_I2C_ADDR_DEFAULT: c_uint = 0x3f;
pub const ADV7511_PACKET_I2C_ADDR_DEFAULT: c_uint = 0x38;

pub const ADV7511_HDMI_CFG_MODE_MASK: c_uint = 0x2;
pub const ADV7511_HDMI_CFG_MODE_DVI: c_uint = 0x0;
pub const ADV7511_HDMI_CFG_MODE_HDMI: c_uint = 0x2;
pub const ADV7511_AUDIO_SELECT_I2C: c_uint = 0x0;
pub const ADV7511_AUDIO_SELECT_SPDIF: c_uint = 0x1;
pub const ADV7511_AUDIO_SELECT_DSD: c_uint = 0x2;
pub const ADV7511_AUDIO_SELECT_HBR: c_uint = 0x3;
pub const ADV7511_AUDIO_SELECT_DST: c_uint = 0x4;
pub const ADV7511_I2S_SAMPLE_LEN_16: c_uint = 0x2;
pub const ADV7511_I2S_SAMPLE_LEN_20: c_uint = 0x3;
pub const ADV7511_I2S_SAMPLE_LEN_18: c_uint = 0x4;
pub const ADV7511_I2S_SAMPLE_LEN_22: c_uint = 0x5;
pub const ADV7511_I2S_SAMPLE_LEN_19: c_uint = 0x8;
pub const ADV7511_I2S_SAMPLE_LEN_23: c_uint = 0x9;
pub const ADV7511_I2S_SAMPLE_LEN_24: c_uint = 0xb;
pub const ADV7511_I2S_SAMPLE_LEN_17: c_uint = 0xc;
pub const ADV7511_I2S_SAMPLE_LEN_21: c_uint = 0xd;
pub const ADV7511_SAMPLE_FREQ_44100: c_uint = 0x0;
pub const ADV7511_SAMPLE_FREQ_48000: c_uint = 0x2;
pub const ADV7511_SAMPLE_FREQ_32000: c_uint = 0x3;
pub const ADV7511_SAMPLE_FREQ_88200: c_uint = 0x8;
pub const ADV7511_SAMPLE_FREQ_96000: c_uint = 0xa;
pub const ADV7511_SAMPLE_FREQ_176400: c_uint = 0xc;
pub const ADV7511_SAMPLE_FREQ_192000: c_uint = 0xe;

pub const ADV7511_REG_POWER2_HPD_SRC_MASK: c_uint = 0xc0;
pub const ADV7511_REG_POWER2_HPD_SRC_BOTH: c_uint = 0x00;
pub const ADV7511_REG_POWER2_HPD_SRC_HPD: c_uint = 0x40;
pub const ADV7511_REG_POWER2_HPD_SRC_CEC: c_uint = 0x80;
pub const ADV7511_REG_POWER2_HPD_SRC_NONE: c_uint = 0xc0;

pub const ADV7511_LOW_REFRESH_RATE_NONE: c_uint = 0x0;
pub const ADV7511_LOW_REFRESH_RATE_24HZ: c_uint = 0x1;
pub const ADV7511_LOW_REFRESH_RATE_25HZ: c_uint = 0x2;
pub const ADV7511_LOW_REFRESH_RATE_30HZ: c_uint = 0x3;
pub const ADV7511_AUDIO_CFG3_LEN_MASK: c_uint = 0x0f;
pub const ADV7511_I2C_FREQ_ID_CFG_RATE_MASK: c_uint = 0xf0;
pub const ADV7511_AUDIO_SOURCE_I2S: c_int = 0;
pub const ADV7511_AUDIO_SOURCE_SPDIF: c_int = 1;
pub const ADV7511_I2S_FORMAT_I2S: c_int = 0;
pub const ADV7511_I2S_FORMAT_RIGHT_J: c_int = 1;
pub const ADV7511_I2S_FORMAT_LEFT_J: c_int = 2;
pub const ADV7511_I2S_IEC958_DIRECT: c_int = 3;

pub const ADV7511_REG_CEC_TX_FRAME_HDR: c_uint = 0x00;
pub const ADV7511_REG_CEC_TX_FRAME_DATA0: c_uint = 0x01;
pub const ADV7511_REG_CEC_TX_FRAME_LEN: c_uint = 0x10;
pub const ADV7511_REG_CEC_TX_ENABLE: c_uint = 0x11;
pub const ADV7511_REG_CEC_TX_RETRY: c_uint = 0x12;
pub const ADV7511_REG_CEC_TX_LOW_DRV_CNT: c_uint = 0x14;
pub const ADV7511_REG_CEC_RX1_FRAME_HDR: c_uint = 0x15;
pub const ADV7511_REG_CEC_RX1_FRAME_DATA0: c_uint = 0x16;
pub const ADV7511_REG_CEC_RX1_FRAME_LEN: c_uint = 0x25;
pub const ADV7511_REG_CEC_RX_STATUS: c_uint = 0x26;
pub const ADV7511_REG_CEC_RX2_FRAME_HDR: c_uint = 0x27;
pub const ADV7511_REG_CEC_RX2_FRAME_DATA0: c_uint = 0x28;
pub const ADV7511_REG_CEC_RX2_FRAME_LEN: c_uint = 0x37;
pub const ADV7511_REG_CEC_RX3_FRAME_HDR: c_uint = 0x38;
pub const ADV7511_REG_CEC_RX3_FRAME_DATA0: c_uint = 0x39;
pub const ADV7511_REG_CEC_RX3_FRAME_LEN: c_uint = 0x48;
pub const ADV7511_REG_CEC_RX_BUFFERS: c_uint = 0x4a;
pub const ADV7511_REG_CEC_LOG_ADDR_MASK: c_uint = 0x4b;
pub const ADV7511_REG_CEC_LOG_ADDR_0_1: c_uint = 0x4c;
pub const ADV7511_REG_CEC_LOG_ADDR_2: c_uint = 0x4d;
pub const ADV7511_REG_CEC_CLK_DIV: c_uint = 0x4e;
pub const ADV7511_REG_CEC_SOFT_RESET: c_uint = 0x50;
pub const ADV7533_REG_CEC_OFFSET: c_uint = 0x70;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv7511_input_clock {
    ADV7511_INPUT_CLOCK_1X,
    ADV7511_INPUT_CLOCK_2X,
    ADV7511_INPUT_CLOCK_DDR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv7511_input_justification {
    ADV7511_INPUT_JUSTIFICATION_EVENLY = 0,
    ADV7511_INPUT_JUSTIFICATION_RIGHT = 1,
    ADV7511_INPUT_JUSTIFICATION_LEFT = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv7511_input_sync_pulse {
    ADV7511_INPUT_SYNC_PULSE_DE = 0,
    ADV7511_INPUT_SYNC_PULSE_HSYNC = 1,
    ADV7511_INPUT_SYNC_PULSE_VSYNC = 2,
    ADV7511_INPUT_SYNC_PULSE_NONE = 3,
}

//
// enum adv7511_sync_polarity - Polarity for the input sync signals
// @ADV7511_SYNC_POLARITY_PASSTHROUGH:  Sync polarity matches that of
// the currently configured mode.
// @ADV7511_SYNC_POLARITY_LOW:	    Sync polarity is low
// @ADV7511_SYNC_POLARITY_HIGH:	    Sync polarity is high
//
// If the polarity is set to either LOW or HIGH the driver will configure the
// ADV7511 to internally invert the sync signal if required to match the sync
// polarity setting for the currently selected output mode.
//
// If the polarity is set to PASSTHROUGH, the ADV7511 will route the signal
// unchanged. This is used when the upstream graphics core already generates
// the sync signals with the correct polarity.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv7511_sync_polarity {
    ADV7511_SYNC_POLARITY_PASSTHROUGH,
    ADV7511_SYNC_POLARITY_LOW,
    ADV7511_SYNC_POLARITY_HIGH,
}

//
// struct adv7511_link_config - Describes adv7511 hardware configuration
// @input_color_depth:		Number of bits per color component (8, 10 or 12)
// @input_colorspace:		The input colorspace (RGB, YUV444, YUV422)
// @input_clock:		The input video clock style (1x, 2x, DDR)
// @input_style:		The input component arrangement variant
// @input_justification:	Video input format bit justification
// @clock_delay:		Clock delay for the input clock (in ps)
// @embedded_sync:		Video input uses BT.656-style embedded sync
// @sync_pulse:			Select the sync pulse
// @vsync_polarity:		vsync input signal configuration
// @hsync_polarity:		hsync input signal configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7511_link_config {
    pub input_color_depth: c_uint,
    pub input_colorspace: hdmi_colorspace,
    pub input_clock: adv7511_input_clock,
    pub input_style: c_uint,
    pub input_justification: adv7511_input_justification,
    pub clock_delay: c_int,
    pub embedded_sync: bool,
    pub sync_pulse: adv7511_input_sync_pulse,
    pub vsync_polarity: adv7511_sync_polarity,
    pub hsync_polarity: adv7511_sync_polarity,
}

//
// enum adv7511_csc_scaling - Scaling factor for the ADV7511 CSC
// @ADV7511_CSC_SCALING_1: CSC results are not scaled
// @ADV7511_CSC_SCALING_2: CSC results are scaled by a factor of two
// @ADV7511_CSC_SCALING_4: CSC results are scalled by a factor of four
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv7511_csc_scaling {
    ADV7511_CSC_SCALING_1 = 0,
    ADV7511_CSC_SCALING_2 = 1,
    ADV7511_CSC_SCALING_4 = 2,
}

//
// struct adv7511_video_config - Describes adv7511 hardware configuration
// @csc_enable:			Whether to enable color space conversion
// @csc_scaling_factor:		Color space conversion scaling factor
// @csc_coefficents:		Color space conversion coefficents
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7511_video_config {
    pub csc_enable: bool,
    pub csc_scaling_factor: adv7511_csc_scaling,
    pub csc_coefficents: *const u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv7511_type {
    ADV7511,
    ADV7533,
    ADV7535,
}

pub const ADV7511_MAX_ADDRS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7511_chip_info {
    pub type: adv7511_type,
    pub max_mode_clock_khz: c_uint,
    pub max_lane_freq_khz: c_uint,
    pub name: *const c_char,
    pub supply_names: *const *const c_char,
    pub num_supplies: c_uint,
    pub reg_cec_offset: c_uint,
    pub has_dsi: bool,
    pub link_config: bool,
    pub hpd_override_enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7511 {
    pub i2c_main: *mut i2c_client,
    pub i2c_edid: *mut i2c_client,
    pub i2c_packet: *mut i2c_client,
    pub i2c_cec: *mut i2c_client,
    pub regmap: *mut regmap,
    pub regmap_packet: *mut regmap,
    pub regmap_cec: *mut regmap,
    pub status: drm_connector_status,
    pub powered: bool,
    pub curr_mode: drm_display_mode,
    pub f_tmds: c_uint,
    pub f_audio: c_uint,
    pub audio_source: c_uint,
    pub current_edid_segment: c_uint,
    pub edid_buf: [u8; 256],
    pub edid_read: bool,
    pub wq: wait_queue_head_t,
    pub hpd_work: work_struct,
    pub bridge: drm_bridge,
    pub cec_connector: *mut drm_connector,
    pub embedded_sync: bool,
    pub vsync_polarity: adv7511_sync_polarity,
    pub hsync_polarity: adv7511_sync_polarity,
    pub rgb: bool,
    pub gpio_pd: *mut gpio_desc,
    pub supplies: *mut regulator_bulk_data,
// ADV7533 DSI RX related params
    pub host_node: *mut device_node,
    pub dsi: *mut mipi_dsi_device,
    pub num_dsi_lanes: u8,
    pub use_timing_gen: bool,
    pub info: *const adv7511_chip_info,
    pub cec_addr: [u8; ADV7511_MAX_ADDRS],
    pub cec_valid_addrs: u8,
    pub cec_enabled_adap: bool,
    pub cec_clk: *mut clk,
    pub cec_clk_freq: u32,
}

extern "C" {
    pub fn container_of(_arg: bridge, adv7511: struct, _arg: bridge) -> return;
}

extern "C" {
    pub fn adv7511_cec_enable(bridge: *mut drm_bridge, enable: bool) -> c_int;
}
extern "C" {
    pub fn adv7511_cec_log_addr(bridge: *mut drm_bridge, addr: u8) -> c_int;
}
extern "C" {
    pub fn adv7511_cec_irq_process(adv7511: *mut adv7511, irq1: c_uint) -> c_int;
}

extern "C" {
    pub fn adv7533_dsi_power_on(adv: *mut adv7511);
}
extern "C" {
    pub fn adv7533_dsi_power_off(adv: *mut adv7511);
}
extern "C" {
    pub fn adv7533_dsi_config_timing_gen(adv: *mut adv7511);
}
extern "C" {
    pub fn adv7533_patch_registers(adv: *mut adv7511) -> c_int;
}
extern "C" {
    pub fn adv7533_patch_cec_registers(adv: *mut adv7511) -> c_int;
}
extern "C" {
    pub fn adv7533_attach_dsi(adv: *mut adv7511) -> c_int;
}
extern "C" {
    pub fn adv7533_parse_dt(np: *mut device_node, adv: *mut adv7511) -> c_int;
}

