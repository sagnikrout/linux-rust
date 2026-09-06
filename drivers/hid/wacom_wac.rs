//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/wacom_wac.h
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

pub const WACOM_NAME_MAX: c_int = 64;
pub const WACOM_MAX_REMOTES: c_int = 5;
pub const WACOM_STATUS_UNKNOWN: c_int = 255;

pub const WACOM_AES_BATTERY_TIMEOUT: c_int = 1800000;
// packet length for individual models
pub const WACOM_PKGLEN_BBFUN: c_int = 9;
pub const WACOM_PKGLEN_TPC1FG: c_int = 5;
pub const WACOM_PKGLEN_TPC1FG_B: c_int = 10;
pub const WACOM_PKGLEN_TPC2FG: c_int = 14;
pub const WACOM_PKGLEN_BBTOUCH: c_int = 20;
pub const WACOM_PKGLEN_BBTOUCH3: c_int = 64;
pub const WACOM_PKGLEN_BBPEN: c_int = 10;
pub const WACOM_PKGLEN_WIRELESS: c_int = 32;
pub const WACOM_PKGLEN_PENABLED: c_int = 8;
pub const WACOM_PKGLEN_BPAD_TOUCH: c_int = 32;
pub const WACOM_PKGLEN_BPAD_TOUCH_USB: c_int = 64;
// wacom data size per MT contact
pub const WACOM_BYTES_PER_MT_PACKET: c_int = 11;
pub const WACOM_BYTES_PER_24HDT_PACKET: c_int = 14;
pub const WACOM_BYTES_PER_QHDTHID_PACKET: c_int = 6;
// device IDs
pub const STYLUS_DEVICE_ID: c_uint = 0x02;
pub const TOUCH_DEVICE_ID: c_uint = 0x03;
pub const CURSOR_DEVICE_ID: c_uint = 0x06;
pub const ERASER_DEVICE_ID: c_uint = 0x0A;
pub const PAD_DEVICE_ID: c_uint = 0x0F;
// wacom data packet report IDs
pub const WACOM_REPORT_PENABLED: c_int = 2;
pub const WACOM_REPORT_PENABLED_BT: c_int = 3;
pub const WACOM_REPORT_INTUOS_ID1: c_int = 5;
pub const WACOM_REPORT_INTUOS_ID2: c_int = 6;
pub const WACOM_REPORT_INTUOSPAD: c_int = 12;
pub const WACOM_REPORT_INTUOS5PAD: c_int = 3;
pub const WACOM_REPORT_DTUSPAD: c_int = 21;
pub const WACOM_REPORT_TPC1FG: c_int = 6;
pub const WACOM_REPORT_TPC2FG: c_int = 13;
pub const WACOM_REPORT_TPCMT: c_int = 13;
pub const WACOM_REPORT_TPCMT2: c_int = 3;
pub const WACOM_REPORT_TPCHID: c_int = 15;
pub const WACOM_REPORT_CINTIQ: c_int = 16;
pub const WACOM_REPORT_CINTIQPAD: c_int = 17;
pub const WACOM_REPORT_TPCST: c_int = 16;
pub const WACOM_REPORT_DTUS: c_int = 17;
pub const WACOM_REPORT_TPC1FGE: c_int = 18;
pub const WACOM_REPORT_24HDT: c_int = 1;
pub const WACOM_REPORT_WL: c_int = 128;
pub const WACOM_REPORT_USB: c_int = 192;
pub const WACOM_REPORT_BPAD_PEN: c_int = 3;
pub const WACOM_REPORT_BPAD_TOUCH: c_int = 16;
pub const WACOM_REPORT_DEVICE_LIST: c_int = 16;
pub const WACOM_REPORT_INTUOS_PEN: c_int = 16;
pub const WACOM_REPORT_REMOTE: c_int = 17;
pub const WACOM_REPORT_INTUOSHT2_ID: c_int = 8;
// wacom command report ids
pub const WAC_CMD_WL_LED_CONTROL: c_uint = 0x03;
pub const WAC_CMD_LED_CONTROL: c_uint = 0x20;
pub const WAC_CMD_ICON_START: c_uint = 0x21;
pub const WAC_CMD_ICON_XFER: c_uint = 0x23;
pub const WAC_CMD_ICON_BT_XFER: c_uint = 0x26;
pub const WAC_CMD_DELETE_PAIRING: c_uint = 0x20;
pub const WAC_CMD_LED_CONTROL_GENERIC: c_uint = 0x32;
pub const WAC_CMD_UNPAIR_ALL: c_uint = 0xFF;
pub const WAC_CMD_WL_INTUOSP2: c_uint = 0x82;
// device quirks
pub const WACOM_QUIRK_BBTOUCH_LOWRES: c_uint = 0x0001;
pub const WACOM_QUIRK_SENSE: c_uint = 0x0002;
pub const WACOM_QUIRK_AESPEN: c_uint = 0x0004;
pub const WACOM_QUIRK_BATTERY: c_uint = 0x0008;
pub const WACOM_QUIRK_TOOLSERIAL: c_uint = 0x0010;
pub const WACOM_QUIRK_PEN_BUTTON3: c_uint = 0x0020;
// device types
pub const WACOM_DEVICETYPE_NONE: c_uint = 0x0000;
pub const WACOM_DEVICETYPE_PEN: c_uint = 0x0001;
pub const WACOM_DEVICETYPE_TOUCH: c_uint = 0x0002;
pub const WACOM_DEVICETYPE_PAD: c_uint = 0x0004;
pub const WACOM_DEVICETYPE_WL_MONITOR: c_uint = 0x0008;
pub const WACOM_DEVICETYPE_DIRECT: c_uint = 0x0010;

pub const WACOM_HID_UP_WACOMDIGITIZER: c_uint = 0xff0d0000;
pub const WACOM_HID_SP_PAD: c_uint = 0x00040000;
pub const WACOM_HID_SP_BUTTON: c_uint = 0x00090000;
pub const WACOM_HID_SP_DIGITIZER: c_uint = 0x000d0000;
pub const WACOM_HID_SP_DIGITIZERINFO: c_uint = 0x00100000;

pub const WACOM_HID_UP_G9: c_uint = 0xff090000;

pub const WACOM_HID_UP_G11: c_uint = 0xff110000;

pub const WACOM_HID_UP_WACOMTOUCH: c_uint = 0xff000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wacom_features {
    pub name: *const c_char,
    pub x_max: c_int,
    pub y_max: c_int,
    pub pressure_max: c_int,
    pub distance_max: c_int,
    pub type: c_int,
    pub x_resolution: c_int,
    pub y_resolution: c_int,
    pub numbered_buttons: c_int,
    pub offset_left: c_int,
    pub offset_right: c_int,
    pub offset_top: c_int,
    pub offset_bottom: c_int,
    pub device_type: c_int,
    pub x_phy: c_int,
    pub y_phy: c_int,
    pub unit: unsigned,
    pub unitExpo: c_int,
    pub x_fuzz: c_int,
    pub y_fuzz: c_int,
    pub pressure_fuzz: c_int,
    pub distance_fuzz: c_int,
    pub tilt_fuzz: c_int,
    pub quirks: unsigned,
    pub touch_max: unsigned,
    pub oVid: c_int,
    pub oPid: c_int,
    pub pktlen: c_uint,
    pub check_for_hid_type: bool,
    pub hid_type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wacom_shared {
    pub stylus_in_proximity: bool,
    pub touch_down: bool,
// for wireless device to access USB interfaces
    pub touch_max: unsigned,
    pub type: c_int,
    pub touch_input: *mut input_dev,
    pub pen: *mut hid_device,
    pub touch: *mut hid_device,
    pub has_mute_touch_switch: bool,
    pub is_touch_on: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_data {
    pub /: *mut *mut __s16 inputmode; / InputMode HID feature, -1 if non-existent,
    pub /: *mut *mut __s16 inputmode_index; / InputMode HID feature index in the report,
    pub /: *mut *mut __s16 inputmode_field_index; / InputMode HID feature field index in the report,
    pub sense_state: bool,
    pub inrange_state: bool,
    pub eraser: bool,
    pub tipswitch: bool,
    pub barrelswitch: bool,
    pub barrelswitch2: bool,
    pub barrelswitch3: bool,
    pub serialhi: bool,
    pub confidence: bool,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub id: c_int,
    pub ring_value: c_int,
    pub ring2_value: c_int,
    pub cc_report: c_int,
    pub cc_index: c_int,
    pub cc_value_index: c_int,
    pub last_slot_field: c_int,
    pub num_expected: c_int,
    pub num_received: c_int,
    pub bat_status: c_int,
    pub battery_capacity: c_int,
    pub bat_charging: c_int,
    pub bat_connected: c_int,
    pub ps_connected: c_int,
    pub pad_input_event_flag: bool,
    pub sequence_number: c_int,
    pub time_delayed: ktime_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wacom_remote_work_data {
    pub serial: u32,
    pub remote: [}; WACOM_MAX_REMOTES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wacom_wac {
    pub name: [c_char; WACOM_NAME_MAX],
    pub pen_name: [c_char; WACOM_NAME_MAX],
    pub touch_name: [c_char; WACOM_NAME_MAX],
    pub pad_name: [c_char; WACOM_NAME_MAX],
    pub data: *mut u8,
    pub tool: [c_int; 2],
    pub id: [c_int; 2],
    pub serial: [__u64; 2],
    pub probe_complete: bool,
    pub reporting_data: bool,
    pub features: wacom_features,
    pub shared: *mut wacom_shared,
    pub pen_input: *mut input_dev,
    pub touch_input: *mut input_dev,
    pub pad_input: *mut input_dev,
    pub pen_fifo: *mut kfifo_rec_ptr_2,
    pub pid: c_int,
    pub num_contacts_left: c_int,
    pub bt_features: u8,
    pub bt_high_speed: u8,
    pub absring_count: u8,
    pub relring_count: u8,
    pub mode_report: c_int,
    pub mode_value: c_int,
    pub hid_data: hid_data,
    pub has_mute_touch_switch: bool,
    pub is_soft_touch_switch: bool,
    pub has_mode_change: bool,
    pub is_direct_mode: bool,
    pub is_invalid_bt_frame: bool,
}
