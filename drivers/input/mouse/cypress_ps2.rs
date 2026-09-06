//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/mouse/cypress_ps2.h
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

pub const CMD_BITS_MASK: c_uint = 0x03;

// Cypress trackpad working mode.

//
// report mode bit is set, firmware working in Remote Mode.
// report mode bit is cleared, firmware working in Stream Mode.
//

// scrolling width values for set HSCROLL and VSCROLL width command.
pub const SCROLL_WIDTH_NARROW: c_int = 1;
pub const SCROLL_WIDTH_NORMAL: c_int = 2;
pub const SCROLL_WIDTH_WIDE: c_int = 3;
pub const PALM_GEOMETRY_ENABLE: c_int = 1;
pub const PALM_GEOMETRY_DISABLE: c_int = 0;
pub const TP_METRICS_MASK: c_uint = 0x80;
pub const FW_VERSION_MASX: c_uint = 0x7f;
pub const FW_VER_HIGH_MASK: c_uint = 0x70;
pub const FW_VER_LOW_MASK: c_uint = 0x0f;
// Times to retry a ps2_command and millisecond delay between tries.
pub const CYTP_PS2_CMD_TRIES: c_int = 3;
pub const CYTP_PS2_CMD_DELAY: c_int = 500;
// time out for PS/2 command only in milliseconds.
pub const CYTP_CMD_TIMEOUT: c_int = 200;
pub const CYTP_DATA_TIMEOUT: c_int = 30;
pub const CYTP_EXT_CMD: c_uint = 0xe8;

pub const CYTP_105001_HIGH: c_int = 59;

pub const CYTP_ABS_MAX_X: c_int = 1600;
pub const CYTP_ABS_MAX_Y: c_int = 900;
pub const CYTP_MAX_PRESSURE: c_int = 255;
pub const CYTP_MIN_PRESSURE: c_int = 0;
// header byte bits of relative package.
pub const BTN_LEFT_BIT: c_uint = 0x01;
pub const BTN_RIGHT_BIT: c_uint = 0x02;
pub const BTN_MIDDLE_BIT: c_uint = 0x04;
pub const REL_X_SIGN_BIT: c_uint = 0x10;
pub const REL_Y_SIGN_BIT: c_uint = 0x20;
// header byte bits of absolute package.
pub const ABS_VSCROLL_BIT: c_uint = 0x10;
pub const ABS_HSCROLL_BIT: c_uint = 0x20;
pub const ABS_MULTIFINGER_TAP: c_uint = 0x04;
pub const ABS_EDGE_MOTION_MASK: c_uint = 0x80;
pub const DFLT_RESP_BITS_VALID: c_uint = 0x88  /* SMBus bit should not be set. */;
pub const DFLT_RESP_SMBUS_BIT: c_uint = 0x80;
pub const DFLT_SMBUS_MODE: c_uint = 0x80;
pub const DFLT_PS2_MODE: c_uint = 0x00;
pub const DFLT_RESP_BIT_MODE: c_uint = 0x40;
pub const DFLT_RESP_REMOTE_MODE: c_uint = 0x40;
pub const DFLT_RESP_STREAM_MODE: c_uint = 0x00;
pub const DFLT_RESP_BIT_REPORTING: c_uint = 0x20;
pub const DFLT_RESP_BIT_SCALING: c_uint = 0x10;
pub const TP_METRICS_BIT_PALM: c_uint = 0x80;
pub const TP_METRICS_BIT_STUBBORN: c_uint = 0x40;
pub const TP_METRICS_BIT_2F_JITTER: c_uint = 0x30;
pub const TP_METRICS_BIT_1F_JITTER: c_uint = 0x0c;
pub const TP_METRICS_BIT_APA: c_uint = 0x02;
pub const TP_METRICS_BIT_MTG: c_uint = 0x01;
pub const TP_METRICS_BIT_ABS_PKT_FORMAT_SET: c_uint = 0xf0;
pub const TP_METRICS_BIT_2F_SPIKE: c_uint = 0x0c;
pub const TP_METRICS_BIT_1F_SPIKE: c_uint = 0x03;
// bits of first byte response of E9h-Status Request command.
pub const RESP_BTN_RIGHT_BIT: c_uint = 0x01;
pub const RESP_BTN_MIDDLE_BIT: c_uint = 0x02;
pub const RESP_BTN_LEFT_BIT: c_uint = 0x04;
pub const RESP_SCALING_BIT: c_uint = 0x10;
pub const RESP_ENABLE_BIT: c_uint = 0x20;
pub const RESP_REMOTE_BIT: c_uint = 0x40;
pub const RESP_SMBUS_BIT: c_uint = 0x80;
pub const CYTP_MAX_MT_SLOTS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cytp_contact {
    pub x: c_int,
    pub y: c_int,
    pub /: *mut *mut int z; / also named as touch pressure.,
}

// The structure of Cypress Trackpad event data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cytp_report_data {
    pub contact_cnt: c_int,
    pub contacts: [cytp_contact; CYTP_MAX_MT_SLOTS],
    pub left:1: c_uint,
    pub right:1: c_uint,
    pub middle:1: c_uint,
    pub /: *mut *mut unsigned int tap:1; / multi-finger tap detected.,
}

// The structure of Cypress Trackpad device private data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cytp_data {
    pub fw_version: c_int,
    pub pkt_size: c_int,
    pub mode: c_int,
    pub tp_min_pressure: c_int,
    pub tp_max_pressure: c_int,
    pub /: *mut *mut int tp_width; / X direction physical size in mm.,
    pub /: *mut *mut int tp_high; / Y direction physical size in mm.,
    pub /: *mut *mut int tp_max_abs_x; / Max X absolute units that can be reported.,
    pub /: *mut *mut int tp_max_abs_y; / Max Y absolute units that can be reported.,
    pub /: *mut *mut int tp_res_x; / X resolution in units/mm.,
    pub /: *mut *mut int tp_res_y; / Y resolution in units/mm.,
    pub tp_metrics_supported: c_int,
}

extern "C" {
    pub fn cypress_detect(psmouse: *mut psmouse, set_properties: bool) -> c_int;
}
extern "C" {
    pub fn cypress_init(psmouse: *mut psmouse) -> c_int;
}
