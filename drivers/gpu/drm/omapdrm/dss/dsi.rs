//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/omapdrm/dss/dsi.h
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
// Copyright (C) 2016 Texas Instruments Incorporated - http://www.ti.com
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_reg {
    pub module: u16,
    pub idx: u16,
}

// DSI Protocol Engine
pub const DSI_PROTO: c_int = 0;
pub const DSI_PROTO_SZ: c_uint = 0x200;

// DSIPHY_SCP
pub const DSI_PHY: c_int = 1;
pub const DSI_PHY_OFFSET: c_uint = 0x200;
pub const DSI_PHY_SZ: c_uint = 0x40;

// DSI_PLL_CTRL_SCP
pub const DSI_PLL: c_int = 2;
pub const DSI_PLL_OFFSET: c_uint = 0x300;
pub const DSI_PLL_SZ: c_uint = 0x20;

// Global interrupts

pub const DSI_IRQ_CHANNEL_MASK: c_uint = 0xf;
// Virtual channel interrupts

// ComplexIO interrupts

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_dsi_mode {
    OMAP_DSS_DSI_CMD_MODE = 0,
    OMAP_DSS_DSI_VIDEO_MODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_dsi_trans_mode {
// Sync Pulses: both sync start and end packets sent
    OMAP_DSS_DSI_PULSE_MODE,
// Sync Events: only sync start packets sent
    OMAP_DSS_DSI_EVENT_MODE,
// Burst: only sync start packets sent, pixels are time compressed
    OMAP_DSS_DSI_BURST_MODE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dss_dsi_videomode_timings {
    pub hsclk: c_ulong,
    pub ndl: c_uint,
    pub bitspp: c_uint,
// pixels
    pub hact: u16,
// lines
    pub vact: u16,
// DSI video mode blanking data
// Unit: byte clock cycles
    pub hss: u16,
    pub hsa: u16,
    pub hse: u16,
    pub hfp: u16,
    pub hbp: u16,
// Unit: line clocks
    pub vsa: u16,
    pub vfp: u16,
    pub vbp: u16,
// DSI blanking modes
    pub blanking_mode: c_int,
    pub hsa_blanking_mode: c_int,
    pub hbp_blanking_mode: c_int,
    pub hfp_blanking_mode: c_int,
    pub trans_mode: omap_dss_dsi_trans_mode,
    pub window_sync: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dss_dsi_config {
    pub mode: omap_dss_dsi_mode,
    pub pixel_format: mipi_dsi_pixel_format,
    pub vm: *const videomode,
    pub hs_clk_max: unsigned long hs_clk_min,,
    pub lp_clk_max: unsigned long lp_clk_min,,
    pub trans_mode: omap_dss_dsi_trans_mode,
}

// DSI PLL HSDIV indices
pub const HSDIV_DISPC: c_int = 0;
pub const HSDIV_DSI: c_int = 1;
pub const DSI_MAX_NR_ISRS: c_int = 2;
pub const DSI_MAX_NR_LANES: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsi_model {
    DSI_MODEL_OMAP3,
    DSI_MODEL_OMAP4,
    DSI_MODEL_OMAP5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsi_lane_function {
    DSI_LANE_UNUSED	= 0,
    DSI_LANE_CLK,
    DSI_LANE_DATA1,
    DSI_LANE_DATA2,
    DSI_LANE_DATA3,
    DSI_LANE_DATA4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_lane_config {
    pub function: dsi_lane_function,
    pub polarity: u8,
}

extern "C" {
    pub fn void(arg: *mut *mut omap_dsi_isr_t) (void, mask: u32) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_isr_data {
    pub isr: omap_dsi_isr_t,
    pub arg: *mut c_void,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fifo_size {
    DSI_FIFO_SIZE_0		= 0,
    DSI_FIFO_SIZE_32	= 1,
    DSI_FIFO_SIZE_64	= 2,
    DSI_FIFO_SIZE_96	= 3,
    DSI_FIFO_SIZE_128	= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsi_vc_source {
    DSI_VC_SOURCE_L4 = 0,
    DSI_VC_SOURCE_VP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_irq_stats {
    pub last_reset: c_ulong,
    pub irq_count: c_uint,
    pub dsi_irqs: [c_uint; 32],
    pub vc_irqs: [c_uint; 4][32],
    pub cio_irqs: [c_uint; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_isr_tables {
    pub isr_table: [dsi_isr_data; DSI_MAX_NR_ISRS],
    pub isr_table_vc: [dsi_isr_data; 4][DSI_MAX_NR_ISRS],
    pub isr_table_cio: [dsi_isr_data; DSI_MAX_NR_ISRS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_lp_clock_info {
    pub lp_clk: c_ulong,
    pub lp_clk_div: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_clk_calc_ctx {
    pub dsi: *mut dsi_data,
    pub pll: *mut dss_pll,
// inputs
    pub config: *const omap_dss_dsi_config,
    pub req_pck_max: unsigned long req_pck_min, req_pck_nom,,
// outputs
    pub dsi_cinfo: dss_pll_clock_info,
    pub dispc_cinfo: dispc_clock_info,
    pub lp_cinfo: dsi_lp_clock_info,
    pub vm: videomode,
    pub dsi_vm: omap_dss_dsi_videomode_timings,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_module_id_data {
    pub address: u32,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsi_quirks {
    DSI_QUIRK_PLL_PWR_BUG = (1 << 0),	/* DSI-PLL power command 0x3 is not working */
    DSI_QUIRK_DCS_CMD_CONFIG_VC = (1 << 1),
    DSI_QUIRK_VC_OCP_WIDTH = (1 << 2),
    DSI_QUIRK_REVERSE_TXCLKESC = (1 << 3),
    DSI_QUIRK_GNQ = (1 << 4),
    DSI_QUIRK_PHY_DCC = (1 << 5),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_of_data {
    pub model: dsi_model,
    pub pll_hw: *const dss_pll_hw,
    pub modules: *const dsi_module_id_data,
    pub max_fck_freq: c_uint,
    pub max_pll_lpdiv: c_uint,
    pub quirks: dsi_quirks,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_data {
    pub dev: *mut device,
    pub proto_base: *mut void __iomem,
    pub phy_base: *mut void __iomem,
    pub pll_base: *mut void __iomem,
    pub data: *const dsi_of_data,
    pub module_id: c_int,
    pub irq: c_int,
    pub is_enabled: bool,
    pub dss_clk: *mut clk,
    pub syscon: *mut regmap,
    pub dss: *mut dss_device,
    pub host: mipi_dsi_host,
    pub user_dispc_cinfo: dispc_clock_info,
    pub user_dsi_cinfo: dss_pll_clock_info,
    pub user_lp_cinfo: dsi_lp_clock_info,
    pub current_lp_cinfo: dsi_lp_clock_info,
    pub pll: dss_pll,
    pub vdds_dsi_enabled: bool,
    pub vdds_dsi_reg: *mut regulator,
    pub dsidev: *mut mipi_dsi_device,
    pub source: dsi_vc_source,
    pub tx_fifo_size: fifo_size,
    pub rx_fifo_size: fifo_size,
    pub vc: [}; 4],
    pub lock: mutex,
    pub bus_lock: semaphore,
    pub irq_lock: spinlock_t,
    pub isr_tables: dsi_isr_tables,
    pub update_vc: c_int,

    pub update_bytes: c_uint,

// external TE GPIO
    pub te_gpio: *mut gpio_desc,
    pub te_irq: c_int,
    pub te_timeout_work: delayed_work,
    pub do_ext_te_update: core::sync::atomic::AtomicI32,
    pub te_enabled: bool,
    pub iface_enabled: bool,
    pub video_enabled: bool,
    pub framedone_timeout_work: delayed_work,

    pub te_timer: timer_list,

    pub cache_req_pck: c_ulong,
    pub cache_clk_freq: c_ulong,
    pub cache_cinfo: dss_pll_clock_info,
    pub errors: u32,
    pub errors_lock: spinlock_t,

    pub perf_setup_time: ktime_t,
    pub perf_start_time: ktime_t,

    pub debug_read: c_int,
    pub debug_write: c_int,
    pub irqs: *mut dss_debugfs_entry,
    pub regs: *mut dss_debugfs_entry,
    pub clks: *mut dss_debugfs_entry,
    pub debugfs: },

    pub irq_stats_lock: spinlock_t,
    pub irq_stats: dsi_irq_stats,

    pub num_lanes_supported: c_uint,
    pub line_buffer_size: c_uint,
    pub lanes: [dsi_lane_config; DSI_MAX_NR_LANES],
    pub num_lanes_used: c_uint,
    pub scp_clk_refcount: c_uint,
    pub config: omap_dss_dsi_config,
    pub mgr_config: dss_lcd_mgr_config,
    pub vm: videomode,
    pub pix_fmt: mipi_dsi_pixel_format,
    pub mode: omap_dss_dsi_mode,
    pub vm_timings: omap_dss_dsi_videomode_timings,
    pub output: omap_dss_device,
    pub bridge: drm_bridge,
    pub dsi_disable_work: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_packet_sent_handler_data {
    pub dsi: *mut dsi_data,
    pub completion: *mut completion,
}
