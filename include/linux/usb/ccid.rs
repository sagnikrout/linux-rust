//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/ccid.h
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
// Copyright (c) 2018  Vincent Pelletier
//

pub const USB_INTERFACE_CLASS_CCID: c_uint = 0x0b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccid_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bcdCCID: __le16,
    pub bMaxSlotIndex: __u8,
    pub bVoltageSupport: __u8,
    pub dwProtocols: __le32,
    pub dwDefaultClock: __le32,
    pub dwMaximumClock: __le32,
    pub bNumClockSupported: __u8,
    pub dwDataRate: __le32,
    pub dwMaxDataRate: __le32,
    pub bNumDataRatesSupported: __u8,
    pub dwMaxIFSD: __le32,
    pub dwSynchProtocols: __le32,
    pub dwMechanical: __le32,
    pub dwFeatures: __le32,
    pub dwMaxCCIDMessageLength: __le32,
    pub bClassGetResponse: __u8,
    pub bClassEnvelope: __u8,
    pub wLcdLayout: __le16,
    pub bPINSupport: __u8,
    pub bMaxCCIDBusySlots: __u8,
// C attribute field omitted
