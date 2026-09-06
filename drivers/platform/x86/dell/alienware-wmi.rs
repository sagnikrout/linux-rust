//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/dell/alienware-wmi.h
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
// Alienware WMI special features driver
//
// Copyright (C) 2014 Dell Inc <Dell.Client.Kernel@dell.com>
// Copyright (C) 2024 Kurt Borja <kuurtb@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum INTERFACE_FLAGS {
    LEGACY,
    WMAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LEGACY_CONTROL_STATES {
    LEGACY_RUNNING = 1,
    LEGACY_BOOTING = 0,
    LEGACY_SUSPEND = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WMAX_CONTROL_STATES {
    WMAX_RUNNING = 0xFF,
    WMAX_BOOTING = 0,
    WMAX_SUSPEND = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alienfx_quirks {
    pub num_zones: u8,
    pub hdmi_mux: bool,
    pub amplifier: bool,
    pub deepslp: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct color_platform {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alienfx_priv {
    pub pdev: *mut platform_device,
    pub global_led: led_classdev,
    pub colors: [color_platform; 4],
    pub global_brightness: u8,
    pub lighting_control_state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alienfx_ops {
    pub location): u8,
    pub brightness): u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alienfx_platdata {
    pub wdev: *mut wmi_device,
    pub ops: alienfx_ops,
}

extern "C" {
    pub fn alienware_alienfx_setup(pdata: *mut alienfx_platdata) -> c_int;
}

extern "C" {
    pub fn alienware_legacy_wmi_init() -> int __init;
}
extern "C" {
    pub fn alienware_legacy_wmi_exit() -> void __exit;
}

extern "C" {
    pub fn alienware_wmax_wmi_init() -> int __init;
}
extern "C" {
    pub fn alienware_wmax_wmi_exit() -> void __exit;
}

// Macro flag: #define WMAX_DEV_GROUPS

