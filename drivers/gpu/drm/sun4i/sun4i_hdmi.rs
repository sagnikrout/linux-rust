//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sun4i/sun4i_hdmi.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2016 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

pub const SUN4I_HDMI_CTRL_REG: c_uint = 0x004;

pub const SUN4I_HDMI_IRQ_REG: c_uint = 0x008;
pub const SUN4I_HDMI_IRQ_STA_MASK: c_uint = 0x73;

pub const SUN4I_HDMI_HPD_REG: c_uint = 0x00c;

pub const SUN4I_HDMI_VID_CTRL_REG: c_uint = 0x010;

pub const SUN4I_HDMI_VID_TIMING_ACT_REG: c_uint = 0x014;
pub const SUN4I_HDMI_VID_TIMING_BP_REG: c_uint = 0x018;
pub const SUN4I_HDMI_VID_TIMING_FP_REG: c_uint = 0x01c;
pub const SUN4I_HDMI_VID_TIMING_SPW_REG: c_uint = 0x020;

pub const SUN4I_HDMI_VID_TIMING_POL_REG: c_uint = 0x024;

pub const SUN4I_HDMI_PAD_CTRL0_REG: c_uint = 0x200;

pub const SUN4I_HDMI_PAD_CTRL1_REG: c_uint = 0x204;

// These bits seem to invert the TMDS data channels

pub const SUN4I_HDMI_PLL_CTRL_REG: c_uint = 0x208;

pub const SUN4I_HDMI_PLL_DBG0_REG: c_uint = 0x20c;

pub const SUN4I_HDMI_PLL_DBG0_TMDS_PARENT_SHIFT: c_int = 21;
pub const SUN4I_HDMI_CEC: c_uint = 0x214;

pub const SUN4I_HDMI_UNKNOWN_REG: c_uint = 0x300;

pub const SUN4I_HDMI_DDC_CTRL_REG: c_uint = 0x500;

pub const SUN4I_HDMI_DDC_ADDR_REG: c_uint = 0x504;

pub const SUN4I_HDMI_DDC_INT_STATUS_REG: c_uint = 0x50c;

pub const SUN4I_HDMI_DDC_FIFO_CTRL_REG: c_uint = 0x510;

pub const SUN4I_HDMI_DDC_FIFO_DATA_REG: c_uint = 0x518;
pub const SUN4I_HDMI_DDC_BYTE_COUNT_REG: c_uint = 0x51c;

pub const SUN4I_HDMI_DDC_CMD_REG: c_uint = 0x520;
pub const SUN4I_HDMI_DDC_CMD_EXPLICIT_EDDC_READ: c_int = 6;
pub const SUN4I_HDMI_DDC_CMD_IMPLICIT_READ: c_int = 5;
pub const SUN4I_HDMI_DDC_CMD_IMPLICIT_WRITE: c_int = 3;
pub const SUN4I_HDMI_DDC_CLK_REG: c_uint = 0x528;

pub const SUN4I_HDMI_DDC_LINE_CTRL_REG: c_uint = 0x540;

pub const SUN4I_HDMI_DDC_FIFO_SIZE: c_int = 16;
// A31 specific
pub const SUN6I_HDMI_DDC_CTRL_REG: c_uint = 0x500;

pub const SUN6I_HDMI_DDC_CMD_REG: c_uint = 0x508;

// command types in lower 3 bits are the same as sun4i
pub const SUN6I_HDMI_DDC_ADDR_REG: c_uint = 0x50c;

pub const SUN6I_HDMI_DDC_INT_STATUS_REG: c_uint = 0x514;

// lower 8 bits are the same as sun4i
pub const SUN6I_HDMI_DDC_FIFO_CTRL_REG: c_uint = 0x518;

// lower 9 bits are the same as sun4i
pub const SUN6I_HDMI_DDC_CLK_REG: c_uint = 0x520;
// DDC CLK bit fields are the same, but the formula is not
pub const SUN6I_HDMI_DDC_FIFO_DATA_REG: c_uint = 0x580;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sun4i_hdmi_pkt_type {
    SUN4I_HDMI_PKT_AVI = 2,
    SUN4I_HDMI_PKT_END = 15,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_hdmi_variant {
    pub has_ddc_parent_clk: bool,
    pub has_reset_control: bool,
    pub pad_ctrl0_init_val: u32,
    pub pad_ctrl1_init_val: u32,
    pub pll_ctrl_init_val: u32,
    pub ddc_clk_reg: reg_field,
    pub ddc_clk_pre_divider: u8,
    pub ddc_clk_m_offset: u8,
    pub tmds_clk_div_offset: u8,
// Register fields for I2C adapter
    pub field_ddc_en: reg_field,
    pub field_ddc_start: reg_field,
    pub field_ddc_reset: reg_field,
    pub field_ddc_addr_reg: reg_field,
    pub field_ddc_slave_addr: reg_field,
    pub field_ddc_int_mask: reg_field,
    pub field_ddc_int_status: reg_field,
    pub field_ddc_fifo_clear: reg_field,
    pub field_ddc_fifo_rx_thres: reg_field,
    pub field_ddc_fifo_tx_thres: reg_field,
    pub field_ddc_byte_count: reg_field,
    pub field_ddc_cmd: reg_field,
    pub field_ddc_sda_en: reg_field,
    pub field_ddc_sck_en: reg_field,
// DDC FIFO register offset
    pub ddc_fifo_reg: u32,
//
// DDC FIFO threshold boundary conditions
//
// This is used to cope with the threshold boundary condition
// being slightly different on sun5i and sun6i.
//
// On sun5i the threshold is exclusive, i.e. does not include,
// the value of the threshold. ( > for RX; < for TX )
// On sun6i the threshold is inclusive, i.e. includes, the
// value of the threshold. ( >= for RX; <= for TX )
//
    pub ddc_fifo_thres_incl: bool,
    pub ddc_fifo_has_dir: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_hdmi {
    pub connector: drm_connector,
    pub encoder: drm_encoder,
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub regmap: *mut regmap,
// Reset control
    pub reset: *mut reset_control,
// Parent clocks
    pub bus_clk: *mut clk,
    pub mod_clk: *mut clk,
    pub ddc_parent_clk: *mut clk,
    pub pll0_clk: *mut clk,
    pub pll1_clk: *mut clk,
// And the clocks we create
    pub ddc_clk: *mut clk,
    pub tmds_clk: *mut clk,
    pub i2c: *mut i2c_adapter,
    pub ddc_i2c: *mut i2c_adapter,
// Regmap fields for I2C adapter
    pub field_ddc_en: *mut regmap_field,
    pub field_ddc_start: *mut regmap_field,
    pub field_ddc_reset: *mut regmap_field,
    pub field_ddc_addr_reg: *mut regmap_field,
    pub field_ddc_slave_addr: *mut regmap_field,
    pub field_ddc_int_mask: *mut regmap_field,
    pub field_ddc_int_status: *mut regmap_field,
    pub field_ddc_fifo_clear: *mut regmap_field,
    pub field_ddc_fifo_rx_thres: *mut regmap_field,
    pub field_ddc_fifo_tx_thres: *mut regmap_field,
    pub field_ddc_byte_count: *mut regmap_field,
    pub field_ddc_cmd: *mut regmap_field,
    pub field_ddc_sda_en: *mut regmap_field,
    pub field_ddc_sck_en: *mut regmap_field,
    pub drv: *mut sun4i_drv,
    pub cec_adap: *mut cec_adapter,
    pub variant: *const sun4i_hdmi_variant,
}

extern "C" {
    pub fn sun4i_ddc_create(hdmi: *mut sun4i_hdmi, clk: *mut clk) -> c_int;
}
extern "C" {
    pub fn sun4i_tmds_create(hdmi: *mut sun4i_hdmi) -> c_int;
}
extern "C" {
    pub fn sun4i_hdmi_i2c_create(dev: *mut device, hdmi: *mut sun4i_hdmi) -> c_int;
}
