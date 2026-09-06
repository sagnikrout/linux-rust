//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/hid-roccat-pyra.h
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
//
// Copyright (c) 2010 Stefan Achatz <erazor_de@users.sourceforge.net>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pyra_control_requests {
    PYRA_CONTROL_REQUEST_PROFILE_SETTINGS = 0x10,
    PYRA_CONTROL_REQUEST_PROFILE_BUTTONS = 0x20
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pyra_settings {
    pub /: *mut *mut uint8_t command; / PYRA_COMMAND_SETTINGS,
    pub /: *mut *mut uint8_t size; / always 3,
    pub /: *mut *mut uint8_t startup_profile; / Range 0-4!,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pyra_profile_settings {
    pub /: *mut *mut uint8_t command; / PYRA_COMMAND_PROFILE_SETTINGS,
    pub /: *mut *mut uint8_t size; / always 0xd,
    pub /: *mut *mut uint8_t number; / Range 0-4,
    pub xysync: u8,
    pub /: *mut *mut uint8_t x_sensitivity; / 0x1-0xa,
    pub y_sensitivity: u8,
    pub /: *mut *mut uint8_t x_cpi; / unused,
    pub /: *mut *mut uint8_t y_cpi; / this value is for x and y,
    pub /: *mut *mut uint8_t lightswitch; / 0 = off, 1 = on,
    pub light_effect: u8,
    pub handedness: u8,
    pub /: *mut *mut uint16_t checksum; / byte sum,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pyra_info {
    pub /: *mut *mut uint8_t command; / PYRA_COMMAND_INFO,
    pub /: *mut *mut uint8_t size; / always 6,
    pub firmware_version: u8,
    pub /: *mut *mut uint8_t unknown1; / always 0,
    pub /: *mut *mut uint8_t unknown2; / always 1,
    pub /: *mut *mut uint8_t unknown3; / always 0,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pyra_commands {
    PYRA_COMMAND_CONTROL = 0x4,
    PYRA_COMMAND_SETTINGS = 0x5,
    PYRA_COMMAND_PROFILE_SETTINGS = 0x6,
    PYRA_COMMAND_PROFILE_BUTTONS = 0x7,
    PYRA_COMMAND_INFO = 0x9,
    PYRA_COMMAND_B = 0xb
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pyra_mouse_report_numbers {
    PYRA_MOUSE_REPORT_NUMBER_HID = 1,
    PYRA_MOUSE_REPORT_NUMBER_AUDIO = 2,
    PYRA_MOUSE_REPORT_NUMBER_BUTTON = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pyra_mouse_event_button {
    pub /: *mut *mut uint8_t report_number; / always 3,
    pub /: *mut *mut uint8_t unknown; / always 0,
    pub type: u8,
    pub data1: u8,
    pub data2: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pyra_mouse_event_audio {
    pub /: *mut *mut uint8_t report_number; / always 2,
    pub type: u8,
    pub /: *mut *mut uint8_t unused; / always 0,
// C attribute field omitted
// hid audio controls
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pyra_mouse_event_audio_types {
    PYRA_MOUSE_EVENT_AUDIO_TYPE_MUTE = 0xe2,
    PYRA_MOUSE_EVENT_AUDIO_TYPE_VOLUME_UP = 0xe9,
    PYRA_MOUSE_EVENT_AUDIO_TYPE_VOLUME_DOWN = 0xea,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pyra_mouse_event_button_types {
//
// Mouse sends tilt events on report_number 1 and 3
// Tilt events are sent repeatedly with 0.94s between first and second
// event and 0.22s on subsequent
//
    PYRA_MOUSE_EVENT_BUTTON_TYPE_TILT = 0x10,

//
// These are sent sequentially
// data1 contains new profile number in range 1-5
//
    PYRA_MOUSE_EVENT_BUTTON_TYPE_PROFILE_1 = 0x20,
    PYRA_MOUSE_EVENT_BUTTON_TYPE_PROFILE_2 = 0x30,

//
// data1 = button_number (rmp index)
// data2 = pressed/released
//
    PYRA_MOUSE_EVENT_BUTTON_TYPE_MACRO = 0x40,
    PYRA_MOUSE_EVENT_BUTTON_TYPE_SHORTCUT = 0x50,

//
// data1 = button_number (rmp index)
//
    PYRA_MOUSE_EVENT_BUTTON_TYPE_QUICKLAUNCH = 0x60,

// data1 = new cpi
    PYRA_MOUSE_EVENT_BUTTON_TYPE_CPI = 0xb0,

// data1 and data2 = new sensitivity
    PYRA_MOUSE_EVENT_BUTTON_TYPE_SENSITIVITY = 0xc0,

    PYRA_MOUSE_EVENT_BUTTON_TYPE_MULTIMEDIA = 0xf0,
}

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pyra_roccat_report {
    pub type: u8,
    pub value: u8,
    pub key: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pyra_device {
    pub actual_profile: c_int,
    pub actual_cpi: c_int,
    pub roccat_claimed: c_int,
    pub chrdev_minor: c_int,
    pub pyra_lock: mutex,
    pub profile_settings: [pyra_profile_settings; 5],
}
