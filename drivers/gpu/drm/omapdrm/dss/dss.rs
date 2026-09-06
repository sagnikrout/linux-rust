//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/omapdrm/dss/dss.h
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
// Copyright (C) 2009 Nokia Corporation
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//
// Some code and ideas taken from drivers/video/omap/ driver
// by Imre Deak.
//

pub const MAX_DSS_LCD_MANAGERS: c_int = 3;
pub const MAX_NUM_DSI: c_int = 2;

// OMAP TRM gives bitfields as start:end, where start is the higher bit

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dss_model {
    DSS_MODEL_OMAP2,
    DSS_MODEL_OMAP3,
    DSS_MODEL_OMAP4,
    DSS_MODEL_OMAP5,
    DSS_MODEL_DRA7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dss_io_pad_mode {
    DSS_IO_PAD_MODE_RESET,
    DSS_IO_PAD_MODE_RFBI,
    DSS_IO_PAD_MODE_BYPASS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dss_hdmi_venc_clk_source_select {
    DSS_VENC_TV_CLK = 0,
    DSS_HDMI_M_PCLK = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dss_dsi_content_type {
    DSS_DSI_CONTENT_DCS,
    DSS_DSI_CONTENT_GENERIC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dss_clk_source {
    DSS_CLK_SRC_FCK = 0,

    DSS_CLK_SRC_PLL1_1,
    DSS_CLK_SRC_PLL1_2,
    DSS_CLK_SRC_PLL1_3,

    DSS_CLK_SRC_PLL2_1,
    DSS_CLK_SRC_PLL2_2,
    DSS_CLK_SRC_PLL2_3,

    DSS_CLK_SRC_HDMI_PLL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dss_pll_id {
    DSS_PLL_DSI1,
    DSS_PLL_DSI2,
    DSS_PLL_HDMI,
    DSS_PLL_VIDEO1,
    DSS_PLL_VIDEO2,
}

pub const DSS_PLL_MAX_HSDIVS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dss_pll_type {
    DSS_PLL_TYPE_A,
    DSS_PLL_TYPE_B,
}

//
// Type-A PLLs: clkout[]/mX[] refer to hsdiv outputs m4, m5, m6, m7.
// Type-B PLLs: clkout[0] refers to m2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dss_pll_clock_info {
// rates that we get with dividers below
    pub fint: c_ulong,
    pub clkdco: c_ulong,
    pub clkout: [c_ulong; DSS_PLL_MAX_HSDIVS],
// dividers
    pub n: u16,
    pub m: u16,
    pub mf: u32,
    pub mX: [u16; DSS_PLL_MAX_HSDIVS],
    pub sd: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dss_pll_ops {
    pub pll): *mut *mut int (enable)(struct dss_pll,
    pub pll): *mut *mut void (disable)(struct dss_pll,
    pub cinfo): *const dss_pll_clock_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dss_pll_hw {
    pub type: dss_pll_type,
    pub n_max: c_uint,
    pub m_min: c_uint,
    pub m_max: c_uint,
    pub mX_max: c_uint,
    pub fint_max: unsigned long fint_min,,
    pub clkdco_max: unsigned long clkdco_min, clkdco_low,,
    pub n_lsb: u8 n_msb,,
    pub m_lsb: u8 m_msb,,
    pub mX_lsb: [u8 mX_msb[DSS_PLL_MAX_HSDIVS],; DSS_PLL_MAX_HSDIVS],
    pub has_stopmode: bool,
    pub has_freqsel: bool,
    pub has_selfreqdco: bool,
    pub has_refsel: bool,
// DRA7 errata i886: use high N & M to avoid jitter
    pub errata_i886: bool,
// DRA7 errata i932: retry pll lock on failure
    pub errata_i932: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dss_pll {
    pub name: *const c_char,
    pub id: dss_pll_id,
    pub dss: *mut dss_device,
    pub clkin: *mut clk,
    pub regulator: *mut regulator,
    pub base: *mut void __iomem,
    pub hw: *const dss_pll_hw,
    pub ops: *const dss_pll_ops,
    pub cinfo: dss_pll_clock_info,
}

// Defines a generic omap register field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dss_reg_field {
    pub end: u8 start,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dispc_clock_info {
// rates that we get with dividers below
    pub lck: c_ulong,
    pub pck: c_ulong,
// dividers
    pub lck_div: u16,
    pub pck_div: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dss_lcd_mgr_config {
    pub io_pad_mode: dss_io_pad_mode,
    pub stallmode: bool,
    pub fifohandcheck: bool,
    pub clock_info: dispc_clock_info,
    pub video_port_width: c_int,
    pub lcden_sig_polarity: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dss_device {
    pub pdev: *mut platform_device,
    pub base: *mut void __iomem,
    pub syscon_pll_ctrl: *mut regmap,
    pub syscon_pll_ctrl_offset: u32,
    pub drm_pdev: *mut platform_device,
    pub parent_clk: *mut clk,
    pub dss_clk: *mut clk,
    pub dss_clk_rate: c_ulong,
    pub cache_req_pck: c_ulong,
    pub cache_prate: c_ulong,
    pub cache_dispc_cinfo: dispc_clock_info,
    pub dsi_clk_source: [dss_clk_source; MAX_NUM_DSI],
    pub dispc_clk_source: dss_clk_source,
    pub lcd_clk_source: [dss_clk_source; MAX_DSS_LCD_MANAGERS],
    pub ctx_valid: bool,
    pub sizeof(u32)]: u32 ctx[DSS_SZ_REGS /,
    pub feat: *const dss_features,
    pub root: *mut dentry,
    pub clk: *mut dss_debugfs_entry,
    pub dss: *mut dss_debugfs_entry,
    pub debugfs: },
    pub plls: [*mut dss_pll; 4],
    pub video1_pll: *mut dss_pll,
    pub video2_pll: *mut dss_pll,
    pub dispc: *mut dispc_device,
    pub mgr_ops_priv: *mut omap_drm_private,
}

// core
// To be implemented when the OMAP platform will provide this feature
// DSS

extern "C" {
    pub fn dss_debugfs_remove_file(entry: *mut dss_debugfs_entry);
}

extern "C" {
    pub fn dss_runtime_get(dss: *mut dss_device) -> c_int;
}
extern "C" {
    pub fn dss_runtime_put(dss: *mut dss_device);
}
extern "C" {
    pub fn dss_get_dispc_clk_rate(dss: *mut dss_device) -> c_ulong;
}
extern "C" {
    pub fn dss_get_max_fck_rate(dss: *mut dss_device) -> c_ulong;
}
// DSS VIDEO PLL
extern "C" {
    pub fn dss_video_pll_uninit(pll: *mut dss_pll);
}
extern "C" {
    pub fn dss_ctrl_pll_enable(pll: *mut dss_pll, enable: bool);
}
extern "C" {
    pub fn dss_sdi_init(dss: *mut dss_device, datapairs: c_int);
}
extern "C" {
    pub fn dss_sdi_enable(dss: *mut dss_device) -> c_int;
}
extern "C" {
    pub fn dss_sdi_disable(dss: *mut dss_device);
}
extern "C" {
    pub fn dss_get_dispc_clk_source(dss: *mut dss_device) -> dss_clk_source;
}
extern "C" {
    pub fn dss_set_venc_output(dss: *mut dss_device, type: omap_dss_venc_type);
}
extern "C" {
    pub fn dss_set_dac_pwrdn_bgz(dss: *mut dss_device, enable: bool);
}
extern "C" {
    pub fn dss_set_fck_rate(dss: *mut dss_device, rate: c_ulong) -> c_int;
}
extern "C" {
    pub fn bool(fck: *mut *mut dss_div_calc_func)(unsigned long, data: *mut c_void) -> typedef;
}
// SDI

extern "C" {
    pub fn sdi_uninit_port(port: *mut device_node);
}

// DSI

extern "C" {
    pub fn dsi_irq_handler();
}

// DPI

extern "C" {
    pub fn dpi_uninit_port(port: *mut device_node);
}

// DISPC
extern "C" {
    pub fn dispc_dump_clocks(dispc: *mut dispc_device, s: *mut seq_file);
}
extern "C" {
    pub fn dispc_runtime_get(dispc: *mut dispc_device) -> c_int;
}
extern "C" {
    pub fn dispc_runtime_put(dispc: *mut dispc_device);
}
extern "C" {
    pub fn dispc_get_num_ovls(dispc: *mut dispc_device) -> c_int;
}
extern "C" {
    pub fn dispc_get_num_mgrs(dispc: *mut dispc_device) -> c_int;
}
extern "C" {
    pub fn dispc_ovl_get_max_size(dispc: *mut dispc_device, width: *mut u16, height: *mut u16);
}
extern "C" {
    pub fn dispc_ovl_get_caps(dispc: *mut dispc_device, plane: omap_plane_id) -> omap_overlay_caps;
}
extern "C" {
    pub fn dispc_read_irqstatus(dispc: *mut dispc_device) -> u32;
}
extern "C" {
    pub fn dispc_clear_irqstatus(dispc: *mut dispc_device, mask: u32);
}
extern "C" {
    pub fn dispc_write_irqenable(dispc: *mut dispc_device, mask: u32);
}
extern "C" {
    pub fn dispc_free_irq(dispc: *mut dispc_device, dev_id: *mut c_void);
}
extern "C" {
    pub fn dispc_get_memory_bandwidth_limit(dispc: *mut dispc_device) -> u32;
}
extern "C" {
    pub fn dispc_mgr_go(dispc: *mut dispc_device, channel: omap_channel);
}
extern "C" {
    pub fn dispc_enable_sidle(dispc: *mut dispc_device);
}
extern "C" {
    pub fn dispc_disable_sidle(dispc: *mut dispc_device);
}
extern "C" {
    pub fn dispc_lcd_enable_signal(dispc: *mut dispc_device, enable: bool);
}
extern "C" {
    pub fn dispc_pck_free_enable(dispc: *mut dispc_device, enable: bool);
}
extern "C" {
    pub fn dispc_set_tv_pclk(dispc: *mut dispc_device, pclk: c_ulong);
}

// PLL
extern "C" {
    pub fn dss_pll_register(dss: *mut dss_device, pll: *mut dss_pll) -> c_int;
}
extern "C" {
    pub fn dss_pll_unregister(pll: *mut dss_pll);
}
extern "C" {
    pub fn dss_pll_get_clkout_idx_for_src(src: dss_clk_source) -> c_uint;
}
extern "C" {
    pub fn dss_pll_enable(pll: *mut dss_pll) -> c_int;
}
extern "C" {
    pub fn dss_pll_disable(pll: *mut dss_pll);
}
extern "C" {
    pub fn dss_pll_wait_reset_done(pll: *mut dss_pll) -> c_int;
}

