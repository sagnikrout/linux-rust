//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/adv7842.h
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
// adv7842 - Analog Devices ADV7842 video decoder driver
//
// Copyright 2013 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//
// Analog input muxing modes (AFE register 0x02, [2:0])
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv7842_ain_sel {
    ADV7842_AIN1_2_3_NC_SYNC_1_2 = 0,
    ADV7842_AIN4_5_6_NC_SYNC_2_1 = 1,
    ADV7842_AIN7_8_9_NC_SYNC_3_1 = 2,
    ADV7842_AIN10_11_12_NC_SYNC_4_1 = 3,
    ADV7842_AIN9_4_5_6_SYNC_2_1 = 4,
}

//
// Bus rotation and reordering. This is used to specify component reordering on
// the board and describes the components order on the bus when the ADV7842
// outputs RGB.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv7842_bus_order {
    ADV7842_BUS_ORDER_RGB,		/* No operation	*/
    ADV7842_BUS_ORDER_GRB,		/* Swap 1-2	*/
    ADV7842_BUS_ORDER_RBG,		/* Swap 2-3	*/
    ADV7842_BUS_ORDER_BGR,		/* Swap 1-3	*/
    ADV7842_BUS_ORDER_BRG,		/* Rotate right	*/
    ADV7842_BUS_ORDER_GBR,		/* Rotate left	*/
}

// Input Color Space (IO register 0x02, [7:4])
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv7842_inp_color_space {
    ADV7842_INP_COLOR_SPACE_LIM_RGB = 0,
    ADV7842_INP_COLOR_SPACE_FULL_RGB = 1,
    ADV7842_INP_COLOR_SPACE_LIM_YCbCr_601 = 2,
    ADV7842_INP_COLOR_SPACE_LIM_YCbCr_709 = 3,
    ADV7842_INP_COLOR_SPACE_XVYCC_601 = 4,
    ADV7842_INP_COLOR_SPACE_XVYCC_709 = 5,
    ADV7842_INP_COLOR_SPACE_FULL_YCbCr_601 = 6,
    ADV7842_INP_COLOR_SPACE_FULL_YCbCr_709 = 7,
    ADV7842_INP_COLOR_SPACE_AUTO = 0xf,
}

// Select output format (IO register 0x03, [4:2])
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv7842_op_format_mode_sel {
    ADV7842_OP_FORMAT_MODE0 = 0x00,
    ADV7842_OP_FORMAT_MODE1 = 0x04,
    ADV7842_OP_FORMAT_MODE2 = 0x08,
}

// Mode of operation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv7842_mode {
    ADV7842_MODE_SDP,
    ADV7842_MODE_COMP,
    ADV7842_MODE_RGB,
    ADV7842_MODE_HDMI
}

// Video standard select (IO register 0x00, [5:0])
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv7842_vid_std_select {
// SDP
    ADV7842_SDP_VID_STD_CVBS_SD_4x1 = 0x01,
    ADV7842_SDP_VID_STD_YC_SD4_x1 = 0x09,
// RGB
    ADV7842_RGB_VID_STD_AUTO_GRAPH_MODE = 0x07,
// HDMI GR
    ADV7842_HDMI_GR_VID_STD_AUTO_GRAPH_MODE = 0x02,
// HDMI COMP
    ADV7842_HDMI_COMP_VID_STD_HD_1250P = 0x1e,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv7842_select_input {
    ADV7842_SELECT_HDMI_PORT_A,
    ADV7842_SELECT_HDMI_PORT_B,
    ADV7842_SELECT_VGA_RGB,
    ADV7842_SELECT_VGA_COMP,
    ADV7842_SELECT_SDP_CVBS,
    ADV7842_SELECT_SDP_YC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adv7842_drive_strength {
    ADV7842_DR_STR_LOW = 0,
    ADV7842_DR_STR_MEDIUM_LOW = 1,
    ADV7842_DR_STR_MEDIUM_HIGH = 2,
    ADV7842_DR_STR_HIGH = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7842_sdp_csc_coeff {
    pub manual: bool,
    pub scaling: u16,
    pub A1: u16,
    pub A2: u16,
    pub A3: u16,
    pub A4: u16,
    pub B1: u16,
    pub B2: u16,
    pub B3: u16,
    pub B4: u16,
    pub C1: u16,
    pub C2: u16,
    pub C3: u16,
    pub C4: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7842_sdp_io_sync_adjustment {
    pub adjust: bool,
    pub hs_beg: u16,
    pub hs_width: u16,
    pub de_beg: u16,
    pub de_end: u16,
    pub vs_beg_o: u8,
    pub vs_beg_e: u8,
    pub vs_end_o: u8,
    pub vs_end_e: u8,
    pub de_v_beg_o: u8,
    pub de_v_beg_e: u8,
    pub de_v_end_o: u8,
    pub de_v_end_e: u8,
}

// Platform dependent definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7842_platform_data {
// chip reset during probe
    pub chip_reset:1: unsigned,
// DIS_PWRDNB: 1 if the PWRDNB pin is unused and unconnected
    pub disable_pwrdnb:1: unsigned,
// DIS_CABLE_DET_RST: 1 if the 5V pins are unused and unconnected
    pub disable_cable_det_rst:1: unsigned,
// Analog input muxing mode
    pub ain_sel: adv7842_ain_sel,
// Bus rotation and reordering
    pub bus_order: adv7842_bus_order,
// Select output format mode
    pub op_format_mode_sel: adv7842_op_format_mode_sel,
// Default mode
    pub mode: adv7842_mode,
// Default input
    pub input: unsigned,
// Video standard
    pub vid_std_select: adv7842_vid_std_select,
// IO register 0x02
    pub alt_gamma:1: unsigned,
// IO register 0x05
    pub blank_data:1: unsigned,
    pub insert_av_codes:1: unsigned,
    pub replicate_av_codes:1: unsigned,
// IO register 0x30
    pub output_bus_lsb_to_msb:1: unsigned,
// IO register 0x14
    pub dr_str_data: adv7842_drive_strength,
    pub dr_str_clk: adv7842_drive_strength,
    pub dr_str_sync: adv7842_drive_strength,
//
// IO register 0x19: Adjustment to the LLC DLL phase in
// increments of 1/32 of a clock period.
//
    pub llc_dll_phase:5: unsigned,
// External RAM for 3-D comb or frame synchronizer
    pub /: *mut *mut unsigned sd_ram_size; / ram size in MB,
    pub /: *mut *mut unsigned sd_ram_ddr:1; / ddr or sdr sdram,
// HDMI free run, CP-reg 0xBA
    pub hdmi_free_run_enable:1: unsigned,
// 0 = Mode 0: run when there is no TMDS clock
    pub hdmi_free_run_mode:1: unsigned,
// SDP free run, CP-reg 0xDD
    pub sdp_free_run_auto:1: unsigned,
    pub sdp_free_run_man_col_en:1: unsigned,
    pub sdp_free_run_cbar_en:1: unsigned,
    pub sdp_free_run_force:1: unsigned,
// HPA manual (0) or auto (1), affects HDMI register 0x69
    pub hpa_auto:1: unsigned,
    pub sdp_csc_coeff: adv7842_sdp_csc_coeff,
    pub sdp_io_sync_625: adv7842_sdp_io_sync_adjustment,
    pub sdp_io_sync_525: adv7842_sdp_io_sync_adjustment,
// i2c addresses
    pub i2c_sdp_io: u8,
    pub i2c_sdp: u8,
    pub i2c_cp: u8,
    pub i2c_vdp: u8,
    pub i2c_afe: u8,
    pub i2c_hdmi: u8,
    pub i2c_repeater: u8,
    pub i2c_edid: u8,
    pub i2c_infoframe: u8,
    pub i2c_cec: u8,
    pub i2c_avlink: u8,
}

// custom ioctl, used to test the external RAM that's used by the
// deinterlacer.

pub const ADV7842_EDID_PORT_A: c_int = 0;
pub const ADV7842_EDID_PORT_B: c_int = 1;
pub const ADV7842_EDID_PORT_VGA: c_int = 2;
pub const ADV7842_PAD_SOURCE: c_int = 3;
