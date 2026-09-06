//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/hid-roccat-isku.h
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
pub struct isku_actual_profile {
    pub /: *mut *mut uint8_t command; / ISKU_COMMAND_ACTUAL_PROFILE,
    pub /: *mut *mut uint8_t size; / always 3,
    pub actual_profile: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isku_commands {
    ISKU_COMMAND_CONTROL = 0x4,
    ISKU_COMMAND_ACTUAL_PROFILE = 0x5,
    ISKU_COMMAND_KEY_MASK = 0x7,
    ISKU_COMMAND_KEYS_FUNCTION = 0x8,
    ISKU_COMMAND_KEYS_EASYZONE = 0x9,
    ISKU_COMMAND_KEYS_MEDIA = 0xa,
    ISKU_COMMAND_KEYS_THUMBSTER = 0xb,
    ISKU_COMMAND_KEYS_MACRO = 0xd,
    ISKU_COMMAND_MACRO = 0xe,
    ISKU_COMMAND_INFO = 0xf,
    ISKU_COMMAND_LIGHT = 0x10,
    ISKU_COMMAND_RESET = 0x11,
    ISKU_COMMAND_KEYS_CAPSLOCK = 0x13,
    ISKU_COMMAND_LAST_SET = 0x14,
    ISKU_COMMAND_15 = 0x15,
    ISKU_COMMAND_TALK = 0x16,
    ISKU_COMMAND_TALKFX = 0x17,
    ISKU_COMMAND_FIRMWARE_WRITE = 0x1b,
    ISKU_COMMAND_FIRMWARE_WRITE_CONTROL = 0x1c,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isku_report_button {
    pub /: *mut *mut uint8_t number; / ISKU_REPORT_NUMBER_BUTTON,
    pub zero: u8,
    pub event: u8,
    pub data1: u8,
    pub data2: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isku_report_numbers {
    ISKU_REPORT_NUMBER_BUTTON = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isku_report_button_events {
    ISKU_REPORT_BUTTON_EVENT_PROFILE = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isku_roccat_report {
    pub event: u8,
    pub data1: u8,
    pub data2: u8,
    pub profile: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isku_device {
    pub roccat_claimed: c_int,
    pub chrdev_minor: c_int,
    pub isku_lock: mutex,
    pub actual_profile: c_int,
}
