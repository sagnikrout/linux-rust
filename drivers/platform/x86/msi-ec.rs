//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/msi-ec.h
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
// msi-ec: MSI laptops' embedded controller driver.
//
// Copyright (C) 2023 Jose Angel Pastrana <japp0005@red.ujaen.es>
// Copyright (C) 2023 Aakash Singh <mail@singhaakash.dev>
// Copyright (C) 2023 Nikita Kravets <teackot@gmail.com>
//

pub const MSI_EC_ADDR_UNKNOWN: c_uint = 0xff01 // unknown address;
pub const MSI_EC_ADDR_UNSUPP: c_uint = 0xff01 // unsupported parameter;
// Firmware info addresses are universal
pub const MSI_EC_FW_VERSION_ADDRESS: c_uint = 0xa0;
pub const MSI_EC_FW_DATE_ADDRESS: c_uint = 0xac;
pub const MSI_EC_FW_TIME_ADDRESS: c_uint = 0xb4;
pub const MSI_EC_FW_VERSION_LENGTH: c_int = 12;
pub const MSI_EC_FW_DATE_LENGTH: c_int = 8;
pub const MSI_EC_FW_TIME_LENGTH: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_ec_charge_control_conf {
    pub address: c_int,
    pub offset_start: c_int,
    pub offset_end: c_int,
    pub range_min: c_int,
    pub range_max: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_ec_webcam_conf {
    pub address: c_int,
    pub block_address: c_int,
    pub bit: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_ec_fn_win_swap_conf {
    pub address: c_int,
    pub bit: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_ec_cooler_boost_conf {
    pub address: c_int,
    pub bit: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_ec_mode {
    pub name: *const c_char,
    pub value: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_ec_shift_mode_conf {
    pub address: c_int,
    pub coding: msi_ec_mode modes[5]; // fixed size for easier hard,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_ec_super_battery_conf {
    pub address: c_int,
    pub mask: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_ec_fan_mode_conf {
    pub address: c_int,
    pub coding: msi_ec_mode modes[5]; // fixed size for easier hard,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_ec_cpu_conf {
    pub rt_temp_address: c_int,
    pub realtime: int rt_fan_speed_address; //,
    pub rt_fan_speed_base_min: c_int,
    pub rt_fan_speed_base_max: c_int,
    pub basic: int bs_fan_speed_address; //,
    pub bs_fan_speed_base_min: c_int,
    pub bs_fan_speed_base_max: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_ec_gpu_conf {
    pub rt_temp_address: c_int,
    pub realtime: int rt_fan_speed_address; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_ec_led_conf {
    pub micmute_led_address: c_int,
    pub mute_led_address: c_int,
    pub bit: c_int,
}

pub const MSI_EC_KBD_BL_STATE_MASK: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_ec_kbd_bl_conf {
    pub bl_mode_address: c_int,
    pub bl_modes: [c_int; 2],
    pub max_mode: c_int,
    pub bl_state_address: c_int,
    pub state_base_value: c_int,
    pub max_state: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_ec_conf {
    pub allowed_fw: *const *const c_char,
    pub charge_control: msi_ec_charge_control_conf,
    pub webcam: msi_ec_webcam_conf,
    pub fn_win_swap: msi_ec_fn_win_swap_conf,
    pub cooler_boost: msi_ec_cooler_boost_conf,
    pub shift_mode: msi_ec_shift_mode_conf,
    pub super_battery: msi_ec_super_battery_conf,
    pub fan_mode: msi_ec_fan_mode_conf,
    pub cpu: msi_ec_cpu_conf,
    pub gpu: msi_ec_gpu_conf,
    pub leds: msi_ec_led_conf,
    pub kbd_bl: msi_ec_kbd_bl_conf,
}
