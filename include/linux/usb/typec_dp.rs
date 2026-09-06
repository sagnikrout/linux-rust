//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/typec_dp.h
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

pub const USB_TYPEC_DP_SID: c_uint = 0xff01;
// USB IF has not assigned a Standard ID (SID) for VirtualLink,
// so the manufacturers of VirtualLink adapters use their Vendor
// IDs as the SVID.
//
pub const USB_TYPEC_NVIDIA_VLINK_SID: c_uint = 0x955	/* NVIDIA VirtualLink */;
pub const USB_TYPEC_DP_MODE: c_int = 1;
//
// Connector states matching the pin assignments in DisplayPort Alt Mode
// Specification.
//
// These values are meant primarily to be used by the mux drivers, but they are
// also used as the "value" part in the alternate mode notification chain, so
// receivers of those notifications will always see them.
//
// Note. DisplayPort USB Type-C Alt Mode Specification version 1.0b deprecated
// pin assignments A, B and F, but they are still defined here for legacy
// purposes.
//
// struct typec_displayport_data - DisplayPort Alt Mode specific data
// @status: Status Update command VDO content
// @conf: Configure command VDO content
//
// This structure is delivered as the data part with the notifications. It
// contains the VDOs from the two DisplayPort Type-C alternate mode specific
// commands: Status Update and Configure.
//
// @status will show for example the status of the HPD signal.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_displayport_data {
    pub status: u32,
    pub conf: u32,
}

// DisplayPort alt mode specific commands

// DisplayPort Capabilities VDO bits (returned with Discover Modes)

pub const DP_CAP_UFP_D: c_int = 1;
pub const DP_CAP_DFP_D: c_int = 2;
pub const DP_CAP_DFP_D_AND_UFP_D: c_int = 3;

pub const DP_CAP_SIGNALLING_HBR3: c_int = 1;
pub const DP_CAP_SIGNALLING_UHBR10: c_int = 2;
pub const DP_CAP_SIGNALLING_UHBR20: c_int = 3;

// Get pin assignment taking plug & receptacle into consideration

pub const DP_CAP_CABLE_TYPE_PASSIVE: c_int = 0;
pub const DP_CAP_CABLE_TYPE_RE_TIMER: c_int = 1;
pub const DP_CAP_CABLE_TYPE_RE_DRIVER: c_int = 2;
pub const DP_CAP_CABLE_TYPE_OPTICAL: c_int = 3;

// DisplayPort Status Update VDO bits

pub const DP_STATUS_CON_DISABLED: c_int = 0;
pub const DP_STATUS_CON_DFP_D: c_int = 1;
pub const DP_STATUS_CON_UFP_D: c_int = 2;
pub const DP_STATUS_CON_BOTH: c_int = 3;

// DisplayPort Configurations VDO bits

pub const DP_CONF_SIGNALLING_SHIFT: c_int = 2;
pub const DP_CONF_SIGNALLING_HBR3: c_int = 1;
pub const DP_CONF_SIGNALLING_UHBR10: c_int = 2;
pub const DP_CONF_SIGNALLING_UHBR20: c_int = 3;
pub const DP_CONF_PIN_ASSIGNEMENT_SHIFT: c_int = 8;

// Helper for setting/getting the pin assignment value to the configuration

pub const DP_CONF_CABLE_TYPE_SHIFT: c_int = 28;
pub const DP_CONF_CABLE_TYPE_PASSIVE: c_int = 0;
pub const DP_CONF_CABLE_TYPE_RE_TIMER: c_int = 1;
pub const DP_CONF_CABLE_TYPE_RE_DRIVER: c_int = 2;
pub const DP_CONF_CABLE_TYPE_OPTICAL: c_int = 3;

