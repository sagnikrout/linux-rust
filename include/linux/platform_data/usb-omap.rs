//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/usb-omap.h
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
// usb-omap.h - Platform data for the various OMAP USB IPs
//
// Copyright (C) 2012 Texas Instruments Incorporated - https://www.ti.com
//
pub const OMAP3_HS_USB_PORTS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usbhs_omap_port_mode {
    OMAP_USBHS_PORT_MODE_UNUSED,
    OMAP_EHCI_PORT_MODE_PHY,
    OMAP_EHCI_PORT_MODE_TLL,
    OMAP_EHCI_PORT_MODE_HSIC,
    OMAP_OHCI_PORT_MODE_PHY_6PIN_DATSE0,
    OMAP_OHCI_PORT_MODE_PHY_6PIN_DPDM,
    OMAP_OHCI_PORT_MODE_PHY_3PIN_DATSE0,
    OMAP_OHCI_PORT_MODE_PHY_4PIN_DPDM,
    OMAP_OHCI_PORT_MODE_TLL_6PIN_DATSE0,
    OMAP_OHCI_PORT_MODE_TLL_6PIN_DPDM,
    OMAP_OHCI_PORT_MODE_TLL_3PIN_DATSE0,
    OMAP_OHCI_PORT_MODE_TLL_4PIN_DPDM,
    OMAP_OHCI_PORT_MODE_TLL_2PIN_DATSE0,
    OMAP_OHCI_PORT_MODE_TLL_2PIN_DPDM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbtll_omap_platform_data {
    pub port_mode: [usbhs_omap_port_mode; OMAP3_HS_USB_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_hcd_omap_platform_data {
    pub port_mode: [usbhs_omap_port_mode; OMAP3_HS_USB_PORTS],
    pub reset_gpio_port: [c_int; OMAP3_HS_USB_PORTS],
    pub regulator: [*mut regulator; OMAP3_HS_USB_PORTS],
    pub phy_reset:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ohci_hcd_omap_platform_data {
    pub port_mode: [usbhs_omap_port_mode; OMAP3_HS_USB_PORTS],
    pub es2_compatibility:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbhs_omap_platform_data {
    pub nports: c_int,
    pub port_mode: [usbhs_omap_port_mode; OMAP3_HS_USB_PORTS],
    pub reset_gpio_port: [c_int; OMAP3_HS_USB_PORTS],
    pub regulator: [*mut regulator; OMAP3_HS_USB_PORTS],
    pub ehci_data: *mut ehci_hcd_omap_platform_data,
    pub ohci_data: *mut ohci_hcd_omap_platform_data,
// OMAP3 <= ES2.1 have a single ulpi bypass control bit
    pub single_ulpi_bypass:1: unsigned,
    pub es2_compatibility:1: unsigned,
    pub phy_reset:1: unsigned,
}

// -------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_musb_board_data {
    pub interface_type: u8,
    pub mode: u8,
    pub power: u16,
    pub extvbus:1: unsigned,
    pub on): *mut *mut void (set_phy_power)(u8,
    pub (*clear_irq)(void): *mut c_void,
    pub mode): *mut *mut void (set_mode)(u8,
    pub (*reset)(void): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum musb_interface {
    MUSB_INTERFACE_ULPI,
    MUSB_INTERFACE_UTMI
}
