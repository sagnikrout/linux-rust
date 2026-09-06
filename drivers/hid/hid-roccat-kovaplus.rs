//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/hid-roccat-kovaplus.h
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
pub enum kovaplus_control_requests {
// write; value = profile number range 0-4
    KOVAPLUS_CONTROL_REQUEST_PROFILE_SETTINGS = 0x10,
// write; value = profile number range 0-4
    KOVAPLUS_CONTROL_REQUEST_PROFILE_BUTTONS = 0x20,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kovaplus_actual_profile {
    pub /: *mut *mut uint8_t command; / KOVAPLUS_COMMAND_ACTUAL_PROFILE,
    pub /: *mut *mut uint8_t size; / always 3,
    pub /: *mut *mut uint8_t actual_profile; / Range 0-4!,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kovaplus_profile_settings {
    pub /: *mut *mut uint8_t command; / KOVAPLUS_COMMAND_PROFILE_SETTINGS,
    pub /: *mut *mut uint8_t size; / 16,
    pub /: *mut *mut uint8_t profile_index; / range 0-4,
    pub unknown1: u8,
    pub /: *mut *mut uint8_t sensitivity_x; / range 1-10,
    pub /: *mut *mut uint8_t sensitivity_y; / range 1-10,
    pub cpi_levels_enabled: u8,
    pub /: *mut *mut uint8_t cpi_startup_level; / range 1-4,
    pub data: [u8; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kovaplus_profile_buttons {
    pub /: *mut *mut uint8_t command; / KOVAPLUS_COMMAND_PROFILE_BUTTONS,
    pub /: *mut *mut uint8_t size; / 23,
    pub /: *mut *mut uint8_t profile_index; / range 0-4,
    pub data: [u8; 20],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kovaplus_info {
    pub /: *mut *mut uint8_t command; / KOVAPLUS_COMMAND_INFO,
    pub /: *mut *mut uint8_t size; / 6,
    pub firmware_version: u8,
    pub unknown: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kovaplus_commands {
    KOVAPLUS_COMMAND_ACTUAL_PROFILE = 0x5,
    KOVAPLUS_COMMAND_CONTROL = 0x4,
    KOVAPLUS_COMMAND_PROFILE_SETTINGS = 0x6,
    KOVAPLUS_COMMAND_PROFILE_BUTTONS = 0x7,
    KOVAPLUS_COMMAND_INFO = 0x9,
    KOVAPLUS_COMMAND_A = 0xa,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kovaplus_mouse_report_numbers {
    KOVAPLUS_MOUSE_REPORT_NUMBER_MOUSE = 1,
    KOVAPLUS_MOUSE_REPORT_NUMBER_AUDIO = 2,
    KOVAPLUS_MOUSE_REPORT_NUMBER_BUTTON = 3,
    KOVAPLUS_MOUSE_REPORT_NUMBER_KBD = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kovaplus_mouse_report_button {
    pub /: *mut *mut uint8_t report_number; / KOVAPLUS_MOUSE_REPORT_NUMBER_BUTTON,
    pub unknown1: u8,
    pub type: u8,
    pub data1: u8,
    pub data2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kovaplus_mouse_report_button_types {
// data1 = profile_number range 1-5; no release event
    KOVAPLUS_MOUSE_REPORT_BUTTON_TYPE_PROFILE_1 = 0x20,
// data1 = profile_number range 1-5; no release event
    KOVAPLUS_MOUSE_REPORT_BUTTON_TYPE_PROFILE_2 = 0x30,
// data1 = button_number range 1-18; data2 = action
    KOVAPLUS_MOUSE_REPORT_BUTTON_TYPE_MACRO = 0x40,
// data1 = button_number range 1-18; data2 = action
    KOVAPLUS_MOUSE_REPORT_BUTTON_TYPE_SHORTCUT = 0x50,
// data1 = button_number range 1-18; data2 = action
    KOVAPLUS_MOUSE_REPORT_BUTTON_TYPE_QUICKLAUNCH = 0x60,
// data1 = button_number range 1-18; data2 = action
    KOVAPLUS_MOUSE_REPORT_BUTTON_TYPE_TIMER = 0x80,
// data1 = 1 = 400, 2 = 800, 4 = 1600, 7 = 3200; no release event
    KOVAPLUS_MOUSE_REPORT_BUTTON_TYPE_CPI = 0xb0,
// data1 + data2 = sense range 1-10; no release event
    KOVAPLUS_MOUSE_REPORT_BUTTON_TYPE_SENSITIVITY = 0xc0,
// data1 = type as in profile_buttons; data2 = action
    KOVAPLUS_MOUSE_REPORT_BUTTON_TYPE_MULTIMEDIA = 0xf0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kovaplus_mouse_report_button_actions {
    KOVAPLUS_MOUSE_REPORT_BUTTON_ACTION_PRESS = 0,
    KOVAPLUS_MOUSE_REPORT_BUTTON_ACTION_RELEASE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kovaplus_roccat_report {
    pub type: u8,
    pub profile: u8,
    pub button: u8,
    pub data1: u8,
    pub data2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kovaplus_device {
    pub actual_profile: c_int,
    pub actual_cpi: c_int,
    pub actual_x_sensitivity: c_int,
    pub actual_y_sensitivity: c_int,
    pub roccat_claimed: c_int,
    pub chrdev_minor: c_int,
    pub kovaplus_lock: mutex,
    pub profile_settings: [kovaplus_profile_settings; 5],
    pub profile_buttons: [kovaplus_profile_buttons; 5],
}
