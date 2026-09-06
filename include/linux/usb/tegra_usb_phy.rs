//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/tegra_usb_phy.h
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
// Copyright (C) 2010 Google, Inc.
//

//
// utmi_pll_config_in_car_module: true if the UTMI PLL configuration registers
// should be set up by clk-tegra, false if by the PHY code
// has_hostpc: true if the USB controller has the HOSTPC extension, which
// changes the location of the PHCD and PTS fields
// requires_usbmode_setup: true if the USBMODE register needs to be set to
// enter host mode
// requires_extra_tuning_parameters: true if xcvr_hsslew, hssquelch_level
// and hsdiscon_level should be set for adequate signal quality
// requires_pmc_ao_power_up: true if USB AO is powered down by default
// uhsic_registers_offset: for Tegra30+ where HSIC registers were offset
// comparing to Tegra20 by 0x400, since Tegra20 has no UTMIP on PHY2
// uhsic_tx_rtune: fine tuned 50 Ohm termination resistor for NMOS/PMOS driver
// uhsic_pts_value: parallel transceiver select enumeration value
// portsc1_offset: register offset of PORTSC1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_phy_soc_config {
    pub utmi_pll_config_in_car_module: bool,
    pub has_hostpc: bool,
    pub requires_usbmode_setup: bool,
    pub requires_extra_tuning_parameters: bool,
    pub requires_pmc_ao_power_up: bool,
    pub uhsic_registers_offset: u32,
    pub uhsic_tx_rtune: u32,
    pub uhsic_pts_value: u32,
    pub portsc1_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_utmip_config {
    pub hssync_start_delay: u8,
    pub elastic_limit: u8,
    pub idle_wait_delay: u8,
    pub term_range_adj: u8,
    pub xcvr_setup_use_fuses: bool,
    pub xcvr_setup: u8,
    pub xcvr_lsfslew: u8,
    pub xcvr_lsrslew: u8,
    pub xcvr_hsslew: u8,
    pub hssquelch_level: u8,
    pub hsdiscon_level: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_usb_phy_port_speed {
    TEGRA_USB_PHY_PORT_SPEED_FULL = 0,
    TEGRA_USB_PHY_PORT_SPEED_LOW,
    TEGRA_USB_PHY_PORT_SPEED_HIGH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_usb_phy {
    pub irq: c_int,
    pub instance: c_int,
    pub freq: *const tegra_xtal_freq,
    pub regs: *mut void __iomem,
    pub pad_regs: *mut void __iomem,
    pub clk: *mut clk,
    pub pll_u: *mut clk,
    pub pad_clk: *mut clk,
    pub vbus: *mut regulator,
    pub pmc_regmap: *mut regmap,
    pub mode: usb_dr_mode,
    pub config: *mut c_void,
    pub soc_config: *const tegra_phy_soc_config,
    pub ulpi: *mut usb_phy,
    pub u_phy: usb_phy,
    pub is_legacy_phy: bool,
    pub phy_type: usb_phy_interface,
    pub reset_gpio: *mut gpio_desc,
    pub pad_rst: *mut reset_control,
    pub wakeup_enabled: bool,
    pub pad_wakeup: bool,
    pub powered_on: bool,
}
