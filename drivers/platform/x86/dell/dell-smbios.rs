//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/dell/dell-smbios.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Common functions for kernel modules using Dell SMBIOS
//
// Copyright (c) Red Hat <mjg@redhat.com>
// Copyright (c) 2014 Gabriele Mazzotta <gabriele.mzt@gmail.com>
// Copyright (c) 2014 Pali Rohár <pali@kernel.org>
//
// Based on documentation in the libsmbios package:
// Copyright (C) 2005-2014 Dell Inc.
//

// Classes and selects used only in kernel drivers
pub const CLASS_KBD_BACKLIGHT: c_int = 4;
pub const SELECT_KBD_BACKLIGHT: c_int = 11;
pub const SELECT_THERMAL_MANAGEMENT: c_int = 19;
// Tokens used in kernel drivers, any of these
// should be filtered from userspace access
//
pub const BRIGHTNESS_TOKEN: c_uint = 0x007d;
pub const KBD_LED_AC_TOKEN: c_uint = 0x0451;
pub const KBD_LED_OFF_TOKEN: c_uint = 0x01E1;
pub const KBD_LED_ON_TOKEN: c_uint = 0x01E2;
pub const KBD_LED_AUTO_TOKEN: c_uint = 0x01E3;
pub const KBD_LED_AUTO_25_TOKEN: c_uint = 0x02EA;
pub const KBD_LED_AUTO_50_TOKEN: c_uint = 0x02EB;
pub const KBD_LED_AUTO_75_TOKEN: c_uint = 0x02EC;
pub const KBD_LED_AUTO_100_TOKEN: c_uint = 0x02F6;
pub const BAT_PRI_AC_MODE_TOKEN: c_uint = 0x0341;
pub const BAT_ADAPTIVE_MODE_TOKEN: c_uint = 0x0342;
pub const BAT_CUSTOM_MODE_TOKEN: c_uint = 0x0343;
pub const BAT_STANDARD_MODE_TOKEN: c_uint = 0x0346;
pub const BAT_EXPRESS_MODE_TOKEN: c_uint = 0x0347;
pub const BAT_CUSTOM_CHARGE_START: c_uint = 0x0349;
pub const BAT_CUSTOM_CHARGE_END: c_uint = 0x034A;
pub const GLOBAL_MIC_MUTE_ENABLE: c_uint = 0x0364;
pub const GLOBAL_MIC_MUTE_DISABLE: c_uint = 0x0365;
pub const GLOBAL_MUTE_ENABLE: c_uint = 0x058C;
pub const GLOBAL_MUTE_DISABLE: c_uint = 0x058D;
extern "C" {
    pub fn int(dev: *mut *mut smbios_callback_fn_t)(struct device, buffer: *mut calling_interface_buffer) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct calling_interface_token {
    pub tokenID: u16,
    pub location: u16,
    pub value: u16,
    pub stringlength: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct calling_interface_structure {
    pub header: dmi_header,
    pub cmdIOAddress: u16,
    pub cmdIOCode: u8,
    pub supportedCmds: u32,
    pub tokens: [calling_interface_token; ],
    pub __packed: },
    pub call_fn): *mut *mut int dell_smbios_register_device(struct device d, int priority, smbios_callback_fn_t,
    pub d): *mut void dell_smbios_unregister_device(struct device,
    pub value): int dell_smbios_error(int,
    pub buffer): *mut calling_interface_buffer,
    pub buffer): *mut int dell_smbios_call(struct calling_interface_buffer,
    pub arg3): u32 arg0, u32 arg1, u32 arg2, u32,
    pub select): u16 class, u16,
    pub tokenid): *mut *mut calling_interface_token dell_smbios_find_token(int,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dell_laptop_notifier_actions {
    DELL_LAPTOP_KBD_BACKLIGHT_BRIGHTNESS_CHANGED,
}

    pub nb): *mut int dell_laptop_register_notifier(struct notifier_block,
    pub nb): *mut int dell_laptop_unregister_notifier(struct notifier_block,
    pub data): *mut void dell_laptop_call_notifier(unsigned long action, void,
    pub class): bool dell_smbios_class_is_supported(u16,
// for the supported backends

    pub init_dell_smbios_wmi(void): c_int,
    pub exit_dell_smbios_wmi(void): c_void,

    pub -ENODEV: return,

    pub init_dell_smbios_smm(void): c_int,
    pub exit_dell_smbios_smm(void): c_void,

    pub -ENODEV: return,

