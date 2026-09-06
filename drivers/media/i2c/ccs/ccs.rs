//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/ccs/ccs.h
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
// drivers/media/i2c/smiapp/ccs.h
//
// Generic driver for MIPI CCS/SMIA/SMIA++ compliant camera sensors
//
// Copyright (C) 2020 Intel Corporation
// Copyright (C) 2010--2012 Nokia Corporation
// Contact: Sakari Ailus <sakari.ailus@linux.intel.com>
//

//
// Standard SMIA++ constants
//
pub const SMIA_VERSION_1: c_int = 10;

pub const SMIAPP_VERSION_1: c_int = 10;
pub const SMIAPP_PROFILE_0: c_int = 0;
pub const SMIAPP_PROFILE_1: c_int = 1;
pub const SMIAPP_PROFILE_2: c_int = 2;

pub const SMIAPP_RESET_DELAY_CLOCKS: c_int = 2400;

pub const CCS_RESET_DELAY_US: c_int = 5000;
pub const CCS_RESET_TIMEOUT_US: c_int = 1000000;
pub const CCS_COLOUR_COMPONENTS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_flash_strobe_parms {
    pub mode: u8,
    pub strobe_width_high_us: u32,
    pub strobe_delay: u16,
    pub stobe_start_point: u16,
    pub trigger: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_hwconfig {
//
// Change the cci address if i2c_addr_alt is set.
// Both default and alternate cci addr need to be present
//
    pub /: *mut *mut unsigned short i2c_addr_dfl; / Default i2c addr,
    pub /: *mut *mut unsigned short i2c_addr_alt; / Alternate i2c addr,
    pub /: *mut *mut u32 ext_clk; / sensor external clk,
    pub /: *mut *mut unsigned int lanes; / Number of CSI-2 lanes,
    pub /: *mut *mut *mut u32 csi_signalling_mode; / CCS_CSI_SIGNALLING_MODE_,
    pub op_sys_clock: *mut u64,
    pub strobe_setup: *mut ccs_flash_strobe_parms,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_module_ident {
    pub mipi_manufacturer_id: u16,
    pub model_id: u16,
    pub smia_manufacturer_id: u8,
    pub revision_number_major: u8,
    pub flags: u8,
    pub name: *mut c_char,
    pub quirk: *const ccs_quirk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_module_info {
    pub smia_manufacturer_id: u32,
    pub mipi_manufacturer_id: u32,
    pub model_id: u32,
    pub revision_number: u32,
    pub module_year: u32,
    pub module_month: u32,
    pub module_day: u32,
    pub sensor_smia_manufacturer_id: u32,
    pub sensor_mipi_manufacturer_id: u32,
    pub sensor_model_id: u32,
    pub sensor_revision_number: u32,
    pub sensor_firmware_version: u32,
    pub smia_version: u32,
    pub smiapp_version: u32,
    pub ccs_version: u32,
    pub name: *mut c_char,
    pub quirk: *const ccs_quirk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_csi_data_format {
    pub code: u32,
    pub width: u8,
    pub compressed: u8,
    pub pixel_order: u8,
}

pub const CCS_SUBDEVS: c_int = 3;
pub const CCS_PA_PAD_SRC: c_int = 0;
pub const CCS_PAD_SINK: c_int = 0;
pub const CCS_PAD_SRC: c_int = 1;
pub const CCS_PADS: c_int = 2;
pub const CCS_STREAM_PIXEL: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_binning_subtype {
    pub horizontal:4: u8,
    pub vertical:4: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_subdev {
    pub sd: v4l2_subdev,
    pub pads: [media_pad; CCS_PADS],
    pub sink_pad: c_ushort,
    pub source_pad: c_ushort,
    pub npads: c_int,
    pub sensor: *mut ccs_sensor,
    pub ctrl_handler: v4l2_ctrl_handler,
}

//
// struct ccs_sensor - Main device structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_sensor {
//
// "mutex" is used to serialise access to all fields here
// except v4l2_ctrls at the end of the struct. "mutex" is also
// used to serialise access to file handle specific
// information.
//
    pub mutex: mutex,
    pub ssds: [ccs_subdev; CCS_SUBDEVS],
    pub ssds_used: u32,
    pub src: *mut ccs_subdev,
    pub binner: *mut ccs_subdev,
    pub scaler: *mut ccs_subdev,
    pub pixel_array: *mut ccs_subdev,
    pub hwcfg: ccs_hwconfig,
    pub regulators: *mut regulator_bulk_data,
    pub ext_clk: *mut clk,
    pub xshutdown: *mut gpio_desc,
    pub reset: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub ccs_limits: *mut c_void,
    pub nbinning_subtypes: u8,
    pub 1]: ccs_binning_subtype binning_subtypes[CCS_LIM_BINNING_SUB_TYPE_MAX_N +,
    pub mbus_frame_fmts: u64,
    pub csi_format: *const ccs_csi_data_format,
    pub internal_csi_format: *const ccs_csi_data_format,
    pub default_mbus_frame_fmts: u64,
    pub default_pixel_order: c_int,
    pub mdata: ccs_data_container sdata,,
    pub frame_skip: u8,
    pub /: *mut *mut u16 embedded_start; / embedded data start line,
    pub embedded_end: u16,
    pub /: *mut *mut u16 image_start; / image data start line,
    pub /: *mut *mut u16 visible_pixel_start; / start pixel of the visible image,
    pub streaming: u8,
    pub dev_init_done: bool,
    pub handler_setup_needed: bool,
    pub compressed_min_bpp: u8,
    pub minfo: ccs_module_info,
    pub pll: ccs_pll,
// Is a default format supported for a given BPP?
    pub valid_link_freqs: *mut c_ulong,
// Pixel array controls
    pub exposure: *mut v4l2_ctrl,
    pub hflip: *mut v4l2_ctrl,
    pub vflip: *mut v4l2_ctrl,
    pub vblank: *mut v4l2_ctrl,
    pub hblank: *mut v4l2_ctrl,
    pub pixel_rate_parray: *mut v4l2_ctrl,
    pub luminance_level: *mut v4l2_ctrl,
// src controls
    pub link_freq: *mut v4l2_ctrl,
    pub pixel_rate_csi: *mut v4l2_ctrl,
// test pattern colour components
    pub test_data: [*mut v4l2_ctrl; CCS_COLOUR_COMPONENTS],
}

