//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/phy/omap_control_phy.h
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
// omap_control_phy.h - Header file for the PHY part of control module.
//
// Copyright (C) 2013 Texas Instruments Incorporated - http://www.ti.com
// Author: Kishon Vijay Abraham I <kishon@ti.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_control_phy_type {
    OMAP_CTRL_TYPE_OTGHS = 1,	/* Mailbox OTGHS_CONTROL */
    OMAP_CTRL_TYPE_USB2,	/* USB2_PHY, power down in CONTROL_DEV_CONF */
    OMAP_CTRL_TYPE_PIPE3,	/* PIPE3 PHY, DPLL & seperate Rx/Tx power */
    OMAP_CTRL_TYPE_PCIE,	/* RX TX control of ACSPCIE */
    OMAP_CTRL_TYPE_DRA7USB2, /* USB2 PHY, power and power_aux e.g. DRA7 */
    OMAP_CTRL_TYPE_AM437USB2, /* USB2 PHY, power e.g. AM437x */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_control_phy {
    pub dev: *mut device,
    pub otghs_control: *mut u32 __iomem,
    pub power: *mut u32 __iomem,
    pub power_aux: *mut u32 __iomem,
    pub pcie_pcs: *mut u32 __iomem,
    pub sys_clk: *mut clk,
    pub type: omap_control_phy_type,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_control_usb_mode {
    USB_MODE_UNDEFINED = 0,
    USB_MODE_HOST,
    USB_MODE_DEVICE,
    USB_MODE_DISCONNECT,
}

pub const OMAP_CTRL_PIPE3_PHY_PWRCTL_CLK_CMD_MASK: c_uint = 0x003FC000;
pub const OMAP_CTRL_PIPE3_PHY_PWRCTL_CLK_CMD_SHIFT: c_uint = 0xE;
pub const OMAP_CTRL_PIPE3_PHY_PWRCTL_CLK_FREQ_MASK: c_uint = 0xFFC00000;
pub const OMAP_CTRL_PIPE3_PHY_PWRCTL_CLK_FREQ_SHIFT: c_uint = 0x16;
pub const OMAP_CTRL_PIPE3_PHY_TX_RX_POWERON: c_uint = 0x3;
pub const OMAP_CTRL_PIPE3_PHY_TX_RX_POWEROFF: c_uint = 0x0;
pub const OMAP_CTRL_PCIE_PCS_MASK: c_uint = 0xff;
pub const OMAP_CTRL_PCIE_PCS_DELAY_COUNT_SHIFT: c_int = 16;

extern "C" {
    pub fn omap_control_phy_power(dev: *mut device, on: c_int);
}
extern "C" {
    pub fn omap_control_pcie_pcs(dev: *mut device, delay: u8);
}

