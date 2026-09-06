//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/omap2/omapfb/dss/dss.h
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
// linux/drivers/video/omap2/dss/dss.h
//
// Copyright (C) 2009 Nokia Corporation
// Author: Tomi Valkeinen <tomi.valkeinen@nokia.com>
//
// Some code and ideas taken from drivers/video/omap/ driver
// by Imre Deak.
//

// OMAP TRM gives bitfields as start:end, where start is the higher bit

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_clk_source {
    OMAP_DSS_CLK_SRC_FCK = 0,		/* OMAP2/3: DSS1_ALWON_FCLK
// OMAP4: DSS_FCLK
    OMAP_DSS_CLK_SRC_DSI_PLL_HSDIV_DISPC,	/* OMAP3: DSI1_PLL_FCLK
// OMAP4: PLL1_CLK1
    OMAP_DSS_CLK_SRC_DSI_PLL_HSDIV_DSI,	/* OMAP3: DSI2_PLL_FCLK
// OMAP4: PLL1_CLK2
    OMAP_DSS_CLK_SRC_DSI2_PLL_HSDIV_DISPC,	/* OMAP4: PLL2_CLK1 */
    OMAP_DSS_CLK_SRC_DSI2_PLL_HSDIV_DSI,	/* OMAP4: PLL2_CLK2 */
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
pub enum dss_pll_id {
    DSS_PLL_DSI1,
    DSS_PLL_DSI2,
    DSS_PLL_HDMI,
    DSS_PLL_VIDEO1,
    DSS_PLL_VIDEO2,
}

pub const DSS_PLL_MAX_HSDIVS: c_int = 4;
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
    pub n_max: unsigned,
    pub m_min: unsigned,
    pub m_max: unsigned,
    pub mX_max: unsigned,
    pub fint_max: unsigned long fint_min,,
    pub clkdco_max: unsigned long clkdco_min, clkdco_low,,
    pub n_lsb: u8 n_msb,,
    pub m_lsb: u8 m_msb,,
    pub mX_lsb: [u8 mX_msb[DSS_PLL_MAX_HSDIVS],; DSS_PLL_MAX_HSDIVS],
    pub has_stopmode: bool,
    pub has_freqsel: bool,
    pub has_selfreqdco: bool,
    pub has_refsel: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dss_pll {
    pub name: *const c_char,
    pub id: dss_pll_id,
    pub clkin: *mut clk,
    pub regulator: *mut regulator,
    pub base: *mut void __iomem,
    pub hw: *const dss_pll_hw,
    pub ops: *const dss_pll_ops,
    pub cinfo: dss_pll_clock_info,
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

// core
extern "C" {
    pub fn dss_dsi_enable_pads(dsi_id: c_int, lane_mask: unsigned) -> c_int;
}
extern "C" {
    pub fn dss_dsi_disable_pads(dsi_id: c_int, lane_mask: unsigned);
}
extern "C" {
    pub fn dss_set_min_bus_tput(dev: *mut device, tput: c_ulong) -> c_int;
}
extern "C" {
    pub fn dss_debugfs_create_file(name: *const c_char, ): *mut *mut void (write)(struct seq_file);
}
// display
extern "C" {
    pub fn dss_suspend_all_devices() -> c_int;
}
extern "C" {
    pub fn dss_resume_all_devices() -> c_int;
}
extern "C" {
    pub fn dss_disable_all_devices();
}
extern "C" {
    pub fn display_init_sysfs(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn display_uninit_sysfs(pdev: *mut platform_device);
}
// manager
extern "C" {
    pub fn dss_init_overlay_managers() -> c_int;
}
extern "C" {
    pub fn dss_uninit_overlay_managers();
}
extern "C" {
    pub fn dss_init_overlay_managers_sysfs(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn dss_uninit_overlay_managers_sysfs(pdev: *mut platform_device);
}
extern "C" {
    pub fn dss_manager_kobj_uninit(mgr: *mut omap_overlay_manager);
}
// overlay
extern "C" {
    pub fn dss_init_overlays(pdev: *mut platform_device);
}
extern "C" {
    pub fn dss_uninit_overlays(pdev: *mut platform_device);
}
extern "C" {
    pub fn dss_overlay_setup_dispc_manager(mgr: *mut omap_overlay_manager);
}
extern "C" {
    pub fn dss_overlay_kobj_uninit(ovl: *mut omap_overlay);
}
// DSS
extern "C" {
    pub fn dss_uninit_platform_driver();
}
extern "C" {
    pub fn dss_runtime_get() -> c_int;
}
extern "C" {
    pub fn dss_runtime_put();
}
extern "C" {
    pub fn dss_get_dispc_clk_rate() -> c_ulong;
}
extern "C" {
    pub fn dss_dpi_select_source(port: c_int, channel: omap_channel) -> c_int;
}
extern "C" {
    pub fn dss_select_hdmi_venc_clk_source(dss_hdmi_venc_clk_source_select: enum);
}
extern "C" {
    pub fn dss_get_hdmi_venc_clk_source() -> dss_hdmi_venc_clk_source_select;
}
extern "C" {
    pub fn dss_dump_clocks(s: *mut seq_file);
}
// DSS VIDEO PLL
extern "C" {
    pub fn dss_video_pll_uninit(pll: *mut dss_pll);
}
// dss-of
extern "C" {
    pub fn dss_of_port_get_port_number(port: *mut device_node) -> u32;
}

extern "C" {
    pub fn dss_debug_dump_clocks(s: *mut seq_file);
}

extern "C" {
    pub fn dss_ctrl_pll_enable(pll_id: dss_pll_id, enable: bool);
}
extern "C" {
    pub fn dss_sdi_init(datapairs: c_int);
}
extern "C" {
    pub fn dss_sdi_enable() -> c_int;
}
extern "C" {
    pub fn dss_sdi_disable();
}
extern "C" {
    pub fn dss_get_dispc_clk_source() -> omap_dss_clk_source;
}
extern "C" {
    pub fn dss_get_dsi_clk_source(dsi_module: c_int) -> omap_dss_clk_source;
}
extern "C" {
    pub fn dss_get_lcd_clk_source(channel: omap_channel) -> omap_dss_clk_source;
}
extern "C" {
    pub fn dss_set_venc_output(type: omap_dss_venc_type);
}
extern "C" {
    pub fn dss_set_dac_pwrdn_bgz(enable: bool);
}
extern "C" {
    pub fn dss_set_fck_rate(rate: c_ulong) -> c_int;
}
extern "C" {
    pub fn bool(fck: *mut *mut dss_div_calc_func)(unsigned long, data: *mut c_void) -> typedef;
}
// SDI
extern "C" {
    pub fn sdi_uninit_platform_driver();
}

extern "C" {
    pub fn sdi_init_port(pdev: *mut platform_device, port: *mut device_node) -> c_int;
}
extern "C" {
    pub fn sdi_uninit_port(port: *mut device_node);
}

// DSI

extern "C" {
    pub fn dsi_uninit_platform_driver();
}
extern "C" {
    pub fn dsi_dump_clocks(s: *mut seq_file);
}
extern "C" {
    pub fn dsi_irq_handler();
}
extern "C" {
    pub fn dsi_get_pixel_size(fmt: omap_dss_dsi_pixel_format) -> u8;
}

// DPI
extern "C" {
    pub fn dpi_uninit_platform_driver();
}

extern "C" {
    pub fn dpi_init_port(pdev: *mut platform_device, port: *mut device_node) -> c_int;
}
extern "C" {
    pub fn dpi_uninit_port(port: *mut device_node);
}

// DISPC
extern "C" {
    pub fn dispc_uninit_platform_driver();
}
extern "C" {
    pub fn dispc_dump_clocks(s: *mut seq_file);
}
extern "C" {
    pub fn dispc_enable_sidle();
}
extern "C" {
    pub fn dispc_disable_sidle();
}
extern "C" {
    pub fn dispc_lcd_enable_signal(enable: bool);
}
extern "C" {
    pub fn dispc_pck_free_enable(enable: bool);
}
extern "C" {
    pub fn dispc_enable_gamma_table(enable: bool);
}
extern "C" {
    pub fn dispc_ovl_set_fifo_threshold(plane: omap_plane, low: u32, high: u32);
}
extern "C" {
    pub fn dispc_set_tv_pclk(pclk: c_ulong);
}
extern "C" {
    pub fn dispc_read_irqstatus() -> u32;
}
extern "C" {
    pub fn dispc_clear_irqstatus(mask: u32);
}
extern "C" {
    pub fn dispc_read_irqenable() -> u32;
}
extern "C" {
    pub fn dispc_write_irqenable(mask: u32);
}
extern "C" {
    pub fn dispc_request_irq(handler: irq_handler_t, dev_id: *mut c_void) -> c_int;
}
extern "C" {
    pub fn dispc_free_irq(dev_id: *mut c_void);
}
extern "C" {
    pub fn dispc_runtime_get() -> c_int;
}
extern "C" {
    pub fn dispc_runtime_put();
}
extern "C" {
    pub fn dispc_mgr_enable(channel: omap_channel, enable: bool);
}
extern "C" {
    pub fn dispc_mgr_is_enabled(channel: omap_channel) -> bool;
}
extern "C" {
    pub fn dispc_mgr_get_vsync_irq(channel: omap_channel) -> u32;
}
extern "C" {
    pub fn dispc_mgr_get_framedone_irq(channel: omap_channel) -> u32;
}
extern "C" {
    pub fn dispc_mgr_get_sync_lost_irq(channel: omap_channel) -> u32;
}
extern "C" {
    pub fn dispc_mgr_go_busy(channel: omap_channel) -> bool;
}
extern "C" {
    pub fn dispc_mgr_go(channel: omap_channel);
}
extern "C" {
    pub fn dispc_ovl_enable(plane: omap_plane, enable: bool) -> c_int;
}
extern "C" {
    pub fn dispc_ovl_enabled(plane: omap_plane) -> bool;
}
// VENC
extern "C" {
    pub fn venc_uninit_platform_driver();
}
// HDMI
extern "C" {
    pub fn hdmi4_uninit_platform_driver();
}
extern "C" {
    pub fn hdmi5_uninit_platform_driver();
}

// PLL
extern "C" {
    pub fn dss_pll_register(pll: *mut dss_pll) -> c_int;
}
extern "C" {
    pub fn dss_pll_unregister(pll: *mut dss_pll);
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
// compat
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dss_mgr_ops {
    pub dst): *mut omap_dss_device,
    pub dst): *mut omap_dss_device,
    pub mgr): *mut *mut void (start_update)(struct omap_overlay_manager,
    pub mgr): *mut *mut int (enable)(struct omap_overlay_manager,
    pub mgr): *mut *mut void (disable)(struct omap_overlay_manager,
    pub timings): *const omap_video_timings,
    pub config): *const dss_lcd_mgr_config,
    pub data): *mut *mut *mut void (handler)(void ), void,
    pub data): *mut *mut *mut void (handler)(void ), void,
}

extern "C" {
    pub fn dss_install_mgr_ops(mgr_ops: *const dss_mgr_ops) -> c_int;
}
extern "C" {
    pub fn dss_uninstall_mgr_ops();
}
extern "C" {
    pub fn dss_mgr_enable(mgr: *mut omap_overlay_manager) -> c_int;
}
extern "C" {
    pub fn dss_mgr_disable(mgr: *mut omap_overlay_manager);
}
extern "C" {
    pub fn dss_mgr_start_update(mgr: *mut omap_overlay_manager);
}
