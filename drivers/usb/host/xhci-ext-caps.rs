//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/xhci-ext-caps.h
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
// xHCI host controller driver
//
// Copyright (C) 2008 Intel Corp.
//
// Author: Sarah Sharp
// Some code borrowed from the Linux EHCI driver.
//
// HC should halt within 16 ms, but use 32 ms as some hosts take longer

// HC not running - set to 1 when run/stop bit is cleared.

// HCCPARAMS offset from PCI base address
pub const XHCI_HCC_PARAMS_OFFSET: c_uint = 0x10;
// HCCPARAMS contains the first extended capability pointer

// Command and Status registers offset from the Operational Registers address
pub const XHCI_CMD_OFFSET: c_uint = 0x00;
pub const XHCI_STS_OFFSET: c_uint = 0x04;
pub const XHCI_MAX_EXT_CAPS: c_int = 50;
// Capability Register
// bits 7:0 - how long is the Capabilities register

// Extended capability register fields

// Extended capability IDs - ID 0 reserved
pub const XHCI_EXT_CAPS_LEGACY: c_int = 1;
pub const XHCI_EXT_CAPS_PROTOCOL: c_int = 2;
pub const XHCI_EXT_CAPS_PM: c_int = 3;
pub const XHCI_EXT_CAPS_VIRT: c_int = 4;
pub const XHCI_EXT_CAPS_ROUTE: c_int = 5;
// IDs 6-9 reserved
pub const XHCI_EXT_CAPS_DEBUG: c_int = 10;
// Vendor caps
pub const XHCI_EXT_CAPS_VENDOR_INTEL: c_int = 192;
pub const XHCI_EXT_CAPS_INTEL_SPR_SHADOW: c_int = 206;
// USB Legacy Support Capability - section 7.1.1

// USB Legacy Support Capability - section 7.1.1
// Add this offset, plus the value of xECP in HCCPARAMS to the base address

// USB Legacy Support Control and Status Register  - section 7.1.2
// Add this offset, plus the value of xECP in HCCPARAMS to the base address

// bits 1:3, 5:12, and 17:19 need to be preserved; bits 21:28 should be zero

// USB 2.0 xHCI 0.96 L1C capability - section 7.2.2.1.3.2

// USB 2.0 xHCI 1.0 hardware LMP capability - section 7.2.2.1.3.2

// Intel SPR shadow capability
pub const XHCI_INTEL_SPR_ESS_PORT_OFFSET: c_uint = 0x8ac4	/* SuperSpeed port control */;

// command register values to disable interrupts and halt the HC
// start/stop HC execution - do not write unless HC is halted

// Event Interrupt Enable - get irq when EINT bit is set in USBSTS register

// Host System Error Interrupt Enable - get irq when HSEIE bit set in USBSTS

// Enable Wrap Event - '1' means xHC generates an event when MFINDEX wraps.

// true: Controller Not Ready to accept doorbell or op reg writes after reset

//
// struct xhci_protocol_caps
// @revision:		major revision, minor revision, capability ID,
// and next capability pointer.
// @name_string:	Four ASCII characters to say which spec this xHC
// follows, typically "USB ".
// @port_info:		Port offset, count, and protocol-defined information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_protocol_caps {
    pub revision: u32,
    pub name_string: u32,
    pub port_info: u32,
}

//
// Find the offset of the extended capabilities with capability ID id.
//
// @base	PCI MMIO registers base address.
// @start	address at which to start looking, (0 or HCC_PARAMS to start at
// beginning of list)
// @id		Extended capability ID to search for, or 0 for the next
// capability
//
// Returns the offset of the next matching extended capability structure.
// Some capabilities can occur several times, e.g., the XHCI_EXT_CAPS_PROTOCOL,
// and this provides a way to find them all.
//
