//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/hid-uclogic-rdesc.h
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
// HID driver for UC-Logic devices not fully compliant with HID standard
// - original and fixed report descriptors
//
// Copyright (c) 2010-2018 Nikolai Kondrashov
// Copyright (c) 2013 Martin Rusko
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//

// Size of the original descriptor of WPXXXXU tablets
pub const UCLOGIC_RDESC_WPXXXXU_ORIG_SIZE: c_int = 212;
// Fixed WP4030U report descriptor
// Fixed WP5540U report descriptor
// Fixed WP8060U report descriptor
// Size of the original descriptor of the new WP5540U tablet
pub const UCLOGIC_RDESC_WP5540U_V2_ORIG_SIZE: c_int = 232;
// Size of the original descriptor of WP1062 tablet
pub const UCLOGIC_RDESC_WP1062_ORIG_SIZE: c_int = 254;
// Fixed WP1062 report descriptor
// Size of the original descriptor of PF1209 tablet
pub const UCLOGIC_RDESC_PF1209_ORIG_SIZE: c_int = 234;
// Fixed PF1209 report descriptor
// Size of the original descriptors of TWHL850 tablet
pub const UCLOGIC_RDESC_TWHL850_ORIG0_SIZE: c_int = 182;
pub const UCLOGIC_RDESC_TWHL850_ORIG1_SIZE: c_int = 161;
pub const UCLOGIC_RDESC_TWHL850_ORIG2_SIZE: c_int = 92;
// Fixed PID 0522 tablet report descriptor, interface 0 (stylus)
// Fixed PID 0522 tablet report descriptor, interface 1 (mouse)
// Fixed PID 0522 tablet report descriptor, interface 2 (frame buttons)
// Size of the original descriptors of TWHA60 tablet
pub const UCLOGIC_RDESC_TWHA60_ORIG0_SIZE: c_int = 254;
pub const UCLOGIC_RDESC_TWHA60_ORIG1_SIZE: c_int = 139;
// Fixed TWHA60 report descriptor, interface 0 (stylus)
// Fixed TWHA60 report descriptor, interface 1 (frame buttons)
// Report descriptor template placeholder head
pub const UCLOGIC_RDESC_PEN_PH_HEAD: c_uint = 0xFE, 0xED, 0x1D;
pub const UCLOGIC_RDESC_FRAME_PH_BTN_HEAD: c_uint = 0xFE, 0xED;
// Apply report descriptor parameters to a report descriptor template
// Report descriptor template placeholder IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uclogic_rdesc_ph_id {
    UCLOGIC_RDESC_PEN_PH_ID_X_LM,
    UCLOGIC_RDESC_PEN_PH_ID_X_PM,
    UCLOGIC_RDESC_PEN_PH_ID_Y_LM,
    UCLOGIC_RDESC_PEN_PH_ID_Y_PM,
    UCLOGIC_RDESC_PEN_PH_ID_PRESSURE_LM,
    UCLOGIC_RDESC_FRAME_PH_ID_UM,
    UCLOGIC_RDESC_PH_ID_NUM
}

// Report descriptor pen template placeholder

// Report descriptor frame buttons template placeholder

// Report ID for v1 pen reports
pub const UCLOGIC_RDESC_V1_PEN_ID: c_uint = 0x07;
// Fixed report descriptor template for (tweaked) v1 pen reports
// Report ID for v2 pen reports
pub const UCLOGIC_RDESC_V2_PEN_ID: c_uint = 0x08;
// Fixed report descriptor template for (tweaked) v2 pen reports
// Report ID for tweaked v1 frame reports
pub const UCLOGIC_RDESC_V1_FRAME_ID: c_uint = 0xf7;
// Fixed report descriptor for (tweaked) v1 frame reports
// Report ID for tweaked v2 frame button reports
pub const UCLOGIC_RDESC_V2_FRAME_BUTTONS_ID: c_uint = 0xf7;
// Fixed report descriptor for (tweaked) v2 frame button reports
// Report ID for tweaked v2 frame touch ring/strip reports
pub const UCLOGIC_RDESC_V2_FRAME_TOUCH_ID: c_uint = 0xf8;
// Fixed report descriptor for (tweaked) v2 frame touch ring reports
// Fixed report descriptor for (tweaked) v2 frame touch strip reports
// Device ID byte offset in v2 frame touch ring/strip reports
pub const UCLOGIC_RDESC_V2_FRAME_TOUCH_DEV_ID_BYTE: c_uint = 0x4;
// Report ID for tweaked v2 frame dial reports
pub const UCLOGIC_RDESC_V2_FRAME_DIAL_ID: c_uint = 0xf9;
// Fixed report descriptor for (tweaked) v2 frame dial reports
// Device ID byte offset in v2 frame dial reports
pub const UCLOGIC_RDESC_V2_FRAME_DIAL_DEV_ID_BYTE: c_uint = 0x4;
// Report ID for tweaked UGEE v2 battery reports
pub const UCLOGIC_RDESC_UGEE_V2_BATTERY_ID: c_uint = 0xba;
// Magic data expected by UGEEv2 devices on probe
// Fixed report descriptor template for UGEE v2 pen reports
// Fixed report descriptor template for UGEE v2 frame reports (buttons only)
// Fixed report descriptor template for UGEE v2 frame reports (dial)
// Fixed report descriptor template for UGEE v2 frame reports (mouse)
// Fixed report descriptor template for UGEE v2 battery reports
// Fixed report descriptor for Ugee EX07 frame
// Fixed report descriptor for XP-Pen Deco 01 frame controls
// Fixed report descriptor for Ugee G5 frame controls
// Report ID of Ugee G5 frame control reports
pub const UCLOGIC_RDESC_UGEE_G5_FRAME_ID: c_uint = 0x06;
// Device ID byte offset in Ugee G5 frame report
pub const UCLOGIC_RDESC_UGEE_G5_FRAME_DEV_ID_BYTE: c_uint = 0x2;
// Least-significant bit of Ugee G5 frame rotary encoder state
pub const UCLOGIC_RDESC_UGEE_G5_FRAME_RE_LSB: c_int = 38;
// Fixed report descriptor for XP-Pen Arist 22R Pro frame
// Fixed report descriptor for XP-Pen Arist 24 Pro frame
