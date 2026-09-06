//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sun4i/sun8i_dw_hdmi.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2018 Jernej Skrabec <jernej.skrabec@siol.net>
//

pub const SUN8I_HDMI_PHY_DBG_CTRL_REG: c_uint = 0x0000;

pub const SUN8I_HDMI_PHY_REXT_CTRL_REG: c_uint = 0x0004;

pub const SUN8I_HDMI_PHY_READ_EN_REG: c_uint = 0x0010;
pub const SUN8I_HDMI_PHY_READ_EN_MAGIC: c_uint = 0x54524545;
pub const SUN8I_HDMI_PHY_UNSCRAMBLE_REG: c_uint = 0x0014;
pub const SUN8I_HDMI_PHY_UNSCRAMBLE_MAGIC: c_uint = 0x42494E47;
pub const SUN8I_HDMI_PHY_ANA_CFG1_REG: c_uint = 0x0020;

pub const SUN8I_HDMI_PHY_ANA_CFG2_REG: c_uint = 0x0024;

pub const SUN8I_HDMI_PHY_ANA_CFG3_REG: c_uint = 0x0028;

pub const SUN8I_HDMI_PHY_PLL_CFG1_REG: c_uint = 0x002c;

pub const SUN8I_HDMI_PHY_PLL_CFG1_CKIN_SEL_SHIFT: c_int = 26;

pub const SUN8I_HDMI_PHY_PLL_CFG1_B_IN_SHIFT: c_int = 0;
pub const SUN8I_HDMI_PHY_PLL_CFG2_REG: c_uint = 0x0030;

pub const SUN8I_HDMI_PHY_PLL_CFG2_PREDIV_SHIFT: c_int = 0;

pub const SUN8I_HDMI_PHY_PLL_CFG3_REG: c_uint = 0x0034;

pub const SUN8I_HDMI_PHY_ANA_STS_REG: c_uint = 0x0038;
pub const SUN8I_HDMI_PHY_ANA_STS_B_OUT_SHIFT: c_int = 11;

pub const SUN8I_HDMI_PHY_CEC_REG: c_uint = 0x003c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_hdmi_phy_variant {
    pub has_phy_clk: bool,
    pub has_second_pll: bool,
    pub cur_ctr: *const dw_hdmi_curr_ctrl,
    pub mpll_cfg: *const dw_hdmi_mpll_config,
    pub phy_cfg: *const dw_hdmi_phy_config,
    pub phy_ops: *const dw_hdmi_phy_ops,
    pub phy): *mut *mut void (phy_init)(struct sun8i_hdmi_phy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_hdmi_phy {
    pub clk_bus: *mut clk,
    pub clk_mod: *mut clk,
    pub clk_phy: *mut clk,
    pub clk_pll0: *mut clk,
    pub clk_pll1: *mut clk,
    pub dev: *mut device,
    pub rcal: c_uint,
    pub regs: *mut regmap,
    pub rst_phy: *mut reset_control,
    pub variant: *const sun8i_hdmi_phy_variant,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_dw_hdmi_quirks {
    pub mode): *const drm_display_mode,
    pub 1: unsigned int use_drm_infoframe :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_dw_hdmi {
    pub clk_tmds: *mut clk,
    pub dev: *mut device,
    pub hdmi: *mut dw_hdmi,
    pub encoder: drm_encoder,
    pub phy: *mut sun8i_hdmi_phy,
    pub plat_data: dw_hdmi_plat_data,
    pub regulator: *mut regulator,
    pub quirks: *const sun8i_dw_hdmi_quirks,
    pub rst_ctrl: *mut reset_control,
}

extern "C" {
    pub fn container_of(_arg: encoder, sun8i_dw_hdmi: struct, _arg: encoder) -> return;
}
extern "C" {
    pub fn sun8i_hdmi_phy_get(hdmi: *mut sun8i_dw_hdmi, node: *mut device_node) -> c_int;
}
extern "C" {
    pub fn sun8i_hdmi_phy_init(phy: *mut sun8i_hdmi_phy) -> c_int;
}
extern "C" {
    pub fn sun8i_hdmi_phy_deinit(phy: *mut sun8i_hdmi_phy);
}
