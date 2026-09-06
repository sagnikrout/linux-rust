//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/mouse/elantech.h
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
// Elantech Touchpad driver (v6)
//
// Copyright (C) 2007-2009 Arjan Opmeer <arjan@opmeer.net>
//
// Trademarks are the property of their respective owners.
//
// Command values for Synaptics style queries
//
pub const ETP_FW_ID_QUERY: c_uint = 0x00;
pub const ETP_FW_VERSION_QUERY: c_uint = 0x01;
pub const ETP_CAPABILITIES_QUERY: c_uint = 0x02;
pub const ETP_SAMPLE_QUERY: c_uint = 0x03;
pub const ETP_RESOLUTION_QUERY: c_uint = 0x04;
pub const ETP_ICBODY_QUERY: c_uint = 0x05;
//
// Command values for register reading or writing
//
pub const ETP_REGISTER_READ: c_uint = 0x10;
pub const ETP_REGISTER_WRITE: c_uint = 0x11;
pub const ETP_REGISTER_READWRITE: c_uint = 0x00;
//
// Hardware version 2 custom PS/2 command value
//
pub const ETP_PS2_CUSTOM_COMMAND: c_uint = 0xf8;
//
// Times to retry a ps2_command and millisecond delay between tries
//
pub const ETP_PS2_COMMAND_TRIES: c_int = 3;
pub const ETP_PS2_COMMAND_DELAY: c_int = 500;
//
// Times to try to read back a register and millisecond delay between tries
//
pub const ETP_READ_BACK_TRIES: c_int = 5;
pub const ETP_READ_BACK_DELAY: c_int = 2000;
//
// Register bitmasks for hardware version 1
//
pub const ETP_R10_ABSOLUTE_MODE: c_uint = 0x04;
pub const ETP_R11_4_BYTE_MODE: c_uint = 0x02;
//
// Capability bitmasks
//
pub const ETP_CAP_HAS_ROCKER: c_uint = 0x04;
//
// One hard to find application note states that X axis range is 0 to 576
// and Y axis range is 0 to 384 for harware version 1.
// Edge fuzz might be necessary because of bezel around the touchpad
//
pub const ETP_EDGE_FUZZ_V1: c_int = 32;

//
// The resolution for older v2 hardware doubled.
// (newer v2's firmware provides command so we can query)
//
pub const ETP_XMIN_V2: c_int = 0;
pub const ETP_XMAX_V2: c_int = 1152;
pub const ETP_YMIN_V2: c_int = 0;
pub const ETP_YMAX_V2: c_int = 768;
pub const ETP_PMIN_V2: c_int = 0;
pub const ETP_PMAX_V2: c_int = 255;
pub const ETP_WMIN_V2: c_int = 0;
pub const ETP_WMAX_V2: c_int = 15;
//
// v3 hardware has 2 kinds of packet types,
// v4 hardware has 3.
//
pub const PACKET_UNKNOWN: c_uint = 0x01;
pub const PACKET_DEBOUNCE: c_uint = 0x02;
pub const PACKET_V3_HEAD: c_uint = 0x03;
pub const PACKET_V3_TAIL: c_uint = 0x04;
pub const PACKET_V4_HEAD: c_uint = 0x05;
pub const PACKET_V4_MOTION: c_uint = 0x06;
pub const PACKET_V4_STATUS: c_uint = 0x07;
pub const PACKET_TRACKPOINT: c_uint = 0x08;
//
// track up to 5 fingers for v4 hardware
//
pub const ETP_MAX_FINGERS: c_int = 5;
//
// weight value for v4 hardware
//
pub const ETP_WEIGHT_VALUE: c_int = 5;
//
// Bus information on 3rd byte of query ETP_RESOLUTION_QUERY(0x04)
//
pub const ETP_BUS_PS2_ONLY: c_int = 0;
pub const ETP_BUS_SMB_ALERT_ONLY: c_int = 1;
pub const ETP_BUS_SMB_HST_NTFY_ONLY: c_int = 2;
pub const ETP_BUS_PS2_SMB_ALERT: c_int = 3;
pub const ETP_BUS_PS2_SMB_HST_NTFY: c_int = 4;
//
// New ICs are either using SMBus Host Notify or just plain PS2.
//
// ETP_FW_VERSION_QUERY is:
// Byte 1:
// - bit 0..3: IC BODY
// Byte 2:
// - bit 4: HiddenButton
// - bit 5: PS2_SMBUS_NOTIFY
// - bit 6: PS2CRCCheck
//

//
// The base position for one finger, v4 hardware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct finger_pos {
    pub x: c_uint,
    pub y: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elantech_device_info {
    pub capabilities: [c_uchar; 3],
    pub samples: [c_uchar; 3],
    pub debug: c_uchar,
    pub hw_version: c_uchar,
    pub pattern: c_uchar,
    pub fw_version: c_uint,
    pub ic_version: c_uint,
    pub product_id: c_uint,
    pub x_min: c_uint,
    pub y_min: c_uint,
    pub x_max: c_uint,
    pub y_max: c_uint,
    pub x_res: c_uint,
    pub y_res: c_uint,
    pub x_traces: c_uint,
    pub y_traces: c_uint,
    pub width: c_uint,
    pub bus: c_uint,
    pub paritycheck: bool,
    pub jumpy_cursor: bool,
    pub reports_pressure: bool,
    pub crc_enabled: bool,
    pub set_hw_resolution: bool,
    pub has_trackpoint: bool,
    pub has_middle_button: bool,
    pub param): *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elantech_data {
    pub /: *mut *mut *mut input_dev tp_dev; / Relative device for trackpoint,
    pub tp_phys: [c_char; 32],
    pub reg_07: c_uchar,
    pub reg_10: c_uchar,
    pub reg_11: c_uchar,
    pub reg_20: c_uchar,
    pub reg_21: c_uchar,
    pub reg_22: c_uchar,
    pub reg_23: c_uchar,
    pub reg_24: c_uchar,
    pub reg_25: c_uchar,
    pub reg_26: c_uchar,
    pub single_finger_reports: c_uint,
    pub y_max: c_uint,
    pub width: c_uint,
    pub mt: [finger_pos; ETP_MAX_FINGERS],
    pub parity: [c_uchar; 256],
    pub info: elantech_device_info,
    pub rate): *mut *mut *mut void (original_set_rate)(struct psmouse psmouse, unsigned int,
}

extern "C" {
    pub fn elantech_detect(psmouse: *mut psmouse, set_properties: bool) -> c_int;
}
extern "C" {
    pub fn elantech_init_ps2(psmouse: *mut psmouse) -> c_int;
}

extern "C" {
    pub fn elantech_init(psmouse: *mut psmouse) -> c_int;
}

extern "C" {
    pub fn elantech_init_smbus(psmouse: *mut psmouse) -> c_int;
}
