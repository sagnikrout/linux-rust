//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/hid-uclogic-params.h
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
// - tablet initialization and parameter retrieval
//
// Copyright (c) 2018 Nikolai Kondrashov
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//

// Types of pen in-range reporting
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uclogic_params_pen_inrange {
// Normal reports: zero - out of proximity, one - in proximity
    UCLOGIC_PARAMS_PEN_INRANGE_NORMAL = 0,
// Inverted reports: zero - in proximity, one - out of proximity
    UCLOGIC_PARAMS_PEN_INRANGE_INVERTED,
// No reports
    UCLOGIC_PARAMS_PEN_INRANGE_NONE,
}

// Types of frames
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uclogic_params_frame_type {
// Frame with buttons
    UCLOGIC_PARAMS_FRAME_BUTTONS = 0,
// Frame with buttons and a dial
    UCLOGIC_PARAMS_FRAME_DIAL,
// Frame with buttons and a mouse (shaped as a dial + touchpad)
    UCLOGIC_PARAMS_FRAME_MOUSE,
}

//
// Pen report's subreport data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uclogic_params_pen_subreport {
//
// The value of the second byte of the pen report indicating this
// subreport. If zero, the subreport should be considered invalid and
// not matched.
//
    pub value: __u8,
//
// The ID to be assigned to the report, if the second byte of the pen
// report is equal to "value". Only valid if "value" is not zero.
//
    pub id: __u8,
}

//
// Tablet interface's pen input parameters.
//
// Must use declarative (descriptive) language, not imperative, to simplify
// understanding and maintain consistency.
//
// Noop (preserving functionality) when filled with zeroes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uclogic_params_pen {
//
// True if pen usage is invalid for this interface and should be
// ignored, false otherwise.
//
    pub usage_invalid: bool,
//
// Pointer to report descriptor part describing the pen inputs.
// Allocated with kmalloc. NULL if the part is not specified.
//
    pub desc_ptr: *const __u8,
//
// Size of the report descriptor.
// Only valid, if "desc_ptr" is not NULL.
//
    pub desc_size: c_uint,
// Report ID, if reports should be tweaked, zero if not
    pub id: c_uint,
// The list of subreports, only valid if "id" is not zero
    pub subreport_list: [uclogic_params_pen_subreport; 3],
// Type of in-range reporting, only valid if "id" is not zero
    pub inrange: uclogic_params_pen_inrange,
//
// True, if reports include fragmented high resolution coords, with
// high-order X and then Y bytes following the pressure field.
// Only valid if "id" is not zero.
//
    pub fragmented_hires: bool,
//
// True if the pen reports tilt in bytes at offset 10 (X) and 11 (Y),
// and the Y tilt direction is flipped.
// Only valid if "id" is not zero.
//
    pub tilt_y_flipped: bool,
//
// True, if reports include fragmented high resolution X coords.
// This moves bytes 10-11 to the LSB of the X coordinate.
//
    pub fragmented_hires2: bool,
}

//
// Parameters of frame control inputs of a tablet interface.
//
// Must use declarative (descriptive) language, not imperative, to simplify
// understanding and maintain consistency.
//
// Noop (preserving functionality) when filled with zeroes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uclogic_params_frame {
//
// Pointer to report descriptor part describing the frame inputs.
// Allocated with kmalloc. NULL if the part is not specified.
//
    pub desc_ptr: *const __u8,
//
// Size of the report descriptor.
// Only valid, if "desc_ptr" is not NULL.
//
    pub desc_size: c_uint,
//
// Report ID, if reports should be tweaked, zero if not.
//
    pub id: c_uint,
//
// The suffix to add to the input device name, if not NULL.
//
    pub suffix: *const c_char,
//
// Number of the least-significant bit of the 2-bit state of a rotary
// encoder, in the report. Cannot point to a 2-bit field crossing a
// byte boundary. Zero if not present. Only valid if "id" is not zero.
//
    pub re_lsb: c_uint,
//
// Offset of the Wacom-style device ID byte in the report, to be set
// to pad device ID (0xf), for compatibility with Wacom drivers. Zero
// if no changes to the report should be made. The ID byte will be set
// to zero whenever the byte pointed by "touch_byte" is zero, if
// the latter is valid. Only valid if "id" is not zero.
//
    pub dev_id_byte: c_uint,
//
// Offset of the touch ring/strip state byte, in the report.
// Zero if not present. If dev_id_byte is also valid and non-zero,
// then the device ID byte will be cleared when the byte pointed to by
// this offset is zero. Only valid if "id" is not zero.
//
    pub touch_byte: c_uint,
//
// The value to anchor the reversed touch ring/strip reports at.
// I.e. one, if the reports should be flipped without offset.
// Zero if no reversal should be done.
// Only valid if "touch_byte" is valid and not zero.
//
    pub touch_flip_at: __s8,
//
// Maximum value of the touch ring/strip report around which the value
// should be wrapped when flipping according to "touch_flip_at".
// The minimum valid value is considered to be one, with zero being
// out-of-proximity (finger lift) value.
// Only valid if "touch_flip_at" is valid and not zero.
//
    pub touch_max: __s8,
//
// Offset of the bitmap dial byte, in the report. Zero if not present.
// Only valid if "id" is not zero. A bitmap dial sends reports with a
// dedicated bit per direction: 1 means clockwise rotation, 2 means
// counterclockwise, as opposed to the normal 1 and -1.
//
    pub bitmap_dial_byte: c_uint,
//
// Destination offset for the second bitmap dial byte, if the tablet
// supports a second dial at all.
//
    pub bitmap_second_dial_destination_byte: c_uint,
}

//
// List of works to be performed when a certain raw event is received.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uclogic_raw_event_hook {
    pub hdev: *mut hid_device,
    pub event: *mut __u8,
    pub size: usize,
    pub work: work_struct,
    pub list: list_head,
}

//
// Tablet interface report parameters.
//
// Must use declarative (descriptive) language, not imperative, to simplify
// understanding and maintain consistency.
//
// When filled with zeros represents a "noop" configuration - passes all
// reports unchanged and lets the generic HID driver handle everything.
//
// The resulting device report descriptor is assembled from all the report
// descriptor parts referenced by the structure. No order of assembly should
// be assumed. The structure represents original device report descriptor if
// all the parts are NULL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uclogic_params {
//
// True if the whole interface is invalid, false otherwise.
//
    pub invalid: bool,
//
// Pointer to the common part of the replacement report descriptor,
// allocated with kmalloc. NULL if no common part is needed.
// Only valid, if "invalid" is false.
//
    pub desc_ptr: *const __u8,
//
// Size of the common part of the replacement report descriptor.
// Only valid, if "desc_ptr" is valid and not NULL.
//
    pub desc_size: c_uint,
//
// Pen parameters and optional report descriptor part.
// Only valid, if "invalid" is false.
//
    pub pen: uclogic_params_pen,
//
// The list of frame control parameters and optional report descriptor
// parts. Only valid, if "invalid" is false.
//
    pub frame_list: [uclogic_params_frame; 3],
//
// List of event hooks.
//
    pub event_hooks: *mut uclogic_raw_event_hook,
}

// Driver data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uclogic_drvdata {
// Interface parameters
    pub params: uclogic_params,
// Pointer to the replacement report descriptor. NULL if none.
    pub desc_ptr: *const __u8,
//
// Size of the replacement report descriptor.
// Only valid if desc_ptr is not NULL
//
    pub desc_size: c_uint,
// Pen input device
    pub pen_input: *mut input_dev,
// In-range timer
    pub inrange_timer: timer_list,
// Last rotary encoder state, or U8_MAX for none
    pub re_state: u8,
// Device quirks
    pub quirks: c_ulong,
}

// Initialize a tablet interface and discover its parameters
// Get a replacement report descriptor for a tablet's interface.
// Free resources used by tablet interface's parameters
extern "C" {
    pub fn uclogic_params_cleanup(params: *mut uclogic_params);
}
// Dump tablet interface parameters with hid_dbg()
