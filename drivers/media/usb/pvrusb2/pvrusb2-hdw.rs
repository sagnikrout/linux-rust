//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/pvrusb2/pvrusb2-hdw.h
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

// Private internal control ids, look these up with
pub const PVR2_CID_STDCUR: c_int = 2;
pub const PVR2_CID_STDAVAIL: c_int = 3;
pub const PVR2_CID_INPUT: c_int = 4;
pub const PVR2_CID_AUDIOMODE: c_int = 5;
pub const PVR2_CID_FREQUENCY: c_int = 6;
pub const PVR2_CID_HRES: c_int = 7;
pub const PVR2_CID_VRES: c_int = 8;
pub const PVR2_CID_CROPL: c_int = 9;
pub const PVR2_CID_CROPT: c_int = 10;
pub const PVR2_CID_CROPW: c_int = 11;
pub const PVR2_CID_CROPH: c_int = 12;
pub const PVR2_CID_CROPCAPPAN: c_int = 13;
pub const PVR2_CID_CROPCAPPAD: c_int = 14;
pub const PVR2_CID_CROPCAPBL: c_int = 15;
pub const PVR2_CID_CROPCAPBT: c_int = 16;
pub const PVR2_CID_CROPCAPBW: c_int = 17;
pub const PVR2_CID_CROPCAPBH: c_int = 18;
pub const PVR2_CID_STDDETECT: c_int = 19;
// Legal values for the INPUT state variable
pub const PVR2_CVAL_INPUT_TV: c_int = 0;
pub const PVR2_CVAL_INPUT_DTV: c_int = 1;
pub const PVR2_CVAL_INPUT_COMPOSITE: c_int = 2;
pub const PVR2_CVAL_INPUT_SVIDEO: c_int = 3;
pub const PVR2_CVAL_INPUT_RADIO: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvr2_config {
    pvr2_config_empty,    /* No configuration */
    pvr2_config_mpeg,     /* Encoded / compressed video */
    pvr2_config_vbi,      /* Standard vbi info */
    pvr2_config_pcm,      /* Audio raw pcm stream */
    pvr2_config_rawvideo, /* Video raw frames */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvr2_v4l_type {
    pvr2_v4l_type_video,
    pvr2_v4l_type_vbi,
    pvr2_v4l_type_radio,
}

// Major states that we can be in:
//
// DEAD - Device is in an unusable state and cannot be recovered.  This
// can happen if we completely lose the ability to communicate with it
// (but it might still on the bus).  In this state there's nothing we can
// do; it must be replugged in order to recover.
//
// COLD - Device is in an unusable state, needs microcontroller firmware.
//
// WARM - We can communicate with the device and the proper
// microcontroller firmware is running, but other device initialization is
// still needed (e.g. encoder firmware).
//
// ERROR - A problem prevents capture operation (e.g. encoder firmware
// missing).
//
// READY - Device is operational, but not streaming.
//
// RUN - Device is streaming.
//
pub const PVR2_STATE_NONE: c_int = 0;
pub const PVR2_STATE_DEAD: c_int = 1;
pub const PVR2_STATE_COLD: c_int = 2;
pub const PVR2_STATE_WARM: c_int = 3;
pub const PVR2_STATE_ERROR: c_int = 4;
pub const PVR2_STATE_READY: c_int = 5;
pub const PVR2_STATE_RUN: c_int = 6;
// Translate configuration enum to a string label
// Create and return a structure for interacting with the underlying
// Perform second stage initialization, passing in a notification callback
// Destroy hardware interaction structure
extern "C" {
    pub fn pvr2_hdw_destroy(: *mut pvr2_hdw);
}
// Return true if in the ready (normal) state
extern "C" {
    pub fn pvr2_hdw_dev_ok(: *mut pvr2_hdw) -> c_int;
}
// Return small integer number [1..N] for logical instance number of this
extern "C" {
    pub fn pvr2_hdw_get_unit_number(: *mut pvr2_hdw) -> c_int;
}
// Get pointer to underlying USB device
// Retrieve serial number of device
extern "C" {
    pub fn pvr2_hdw_get_sn(: *mut pvr2_hdw) -> c_ulong;
}
// Retrieve bus location info of device
// Retrieve per-instance string identifier for this specific device
// Called when hardware has been unplugged
extern "C" {
    pub fn pvr2_hdw_disconnect(: *mut pvr2_hdw);
}
// Sets v4l2_dev of a video_device struct
extern "C" {
    pub fn pvr2_hdw_set_v4l2_dev(: *mut pvr2_hdw, : *mut video_device);
}
// Get the number of defined controls
extern "C" {
    pub fn pvr2_hdw_get_ctrl_count(: *mut pvr2_hdw) -> c_uint;
}
// Retrieve a control handle given its index (0..count-1)
// Retrieve a control handle given its internal ID (if any)
// Retrieve a control handle given its V4L ID (if any)
// Retrieve a control handle given its immediate predecessor V4L ID (if any)
// Commit all control changes made up to this point
extern "C" {
    pub fn pvr2_hdw_commit_ctl(: *mut pvr2_hdw) -> c_int;
}
// Return a bit mask of valid input selections for this device.  Mask bits
// will be according to PVR_CVAL_INPUT_xxxx definitions.
extern "C" {
    pub fn pvr2_hdw_get_input_available(: *mut pvr2_hdw) -> c_uint;
}
// Return a bit mask of allowed input selections for this device.  Mask bits
// will be according to PVR_CVAL_INPUT_xxxx definitions.
extern "C" {
    pub fn pvr2_hdw_get_input_allowed(: *mut pvr2_hdw) -> c_uint;
}
// Change the set of allowed input selections for this device.  Both
// Return name for this driver instance
// Mark tuner status stale so that it will be re-fetched
extern "C" {
    pub fn pvr2_hdw_execute_tuner_poll(: *mut pvr2_hdw);
}
// Return information about the tuner
extern "C" {
    pub fn pvr2_hdw_get_tuner_status(: *mut pvr2_hdw, : *mut v4l2_tuner) -> c_int;
}
// Return information about cropping capabilities
extern "C" {
    pub fn pvr2_hdw_get_cropcap(: *mut pvr2_hdw, : *mut v4l2_cropcap) -> c_int;
}
// Query device and see if it thinks it is on a high-speed USB link
extern "C" {
    pub fn pvr2_hdw_is_hsm(: *mut pvr2_hdw) -> c_int;
}
// Return a string token representative of the hardware type
// Return a single line description of the hardware type
// Turn streaming on/off
extern "C" {
    pub fn pvr2_hdw_set_streaming(: *mut pvr2_hdw, _arg: c_int) -> c_int;
}
// Find out if streaming is on
extern "C" {
    pub fn pvr2_hdw_get_streaming(: *mut pvr2_hdw) -> c_int;
}
// Retrieve driver overall state
extern "C" {
    pub fn pvr2_hdw_get_state(: *mut pvr2_hdw) -> c_int;
}
// Configure the type of stream to generate
extern "C" {
    pub fn pvr2_hdw_set_stream_type(: *mut pvr2_hdw, pvr2_config: enum) -> c_int;
}
// Get handle to video output stream
// Enable / disable retrieval of CPU firmware or prom contents.  This must
// Return true if we're in a mode for retrieval CPU firmware
extern "C" {
    pub fn pvr2_hdw_cpufw_get_enabled(: *mut pvr2_hdw) -> c_int;
}
// Retrieve a piece of the CPU's firmware at the given offset.  Return
// Retrieve a previously stored v4l minor device number
extern "C" {
    pub fn pvr2_hdw_v4l_get_minor_number(: *mut pvr2_hdw, index: pvr2_v4l_type) -> c_int;
}
// Store a v4l minor device number
// The following entry points are all lower level things you normally don't
// Issue a command and get a response from the device.  LOTS of higher
// Slightly higher level device communication functions.
extern "C" {
    pub fn pvr2_write_register(: *mut pvr2_hdw, _arg: u16, _arg: u32) -> c_int;
}
// Call if for any reason we can't talk to the hardware anymore - this will
extern "C" {
    pub fn pvr2_hdw_render_useless(: *mut pvr2_hdw);
}
// Set / clear 8051's reset bit
extern "C" {
    pub fn pvr2_hdw_cpureset_assert(: *mut pvr2_hdw, _arg: c_int);
}
// Execute a USB-commanded device reset
extern "C" {
    pub fn pvr2_hdw_device_reset(: *mut pvr2_hdw);
}
// Reset worker's error trapping circuit breaker
extern "C" {
    pub fn pvr2_hdw_untrip(: *mut pvr2_hdw) -> c_int;
}
// Execute hard reset command (after this point it's likely that the
extern "C" {
    pub fn pvr2_hdw_cmd_deep_reset(: *mut pvr2_hdw) -> c_int;
}
// Execute simple reset command
extern "C" {
    pub fn pvr2_hdw_cmd_powerup(: *mut pvr2_hdw) -> c_int;
}
// Order decoder to reset
extern "C" {
    pub fn pvr2_hdw_cmd_decoder_reset(: *mut pvr2_hdw) -> c_int;
}
// Direct manipulation of GPIO bits
extern "C" {
    pub fn pvr2_hdw_gpio_get_dir(hdw: *mut pvr2_hdw, : *mut u32) -> c_int;
}
extern "C" {
    pub fn pvr2_hdw_gpio_get_out(hdw: *mut pvr2_hdw, : *mut u32) -> c_int;
}
extern "C" {
    pub fn pvr2_hdw_gpio_get_in(hdw: *mut pvr2_hdw, : *mut u32) -> c_int;
}
extern "C" {
    pub fn pvr2_hdw_gpio_chg_dir(hdw: *mut pvr2_hdw, msk: u32, val: u32) -> c_int;
}
extern "C" {
    pub fn pvr2_hdw_gpio_chg_out(hdw: *mut pvr2_hdw, msk: u32, val: u32) -> c_int;
}
// This data structure is specifically for the next function...
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_hdw_debug_info {
    pub big_lock_held: c_int,
    pub ctl_lock_held: c_int,
    pub flag_disconnected: c_int,
    pub flag_init_ok: c_int,
    pub flag_ok: c_int,
    pub fw1_state: c_int,
    pub flag_decoder_missed: c_int,
    pub flag_tripped: c_int,
    pub state_encoder_ok: c_int,
    pub state_encoder_run: c_int,
    pub state_decoder_run: c_int,
    pub state_decoder_ready: c_int,
    pub state_usbstream_run: c_int,
    pub state_decoder_quiescent: c_int,
    pub state_pipeline_config: c_int,
    pub state_pipeline_req: c_int,
    pub state_pipeline_pause: c_int,
    pub state_pipeline_idle: c_int,
    pub cmd_debug_state: c_int,
    pub cmd_debug_write_len: c_int,
    pub cmd_debug_read_len: c_int,
    pub cmd_debug_write_pend: c_int,
    pub cmd_debug_read_pend: c_int,
    pub cmd_debug_timeout: c_int,
    pub cmd_debug_rstatus: c_int,
    pub cmd_debug_wstatus: c_int,
    pub cmd_code: c_uchar,
}

// Non-intrusively retrieve internal state info - this is useful for
// Intrusively retrieve internal state info - this is useful for
// Report out several lines of text that describes driver internal state.
// Cause modules to log their state once
extern "C" {
    pub fn pvr2_hdw_trigger_module_log(hdw: *mut pvr2_hdw);
}
// Cause encoder firmware to be uploaded into the device.  This is normally
extern "C" {
    pub fn pvr2_upload_firmware2(hdw: *mut pvr2_hdw) -> c_int;
}
