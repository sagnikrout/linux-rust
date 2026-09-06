//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/camss/camss-csid.h
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
// camss-csid.h
//
// Qualcomm MSM Camera Subsystem - CSID (CSI Decoder) Module
//
// Copyright (c) 2011-2014, The Linux Foundation. All rights reserved.
// Copyright (C) 2015-2018 Linaro Ltd.
//

pub const MSM_CSID_PAD_SINK: c_int = 0;
pub const MSM_CSID_PAD_FIRST_SRC: c_int = 1;
pub const MSM_CSID_PADS_NUM: c_int = 5;

// CSID hardware can demultiplex up to 4 outputs
pub const MSM_CSID_MAX_SRC_STREAMS: c_int = 4;
pub const CSID_RESET_TIMEOUT_MS: c_int = 500;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csid_testgen_mode {
    CSID_PAYLOAD_MODE_DISABLED = 0,
    CSID_PAYLOAD_MODE_INCREMENTING = 1,
    CSID_PAYLOAD_MODE_ALTERNATING_55_AA = 2,
    CSID_PAYLOAD_MODE_ALL_ZEROES = 3,
    CSID_PAYLOAD_MODE_ALL_ONES = 4,
    CSID_PAYLOAD_MODE_RANDOM = 5,
    CSID_PAYLOAD_MODE_USER_SPECIFIED = 6,
    CSID_PAYLOAD_MODE_NUM_SUPPORTED_GEN1 = 6, /* excluding disabled */
    CSID_PAYLOAD_MODE_COMPLEX_PATTERN = 7,
    CSID_PAYLOAD_MODE_COLOR_BOX = 8,
    CSID_PAYLOAD_MODE_COLOR_BARS = 9,
    CSID_PAYLOAD_MODE_NUM_SUPPORTED_GEN2 = 9, /* excluding disabled */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csid_format_info {
    pub code: u32,
    pub data_type: u8,
    pub decode_format: u8,
    pub bpp: u8,
    pub /: *mut *mut u8 spp; / bus samples per pixel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csid_formats {
    pub nformats: c_uint,
    pub formats: *const csid_format_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csid_testgen_config {
    pub mode: csid_testgen_mode,
    pub const*modes: *const *const c_char,
    pub nmodes: u8,
    pub enabled: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csid_phy_config {
    pub csiphy_id: u8,
    pub lane_cnt: u8,
    pub lane_assign: u32,
    pub en_vc: u32,
    pub need_vc_update: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csid_hw_ops {
//
// configure_stream - Configures and starts CSID input stream
// @csid: CSID device
//
    pub enable): *mut *mut *mut void (configure_stream)(struct csid_device csid, u8,
//
// configure_testgen_pattern - Validates and configures output pattern mode
// of test pattern generator
// @csid: CSID device
//
    pub val): *mut *mut *mut int (configure_testgen_pattern)(struct csid_device csid, s32,
//
// hw_version - Read hardware version register from hardware
// @csid: CSID device
//
    pub csid): *mut *mut u32 (hw_version)(struct csid_device,
//
// isr - CSID module interrupt service routine
// @irq: Interrupt line
// @dev: CSID device
//
// Return IRQ_HANDLED on success
//
    pub dev): *mut *mut irqreturn_t (isr)(int irq, void,
//
// reset - Trigger reset on CSID module and wait to complete
// @csid: CSID device
//
// Return 0 on success or a negative error code otherwise
//
    pub csid): *mut *mut int (reset)(struct csid_device,
//
// src_pad_code - Pick an output/src format based on the input/sink format
// @csid: CSID device
// @sink_code: The sink format of the input
// @match_format_idx: Request preferred index, as defined by subdevice csid_format.
// Set @match_code to 0 if used.
// @match_code: Request preferred code, set @match_format_idx to 0 if used
//
// Return 0 on failure or src format code otherwise
//
    pub match_code): unsigned int match_format_idx, u32,
//
// subdev_init - Initialize CSID device according for hardware revision
// @csid: CSID device
//
    pub csid): *mut *mut void (subdev_init)(struct csid_device,
//
// reg_update - receive message from other sub device
// @csid: CSID device
// @port_id: Port id
// @is_clear: Indicate if it is clearing reg update or setting reg update
//
    pub is_clear): *mut *mut *mut void (reg_update)(struct csid_device csid, int port_id, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csid_subdev_resources {
    pub is_lite: bool,
    pub hw_ops: *const csid_hw_ops,
    pub parent_dev_ops: *const parent_dev_ops,
    pub formats: *const csid_formats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csid_device {
    pub camss: *mut camss,
    pub id: u8,
    pub subdev: v4l2_subdev,
    pub pads: [media_pad; MSM_CSID_PADS_NUM],
    pub base: *mut void __iomem,
    pub irq: u32,
    pub irq_name: [c_char; 30],
    pub reg_update: u32,
    pub clock: *mut camss_clock,
    pub nclocks: c_int,
    pub supplies: *mut regulator_bulk_data,
    pub num_supplies: c_int,
    pub reset_complete: completion,
    pub testgen: csid_testgen_config,
    pub tpg_linked: bool,
    pub phy: csid_phy_config,
    pub fmt: [v4l2_mbus_framefmt; MSM_CSID_PADS_NUM],
    pub ctrls: v4l2_ctrl_handler,
    pub testgen_mode: *mut v4l2_ctrl,
    pub res: *const csid_subdev_resources,
}

//
// csid_find_code - Find a format code in an array using array index or format code
// @codes: Array of format codes
// @ncodes: Length of @code array
// @req_format_idx: Request preferred index, as defined by subdevice csid_format.
// Set @match_code to 0 if used.
// @match_code: Request preferred code, set @req_format_idx to 0 if used
//
// Return 0 on failure or format code otherwise
//
// csid_get_fmt_entry - Find csid_format_info entry with matching format code
// @formats: Array of format csid_format_info entries
// @nformats: Length of @nformats array
// @code: Desired format code
//
// Return formats[0] on failure to find code
//
extern "C" {
    pub fn msm_csid_unregister_entity(csid: *mut csid_device);
}
extern "C" {
    pub fn msm_csid_get_csid_id(entity: *mut media_entity, id: *mut u8);
}
//
// csid_is_lite - Check if CSID is CSID lite.
// @csid: CSID Device
//
// Return whether CSID is CSID lite
//
extern "C" {
    pub fn csid_is_lite(csid: *mut csid_device) -> bool;
}
//
// csid_hw_version - CSID hardware version query
// @csid: CSID device
//
// Return HW version or error
//
extern "C" {
    pub fn csid_hw_version(csid: *mut csid_device) -> u32;
}
//
// csid_src_pad_code - Pick an output/src format based on the input/sink format
// @csid: CSID device
// @sink_code: The sink format of the input
// @match_format_idx: Request preferred index, as defined by subdevice csid
// format. Set @match_code to 0 if used.
// @match_code: Request preferred code, set @match_format_idx to 0 if used
//
// Return 0 on failure or src format code otherwise
//
