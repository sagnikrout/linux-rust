//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/cadence/cdns-mhdp8546-core.h
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
// Cadence MHDP8546 DP bridge driver.
//
// Copyright (C) 2020 Cadence Design Systems, Inc.
//
// Author: Quentin Schulz <quentin.schulz@free-electrons.com>
// Swapnil Jakhade <sjakhade@cadence.com>
//

// Register offsets
pub const CDNS_APB_CTRL: c_uint = 0x00000;

pub const CDNS_MAILBOX_FULL: c_uint = 0x00008;
pub const CDNS_MAILBOX_EMPTY: c_uint = 0x0000c;
pub const CDNS_MAILBOX_TX_DATA: c_uint = 0x00010;
pub const CDNS_MAILBOX_RX_DATA: c_uint = 0x00014;
pub const CDNS_KEEP_ALIVE: c_uint = 0x00018;

pub const CDNS_VER_L: c_uint = 0x0001C;
pub const CDNS_VER_H: c_uint = 0x00020;
pub const CDNS_LIB_L_ADDR: c_uint = 0x00024;
pub const CDNS_LIB_H_ADDR: c_uint = 0x00028;
pub const CDNS_MB_INT_MASK: c_uint = 0x00034;
pub const CDNS_MB_INT_STATUS: c_uint = 0x00038;
pub const CDNS_SW_CLK_L: c_uint = 0x0003c;
pub const CDNS_SW_CLK_H: c_uint = 0x00040;
pub const CDNS_SW_EVENT0: c_uint = 0x00044;

pub const CDNS_SW_EVENT1: c_uint = 0x00048;
pub const CDNS_SW_EVENT2: c_uint = 0x0004c;
pub const CDNS_SW_EVENT3: c_uint = 0x00050;
pub const CDNS_APB_INT_MASK: c_uint = 0x0006C;

pub const CDNS_APB_INT_STATUS: c_uint = 0x00070;
pub const CDNS_DPTX_CAR: c_uint = 0x00904;

pub const CDNS_DPTX_PHY_CONFIG: c_uint = 0x02000;

pub const CDNS_DP_FRAMER_GLOBAL_CONFIG: c_uint = 0x02200;

pub const CDNS_DP_FRAMER_TU: c_uint = 0x02208;

pub const CDNS_DP_MTPH_CONTROL: c_uint = 0x02264;

pub const CDNS_DP_MTPH_STATUS: c_uint = 0x0226C;

pub const CDNS_DP_LANE_EN: c_uint = 0x02300;

pub const CDNS_DP_ENHNCD: c_uint = 0x02304;

pub const CDNS_DP_FRAMER_PXL_FORMAT: c_uint = 0x8;

pub const CDNS_DP_BYTE_COUNT_BYTES_IN_CHUNK_SHIFT: c_int = 16;
// mailbox
pub const MAILBOX_RETRY_US: c_int = 1000;
pub const MAILBOX_TIMEOUT_US: c_int = 2000000;
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
// firmware and opcodes

pub const CDNS_MHDP_IMEM: c_uint = 0x10000;
pub const GENERAL_MAIN_CONTROL: c_uint = 0x01;
pub const GENERAL_TEST_ECHO: c_uint = 0x02;
pub const GENERAL_BUS_SETTINGS: c_uint = 0x03;
pub const GENERAL_TEST_ACCESS: c_uint = 0x04;
pub const GENERAL_REGISTER_READ: c_uint = 0x07;
pub const DPTX_SET_POWER_MNG: c_uint = 0x00;
pub const DPTX_GET_EDID: c_uint = 0x02;
pub const DPTX_READ_DPCD: c_uint = 0x03;
pub const DPTX_WRITE_DPCD: c_uint = 0x04;
pub const DPTX_ENABLE_EVENT: c_uint = 0x05;
pub const DPTX_WRITE_REGISTER: c_uint = 0x06;
pub const DPTX_READ_REGISTER: c_uint = 0x07;
pub const DPTX_WRITE_FIELD: c_uint = 0x08;
pub const DPTX_READ_EVENT: c_uint = 0x0a;
pub const DPTX_GET_LAST_AUX_STAUS: c_uint = 0x0e;
pub const DPTX_HPD_STATE: c_uint = 0x11;
pub const DPTX_ADJUST_LT: c_uint = 0x12;
pub const FW_STANDBY: c_int = 0;
pub const FW_ACTIVE: c_int = 1;
// HPD

// general
pub const CDNS_DP_TRAINING_PATTERN_4: c_uint = 0x7;
pub const CDNS_KEEP_ALIVE_TIMEOUT: c_int = 2000;

pub const CDNS_LANE_MAPPING_NORMAL: c_uint = 0xe4;
pub const CDNS_LANE_MAPPING_FLIPPED: c_uint = 0x1b;
pub const CDNS_DP_MAX_NUM_LANES: c_int = 4;

pub const CDNS_MHDP_MAX_STREAMS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_mhdp_link {
    pub revision: c_uchar,
    pub rate: c_uint,
    pub num_lanes: c_uint,
    pub capabilities: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_mhdp_host {
    pub link_rate: c_uint,
    pub lanes_cnt: u8,
    pub volt_swing: u8,
    pub pre_emphasis: u8,
    pub pattern_supp: u8,
    pub lane_mapping: u8,
    pub fast_link: bool,
    pub enhanced: bool,
    pub scrambler: bool,
    pub ssc: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_mhdp_sink {
    pub link_rate: c_uint,
    pub lanes_cnt: u8,
    pub pattern_supp: u8,
    pub fast_link: bool,
    pub enhanced: bool,
    pub ssc: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_mhdp_display_fmt {
    pub color_format: drm_output_color_format,
    pub bpc: u32,
    pub y_only: bool,
}

//
// These enums present MHDP hw initialization state
// Legal state transitions are:
// MHDP_HW_READY <-> MHDP_HW_STOPPED
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhdp_hw_state {
    MHDP_HW_READY = 1,	/* HW ready, FW active */
    MHDP_HW_STOPPED		/* Driver removal FW to be stopped */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhdp_platform_ops {
    pub mhdp): *mut *mut int (init)(struct cdns_mhdp_device,
    pub mhdp): *mut *mut void (exit)(struct cdns_mhdp_device,
    pub mhdp): *mut *mut void (enable)(struct cdns_mhdp_device,
    pub mhdp): *mut *mut void (disable)(struct cdns_mhdp_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_mhdp_bridge_state {
    pub base: drm_bridge_state,
    pub current_mode: *mut drm_display_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_mhdp_platform_info {
    pub input_bus_flags: *const u32,
    pub ops: *const mhdp_platform_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_mhdp_hdcp {
    pub check_work: delayed_work,
    pub prop_work: work_struct,
    pub /: *mut *mut mutex mutex; / mutex to protect hdcp.value,
    pub value: u32,
    pub hdcp_content_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_mhdp_device {
    pub regs: *mut void __iomem,
    pub sapb_regs: *mut void __iomem,
    pub j721e_regs: *mut void __iomem,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub phy: *mut phy,
    pub info: *const cdns_mhdp_platform_info,
// This is to protect mailbox communications with the firmware
    pub mbox_mutex: mutex,
//
// "link_mutex" protects the access to all the link parameters
// including the link training process. Link training will be
// invoked both from threaded interrupt handler and from atomic
// callbacks when link_up is not set. So this mutex protects
// flags such as link_up, bridge_enabled, link.num_lanes,
// link.rate etc.
//
    pub link_mutex: mutex,
    pub connector: *mut drm_connector,
    pub bridge: drm_bridge,
    pub link: cdns_mhdp_link,
    pub aux: drm_dp_aux,
    pub host: cdns_mhdp_host,
    pub sink: cdns_mhdp_sink,
    pub display_fmt: cdns_mhdp_display_fmt,
    pub stream_id: u8,
    pub link_up: bool,
    pub plugged: bool,
//
// "start_lock" protects the access to bridge_attached and
// hw_state data members that control the delayed firmware
// loading and attaching the bridge. They are accessed from
// both the DRM core and cdns_mhdp_fw_cb(). In most cases just
// protecting the data members is enough, but the irq mask
// setting needs to be protected when enabling the FW.
//
    pub start_lock: spinlock_t,
    pub bridge_attached: bool,
    pub bridge_enabled: bool,
    pub hw_state: mhdp_hw_state,
    pub fw_load_wq: wait_queue_head_t,
// Work struct to schedule a uevent on link train failure
    pub modeset_retry_work: work_struct,
    pub hpd_work: work_struct,
    pub sw_events_wq: wait_queue_head_t,
    pub sw_events: u32,
    pub hdcp: cdns_mhdp_hdcp,
    pub hdcp_supported: bool,
}

extern "C" {
    pub fn cdns_mhdp_wait_for_sw_event(mhdp: *mut cdns_mhdp_device, event: u32) -> u32;
}
