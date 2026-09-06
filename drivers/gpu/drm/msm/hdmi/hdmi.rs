//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/hdmi/hdmi.h
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
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_audio {
    pub enabled: bool,
    pub rate: c_int,
    pub channels: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi {
    pub dev: *mut drm_device,
    pub pdev: *mut platform_device,
    pub config: *const hdmi_platform_config,
// audio state:
    pub audio: hdmi_audio,
// video state:
    pub power_on: bool,
    pub hpd_enabled: bool,
    pub /: *mut *mut mutex state_mutex; / protects two booleans,
    pub pixclock: c_ulong,
    pub mmio: *mut void __iomem,
    pub qfprom_mmio: *mut void __iomem,
    pub mmio_phy_addr: phys_addr_t,
    pub pwr_regs: *mut regulator_bulk_data,
    pub pwr_clks: *mut clk_bulk_data,
    pub extp_clk: *mut clk,
    pub hpd_gpiod: *mut gpio_desc,
    pub phy: *mut hdmi_phy,
    pub phy_dev: *mut device,
    pub i2c: *mut i2c_adapter,
    pub connector: *mut drm_connector,
    pub bridge: *mut drm_bridge,
    pub next_bridge: *mut drm_bridge,
// the encoder we are hooked to (outside of hdmi block)
    pub encoder: *mut drm_encoder,
    pub irq: c_int,
    pub workq: *mut workqueue_struct,
    pub hdcp_ctrl: *mut hdmi_hdcp_ctrl,
//
// spinlock to protect registers shared by different execution
// REG_HDMI_CTRL
// REG_HDMI_DDC_ARBITRATION
// REG_HDMI_HDCP_INT_CTRL
// REG_HDMI_HPD_CTRL
//
    pub reg_lock: spinlock_t,
}

// platform config data (ie. from DT, or pdata)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_platform_config {
// regulators that need to be on for screen pwr:
    pub pwr_reg_names: *const *const c_char,
    pub pwr_reg_cnt: c_int,
// clks that need to be on:
    pub pwr_clk_names: *const *const c_char,
    pub pwr_clk_cnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_bridge {
    pub base: drm_bridge,
    pub hdmi: *mut hdmi,
    pub hpd_work: work_struct,
}

extern "C" {
    pub fn msm_hdmi_set_mode(hdmi: *mut hdmi, power_on: bool);
}
extern "C" {
    pub fn readl(reg: hdmi->mmio +) -> return;
}
extern "C" {
    pub fn readl(reg: hdmi->qfprom_mmio +) -> return;
}
//
// hdmi phy:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_phy_type {
    MSM_HDMI_PHY_8x60,
    MSM_HDMI_PHY_8960,
    MSM_HDMI_PHY_8x74,
    MSM_HDMI_PHY_8996,
    MSM_HDMI_PHY_8998,
    MSM_HDMI_PHY_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_phy_cfg {
    pub type: hdmi_phy_type,
    pub pixclock): *mut *mut *mut void (powerup)(struct hdmi_phy phy, unsigned long,
    pub phy): *mut *mut void (powerdown)(struct hdmi_phy,
    pub reg_names: *const *const c_char,
    pub num_regs: c_int,
    pub clk_names: *const *const c_char,
    pub num_clks: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_phy {
    pub pdev: *mut platform_device,
    pub mmio: *mut void __iomem,
    pub cfg: *mut hdmi_phy_cfg,
    pub funcs: *const hdmi_phy_funcs,
    pub regs: *mut regulator_bulk_data,
    pub clks: *mut clk,
}

extern "C" {
    pub fn readl(reg: phy->mmio +) -> return;
}
extern "C" {
    pub fn msm_hdmi_phy_resource_enable(phy: *mut hdmi_phy) -> c_int;
}
extern "C" {
    pub fn msm_hdmi_phy_resource_disable(phy: *mut hdmi_phy);
}
extern "C" {
    pub fn msm_hdmi_phy_powerup(phy: *mut hdmi_phy, pixclock: c_ulong);
}
extern "C" {
    pub fn msm_hdmi_phy_powerdown(phy: *mut hdmi_phy);
}
extern "C" {
    pub fn msm_hdmi_phy_driver_register() -> void __init;
}
extern "C" {
    pub fn msm_hdmi_phy_driver_unregister() -> void __exit;
}

extern "C" {
    pub fn msm_hdmi_pll_8960_init(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn msm_hdmi_pll_8996_init(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn msm_hdmi_pll_8998_init(pdev: *mut platform_device) -> c_int;
}

//
// audio:
//
extern "C" {
    pub fn msm_hdmi_audio_update(hdmi: *mut hdmi) -> c_int;
}
//
// hdmi bridge:
//
extern "C" {
    pub fn msm_hdmi_bridge_init(hdmi: *mut hdmi) -> c_int;
}
extern "C" {
    pub fn msm_hdmi_hpd_irq(bridge: *mut drm_bridge);
}
extern "C" {
    pub fn msm_hdmi_hpd_enable(bridge: *mut drm_bridge);
}
extern "C" {
    pub fn msm_hdmi_hpd_disable(bridge: *mut drm_bridge);
}
//
// i2c adapter for ddc:
//
extern "C" {
    pub fn msm_hdmi_i2c_irq(i2c: *mut i2c_adapter);
}
extern "C" {
    pub fn msm_hdmi_i2c_destroy(i2c: *mut i2c_adapter);
}
//
// hdcp
//

extern "C" {
    pub fn msm_hdmi_hdcp_destroy(hdmi: *mut hdmi);
}
extern "C" {
    pub fn msm_hdmi_hdcp_on(hdcp_ctrl: *mut hdmi_hdcp_ctrl);
}
extern "C" {
    pub fn msm_hdmi_hdcp_off(hdcp_ctrl: *mut hdmi_hdcp_ctrl);
}
extern "C" {
    pub fn msm_hdmi_hdcp_irq(hdcp_ctrl: *mut hdmi_hdcp_ctrl);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENXIO) -> return;
}

