//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/hid-roccat-koneplus.h
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
pub enum koneplus_control_requests {
    KONEPLUS_CONTROL_REQUEST_PROFILE_SETTINGS = 0x80,
    KONEPLUS_CONTROL_REQUEST_PROFILE_BUTTONS = 0x90,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct koneplus_actual_profile {
    pub /: *mut *mut uint8_t command; / KONEPLUS_COMMAND_ACTUAL_PROFILE,
    pub /: *mut *mut uint8_t size; / always 3,
    pub /: *mut *mut uint8_t actual_profile; / Range 0-4!,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct koneplus_info {
    pub /: *mut *mut uint8_t command; / KONEPLUS_COMMAND_INFO,
    pub /: *mut *mut uint8_t size; / always 6,
    pub firmware_version: u8,
    pub unknown: [u8; 3],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum koneplus_commands {
    KONEPLUS_COMMAND_ACTUAL_PROFILE = 0x5,
    KONEPLUS_COMMAND_CONTROL = 0x4,
    KONEPLUS_COMMAND_PROFILE_SETTINGS = 0x6,
    KONEPLUS_COMMAND_PROFILE_BUTTONS = 0x7,
    KONEPLUS_COMMAND_MACRO = 0x8,
    KONEPLUS_COMMAND_INFO = 0x9,
    KONEPLUS_COMMAND_TCU = 0xc,
    KONEPLUS_COMMAND_TCU_IMAGE = 0xc,
    KONEPLUS_COMMAND_E = 0xe,
    KONEPLUS_COMMAND_SENSOR = 0xf,
    KONEPLUS_COMMAND_TALK = 0x10,
    KONEPLUS_COMMAND_FIRMWARE_WRITE = 0x1b,
    KONEPLUS_COMMAND_FIRMWARE_WRITE_CONTROL = 0x1c,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum koneplus_mouse_report_numbers {
    KONEPLUS_MOUSE_REPORT_NUMBER_HID = 1,
    KONEPLUS_MOUSE_REPORT_NUMBER_AUDIO = 2,
    KONEPLUS_MOUSE_REPORT_NUMBER_BUTTON = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct koneplus_mouse_report_button {
    pub /: *mut *mut uint8_t report_number; / always KONEPLUS_MOUSE_REPORT_NUMBER_BUTTON,
    pub zero1: u8,
    pub type: u8,
    pub data1: u8,
    pub data2: u8,
    pub zero2: u8,
    pub unknown: [u8; 2],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum koneplus_mouse_report_button_types {
// data1 = new profile range 1-5
    KONEPLUS_MOUSE_REPORT_BUTTON_TYPE_PROFILE = 0x20,

// data1 = button number range 1-24; data2 = action
    KONEPLUS_MOUSE_REPORT_BUTTON_TYPE_QUICKLAUNCH = 0x60,

// data1 = button number range 1-24; data2 = action
    KONEPLUS_MOUSE_REPORT_BUTTON_TYPE_TIMER = 0x80,

// data1 = setting number range 1-5
    KONEPLUS_MOUSE_REPORT_BUTTON_TYPE_CPI = 0xb0,

// data1 and data2 = range 0x1-0xb
    KONEPLUS_MOUSE_REPORT_BUTTON_TYPE_SENSITIVITY = 0xc0,

// data1 = 22 = next track...
// data2 = action
//
    KONEPLUS_MOUSE_REPORT_BUTTON_TYPE_MULTIMEDIA = 0xf0,
    KONEPLUS_MOUSE_REPORT_TALK = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum koneplus_mouse_report_button_action {
    KONEPLUS_MOUSE_REPORT_BUTTON_ACTION_PRESS = 0,
    KONEPLUS_MOUSE_REPORT_BUTTON_ACTION_RELEASE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct koneplus_roccat_report {
    pub type: u8,
    pub data1: u8,
    pub data2: u8,
    pub profile: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct koneplus_device {
    pub actual_profile: c_int,
    pub roccat_claimed: c_int,
    pub chrdev_minor: c_int,
    pub koneplus_lock: mutex,
}
