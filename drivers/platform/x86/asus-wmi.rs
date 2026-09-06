//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/asus-wmi.h
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
// Asus PC WMI hotkey driver
//
// Copyright(C) 2010 Intel Corporation.
// Copyright(C) 2010-2011 Corentin Chary <corentin.chary@gmail.com>
//
// Portions based on wistron_btns.c:
// Copyright (C) 2005 Miloslav Trmac <mitr@volny.cz>
// Copyright (C) 2005 Bernhard Rosenkraenzer <bero@arklinux.org>
// Copyright (C) 2005 Dmitry Torokhov <dtor@mail.ru>
//

pub const ASUS_WMI_KEY_ARMOURY: c_uint = 0xffff01;
pub const ASUS_WMI_BRN_DOWN: c_uint = 0x2e;
pub const ASUS_WMI_BRN_UP: c_uint = 0x2f;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum asus_wmi_tablet_switch_mode {
    asus_wmi_no_tablet_switch,
    asus_wmi_kbd_dock_devid,
    asus_wmi_lid_flip_devid,
    asus_wmi_lid_flip_rog_devid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct quirk_entry {
    pub hotplug_wireless: bool,
    pub scalar_panel_brightness: bool,
    pub store_backlight_power: bool,
    pub wmi_backlight_set_devstate: bool,
    pub wmi_force_als_set: bool,
    pub wmi_ignore_fan: bool,
    pub filter_i8042_e1_extended_codes: bool,
    pub key_wlan_event: c_int,
    pub tablet_switch_mode: asus_wmi_tablet_switch_mode,
    pub wapf: c_int,
//
// For machines with AMD graphic chips, it will send out WMI event
// and ACPI interrupt at the same time while hitting the hotkey.
// To simplify the problem, we just have to ignore the WMI event,
// and let the ACPI interrupt to send out the key event.
//
    pub no_display_toggle: c_int,
    pub xusb2pr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asus_wmi_driver {
    pub brightness: c_int,
    pub panel_power: c_int,
    pub screenpad_brightness: c_int,
    pub wlan_ctrl_by_user: c_int,
    pub name: *const c_char,
    pub owner: *mut module,
    pub event_guid: *const c_char,
    pub keymap: *const key_entry,
    pub input_name: *const c_char,
    pub input_phys: *const c_char,
    pub quirks: *mut quirk_entry,
// Returns new code, value, and autorelease values in arguments.
// Return ASUS_WMI_KEY_IGNORE in code if event should be ignored.
    pub autorelease): *mut *mut unsigned int value, bool,
// Optional standard i8042 filter
    pub i8042_filter: i8042_filter_t,
    pub device): *mut *mut int (probe) (struct platform_device,
    pub driver): *mut *mut void (detect_quirks) (struct asus_wmi_driver,
    pub platform_driver: platform_driver,
    pub platform_device: *mut platform_device,
}

extern "C" {
    pub fn asus_wmi_register_driver(driver: *mut asus_wmi_driver) -> c_int;
}
extern "C" {
    pub fn asus_wmi_unregister_driver(driver: *mut asus_wmi_driver);
}
