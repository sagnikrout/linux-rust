//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/ndis.h
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


//
// ndis.h
//
// ntddndis.h modified by Benedikt Spranger <b.spranger@pengutronix.de>
//
// Thanks to the cygwin development team,
// espacially to Casper S. Hornstrup <chorns@users.sourceforge.net>
//
// THIS SOFTWARE IS NOT COPYRIGHTED
//
// This source code is offered for use in the public domain. You may
// use, modify or distribute it freely.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NDIS_DEVICE_POWER_STATE {
    NdisDeviceStateUnspecified = 0,
    NdisDeviceStateD0,
    NdisDeviceStateD1,
    NdisDeviceStateD2,
    NdisDeviceStateD3,
    NdisDeviceStateMaximum
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NDIS_PM_WAKE_UP_CAPABILITIES {
    pub MinMagicPacketWakeUp: NDIS_DEVICE_POWER_STATE,
    pub MinPatternWakeUp: NDIS_DEVICE_POWER_STATE,
    pub MinLinkChangeWakeUp: NDIS_DEVICE_POWER_STATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NDIS_PNP_CAPABILITIES {
    pub Flags: __le32,
    pub WakeUpCapabilities: NDIS_PM_WAKE_UP_CAPABILITIES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NDIS_PM_PACKET_PATTERN {
    pub Priority: __le32,
    pub Reserved: __le32,
    pub MaskSize: __le32,
    pub PatternOffset: __le32,
    pub PatternSize: __le32,
    pub PatternFlags: __le32,
}
