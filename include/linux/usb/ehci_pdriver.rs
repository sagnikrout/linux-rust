//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/ehci_pdriver.h
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
// Copyright (C) 2012 Hauke Mehrtens <hauke@hauke-m.de>
//
// struct usb_ehci_pdata - platform_data for generic ehci driver
//
// @caps_offset:	offset of the EHCI Capability Registers to the start of
// the io memory region provided to the driver.
// @has_tt:		set to 1 if TT is integrated in root hub.
// @port_power_on:	set to 1 if the controller needs a power up after
// initialization.
// @port_power_off:	set to 1 if the controller needs to be powered down
// after initialization.
// @no_io_watchdog:	set to 1 if the controller does not need the I/O
// watchdog to run.
// @reset_on_resume:	set to 1 if the controller needs to be reset after
// a suspend / resume cycle (but can't detect that itself).
//
// These are general configuration options for the EHCI controller. All of
// these options are activating more or less workarounds for some hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ehci_pdata {
    pub caps_offset: c_int,
    pub has_tt:1: unsigned,
    pub has_synopsys_hc_bug:1: unsigned,
    pub big_endian_desc:1: unsigned,
    pub big_endian_mmio:1: unsigned,
    pub no_io_watchdog:1: unsigned,
    pub reset_on_resume:1: unsigned,
    pub dma_mask_64:1: unsigned,
    pub spurious_oc:1: unsigned,
// Turn on all power and clocks
    pub pdev): *mut *mut int (power_on)(struct platform_device,
// Turn off all power and clocks
    pub pdev): *mut *mut void (power_off)(struct platform_device,
// Turn on only VBUS suspend power and hotplug detection,
// turn off everything else
    pub pdev): *mut *mut void (power_suspend)(struct platform_device,
    pub hcd): *mut *mut int (pre_setup)(struct usb_hcd,
}
