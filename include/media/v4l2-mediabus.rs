//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-mediabus.h
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
// Media Bus API header
//
// Copyright (C) 2009, Guennadi Liakhovetski <g.liakhovetski@gmx.de>
//

//
// How to use the V4L2_MBUS_* flags:
// Flags are defined for each of the possible states and values of a media
// bus configuration parameter. One and only one bit of each group of flags
// shall be set by the users of the v4l2_subdev_pad_ops.get_mbus_config
// operation to ensure that no conflicting settings are specified when
// reporting the media bus configuration. For example, it is invalid to set or
// clear both the V4L2_MBUS_HSYNC_ACTIVE_HIGH and the
// V4L2_MBUS_HSYNC_ACTIVE_LOW flag at the same time. Instead either flag
// V4L2_MBUS_HSYNC_ACTIVE_HIGH or flag V4L2_MBUS_HSYNC_ACTIVE_LOW shall be set.
//
// TODO: replace the existing V4L2_MBUS_* flags with structures of fields
// to avoid conflicting settings.
//
// In example:
// #define V4L2_MBUS_HSYNC_ACTIVE_HIGH             BIT(2)
// #define V4L2_MBUS_HSYNC_ACTIVE_LOW              BIT(3)
// will be replaced by a field whose value reports the intended active state of
// the signal:
// unsigned int v4l2_mbus_hsync_active : 1;
//
// Parallel flags
//
// The client runs in master or in slave mode. By "Master mode" an operation
// mode is meant, when the client (e.g., a camera sensor) is producing
// horizontal and vertical synchronisation. In "Slave mode" the host is
// providing these signals to the slave.
//

//
// Signal polarity flags
// Note: in BT.656 mode HSYNC, FIELD, and VSYNC are unused
// V4L2_MBUS_[HV]SYNC* flags should be also used for specifying
// configuration of hardware that uses [HV]REF signals
//

// FIELD = 0/1 - Field1 (odd)/Field2 (even)

// FIELD = 1/0 - Field1 (odd)/Field2 (even)

// Active state of Sync-on-green (SoG) signal, 0/1 for LOW/HIGH respectively.

// Serial flags
// Clock non-continuous mode support.

pub const V4L2_MBUS_CSI2_MAX_DATA_LANES: c_int = 8;
//
// enum v4l2_mbus_csi2_cphy_line_orders_type - CSI-2 C-PHY line order
// @V4L2_MBUS_CSI2_CPHY_LINE_ORDER_ABC: C-PHY line order ABC (default)
// @V4L2_MBUS_CSI2_CPHY_LINE_ORDER_ACB: C-PHY line order ACB
// @V4L2_MBUS_CSI2_CPHY_LINE_ORDER_BAC: C-PHY line order BAC
// @V4L2_MBUS_CSI2_CPHY_LINE_ORDER_BCA: C-PHY line order BCA
// @V4L2_MBUS_CSI2_CPHY_LINE_ORDER_CAB: C-PHY line order CAB
// @V4L2_MBUS_CSI2_CPHY_LINE_ORDER_CBA: C-PHY line order CBA
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mbus_csi2_cphy_line_orders_type {
    V4L2_MBUS_CSI2_CPHY_LINE_ORDER_ABC,
    V4L2_MBUS_CSI2_CPHY_LINE_ORDER_ACB,
    V4L2_MBUS_CSI2_CPHY_LINE_ORDER_BAC,
    V4L2_MBUS_CSI2_CPHY_LINE_ORDER_BCA,
    V4L2_MBUS_CSI2_CPHY_LINE_ORDER_CAB,
    V4L2_MBUS_CSI2_CPHY_LINE_ORDER_CBA,
}

//
// struct v4l2_mbus_config_mipi_csi2 - MIPI CSI-2 data bus configuration
// @flags: media bus (V4L2_MBUS_*) flags
// @data_lanes: an array of physical data lane indexes
// @clock_lane: physical lane index of the clock lane
// @num_data_lanes: number of data lanes
// @lane_polarities: polarity of the lanes. The order is the same of
// the physical lanes.
// @line_orders: line order of the data lanes. The order is the same of the
// physical lanes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_mbus_config_mipi_csi2 {
    pub flags: c_uint,
    pub data_lanes: [c_uchar; V4L2_MBUS_CSI2_MAX_DATA_LANES],
    pub clock_lane: c_uchar,
    pub num_data_lanes: c_uchar,
    pub V4L2_MBUS_CSI2_MAX_DATA_LANES]: bool lane_polarities[1 +,
    pub line_orders: [v4l2_mbus_csi2_cphy_line_orders_type; V4L2_MBUS_CSI2_MAX_DATA_LANES],
}

//
// struct v4l2_mbus_config_parallel - parallel data bus configuration
// @flags: media bus (V4L2_MBUS_*) flags
// @bus_width: bus width in bits
// @data_shift: data shift in bits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_mbus_config_parallel {
    pub flags: c_uint,
    pub bus_width: c_uchar,
    pub data_shift: c_uchar,
}

//
// struct v4l2_mbus_config_mipi_csi1 - CSI-1/CCP2 data bus configuration
// @clock_inv: polarity of clock/strobe signal
// false - not inverted, true - inverted
// @strobe: false - data/clock, true - data/strobe
// @lane_polarity: the polarities of the clock (index 0) and data lanes
// index (1)
// @data_lane: the number of the data lane
// @clock_lane: the number of the clock lane
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_mbus_config_mipi_csi1 {
    pub clock_inv:1: c_uchar,
    pub strobe:1: c_uchar,
    pub lane_polarity: [bool; 2],
    pub data_lane: c_uchar,
    pub clock_lane: c_uchar,
}

//
// enum v4l2_mbus_type - media bus type
// @V4L2_MBUS_UNKNOWN:	unknown bus type, no V4L2 mediabus configuration
// @V4L2_MBUS_PARALLEL:	parallel interface with hsync and vsync
// @V4L2_MBUS_BT656:	parallel interface with embedded synchronisation, can
// also be used for BT.1120
// @V4L2_MBUS_CSI1:	MIPI CSI-1 serial interface
// @V4L2_MBUS_CCP2:	CCP2 (Compact Camera Port 2)
// @V4L2_MBUS_CSI2_DPHY: MIPI CSI-2 serial interface, with D-PHY
// @V4L2_MBUS_CSI2_CPHY: MIPI CSI-2 serial interface, with C-PHY
// @V4L2_MBUS_DPI:      MIPI VIDEO DPI interface
// @V4L2_MBUS_INVALID:	invalid bus type (keep as last)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mbus_type {
    V4L2_MBUS_UNKNOWN,
    V4L2_MBUS_PARALLEL,
    V4L2_MBUS_BT656,
    V4L2_MBUS_CSI1,
    V4L2_MBUS_CCP2,
    V4L2_MBUS_CSI2_DPHY,
    V4L2_MBUS_CSI2_CPHY,
    V4L2_MBUS_DPI,
    V4L2_MBUS_INVALID,
}

//
// struct v4l2_mbus_config - media bus configuration
// @type: interface type
// @link_freq: The link frequency. See also V4L2_CID_LINK_FREQ control.
// @bus: bus configuration data structure
// @bus.parallel: embedded &struct v4l2_mbus_config_parallel.
// Used if the bus is parallel or BT.656.
// @bus.mipi_csi1: embedded &struct v4l2_mbus_config_mipi_csi1.
// Used if the bus is MIPI Alliance's Camera Serial
// Interface version 1 (MIPI CSI1) or Standard
// Mobile Imaging Architecture's Compact Camera Port 2
// (SMIA CCP2).
// @bus.mipi_csi2: embedded &struct v4l2_mbus_config_mipi_csi2.
// Used if the bus is MIPI Alliance's Camera Serial
// Interface version 2 (MIPI CSI2).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_mbus_config {
    pub type: v4l2_mbus_type,
    pub link_freq: u64,
    pub parallel: v4l2_mbus_config_parallel,
    pub mipi_csi1: v4l2_mbus_config_mipi_csi1,
    pub mipi_csi2: v4l2_mbus_config_mipi_csi2,
    pub bus: },
}

//
// v4l2_fill_pix_format - Ancillary routine that fills a &struct
// v4l2_pix_format fields from a &struct v4l2_mbus_framefmt.
//
// @pix_fmt:	pointer to &struct v4l2_pix_format to be filled
// @mbus_fmt:	pointer to &struct v4l2_mbus_framefmt to be used as model
//
// v4l2_fill_mbus_format - Ancillary routine that fills a &struct
// v4l2_mbus_framefmt from a &struct v4l2_pix_format and a
// data format code.
//
// @mbus_fmt:	pointer to &struct v4l2_mbus_framefmt to be filled
// @pix_fmt:	pointer to &struct v4l2_pix_format to be used as model
// @code:	data format code (from &enum v4l2_mbus_pixelcode)
//
// v4l2_fill_pix_format_mplane - Ancillary routine that fills a &struct
// v4l2_pix_format_mplane fields from a media bus structure.
//
// @pix_mp_fmt:	pointer to &struct v4l2_pix_format_mplane to be filled
// @mbus_fmt:	pointer to &struct v4l2_mbus_framefmt to be used as model
//
// v4l2_fill_mbus_format_mplane - Ancillary routine that fills a &struct
// v4l2_mbus_framefmt from a &struct v4l2_pix_format_mplane.
//
// @mbus_fmt:	pointer to &struct v4l2_mbus_framefmt to be filled
// @pix_mp_fmt:	pointer to &struct v4l2_pix_format_mplane to be used as model
//
