//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dsi_vbt_defs.h
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


// SPDX-License-Identifier: MIT
// Copyright © 2025 Intel Corporation

//
// MIPI Sequence Block definitions
//
// Note the VBT spec has AssertReset / DeassertReset swapped from their
// usual naming, we use the proper names here to avoid confusion when
// reading the code.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_seq {
    MIPI_SEQ_END = 0,
    MIPI_SEQ_DEASSERT_RESET,	/* Spec says MipiAssertResetPin */
    MIPI_SEQ_INIT_OTP,
    MIPI_SEQ_DISPLAY_ON,
    MIPI_SEQ_DISPLAY_OFF,
    MIPI_SEQ_ASSERT_RESET,		/* Spec says MipiDeassertResetPin */
    MIPI_SEQ_BACKLIGHT_ON,		/* sequence block v2+ */
    MIPI_SEQ_BACKLIGHT_OFF,		/* sequence block v2+ */
    MIPI_SEQ_TEAR_ON,		/* sequence block v2+ */
    MIPI_SEQ_TEAR_OFF,		/* sequence block v3+ */
    MIPI_SEQ_POWER_ON,		/* sequence block v3+ */
    MIPI_SEQ_POWER_OFF,		/* sequence block v3+ */
    MIPI_SEQ_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_seq_element {
    MIPI_SEQ_ELEM_END = 0,
    MIPI_SEQ_ELEM_SEND_PKT,
    MIPI_SEQ_ELEM_DELAY,
    MIPI_SEQ_ELEM_GPIO,
    MIPI_SEQ_ELEM_I2C,		/* sequence block v2+ */
    MIPI_SEQ_ELEM_SPI,		/* sequence block v3+ */
    MIPI_SEQ_ELEM_PMIC,		/* sequence block v3+ */
    MIPI_SEQ_ELEM_MAX
}

pub const MIPI_DSI_UNDEFINED_PANEL_ID: c_int = 0;
pub const MIPI_DSI_GENERIC_PANEL_ID: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_config {
    pub panel_id: u16,
// General Params
    pub enable_dithering:1: u32,
    pub rsvd1:1: u32,
    pub is_bridge:1: u32,
    pub panel_arch_type:2: u32,
    pub is_cmd_mode:1: u32,
pub const NON_BURST_SYNC_PULSE: c_uint = 0x1;
pub const NON_BURST_SYNC_EVENTS: c_uint = 0x2;
pub const BURST_MODE: c_uint = 0x3;
    pub video_transfer_mode:2: u32,
    pub cabc_supported:1: u32,
pub const PPS_BLC_PMIC: c_int = 0;
pub const PPS_BLC_SOC: c_int = 1;
    pub pwm_blc:1: u32,
pub const PIXEL_FORMAT_RGB565: c_uint = 0x1;
pub const PIXEL_FORMAT_RGB666: c_uint = 0x2;
pub const PIXEL_FORMAT_RGB666_LOOSELY_PACKED: c_uint = 0x3;
pub const PIXEL_FORMAT_RGB888: c_uint = 0x4;
    pub videomode_color_format:4: u32,
pub const ENABLE_ROTATION_0: c_uint = 0x0;
pub const ENABLE_ROTATION_90: c_uint = 0x1;
pub const ENABLE_ROTATION_180: c_uint = 0x2;
pub const ENABLE_ROTATION_270: c_uint = 0x3;
    pub rotation:2: u32,
    pub bta_disable:1: u32,
    pub rsvd2:15: u32,
    pub __packed: },
// Port Desc
pub const DUAL_LINK_NOT_SUPPORTED: c_int = 0;
pub const DUAL_LINK_FRONT_BACK: c_int = 1;
pub const DUAL_LINK_PIXEL_ALT: c_int = 2;
    pub dual_link:2: u16,
    pub lane_cnt:2: u16,
    pub pixel_overlap:3: u16,
    pub rgb_flip:1: u16,
pub const DL_DCS_PORT_A: c_uint = 0x00;
pub const DL_DCS_PORT_C: c_uint = 0x01;
pub const DL_DCS_PORT_A_AND_C: c_uint = 0x02;
    pub dl_dcs_cabc_ports:2: u16,
    pub dl_dcs_backlight_ports:2: u16,
    pub /: *mut *mut u16 port_sync:1; / 219-230,
    pub rsvd3:3: u16,
    pub __packed: },
// DSI Controller Parameters
    pub dsi_usage:1: u16,
    pub rsvd4:15: u16,
    pub __packed: },
    pub rsvd5: u8,
    pub target_burst_mode_freq: u32,
    pub dsi_ddr_clk: u32,
    pub bridge_ref_clk: u32,
// LP Byte Clock
pub const BYTE_CLK_SEL_20MHZ: c_int = 0;
pub const BYTE_CLK_SEL_10MHZ: c_int = 1;
pub const BYTE_CLK_SEL_5MHZ: c_int = 2;
    pub byte_clk_sel:2: u8,
    pub rsvd6:6: u8,
    pub __packed: },
// DPhy Flags
    pub dphy_param_valid:1: u16,
    pub eot_pkt_disabled:1: u16,
    pub enable_clk_stop:1: u16,
    pub /: *mut *mut u16 blanking_packets_during_bllp:1; / 219+,
    pub /: *mut *mut u16 lp_clock_during_lpm:1; / 219+,
    pub rsvd7:11: u16,
    pub __packed: },
    pub hs_tx_timeout: u32,
    pub lp_rx_timeout: u32,
    pub turn_around_timeout: u32,
    pub device_reset_timer: u32,
    pub master_init_timer: u32,
    pub dbi_bw_timer: u32,
    pub lp_byte_clk_val: u32,
// DPhy Params
    pub prepare_cnt:6: u32,
    pub rsvd8:2: u32,
    pub clk_zero_cnt:8: u32,
    pub trail_cnt:5: u32,
    pub rsvd9:3: u32,
    pub exit_zero_cnt:6: u32,
    pub rsvd10:2: u32,
    pub __packed: },
    pub clk_lane_switch_cnt: u32,
    pub hl_switch_cnt: u32,
    pub rsvd11: [u32; 6],
// timings based on dphy spec
    pub tclk_miss: u8,
    pub tclk_post: u8,
    pub rsvd12: u8,
    pub tclk_pre: u8,
    pub tclk_prepare: u8,
    pub tclk_settle: u8,
    pub tclk_term_enable: u8,
    pub tclk_trail: u8,
    pub tclk_prepare_clkzero: u16,
    pub rsvd13: u8,
    pub td_term_enable: u8,
    pub teot: u8,
    pub ths_exit: u8,
    pub ths_prepare: u8,
    pub ths_prepare_hszero: u16,
    pub rsvd14: u8,
    pub ths_settle: u8,
    pub ths_skip: u8,
    pub ths_trail: u8,
    pub tinit: u8,
    pub tlpx: u8,
    pub rsvd15: [u8; 3],
// GPIOs
    pub panel_enable: u8,
    pub bl_enable: u8,
    pub pwm_enable: u8,
    pub reset_r_n: u8,
    pub pwr_down_r: u8,
    pub stdby_r_n: u8,
    pub __packed: },
// all delays have a unit of 100us
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_pps_data {
    pub panel_on_delay: u16,
    pub bl_enable_delay: u16,
    pub bl_disable_delay: u16,
    pub panel_off_delay: u16,
    pub panel_power_cycle_delay: u16,
    pub __packed: },
