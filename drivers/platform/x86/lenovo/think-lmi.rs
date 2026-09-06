//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/lenovo/think-lmi.h
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

pub const TLMI_SETTINGS_COUNT: c_int = 256;
pub const TLMI_SETTINGS_MAXLEN: c_int = 512;
pub const TLMI_PWD_BUFSIZE: c_int = 129;
pub const TLMI_LANG_MAXLEN: c_int = 4;
pub const TLMI_INDEX_MAX: c_int = 32;
// Possible error values
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlmi_err_codes {
    pub err_str: *const c_char,
    pub err_code: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum encoding_option {
    TLMI_ENCODING_ASCII,
    TLMI_ENCODING_SCANCODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum level_option {
    TLMI_LEVEL_USER,
    TLMI_LEVEL_MASTER,
}

//
// There are a limit on the number of WMI operations you can do if you use
// the default implementation of saving on every set. This is due to a
// limitation in EFI variable space used.
// Have a 'bulk save' mode where you can manually trigger the save, and can
// therefore set unlimited variables - for users that need it.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum save_mode {
    TLMI_SAVE_SINGLE,
    TLMI_SAVE_BULK,
    TLMI_SAVE_SAVE,
}

// GUIDs can differ between platforms
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlmi_cert_guids {
    pub thumbprint: *const c_char,
    pub set_bios_setting: *const c_char,
    pub save_bios_setting: *const c_char,
    pub cert_to_password: *const c_char,
    pub clear_bios_cert: *const c_char,
    pub update_bios_cert: *const c_char,
    pub set_bios_cert: *const c_char,
}

// password configuration details
pub const TLMI_PWDCFG_MODE_LEGACY: c_int = 0;
pub const TLMI_PWDCFG_MODE_PASSWORD: c_int = 1;
pub const TLMI_PWDCFG_MODE_MULTICERT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlmi_pwdcfg_core {
    pub password_mode: u32,
    pub password_state: u32,
    pub min_length: u32,
    pub max_length: u32,
    pub supported_encodings: u32,
    pub supported_keyboard: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlmi_pwdcfg_ext {
    pub hdd_user_password: u32,
    pub hdd_master_password: u32,
    pub nvme_user_password: u32,
    pub nvme_master_password: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlmi_pwdcfg {
    pub core: tlmi_pwdcfg_core,
    pub ext: tlmi_pwdcfg_ext,
}

// password setting details
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlmi_pwd_setting {
    pub kobj: kobject,
    pub pwd_enabled: bool,
    pub password: [c_char; TLMI_PWD_BUFSIZE],
    pub pwd_type: *const c_char,
    pub role: *const c_char,
    pub minlen: c_int,
    pub maxlen: c_int,
    pub encoding: encoding_option,
    pub kbdlang: [c_char; TLMI_LANG_MAXLEN],
    pub /: *mut *mut int index; /Used for HDD and NVME auth,
    pub level: level_option,
    pub cert_installed: bool,
    pub signature: *mut c_char,
    pub save_signature: *mut c_char,
}

// Attribute setting details
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlmi_attr_setting {
    pub kobj: kobject,
    pub wdev: *mut wmi_device,
    pub index: c_int,
    pub name: [c_char; TLMI_SETTINGS_MAXLEN],
    pub display_name: [c_char; TLMI_SETTINGS_MAXLEN],
    pub possible_values: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct think_lmi {
    pub wmi_device: *mut wmi_device,
    pub can_set_bios_settings: bool,
    pub can_get_bios_selections: bool,
    pub can_set_bios_password: bool,
    pub can_get_password_settings: bool,
    pub pending_changes: bool,
    pub can_debug_cmd: bool,
    pub opcode_support: bool,
    pub certificate_support: bool,
    pub save_mode: save_mode,
    pub save_required: bool,
    pub reboot_required: bool,
    pub thinkcenter_mode: bool,
    pub setting: [*mut tlmi_attr_setting; TLMI_SETTINGS_COUNT],
    pub class_dev: *mut device,
    pub attribute_kset: *mut kset,
    pub authentication_kset: *mut kset,
    pub pwdcfg: tlmi_pwdcfg,
    pub pwd_admin: *mut tlmi_pwd_setting,
    pub pwd_power: *mut tlmi_pwd_setting,
    pub pwd_system: *mut tlmi_pwd_setting,
    pub pwd_hdd: *mut tlmi_pwd_setting,
    pub pwd_nvme: *mut tlmi_pwd_setting,
    pub cert_guid: *const tlmi_cert_guids,
}
