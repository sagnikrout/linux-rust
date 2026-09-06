//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device-id/dmi.h
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


// SPDX-License-Identifier: GPL-2.0

// dmi
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmi_field {
    DMI_NONE,
    DMI_BIOS_VENDOR,
    DMI_BIOS_VERSION,
    DMI_BIOS_DATE,
    DMI_BIOS_RELEASE,
    DMI_EC_FIRMWARE_RELEASE,
    DMI_SYS_VENDOR,
    DMI_PRODUCT_NAME,
    DMI_PRODUCT_VERSION,
    DMI_PRODUCT_SERIAL,
    DMI_PRODUCT_UUID,
    DMI_PRODUCT_SKU,
    DMI_PRODUCT_FAMILY,
    DMI_BOARD_VENDOR,
    DMI_BOARD_NAME,
    DMI_BOARD_VERSION,
    DMI_BOARD_SERIAL,
    DMI_BOARD_ASSET_TAG,
    DMI_CHASSIS_VENDOR,
    DMI_CHASSIS_TYPE,
    DMI_CHASSIS_VERSION,
    DMI_CHASSIS_SERIAL,
    DMI_CHASSIS_ASSET_TAG,
    DMI_STRING_MAX,
    DMI_OEM_STRING,	/* special case - will not be in dmi_ident */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmi_strmatch {
    pub slot:7: c_uchar,
    pub exact_match:1: c_uchar,
    pub substr: [c_char; 79],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmi_system_id {
    pub ): *const *const int (callback)(struct dmi_system_id,
    pub ident: *const c_char,
    pub matches: [dmi_strmatch; 4],
    pub driver_data: *mut c_void,
}

//
// struct dmi_device_id appears during expansion of
// "MODULE_DEVICE_TABLE(dmi, x)". Compiler doesn't look inside it
// but this is enough for gcc 3.4.6 to error out:
// error: storage size of '__mod_dmi_device_table' isn't known
//

