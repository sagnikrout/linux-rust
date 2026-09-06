//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/mouse/trackpoint.h
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
// IBM TrackPoint PS/2 mouse driver
//
// Stephen Evanchik <evanchsa@gmail.com>
//
// These constants are from the TrackPoint System
// Engineering documentation Version 4 from IBM Watson
// research:
// http://wwwcssrv.almaden.ibm.com/trackpoint/download.html
//
pub const TP_COMMAND: c_uint = 0xE2	/* Commands start with this */;
pub const TP_READ_ID: c_uint = 0xE1	/* Sent for device identification */;
//
// Valid first byte responses to the "Read Secondary ID" (0xE1) command.
// 0x01 was the original IBM trackpoint, others implement very limited
// subset of trackpoint features.
//
pub const TP_VARIANT_IBM: c_uint = 0x01;
pub const TP_VARIANT_ALPS: c_uint = 0x02;
pub const TP_VARIANT_ELAN: c_uint = 0x03;
pub const TP_VARIANT_NXP: c_uint = 0x04;
pub const TP_VARIANT_JYT_SYNAPTICS: c_uint = 0x05;
pub const TP_VARIANT_SYNAPTICS: c_uint = 0x06;
//
// Commands
//
pub const TP_RECALIB: c_uint = 0x51	/* Recalibrate */;
pub const TP_POWER_DOWN: c_uint = 0x44	/* Can only be undone through HW reset */;
pub const TP_EXT_DEV: c_uint = 0x21	/* Determines if external device is connected (RO) */;
pub const TP_EXT_BTN: c_uint = 0x4B	/* Read extended button status */;
pub const TP_POR: c_uint = 0x7F	/* Execute Power on Reset */;
pub const TP_POR_RESULTS: c_uint = 0x25	/* Read Power on Self test results */;
pub const TP_DISABLE_EXT: c_uint = 0x40	/* Disable external pointing device */;
pub const TP_ENABLE_EXT: c_uint = 0x41	/* Enable external pointing device */;
//
// Mode manipulation
//
pub const TP_SET_SOFT_TRANS: c_uint = 0x4E	/* Set mode */;
pub const TP_CANCEL_SOFT_TRANS: c_uint = 0xB9	/* Cancel mode */;
pub const TP_SET_HARD_TRANS: c_uint = 0x45	/* Mode can only be set */;
//
// Register oriented commands/properties
//
pub const TP_WRITE_MEM: c_uint = 0x81;
pub const TP_READ_MEM: c_uint = 0x80	/* Not used in this implementation */;
//
// RAM Locations for properties
//
pub const TP_SENS: c_uint = 0x4A	/* Sensitivity */;
pub const TP_MB: c_uint = 0x4C	/* Read Middle Button Status (RO) */;
pub const TP_INERTIA: c_uint = 0x4D	/* Negative Inertia */;
pub const TP_SPEED: c_uint = 0x60	/* Speed of TP Cursor */;
pub const TP_REACH: c_uint = 0x57	/* Backup for Z-axis press */;
pub const TP_DRAGHYS: c_uint = 0x58	/* Drag Hysteresis */;
// (how hard it is to drag
// with Z-axis pressed)
pub const TP_DOUBLETAP: c_uint = 0x58	/* TrackPoint doubletap register */;
pub const TP_MINDRAG: c_uint = 0x59	/* Minimum amount of force needed */;
// to trigger dragging
pub const TP_THRESH: c_uint = 0x5C	/* Minimum value for a Z-axis press */;
pub const TP_UP_THRESH: c_uint = 0x5A	/* Used to generate a 'click' on Z-axis */;
pub const TP_Z_TIME: c_uint = 0x5E	/* How sharp of a press */;
pub const TP_JENKS_CURV: c_uint = 0x5D	/* Minimum curvature for double click */;
pub const TP_DRIFT_TIME: c_uint = 0x5F	/* How long a 'hands off' condition */;
// must last (x*107ms) for drift
// correction to occur
//
// Toggling Flag bits
//
pub const TP_TOGGLE: c_uint = 0x47	/* Toggle command */;
pub const TP_TOGGLE_MB: c_uint = 0x23	/* Disable/Enable Middle Button */;
pub const TP_MASK_MB: c_uint = 0x01;
pub const TP_TOGGLE_EXT_DEV: c_uint = 0x23	/* Disable external device */;
pub const TP_MASK_EXT_DEV: c_uint = 0x02;
pub const TP_TOGGLE_DRIFT: c_uint = 0x23	/* Drift Correction */;
pub const TP_MASK_DRIFT: c_uint = 0x80;
pub const TP_TOGGLE_BURST: c_uint = 0x28	/* Burst Mode */;
pub const TP_MASK_BURST: c_uint = 0x80;
pub const TP_TOGGLE_PTSON: c_uint = 0x2C	/* Press to Select */;
pub const TP_MASK_PTSON: c_uint = 0x01;
pub const TP_TOGGLE_HARD_TRANS: c_uint = 0x2C	/* Alternate method to set Hard Transparency */;
pub const TP_MASK_HARD_TRANS: c_uint = 0x80;
pub const TP_TOGGLE_TWOHAND: c_uint = 0x2D	/* Two handed */;
pub const TP_MASK_TWOHAND: c_uint = 0x01;
pub const TP_TOGGLE_STICKY_TWO: c_uint = 0x2D	/* Sticky two handed */;
pub const TP_MASK_STICKY_TWO: c_uint = 0x04;
pub const TP_TOGGLE_SKIPBACK: c_uint = 0x2D	/* Suppress movement after drag release */;
pub const TP_MASK_SKIPBACK: c_uint = 0x08;
pub const TP_TOGGLE_SOURCE_TAG: c_uint = 0x20	/* Bit 3 of the first packet will be set to;
pub const TP_MASK_SOURCE_TAG: c_uint = 0x80;
pub const TP_TOGGLE_EXT_TAG: c_uint = 0x22	/* Bit 3 of the first packet coming from the;
pub const TP_MASK_EXT_TAG: c_uint = 0x04;
// Doubletap register values
pub const TP_DOUBLETAP_ENABLE: c_uint = 0xFF	/* Enable value */;
pub const TP_DOUBLETAP_DISABLE: c_uint = 0xFE	/* Disable value */;
// Power on Self Test Results
pub const TP_POR_SUCCESS: c_uint = 0x3B;
//
// Default power on values
//
pub const TP_DEF_SENS: c_uint = 0x80;
pub const TP_DEF_INERTIA: c_uint = 0x06;
pub const TP_DEF_SPEED: c_uint = 0x61;
pub const TP_DEF_REACH: c_uint = 0x0A;
pub const TP_DEF_DRAGHYS: c_uint = 0xFF;
pub const TP_DEF_MINDRAG: c_uint = 0x14;
pub const TP_DEF_THRESH: c_uint = 0x08;
pub const TP_DEF_UP_THRESH: c_uint = 0xFF;
pub const TP_DEF_Z_TIME: c_uint = 0x26;
pub const TP_DEF_JENKS_CURV: c_uint = 0x87;
pub const TP_DEF_DRIFT_TIME: c_uint = 0x05;
// Toggles
pub const TP_DEF_MB: c_uint = 0x00;
pub const TP_DEF_PTSON: c_uint = 0x00;
pub const TP_DEF_SKIPBACK: c_uint = 0x00;
pub const TP_DEF_EXT_DEV: c_uint = 0x00	/* 0 means enabled */;
pub const TP_DEF_TWOHAND: c_uint = 0x00;
pub const TP_DEF_SOURCE_TAG: c_uint = 0x00;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trackpoint_data {
    pub variant_id: u8,
    pub firmware_id: u8,
    pub reach: u8 sensitivity, speed, inertia,,
    pub mindrag: u8 draghys,,
    pub upthresh: u8 thresh,,
    pub jenks: u8 ztime,,
    pub drift_time: u8,
// toggles
    pub press_to_select: bool,
    pub skipback: bool,
    pub ext_dev: bool,
}

extern "C" {
    pub fn trackpoint_detect(psmouse: *mut psmouse, set_properties: bool) -> c_int;
}
