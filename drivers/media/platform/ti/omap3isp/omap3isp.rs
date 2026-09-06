//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/omap3isp/omap3isp.h
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
// omap3isp.h
//
// TI OMAP3 ISP - Bus Configuration
//
// Copyright (C) 2011 Nokia Corporation
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_interface_type {
    ISP_INTERFACE_PARALLEL,
    ISP_INTERFACE_CSI2A_PHY2,
    ISP_INTERFACE_CCP2B_PHY1,
    ISP_INTERFACE_CCP2B_PHY2,
    ISP_INTERFACE_CSI2C_PHY1,
}

//
// struct isp_parallel_cfg - Parallel interface configuration
// @data_lane_shift: Data lane shifter
// 0 - CAMEXT[13:0] -> CAM[13:0]
// 2 - CAMEXT[13:2] -> CAM[11:0]
// 4 - CAMEXT[13:4] -> CAM[9:0]
// 6 - CAMEXT[13:6] -> CAM[7:0]
// @clk_pol: Pixel clock polarity
// 0 - Sample on rising edge, 1 - Sample on falling edge
// @hs_pol: Horizontal synchronization polarity
// 0 - Active high, 1 - Active low
// @vs_pol: Vertical synchronization polarity
// 0 - Active high, 1 - Active low
// @fld_pol: Field signal polarity
// 0 - Positive, 1 - Negative
// @data_pol: Data polarity
// 0 - Normal, 1 - One's complement
// @bt656: Data contain BT.656 embedded synchronization
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_parallel_cfg {
    pub data_lane_shift:3: c_uint,
    pub clk_pol:1: c_uint,
    pub hs_pol:1: c_uint,
    pub vs_pol:1: c_uint,
    pub fld_pol:1: c_uint,
    pub data_pol:1: c_uint,
    pub bt656:1: c_uint,
}

//
// struct isp_csiphy_lane: CCP2/CSI2 lane position and polarity
// @pos: position of the lane
// @pol: polarity of the lane
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_csiphy_lane {
    pub pos: u8,
    pub pol: u8,
}

pub const ISP_CSIPHY1_NUM_DATA_LANES: c_int = 1;
pub const ISP_CSIPHY2_NUM_DATA_LANES: c_int = 2;
//
// struct isp_csiphy_lanes_cfg - CCP2/CSI2 lane configuration
// @data: Configuration of one or two data lanes
// @clk: Clock lane configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_csiphy_lanes_cfg {
    pub data: [isp_csiphy_lane; ISP_CSIPHY2_NUM_DATA_LANES],
    pub clk: isp_csiphy_lane,
}

//
// struct isp_ccp2_cfg - CCP2 interface configuration
// @strobe_clk_pol: Strobe/clock polarity
// 0 - Non Inverted, 1 - Inverted
// @crc: Enable the cyclic redundancy check
// @ccp2_mode: Enable CCP2 compatibility mode
// ISP_CCP2_MODE_MIPI - MIPI-CSI1 mode
// ISP_CCP2_MODE_CCP2 - CCP2 mode
// @phy_layer: Physical layer selection
// ISP_CCP2_PHY_DATA_CLOCK - Data/clock physical layer
// ISP_CCP2_PHY_DATA_STROBE - Data/strobe physical layer
// @vpclk_div: Video port output clock control
// @vp_clk_pol: Video port output clock polarity
// @lanecfg: CCP2/CSI2 lane configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_ccp2_cfg {
    pub strobe_clk_pol:1: c_uint,
    pub crc:1: c_uint,
    pub ccp2_mode:1: c_uint,
    pub phy_layer:1: c_uint,
    pub vpclk_div:2: c_uint,
    pub vp_clk_pol:1: c_uint,
    pub lanecfg: isp_csiphy_lanes_cfg,
}

//
// struct isp_csi2_cfg - CSI2 interface configuration
// @crc: Enable the cyclic redundancy check
// @lanecfg: CSI-2 lane configuration
// @num_data_lanes: The number of data lanes in use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_csi2_cfg {
    pub crc:1: unsigned,
    pub lanecfg: isp_csiphy_lanes_cfg,
    pub num_data_lanes: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_bus_cfg {
    pub interface: isp_interface_type,
    pub parallel: isp_parallel_cfg,
    pub ccp2: isp_ccp2_cfg,
    pub csi2: isp_csi2_cfg,
    pub /: *mut *mut } bus; / gcc < 4.6.0 chokes on anonymous union initializers,
}
