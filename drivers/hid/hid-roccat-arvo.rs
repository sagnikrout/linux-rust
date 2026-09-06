//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/hid-roccat-arvo.h
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
// Copyright (c) 2011 Stefan Achatz <erazor_de@users.sourceforge.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arvo_mode_key {
    pub /: *mut *mut uint8_t command; / ARVO_COMMAND_MODE_KEY,
    pub state: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arvo_button {
    pub unknown: [u8; 24],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arvo_info {
    pub unknown: [u8; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arvo_key_mask {
    pub /: *mut *mut uint8_t command; / ARVO_COMMAND_KEY_MASK,
    pub key_mask: u8,
    pub __packed: },
// selected profile is persistent
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arvo_actual_profile {
    pub /: *mut *mut uint8_t command; / ARVO_COMMAND_ACTUAL_PROFILE,
    pub actual_profile: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arvo_commands {
    ARVO_COMMAND_MODE_KEY = 0x3,
    ARVO_COMMAND_BUTTON = 0x4,
    ARVO_COMMAND_INFO = 0x5,
    ARVO_COMMAND_KEY_MASK = 0x6,
    ARVO_COMMAND_ACTUAL_PROFILE = 0x7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arvo_special_report {
    pub /: *mut *mut uint8_t unknown1; / always 0x01,
    pub event: u8,
    pub /: *mut *mut uint8_t unknown2; / always 0x70,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arvo_special_report_events {
    ARVO_SPECIAL_REPORT_EVENT_ACTION_PRESS = 0x10,
    ARVO_SPECIAL_REPORT_EVENT_ACTION_RELEASE = 0x0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arvo_special_report_event_masks {
    ARVO_SPECIAL_REPORT_EVENT_MASK_ACTION = 0xf0,
    ARVO_SPECIAL_REPORT_EVENT_MASK_BUTTON = 0x0f,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arvo_roccat_report {
    pub profile: u8,
    pub button: u8,
    pub action: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arvo_roccat_report_action {
    ARVO_ROCCAT_REPORT_ACTION_RELEASE = 0,
    ARVO_ROCCAT_REPORT_ACTION_PRESS = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arvo_device {
    pub roccat_claimed: c_int,
    pub chrdev_minor: c_int,
    pub arvo_lock: mutex,
    pub actual_profile: c_int,
}
