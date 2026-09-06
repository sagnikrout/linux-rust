//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/adv748x/adv748x.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Driver for Analog Devices ADV748X video decoder and HDMI receiver
//
// Copyright (C) 2017 Renesas Electronics Corp.
//
// Authors:
// Koji Matsuoka <koji.matsuoka.xm@renesas.com>
// Niklas Söderlund <niklas.soderlund@ragnatech.se>
// Kieran Bingham <kieran.bingham@ideasonboard.com>
//
// The ADV748x range of receivers have the following configurations:
//
// Analog   HDMI  MHL  4-Lane  1-Lane
// In      In         CSI     CSI
// ADV7480               X    X     X
// ADV7481      X        X    X     X       X
// ADV7482      X        X          X       X
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv748x_page {
    ADV748X_PAGE_IO,
    ADV748X_PAGE_DPLL,
    ADV748X_PAGE_CP,
    ADV748X_PAGE_HDMI,
    ADV748X_PAGE_EDID,
    ADV748X_PAGE_REPEATER,
    ADV748X_PAGE_INFOFRAME,
    ADV748X_PAGE_CBUS,
    ADV748X_PAGE_CEC,
    ADV748X_PAGE_SDP,
    ADV748X_PAGE_TXB,
    ADV748X_PAGE_TXA,
    ADV748X_PAGE_MAX,

// Fake pages for register sequences
    ADV748X_PAGE_EOR,		/* End Mark */
}

//
// Device tree port number definitions
//
// The ADV748X ports define the mapping between subdevices
// and the device tree specification
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv748x_ports {
    ADV748X_PORT_AIN0 = 0,
    ADV748X_PORT_AIN1 = 1,
    ADV748X_PORT_AIN2 = 2,
    ADV748X_PORT_AIN3 = 3,
    ADV748X_PORT_AIN4 = 4,
    ADV748X_PORT_AIN5 = 5,
    ADV748X_PORT_AIN6 = 6,
    ADV748X_PORT_AIN7 = 7,
    ADV748X_PORT_HDMI = 8,
    ADV748X_PORT_TTL = 9,
    ADV748X_PORT_TXA = 10,
    ADV748X_PORT_TXB = 11,
    ADV748X_PORT_MAX = 12,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv748x_csi2_pads {
    ADV748X_CSI2_SINK,
    ADV748X_CSI2_SOURCE,
    ADV748X_CSI2_NR_PADS,
}

// CSI2 transmitters can have 2 internal connections, HDMI/AFE
pub const ADV748X_CSI2_MAX_SUBDEVS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv748x_csi2 {
    pub state: *mut adv748x_state,
    pub page: c_uint,
    pub port: c_uint,
    pub num_lanes: c_uint,
    pub active_lanes: c_uint,
    pub pads: [media_pad; ADV748X_CSI2_NR_PADS],
    pub ctrl_hdl: v4l2_ctrl_handler,
    pub pixel_rate: *mut v4l2_ctrl,
    pub src: *mut v4l2_subdev,
    pub sd: v4l2_subdev,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv748x_hdmi_pads {
    ADV748X_HDMI_SINK,
    ADV748X_HDMI_SOURCE,
    ADV748X_HDMI_NR_PADS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv748x_hdmi {
    pub pads: [media_pad; ADV748X_HDMI_NR_PADS],
    pub ctrl_hdl: v4l2_ctrl_handler,
    pub sd: v4l2_subdev,
    pub format: v4l2_mbus_framefmt,
    pub timings: v4l2_dv_timings,
    pub aspect_ratio: v4l2_fract,
    pub tx: *mut adv748x_csi2,
    pub edid: [u8; 512],
    pub present: u32,
    pub blocks: c_uint,
    pub edid: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv748x_afe_pads {
    ADV748X_AFE_SINK_AIN0,
    ADV748X_AFE_SINK_AIN1,
    ADV748X_AFE_SINK_AIN2,
    ADV748X_AFE_SINK_AIN3,
    ADV748X_AFE_SINK_AIN4,
    ADV748X_AFE_SINK_AIN5,
    ADV748X_AFE_SINK_AIN6,
    ADV748X_AFE_SINK_AIN7,
    ADV748X_AFE_SOURCE,
    ADV748X_AFE_NR_PADS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv748x_afe {
    pub pads: [media_pad; ADV748X_AFE_NR_PADS],
    pub ctrl_hdl: v4l2_ctrl_handler,
    pub sd: v4l2_subdev,
    pub format: v4l2_mbus_framefmt,
    pub tx: *mut adv748x_csi2,
    pub streaming: bool,
    pub curr_norm: v4l2_std_id,
    pub input: c_uint,
}

//
// struct adv748x_state - State of ADV748X
// @dev:		(OF) device
// @client:		I2C client
// @mutex:		protect global state
//
// @endpoints:		parsed device node endpoints for each port
//
// @i2c_clients:	I2C clients for the page accesses
// @regmap:		regmap configuration pages.
//
// @hdmi:		state of HDMI receiver context
// @afe:		state of AFE receiver context
// @txa:		state of TXA transmitter context
// @txb:		state of TXB transmitter context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv748x_state {
    pub dev: *mut device,
    pub client: *mut i2c_client,
    pub mutex: mutex,
    pub endpoints: [*mut device_node; ADV748X_PORT_MAX],
    pub i2c_clients: [*mut i2c_client; ADV748X_PAGE_MAX],
    pub regmap: [*mut regmap; ADV748X_PAGE_MAX],
    pub hdmi: adv748x_hdmi,
    pub afe: adv748x_afe,
    pub txa: adv748x_csi2,
    pub txb: adv748x_csi2,
}

// Register Mappings
// IO Map
pub const ADV748X_IO_PD: c_uint = 0x00	/* power down controls */;

pub const ADV748X_IO_REG_01: c_uint = 0x01	/* pwrdn{2}b, prog_xtal_freq */;

pub const ADV748X_IO_REG_04: c_uint = 0x04;

pub const ADV748X_IO_DATAPATH: c_uint = 0x03	/* datapath cntrl */;
pub const ADV748X_IO_DATAPATH_VFREQ_M: c_uint = 0x70;
pub const ADV748X_IO_DATAPATH_VFREQ_SHIFT: c_int = 4;
pub const ADV748X_IO_VID_STD: c_uint = 0x05;
pub const ADV748X_IO_10: c_uint = 0x10	/* io_reg_10 */;

pub const ADV748X_IO_CHIP_REV_ID_1: c_uint = 0xdf;
pub const ADV748X_IO_CHIP_REV_ID_2: c_uint = 0xe0;
pub const ADV748X_IO_REG_F2: c_uint = 0xf2;

// For PAGE slave address offsets
pub const ADV748X_IO_SLAVE_ADDR_BASE: c_uint = 0xf2;
//
// The ADV748x_Recommended_Settings_PrA_2014-08-20.pdf details both 0x80 and
// 0xff as examples for performing a software reset.
//
pub const ADV748X_IO_REG_FF: c_uint = 0xff;
pub const ADV748X_IO_REG_FF_MAIN_RESET: c_uint = 0xff;
// HDMI RX Map
pub const ADV748X_HDMI_LW1: c_uint = 0x07	/* line width_1 */;

pub const ADV748X_HDMI_LW1_WIDTH_MASK: c_uint = 0x1fff;
pub const ADV748X_HDMI_F0H1: c_uint = 0x09	/* field0 height_1 */;
pub const ADV748X_HDMI_F0H1_HEIGHT_MASK: c_uint = 0x1fff;
pub const ADV748X_HDMI_F1H1: c_uint = 0x0b	/* field1 height_1 */;

pub const ADV748X_HDMI_HFRONT_PORCH: c_uint = 0x20	/* hsync_front_porch_1 */;
pub const ADV748X_HDMI_HFRONT_PORCH_MASK: c_uint = 0x1fff;
pub const ADV748X_HDMI_HSYNC_WIDTH: c_uint = 0x22	/* hsync_pulse_width_1 */;
pub const ADV748X_HDMI_HSYNC_WIDTH_MASK: c_uint = 0x1fff;
pub const ADV748X_HDMI_HBACK_PORCH: c_uint = 0x24	/* hsync_back_porch_1 */;
pub const ADV748X_HDMI_HBACK_PORCH_MASK: c_uint = 0x1fff;
pub const ADV748X_HDMI_VFRONT_PORCH: c_uint = 0x2a	/* field0_vs_front_porch_1 */;
pub const ADV748X_HDMI_VFRONT_PORCH_MASK: c_uint = 0x3fff;
pub const ADV748X_HDMI_VSYNC_WIDTH: c_uint = 0x2e	/* field0_vs_pulse_width_1 */;
pub const ADV748X_HDMI_VSYNC_WIDTH_MASK: c_uint = 0x3fff;
pub const ADV748X_HDMI_VBACK_PORCH: c_uint = 0x32	/* field0_vs_back_porch_1 */;
pub const ADV748X_HDMI_VBACK_PORCH_MASK: c_uint = 0x3fff;
pub const ADV748X_HDMI_TMDS_1: c_uint = 0x51	/* hdmi_reg_51 */;
pub const ADV748X_HDMI_TMDS_2: c_uint = 0x52	/* hdmi_reg_52 */;
// HDMI RX Repeater Map
pub const ADV748X_REPEATER_EDID_SZ: c_uint = 0x70	/* primary_edid_size */;
pub const ADV748X_REPEATER_EDID_SZ_SHIFT: c_int = 4;
pub const ADV748X_REPEATER_EDID_CTL: c_uint = 0x74	/* hdcp edid controls */;

// SDP Main Map
pub const ADV748X_SDP_INSEL: c_uint = 0x00	/* user_map_rw_reg_00 */;
pub const ADV748X_SDP_VID_SEL: c_uint = 0x02	/* user_map_rw_reg_02 */;
pub const ADV748X_SDP_VID_SEL_MASK: c_uint = 0xf0;
pub const ADV748X_SDP_VID_SEL_SHIFT: c_int = 4;
// Contrast - Unsigned
pub const ADV748X_SDP_CON: c_uint = 0x08	/* user_map_rw_reg_08 */;
pub const ADV748X_SDP_CON_MIN: c_int = 0;
pub const ADV748X_SDP_CON_DEF: c_int = 128;
pub const ADV748X_SDP_CON_MAX: c_int = 255;
// Brightness - Signed
pub const ADV748X_SDP_BRI: c_uint = 0x0a	/* user_map_rw_reg_0a */;

pub const ADV748X_SDP_BRI_DEF: c_int = 0;
pub const ADV748X_SDP_BRI_MAX: c_int = 127;
// Hue - Signed, inverted
pub const ADV748X_SDP_HUE: c_uint = 0x0b	/* user_map_rw_reg_0b */;

pub const ADV748X_SDP_HUE_DEF: c_int = 0;
pub const ADV748X_SDP_HUE_MAX: c_int = 128;
// Test Patterns / Default Values
pub const ADV748X_SDP_DEF: c_uint = 0x0c	/* user_map_rw_reg_0c */;

pub const ADV748X_SDP_MAP_SEL: c_uint = 0x0e	/* user_map_rw_reg_0e */;
pub const ADV748X_SDP_MAP_SEL_RO_MAIN: c_int = 1;
// Free run pattern select
pub const ADV748X_SDP_FRP: c_uint = 0x14;

// Saturation
pub const ADV748X_SDP_SD_SAT_U: c_uint = 0xe3	/* user_map_rw_reg_e3 */;
pub const ADV748X_SDP_SD_SAT_V: c_uint = 0xe4	/* user_map_rw_reg_e4 */;
pub const ADV748X_SDP_SAT_MIN: c_int = 0;
pub const ADV748X_SDP_SAT_DEF: c_int = 128;
pub const ADV748X_SDP_SAT_MAX: c_int = 255;
// SDP RO Main Map
pub const ADV748X_SDP_RO_10: c_uint = 0x10;

// CP Map
pub const ADV748X_CP_PAT_GEN: c_uint = 0x37	/* int_pat_gen_1 */;

// Contrast Control - Unsigned
pub const ADV748X_CP_CON: c_uint = 0x3a	/* contrast_cntrl */;

// Saturation Control - Unsigned
pub const ADV748X_CP_SAT: c_uint = 0x3b	/* saturation_cntrl */;

// Brightness Control - Signed
pub const ADV748X_CP_BRI: c_uint = 0x3c	/* brightness_cntrl */;

// Hue Control
pub const ADV748X_CP_HUE: c_uint = 0x3d	/* hue_cntrl */;

pub const ADV748X_CP_VID_ADJ: c_uint = 0x3e	/* vid_adj_0 */;

pub const ADV748X_CP_DE_POS_HIGH: c_uint = 0x8b	/* de_pos_adj_6 */;

pub const ADV748X_CP_DE_POS_END_LOW: c_uint = 0x8c	/* de_pos_adj_7 */;
pub const ADV748X_CP_DE_POS_START_LOW: c_uint = 0x8d	/* de_pos_adj_8 */;
pub const ADV748X_CP_VID_ADJ_2: c_uint = 0x91;

pub const ADV748X_CP_CLMP_POS: c_uint = 0xc9	/* clmp_pos_cntrl_4 */;

// CSI : TXA/TXB Maps
pub const ADV748X_CSI_VC_REF: c_uint = 0x0d	/* csi_tx_top_reg_0d */;
pub const ADV748X_CSI_VC_REF_SHIFT: c_int = 6;
pub const ADV748X_CSI_FS_AS_LS: c_uint = 0x1e	/* csi_tx_top_reg_1e */;

// Register handling
extern "C" {
    pub fn adv748x_read(state: *mut adv748x_state, addr: u8, reg: u8) -> c_int;
}
extern "C" {
    pub fn adv748x_write(state: *mut adv748x_state, page: u8, reg: u8, value: u8) -> c_int;
}

extern "C" {
    pub fn media_entity_to_v4l2_subdev(_arg: pad->entity) -> return;
}
extern "C" {
    pub fn adv748x_tx_power(tx: *mut adv748x_csi2, on: bool) -> c_int;
}
extern "C" {
    pub fn adv748x_afe_init(afe: *mut adv748x_afe) -> c_int;
}
extern "C" {
    pub fn adv748x_afe_cleanup(afe: *mut adv748x_afe);
}
extern "C" {
    pub fn adv748x_afe_s_input(afe: *mut adv748x_afe, input: c_uint) -> c_int;
}
extern "C" {
    pub fn adv748x_csi2_init(state: *mut adv748x_state, tx: *mut adv748x_csi2) -> c_int;
}
extern "C" {
    pub fn adv748x_csi2_cleanup(tx: *mut adv748x_csi2);
}
extern "C" {
    pub fn adv748x_csi2_set_virtual_channel(tx: *mut adv748x_csi2, vc: c_uint) -> c_int;
}
extern "C" {
    pub fn adv748x_csi2_set_pixelrate(sd: *mut v4l2_subdev, rate: i64) -> c_int;
}
extern "C" {
    pub fn adv748x_hdmi_init(hdmi: *mut adv748x_hdmi) -> c_int;
}
extern "C" {
    pub fn adv748x_hdmi_cleanup(hdmi: *mut adv748x_hdmi);
}
