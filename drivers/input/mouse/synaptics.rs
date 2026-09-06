//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/mouse/synaptics.h
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
// Synaptics TouchPad PS/2 mouse driver
//
// synaptics queries
pub const SYN_QUE_IDENTIFY: c_uint = 0x00;
pub const SYN_QUE_MODES: c_uint = 0x01;
pub const SYN_QUE_CAPABILITIES: c_uint = 0x02;
pub const SYN_QUE_MODEL: c_uint = 0x03;
pub const SYN_QUE_SERIAL_NUMBER_PREFIX: c_uint = 0x06;
pub const SYN_QUE_SERIAL_NUMBER_SUFFIX: c_uint = 0x07;
pub const SYN_QUE_RESOLUTION: c_uint = 0x08;
pub const SYN_QUE_EXT_CAPAB: c_uint = 0x09;
pub const SYN_QUE_FIRMWARE_ID: c_uint = 0x0a;
pub const SYN_QUE_EXT_CAPAB_0C: c_uint = 0x0c;
pub const SYN_QUE_EXT_MAX_COORDS: c_uint = 0x0d;
pub const SYN_QUE_EXT_MIN_COORDS: c_uint = 0x0f;
pub const SYN_QUE_MEXT_CAPAB_10: c_uint = 0x10;
// synaptics modes

// synaptics model ID bits

// synaptics capability bits

//
// The following describes response for the 0x0c query.
//
// byte	mask	name			meaning
// ----	----	-------			------------
// 1	0x01	adjustable threshold	capacitive button sensitivity
// can be adjusted
// 1	0x02	report max		query 0x0d gives max coord reported
// 1	0x04	clearpad		sensor is ClearPad product
// 1	0x08	advanced gesture	not particularly meaningful
// 1	0x10	clickpad bit 0		1-button ClickPad
// 1	0x60	multifinger mode	identifies firmware finger counting
// (not reporting!) algorithm.
// Not particularly meaningful
// 1	0x80	covered pad		W clipped to 14, 15 == pad mostly covered
// 2	0x01	clickpad bit 1		2-button ClickPad
// 2	0x02	deluxe LED controls	touchpad support LED commands
// ala multimedia control bar
// 2	0x04	reduced filtering	firmware does less filtering on
// position data, driver should watch
// for noise.
// 2	0x08	image sensor		image sensor tracks 5 fingers, but only
// reports 2.
// 2	0x01	uniform clickpad	whole clickpad moves instead of being
// hinged at the top.
// 2	0x20	report min		query 0x0f gives min coord reported
//

//
// The following descibes response for the 0x10 query.
//
// byte	mask	name			meaning
// ----	----	-------			------------
// 1	0x01	ext buttons are stick	buttons exported in the extended
// capability are actually meant to be used
// by the tracktick (pass-through).
// 1	0x02	SecurePad		the touchpad is a SecurePad, so it
// contains a built-in fingerprint reader.
// 1	0xe0	more ext count		how many more extented queries are
// available after this one.
// 2	0xff	SecurePad width		the width of the SecurePad fingerprint
// reader.
// 3	0xff	SecurePad height	the height of the SecurePad fingerprint
// reader.
//

// synaptics modes query bits

// synaptics identify query bits

// synaptics special commands
pub const SYN_PS_SET_MODE2: c_uint = 0x14;
pub const SYN_PS_CLIENT_CMD: c_uint = 0x28;
// amount to fuzz position data when touchpad reports reduced filtering
pub const SYN_REDUCED_FILTER_FUZZ: c_int = 8;
// synaptics packet types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum synaptics_pkt_type {
    SYN_NEWABS,
    SYN_NEWABS_STRICT,
    SYN_NEWABS_RELAXED,
    SYN_OLDABS,
}

//
// A structure to describe the state of the touchpad hardware (buttons and pad)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synaptics_hw_state {
    pub x: c_int,
    pub y: c_int,
    pub z: c_int,
    pub w: c_int,
    pub left:1: c_uint,
    pub right:1: c_uint,
    pub middle:1: c_uint,
    pub up:1: c_uint,
    pub down:1: c_uint,
    pub ext_buttons: u8,
    pub scroll: i8,
}

// Data read from the touchpad
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synaptics_device_info {
    pub /: *mut *mut u32 model_id; / Model-ID,
    pub /: *mut *mut u32 firmware_id; / Firmware-ID,
    pub /: *mut *mut u32 board_id; / Board-ID,
    pub /: *mut *mut u32 capabilities; / Capabilities,
    pub /: *mut *mut u32 ext_cap; / Extended Capabilities,
    pub /: *mut *mut u32 ext_cap_0c; / Ext Caps from 0x0c query,
    pub /: *mut *mut u32 ext_cap_10; / Ext Caps from 0x10 query,
    pub /: *mut *mut u32 identity; / Identification,
    pub /: *mut *mut u32 x_res, y_res; / X/Y resolution in units/mm,
    pub /: *mut *mut u32 x_max, y_max; / Max coordinates (from FW),
    pub /: *mut *mut u32 x_min, y_min; / Min coordinates (from FW),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct synaptics_data {
    pub info: synaptics_device_info,
    pub /: *mut *mut synaptics_pkt_type pkt_type; / packet type - old, new, etc,
    pub /: *mut *mut u8 mode; / current mode byte,
    pub scroll: c_int,
    pub /: *mut *mut bool absolute_mode; / run in Absolute mode,
    pub /: *mut *mut bool disable_gesture; / disable gestures,
    pub /: *mut *mut *mut serio pt_port; / Pass-through serio port,
    pub pt_port_open: bool,
//
// Last received Advanced Gesture Mode (AGM) packet. An AGM packet
// contains position data for a second contact, at half resolution.
//
    pub agm: synaptics_hw_state,
    pub /: *mut *mut unsigned int agm_count; / finger count reported by agm,
// ForcePad handling
    pub press_start: c_ulong,
    pub press: bool,
    pub report_press: bool,
    pub is_forcepad: bool,
}

extern "C" {
    pub fn synaptics_module_init();
}
extern "C" {
    pub fn synaptics_detect(psmouse: *mut psmouse, set_properties: bool) -> c_int;
}
extern "C" {
    pub fn synaptics_init_absolute(psmouse: *mut psmouse) -> c_int;
}
extern "C" {
    pub fn synaptics_init_relative(psmouse: *mut psmouse) -> c_int;
}
extern "C" {
    pub fn synaptics_init_smbus(psmouse: *mut psmouse) -> c_int;
}
extern "C" {
    pub fn synaptics_init(psmouse: *mut psmouse) -> c_int;
}
extern "C" {
    pub fn synaptics_reset(psmouse: *mut psmouse);
}
