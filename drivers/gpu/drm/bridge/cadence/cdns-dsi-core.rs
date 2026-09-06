//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/cadence/cdns-dsi-core.h
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
// Copyright: 2017 Cadence Design Systems, Inc.
//
// Author: Boris Brezillon <boris.brezillon@bootlin.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_dsi_output {
    pub dev: *mut mipi_dsi_device,
    pub bridge: *mut drm_bridge,
    pub phy_opts: phy_configure_opts,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cdns_dsi_input_id {
    CDNS_SDI_INPUT,
    CDNS_DPI_INPUT,
    CDNS_DSC_INPUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_dsi_cfg {
    pub hfp: c_uint,
    pub hsa: c_uint,
    pub hbp: c_uint,
    pub hact: c_uint,
    pub htotal: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_dsi_input {
    pub id: cdns_dsi_input_id,
    pub bridge: drm_bridge,
}

//
// struct cdns_dsi_platform_ops - CDNS DSI Platform operations
// @init: Called in the CDNS DSI probe
// @deinit: Called in the CDNS DSI remove
// @enable: Called at the beginning of CDNS DSI bridge enable
// @disable: Called at the end of CDNS DSI bridge disable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_dsi_platform_ops {
    pub dsi): *mut *mut int (init)(struct cdns_dsi,
    pub dsi): *mut *mut void (deinit)(struct cdns_dsi,
    pub dsi): *mut *mut void (enable)(struct cdns_dsi,
    pub dsi): *mut *mut void (disable)(struct cdns_dsi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_dsi {
    pub base: mipi_dsi_host,
    pub regs: *mut void __iomem,

    pub j721e_regs: *mut void __iomem,

    pub platform_ops: *const cdns_dsi_platform_ops,
    pub input: cdns_dsi_input,
    pub output: cdns_dsi_output,
    pub direct_cmd_fifo_depth: c_uint,
    pub rx_fifo_depth: c_uint,
    pub direct_cmd_comp: completion,
    pub dsi_p_clk: *mut clk,
    pub dsi_p_rst: *mut reset_control,
    pub dsi_sys_clk: *mut clk,
    pub link_initialized: bool,
    pub phy_initialized: bool,
    pub dphy: *mut phy,
}
