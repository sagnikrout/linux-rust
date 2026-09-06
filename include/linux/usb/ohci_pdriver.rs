//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/ohci_pdriver.h
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
// struct usb_ohci_pdata - platform_data for generic ohci driver
//
// @big_endian_desc:	BE descriptors
// @big_endian_mmio:	BE registers
// @no_big_frame_no:	no big endian frame_no shift
// @num_ports:		number of ports
//
// These are general configuration options for the OHCI controller. All of
// these options are activating more or less workarounds for some hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ohci_pdata {
    pub big_endian_desc:1: unsigned,
    pub big_endian_mmio:1: unsigned,
    pub no_big_frame_no:1: unsigned,
    pub num_ports: c_uint,
// Turn on all power and clocks
    pub pdev): *mut *mut int (power_on)(struct platform_device,
// Turn off all power and clocks
    pub pdev): *mut *mut void (power_off)(struct platform_device,
// Turn on only VBUS suspend power and hotplug detection,
// turn off everything else
    pub pdev): *mut *mut void (power_suspend)(struct platform_device,
}
