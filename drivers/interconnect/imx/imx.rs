//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/interconnect/imx/imx.h
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
// Interconnect framework driver for i.MX SoC
//
// Copyright (c) 2019, BayLibre
// Copyright (c) 2019-2020, NXP
// Author: Alexandre Bailon <abailon@baylibre.com>
// Author: Leonard Crestez <leonard.crestez@nxp.com>
//

pub const IMX_ICC_MAX_LINKS: c_int = 4;
//
// High throughput priority level in Regulator mode
// Read Priority in Fixed/Limiter mode
//
pub const PRIORITY0_SHIFT: c_int = 0;
//
// Low throughput priority level in Regulator mode
// Write Priority in Fixed/Limiter mode
//
pub const PRIORITY1_SHIFT: c_int = 8;
pub const PRIORITY_MASK: c_uint = 0x7;

pub const IMX_NOC_MODE_FIXED: c_int = 0;
pub const IMX_NOC_MODE_LIMITER: c_int = 1;
pub const IMX_NOC_MODE_BYPASS: c_int = 2;
pub const IMX_NOC_MODE_REGULATOR: c_int = 3;
pub const IMX_NOC_MODE_UNCONFIGURED: c_uint = 0xFF;
pub const IMX_NOC_PRIO_REG: c_uint = 0x8;
pub const IMX_NOC_MODE_REG: c_uint = 0xC;
pub const IMX_NOC_BANDWIDTH_REG: c_uint = 0x10;
pub const IMX_NOC_SATURATION: c_uint = 0x14;
pub const IMX_NOC_EXT_CTL_REG: c_uint = 0x18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_icc_provider {
    pub noc_base: *mut void __iomem,
    pub provider: icc_provider,
}

//
// struct imx_icc_node_adj - Describe a dynamic adjustable node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_icc_node_adj_desc {
    pub bw_div: unsigned int bw_mul,,
    pub phandle_name: *const c_char,
    pub main_noc: bool,
}

//
// struct imx_icc_node - Describe an interconnect node
// @name: name of the node
// @id: an unique id to identify the node
// @links: an array of slaves' node id
// @num_links: number of id defined in links
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_icc_node_desc {
    pub name: *const c_char,
    pub id: u16,
    pub links: [u16; IMX_ICC_MAX_LINKS],
    pub num_links: u16,
    pub adj: *const imx_icc_node_adj_desc,
}

//
// struct imx_icc_noc_setting - Describe an interconnect node setting
// @reg: register offset inside the NoC
// @prio_level: priority level
// @mode: functional mode
// @ext_control: external input control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_icc_noc_setting {
    pub reg: u32,
    pub prio_level: u32,
    pub mode: u32,
    pub ext_control: u32,
}

extern "C" {
    pub fn imx_icc_unregister(pdev: *mut platform_device);
}
