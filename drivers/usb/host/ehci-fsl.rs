//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/ehci-fsl.h
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
// Copyright (C) 2005-2010,2012 Freescale Semiconductor, Inc.
// Copyright (c) 2005 MontaVista Software
//
// offsets for the non-ehci registers in the FSL SOC USB controller
pub const FSL_SOC_USB_SBUSCFG: c_uint = 0x90;
pub const SBUSCFG_INCR8: c_uint = 0x02	/* INCR8, specified */;
pub const FSL_SOC_USB_ULPIVP: c_uint = 0x170;
pub const FSL_SOC_USB_PORTSC1: c_uint = 0x184;

pub const FSL_SOC_USB_PORTSC2: c_uint = 0x188;
pub const FSL_SOC_USB_USBMODE: c_uint = 0x1a8;

pub const FSL_SOC_USB_USBGENCTRL: c_uint = 0x200;

pub const FSL_SOC_USB_ISIPHYCTRL: c_uint = 0x204;

pub const FSL_SOC_USB_SNOOP1: c_uint = 0x400	/* NOTE: big-endian */;
pub const FSL_SOC_USB_SNOOP2: c_uint = 0x404	/* NOTE: big-endian */;
pub const FSL_SOC_USB_AGECNTTHRSH: c_uint = 0x408	/* NOTE: big-endian */;
pub const FSL_SOC_USB_PRICTRL: c_uint = 0x40c	/* NOTE: big-endian */;
pub const FSL_SOC_USB_SICTRL: c_uint = 0x410	/* NOTE: big-endian */;
pub const FSL_SOC_USB_CTRL: c_uint = 0x500	/* NOTE: big-endian */;

pub const SNOOP_SIZE_2GB: c_uint = 0x1e;
// control Register Bit Masks
pub const CONTROL_REGISTER_W1C_MASK: c_uint = 0x00020000  /* W1C: PHY_CLK_VALID */;

// Retry count for checking UTMI PHY CLK validity
pub const UTMI_PHY_CLK_VALID_CHK_RETRY: c_int = 5;
