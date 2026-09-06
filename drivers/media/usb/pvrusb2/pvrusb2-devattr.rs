//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/pvrusb2/pvrusb2-devattr.h
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
// Copyright (C) 2005 Mike Isely <isely@pobox.com>
//

//
pub const PVR2_CLIENT_ID_NULL: c_int = 0;
pub const PVR2_CLIENT_ID_MSP3400: c_int = 1;
pub const PVR2_CLIENT_ID_CX25840: c_int = 2;
pub const PVR2_CLIENT_ID_SAA7115: c_int = 3;
pub const PVR2_CLIENT_ID_TUNER: c_int = 4;
pub const PVR2_CLIENT_ID_CS53L32A: c_int = 5;
pub const PVR2_CLIENT_ID_WM8775: c_int = 6;
pub const PVR2_CLIENT_ID_DEMOD: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_device_client_desc {
// One ovr PVR2_CLIENT_ID_xxxx
    pub module_id: c_uchar,
// Null-terminated array of I2C addresses to try in order
    pub i2c_address_list: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_device_client_table {
    pub lst: *const pvr2_device_client_desc,
    pub cnt: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_string_table {
    pub lst: *const c_char,
    pub cnt: c_uint,
}

pub const PVR2_ROUTING_SCHEME_HAUPPAUGE: c_int = 0;
pub const PVR2_ROUTING_SCHEME_GOTVIEW: c_int = 1;
pub const PVR2_ROUTING_SCHEME_ONAIR: c_int = 2;
pub const PVR2_ROUTING_SCHEME_AV400: c_int = 3;
pub const PVR2_ROUTING_SCHEME_HAUP160XXX: c_int = 4;
pub const PVR2_DIGITAL_SCHEME_NONE: c_int = 0;
pub const PVR2_DIGITAL_SCHEME_HAUPPAUGE: c_int = 1;
pub const PVR2_DIGITAL_SCHEME_ONAIR: c_int = 2;
pub const PVR2_LED_SCHEME_NONE: c_int = 0;
pub const PVR2_LED_SCHEME_HAUPPAUGE: c_int = 1;
pub const PVR2_IR_SCHEME_NONE: c_int = 0;

// This describes a particular hardware type (except for the USB device ID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_device_desc {
// Single line text description of hardware
    pub description: *const c_char,
// Single token identifier for hardware
    pub shortname: *const c_char,
// List of additional client modules we need to load
    pub client_modules: pvr2_string_table,
// List of defined client modules we need to load
    pub client_table: pvr2_device_client_table,
// List of FX2 firmware file names we should search; if empty then
    pub fx2_firmware: pvr2_string_table,

// callback functions to handle attachment of digital tuner & demod
    pub dvb_props: *const pvr2_dvb_props,

// Initial standard bits to use for this device, if not zero.
    pub default_std_mask: v4l2_std_id,
// V4L tuner type ID to use with this device (only used if the
    pub default_tuner_type: c_int,
// Signal routing scheme used by device, contains one of
    pub its: encounter them. This is an arbitrary integer scheme id;,
    pub signal_routing_scheme: c_uchar,
// Indicates scheme for controlling device's LED (if any).  The
    pub led_scheme: c_uchar,
// Control scheme to use if there is a digital tuner.  This
    pub the: integer scheme id; its meaning is contained entirely within,
    pub digital_control_scheme: c_uchar,
// If set, we don't bother trying to load cx23416 firmware.
    pub flag_skip_cx23416_firmware:1: c_uint,
// If set, the encoder must be healthy in order for digital mode to
    pub flag_digital_requires_cx23416:1: c_uint,
// Device has a hauppauge eeprom which we can interrogate.
    pub flag_has_hauppauge_rom:1: c_uint,
// Device does not require a powerup command to be issued.
    pub flag_no_powerup:1: c_uint,
// Device has a cx25840 - this enables special additional logic to
    pub flag_has_cx25840:1: c_uint,
// Device has a wm8775 - this enables special additional logic to
    pub flag_has_wm8775:1: c_uint,
// Indicate IR scheme of hardware.  If not set, then it is assumed
    pub ir_scheme:3: c_uint,
// These bits define which kinds of sources the device can handle.
    pub /: *mut *mut unsigned int flag_has_fmradio:1; / Has FM radio receiver,
    pub /: *mut *mut unsigned int flag_has_analogtuner:1; / Has analog tuner,
    pub /: *mut *mut unsigned int flag_has_composite:1; / Has composite input,
    pub /: *mut *mut unsigned int flag_has_svideo:1; / Has s-video input,
    pub /: *mut *mut unsigned int flag_fx2_16kb:1; / 16KB FX2 firmware OK here,
// If this driver is considered experimental, i.e. not all aspects
    pub flag_is_experimental:1: c_uint,
}
