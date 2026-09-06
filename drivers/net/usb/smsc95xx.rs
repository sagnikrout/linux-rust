//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/usb/smsc95xx.h
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
// Copyright (C) 2007-2008 SMSC
//
// Tx command words

// Rx status word

// SCSRs - System Control and Status Registers
// Device ID and Revision Register

// Interrupt Status Register

// Receive Configuration Register

// Transmit Configuration Register

// Hardware Configuration Register

// Receive FIFO Information Register

// Transmit FIFO Information Register

// Power Management Control Register

// LED General Purpose IO Configuration Register

// General Purpose IO Configuration Register

// Automatic Flow Control Configuration Register

// Hi watermark = 15.5Kb (~10 mtu pkts)
// low watermark = 3k (~2 mtu pkts)
// backpressure duration = ~ 350us
// Apply FC on any frame.

// EEPROM Command Register

// EEPROM Data Register

// Burst Cap Register

// Configuration Straps Status Register

// Data Port Select Register

// Data Port Command Register

// Data Port Address Register

// Data Port Data 0 Register

// Data Port Data 1 Register

// General Purpose IO Wake Enable and Polarity Register

// Interrupt Endpoint Control Register

// Bulk In Delay Register (units of 16.667ns, until ~1092µs)

// MAC CSRs - MAC Control and Status Registers
// MAC Control Register

// MAC Address High Register

// MAC Address Low Register

// Multicast Hash Table High Register

// Multicast Hash Table Low Register

// MII Access Register

// MII Data Register

// Flow Control Register

// VLAN1 Tag Register

// VLAN2 Tag Register

// Wake Up Frame Filter Register

// Wake Up Control and Status Register

// Checksum Offload Engine Control Register

// Vendor-specific PHY Definitions (via MII access)
// EDPD NLP / crossover time configuration (LAN9500A only)

// Mode Control/Status Register

// Control/Status Indication Register

// Interrupt Source Register

// Interrupt Mask Register

// PHY Special Control/Status Register

// USB Vendor Requests
pub const USB_VENDOR_REQUEST_WRITE_REGISTER: c_uint = 0xA0;
pub const USB_VENDOR_REQUEST_READ_REGISTER: c_uint = 0xA1;
pub const USB_VENDOR_REQUEST_GET_STATS: c_uint = 0xA2;
// Interrupt Endpoint status word bitfields

