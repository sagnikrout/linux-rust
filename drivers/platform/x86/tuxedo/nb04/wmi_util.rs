//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/tuxedo/nb04/wmi_util.h
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
// This code gives functions to avoid code duplication while interacting with
// the TUXEDO NB04 wmi interfaces.
//
// Copyright (C) 2024-2025 Werner Sembach <wse@tuxedocomputers.com>
//

pub const TUX_GET_DEVICE_STATUS_DEVICE_ID_TOUCHPAD: c_int = 1;
pub const TUX_GET_DEVICE_STATUS_DEVICE_ID_KEYBOARD: c_int = 2;
pub const TUX_GET_DEVICE_STATUS_DEVICE_ID_APP_PAGES: c_int = 3;
pub const TUX_GET_DEVICE_STATUS_KBL_TYPE_NONE: c_int = 0;
pub const TUX_GET_DEVICE_STATUS_KBL_TYPE_PER_KEY: c_int = 1;
pub const TUX_GET_DEVICE_STATUS_KBL_TYPE_FOUR_ZONE: c_int = 2;
pub const TUX_GET_DEVICE_STATUS_KBL_TYPE_WHITE_ONLY: c_int = 3;
pub const TUX_GET_DEVICE_STATUS_KEYBOARD_LAYOUT_ANSII: c_int = 0;
pub const TUX_GET_DEVICE_STATUS_KEYBOARD_LAYOUT_ISO: c_int = 1;
pub const TUX_GET_DEVICE_STATUS_COLOR_ID_RED: c_int = 1;
pub const TUX_GET_DEVICE_STATUS_COLOR_ID_GREEN: c_int = 2;
pub const TUX_GET_DEVICE_STATUS_COLOR_ID_YELLOW: c_int = 3;
pub const TUX_GET_DEVICE_STATUS_COLOR_ID_BLUE: c_int = 4;
pub const TUX_GET_DEVICE_STATUS_COLOR_ID_PURPLE: c_int = 5;
pub const TUX_GET_DEVICE_STATUS_COLOR_ID_INDIGO: c_int = 6;
pub const TUX_GET_DEVICE_STATUS_COLOR_ID_WHITE: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub union tux_wmi_xx_8in_80out_in_t {
    pub raw: [u8; 8],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub device_type: u8,
    pub reserved: [u8; 7],
    pub get_device_status_in: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union tux_wmi_xx_8in_80out_out_t {
    pub raw: [u8; 80],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub return_status: u16,
    pub device_enabled: u8,
    pub kbl_type: u8,
    pub kbl_side_bar_supported: u8,
    pub keyboard_physical_layout: u8,
    pub app_pages: u8,
    pub per_key_kbl_default_color: u8,
    pub four_zone_kbl_default_color_1: u8,
    pub four_zone_kbl_default_color_2: u8,
    pub four_zone_kbl_default_color_3: u8,
    pub four_zone_kbl_default_color_4: u8,
    pub light_bar_kbl_default_color: u8,
    pub reserved_0: [u8; 1],
    pub dedicated_gpu_id: u16,
    pub reserved_1: [u8; 64],
    pub get_device_status_out: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tux_wmi_xx_8in_80out_methods {
    TUX_GET_DEVICE_STATUS	= 2,
}

pub const TUX_KBL_SET_MULTIPLE_KEYS_LIGHTING_SETTINGS_COUNT_MAX: c_int = 120;
#[repr(C)]
#[derive(Copy, Clone)]
pub union tux_wmi_xx_496in_80out_in_t {
    pub raw: [u8; 496],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub reserved: [u8; 15],
    pub rgb_configs_cnt: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tux_kbl_set_multiple_keys_in_rgb_config_t {
    pub key_id: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub rgb_configs: [}; TUX_KBL_SET_MULTIPLE_KEYS_LIGHTING_SETTINGS_COUNT_MAX],
    pub kbl_set_multiple_keys_in: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union tux_wmi_xx_496in_80out_out_t {
    pub raw: [u8; 80],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub return_value: u8,
    pub reserved: [u8; 79],
    pub kbl_set_multiple_keys_out: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tux_wmi_xx_496in_80out_methods {
    TUX_KBL_SET_MULTIPLE_KEYS	= 6,
}
