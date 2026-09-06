//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/tegra/xusb.h
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
// Copyright (c) 2014-2022, NVIDIA CORPORATION.  All rights reserved.
// Copyright (c) 2015, Google Inc.
//

// legacy entry points for backwards-compatibility
extern "C" {
    pub fn tegra_xusb_padctl_legacy_probe(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn tegra_xusb_padctl_legacy_remove(pdev: *mut platform_device) -> c_int;
}
//
// lanes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_lane_soc {
    pub name: *const c_char,
    pub offset: c_uint,
    pub shift: c_uint,
    pub mask: c_uint,
    pub funcs: *const *const c_char,
    pub num_funcs: c_uint,
    pub misc_ctl2: c_uint,
    pub regs: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_lane {
    pub soc: *const tegra_xusb_lane_soc,
    pub pad: *mut tegra_xusb_pad,
    pub np: *mut device_node,
    pub list: list_head,
    pub function: c_uint,
    pub index: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_usb3_lane {
    pub base: tegra_xusb_lane,
}

extern "C" {
    pub fn container_of(_arg: lane, tegra_xusb_usb3_lane: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_usb2_lane {
    pub base: tegra_xusb_lane,
    pub hs_curr_level_offset: u32,
}

extern "C" {
    pub fn container_of(_arg: lane, tegra_xusb_usb2_lane: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_ulpi_lane {
    pub base: tegra_xusb_lane,
}

extern "C" {
    pub fn container_of(_arg: lane, tegra_xusb_ulpi_lane: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_hsic_lane {
    pub base: tegra_xusb_lane,
    pub strobe_trim: u32,
    pub rx_strobe_trim: u32,
    pub rx_data_trim: u32,
    pub tx_rtune_n: u32,
    pub tx_rtune_p: u32,
    pub tx_rslew_n: u32,
    pub tx_rslew_p: u32,
    pub auto_term: bool,
}

extern "C" {
    pub fn container_of(_arg: lane, tegra_xusb_hsic_lane: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_pcie_lane {
    pub base: tegra_xusb_lane,
}

extern "C" {
    pub fn container_of(_arg: lane, tegra_xusb_pcie_lane: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_sata_lane {
    pub base: tegra_xusb_lane,
}

extern "C" {
    pub fn container_of(_arg: lane, tegra_xusb_sata_lane: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_lane_ops {
    pub index): c_uint,
    pub lane): *mut *mut void (remove)(struct tegra_xusb_lane,
    pub lane): *mut *mut void (iddq_enable)(struct tegra_xusb_lane,
    pub lane): *mut *mut void (iddq_disable)(struct tegra_xusb_lane,
    pub speed): *mut *mut *mut int (enable_phy_sleepwalk)(struct tegra_xusb_lane lane, enum usb_device_speed,
    pub lane): *mut *mut int (disable_phy_sleepwalk)(struct tegra_xusb_lane,
    pub lane): *mut *mut int (enable_phy_wake)(struct tegra_xusb_lane,
    pub lane): *mut *mut int (disable_phy_wake)(struct tegra_xusb_lane,
    pub lane): *mut *mut bool (remote_wake_detected)(struct tegra_xusb_lane,
}

extern "C" {
    pub fn tegra_xusb_lane_check(lane: *mut tegra_xusb_lane, function: *const c_char) -> bool;
}
//
// pads
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_pad_ops {
    pub np): *mut device_node,
    pub pad): *mut *mut void (remove)(struct tegra_xusb_pad,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_pad_soc {
    pub name: *const c_char,
    pub lanes: *const tegra_xusb_lane_soc,
    pub num_lanes: c_uint,
    pub ops: *const tegra_xusb_pad_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_pad {
    pub soc: *const tegra_xusb_pad_soc,
    pub padctl: *mut tegra_xusb_padctl,
    pub provider: *mut phy_provider,
    pub lanes: *mut phy,
    pub dev: device,
    pub ops: *const tegra_xusb_lane_ops,
    pub list: list_head,
}

extern "C" {
    pub fn container_of(_arg: dev, tegra_xusb_pad: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn tegra_xusb_pad_unregister(pad: *mut tegra_xusb_pad);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_usb3_pad {
    pub base: tegra_xusb_pad,
    pub enable: c_uint,
    pub lock: mutex,
}

extern "C" {
    pub fn container_of(_arg: pad, tegra_xusb_usb3_pad: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_usb2_pad {
    pub base: tegra_xusb_pad,
    pub clk: *mut clk,
    pub enable: c_uint,
    pub lock: mutex,
}

extern "C" {
    pub fn container_of(_arg: pad, tegra_xusb_usb2_pad: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_ulpi_pad {
    pub base: tegra_xusb_pad,
}

extern "C" {
    pub fn container_of(_arg: pad, tegra_xusb_ulpi_pad: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_hsic_pad {
    pub base: tegra_xusb_pad,
    pub supply: *mut regulator,
    pub clk: *mut clk,
}

extern "C" {
    pub fn container_of(_arg: pad, tegra_xusb_hsic_pad: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_pcie_pad {
    pub base: tegra_xusb_pad,
    pub rst: *mut reset_control,
    pub pll: *mut clk,
    pub enable: bool,
}

extern "C" {
    pub fn container_of(_arg: pad, tegra_xusb_pcie_pad: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_sata_pad {
    pub base: tegra_xusb_pad,
    pub rst: *mut reset_control,
    pub pll: *mut clk,
    pub enable: bool,
}

extern "C" {
    pub fn container_of(_arg: pad, tegra_xusb_sata_pad: struct, _arg: base) -> return;
}
//
// ports
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_port {
    pub padctl: *mut tegra_xusb_padctl,
    pub lane: *mut tegra_xusb_lane,
    pub index: c_uint,
    pub list: list_head,
    pub dev: device,
    pub usb_role_sw: *mut usb_role_switch,
    pub usb_phy_work: work_struct,
    pub usb_phy: usb_phy,
    pub ops: *const tegra_xusb_port_ops,
}

extern "C" {
    pub fn container_of(_arg: dev, tegra_xusb_port: struct, _arg: dev) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_lane_map {
    pub port: c_uint,
    pub type: *const c_char,
    pub index: c_uint,
    pub func: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_usb2_port {
    pub base: tegra_xusb_port,
    pub supply: *mut regulator,
    pub mode: usb_dr_mode,
    pub internal: bool,
    pub usb3_port_fake: c_int,
}

extern "C" {
    pub fn container_of(_arg: port, tegra_xusb_usb2_port: struct, _arg: base) -> return;
}
extern "C" {
    pub fn tegra_xusb_usb2_port_release(port: *mut tegra_xusb_port);
}
extern "C" {
    pub fn tegra_xusb_usb2_port_remove(port: *mut tegra_xusb_port);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_ulpi_port {
    pub base: tegra_xusb_port,
    pub supply: *mut regulator,
    pub internal: bool,
}

extern "C" {
    pub fn container_of(_arg: port, tegra_xusb_ulpi_port: struct, _arg: base) -> return;
}
extern "C" {
    pub fn tegra_xusb_ulpi_port_release(port: *mut tegra_xusb_port);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_hsic_port {
    pub base: tegra_xusb_port,
}

extern "C" {
    pub fn container_of(_arg: port, tegra_xusb_hsic_port: struct, _arg: base) -> return;
}
extern "C" {
    pub fn tegra_xusb_hsic_port_release(port: *mut tegra_xusb_port);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_usb3_port {
    pub base: tegra_xusb_port,
    pub context_saved: bool,
    pub port: c_uint,
    pub internal: bool,
    pub disable_gen2: bool,
    pub tap1: u32,
    pub amp: u32,
    pub ctle_z: u32,
    pub ctle_g: u32,
}

extern "C" {
    pub fn container_of(_arg: port, tegra_xusb_usb3_port: struct, _arg: base) -> return;
}
extern "C" {
    pub fn tegra_xusb_usb3_port_release(port: *mut tegra_xusb_port);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_port_ops {
    pub port): *mut *mut void (release)(struct tegra_xusb_port,
    pub port): *mut *mut void (remove)(struct tegra_xusb_port,
    pub port): *mut *mut int (enable)(struct tegra_xusb_port,
    pub port): *mut *mut void (disable)(struct tegra_xusb_port,
    pub port): *mut *mut *mut tegra_xusb_lane (map)(tegra_xusb_port,
}

//
// pad controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_padctl_ops {
    pub soc): *const tegra_xusb_padctl_soc,
    pub padctl): *mut *mut void (remove)(struct tegra_xusb_padctl,
    pub padctl): *mut *mut int (suspend_noirq)(struct tegra_xusb_padctl,
    pub padctl): *mut *mut int (resume_noirq)(struct tegra_xusb_padctl,
    pub index): c_uint,
    pub idle): unsigned int index, bool,
    pub enable): unsigned int index, bool,
    pub set): *mut *mut *mut int (vbus_override)(struct tegra_xusb_padctl padctl, bool,
    pub phy): *mut *mut int (utmi_port_reset)(struct phy,
    pub phy): *mut *mut void (utmi_pad_power_on)(struct phy,
    pub phy): *mut *mut void (utmi_pad_power_down)(struct phy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_padctl_soc {
    pub pads: *const *const tegra_xusb_pad_soc,
    pub num_pads: c_uint,
    pub ops: *const tegra_xusb_port_ops,
    pub count: c_uint,
    pub usb3: } usb2, ulpi, hsic,,
    pub ports: },
    pub ops: *const tegra_xusb_padctl_ops,
    pub supply_names: *const *const c_char,
    pub num_supplies: c_uint,
    pub supports_gen2: bool,
    pub need_fake_usb3_port: bool,
    pub poll_trk_completed: bool,
    pub trk_hw_mode: bool,
    pub trk_update_on_idle: bool,
    pub supports_lp_cfg_en: bool,
    pub has_per_pad_term: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_padctl {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub lock: mutex,
    pub rst: *mut reset_control,
    pub soc: *const tegra_xusb_padctl_soc,
    pub pcie: *mut tegra_xusb_pad,
    pub sata: *mut tegra_xusb_pad,
    pub ulpi: *mut tegra_xusb_pad,
    pub usb2: *mut tegra_xusb_pad,
    pub hsic: *mut tegra_xusb_pad,
    pub ports: list_head,
    pub lanes: list_head,
    pub pads: list_head,
    pub enable: c_uint,
    pub clk: *mut clk,
    pub supplies: *mut regulator_bulk_data,
}

