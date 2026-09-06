//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/uniwill/uniwill-wmi.h
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
// Linux hotkey driver for Uniwill notebooks.
//
// Copyright (C) 2025 Armin Wolf <W_Armin@gmx.de>
//

pub const UNIWILL_OSD_CAPSLOCK: c_uint = 0x01;
pub const UNIWILL_OSD_NUMLOCK: c_uint = 0x02;
pub const UNIWILL_OSD_SCROLLLOCK: c_uint = 0x03;
pub const UNIWILL_OSD_TOUCHPAD_ON: c_uint = 0x04;
pub const UNIWILL_OSD_TOUCHPAD_OFF: c_uint = 0x05;
pub const UNIWILL_OSD_SILENT_MODE_ON: c_uint = 0x06;
pub const UNIWILL_OSD_SILENT_MODE_OFF: c_uint = 0x07;
pub const UNIWILL_OSD_WLAN_ON: c_uint = 0x08;
pub const UNIWILL_OSD_WLAN_OFF: c_uint = 0x09;
pub const UNIWILL_OSD_WIMAX_ON: c_uint = 0x0A;
pub const UNIWILL_OSD_WIMAX_OFF: c_uint = 0x0B;
pub const UNIWILL_OSD_BLUETOOTH_ON: c_uint = 0x0C;
pub const UNIWILL_OSD_BLUETOOTH_OFF: c_uint = 0x0D;
pub const UNIWILL_OSD_RF_ON: c_uint = 0x0E;
pub const UNIWILL_OSD_RF_OFF: c_uint = 0x0F;
pub const UNIWILL_OSD_3G_ON: c_uint = 0x10;
pub const UNIWILL_OSD_3G_OFF: c_uint = 0x11;
pub const UNIWILL_OSD_WEBCAM_ON: c_uint = 0x12;
pub const UNIWILL_OSD_WEBCAM_OFF: c_uint = 0x13;
pub const UNIWILL_OSD_BRIGHTNESSUP: c_uint = 0x14;
pub const UNIWILL_OSD_BRIGHTNESSDOWN: c_uint = 0x15;
pub const UNIWILL_OSD_RADIOON: c_uint = 0x1A;
pub const UNIWILL_OSD_RADIOOFF: c_uint = 0x1B;
pub const UNIWILL_OSD_POWERSAVE_ON: c_uint = 0x31;
pub const UNIWILL_OSD_POWERSAVE_OFF: c_uint = 0x32;
pub const UNIWILL_OSD_MENU: c_uint = 0x34;
pub const UNIWILL_OSD_MUTE: c_uint = 0x35;
pub const UNIWILL_OSD_VOLUMEDOWN: c_uint = 0x36;
pub const UNIWILL_OSD_VOLUMEUP: c_uint = 0x37;
pub const UNIWILL_OSD_MENU_2: c_uint = 0x38;
pub const UNIWILL_OSD_LIGHTBAR_ON: c_uint = 0x39;
pub const UNIWILL_OSD_LIGHTBAR_OFF: c_uint = 0x3A;
pub const UNIWILL_OSD_KB_LED_LEVEL0: c_uint = 0x3B;
pub const UNIWILL_OSD_KB_LED_LEVEL1: c_uint = 0x3C;
pub const UNIWILL_OSD_KB_LED_LEVEL2: c_uint = 0x3D;
pub const UNIWILL_OSD_KB_LED_LEVEL3: c_uint = 0x3E;
pub const UNIWILL_OSD_KB_LED_LEVEL4: c_uint = 0x3F;
pub const UNIWILL_OSD_SUPER_KEY_DISABLE: c_uint = 0x40;
pub const UNIWILL_OSD_SUPER_KEY_ENABLE: c_uint = 0x41;
pub const UNIWILL_OSD_MENU_JP: c_uint = 0x42;
pub const UNIWILL_OSD_CAMERA_ON: c_uint = 0x90;
pub const UNIWILL_OSD_CAMERA_OFF: c_uint = 0x91;
pub const UNIWILL_OSD_RFKILL: c_uint = 0xA4;
pub const UNIWILL_OSD_SUPER_KEY_STATE_CHANGED: c_uint = 0xA5;
pub const UNIWILL_OSD_LIGHTBAR_STATE_CHANGED: c_uint = 0xA6;
pub const UNIWILL_OSD_FAN_BOOST_STATE_CHANGED: c_uint = 0xA7;
pub const UNIWILL_OSD_LCD_SW: c_uint = 0xA9;
pub const UNIWILL_OSD_FAN_OVERTEMP: c_uint = 0xAA;
pub const UNIWILL_OSD_DC_ADAPTER_CHANGED: c_uint = 0xAB;
pub const UNIWILL_OSD_BAT_HP_OFF: c_uint = 0xAC;
pub const UNIWILL_OSD_FAN_DOWN_TEMP: c_uint = 0xAD;
pub const UNIWILL_OSD_BATTERY_ALERT: c_uint = 0xAE;
pub const UNIWILL_OSD_TIMAP_HAIERLB_SW: c_uint = 0xAF;
pub const UNIWILL_OSD_PERFORMANCE_MODE_TOGGLE: c_uint = 0xB0;
pub const UNIWILL_OSD_KBDILLUMDOWN: c_uint = 0xB1;
pub const UNIWILL_OSD_KBDILLUMUP: c_uint = 0xB2;
pub const UNIWILL_OSD_BACKLIGHT_LEVEL_CHANGE: c_uint = 0xB3;
pub const UNIWILL_OSD_BACKLIGHT_POWER_CHANGE: c_uint = 0xB4;
pub const UNIWILL_OSD_MIC_MUTE: c_uint = 0xB7;
pub const UNIWILL_OSD_FN_LOCK: c_uint = 0xB8;
pub const UNIWILL_OSD_KBDILLUMTOGGLE: c_uint = 0xB9;
pub const UNIWILL_OSD_BAT_CHARGE_FULL_24_H: c_uint = 0xBE;
pub const UNIWILL_OSD_BAT_ERM_UPDATE: c_uint = 0xBF;
pub const UNIWILL_OSD_BENCHMARK_MODE_TOGGLE: c_uint = 0xC0;
pub const UNIWILL_OSD_SCREEN_STATE_CHANGED: c_uint = 0xCC;
pub const UNIWILL_OSD_WEBCAM_TOGGLE: c_uint = 0xCF;
pub const UNIWILL_OSD_KBD_BACKLIGHT_CHANGED: c_uint = 0xF0;
extern "C" {
    pub fn devm_uniwill_wmi_register_notifier(dev: *mut device, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn uniwill_wmi_register_driver() -> int __init;
}
extern "C" {
    pub fn uniwill_wmi_unregister_driver() -> void __exit;
}
