//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/sbshc.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_smb_protocol {
    SMBUS_WRITE_QUICK = 2,
    SMBUS_READ_QUICK = 3,
    SMBUS_SEND_BYTE = 4,
    SMBUS_RECEIVE_BYTE = 5,
    SMBUS_WRITE_BYTE = 6,
    SMBUS_READ_BYTE = 7,
    SMBUS_WRITE_WORD  = 8,
    SMBUS_READ_WORD  = 9,
    SMBUS_WRITE_BLOCK = 0xa,
    SMBUS_READ_BLOCK = 0xb,
    SMBUS_PROCESS_CALL = 0xc,
    SMBUS_BLOCK_PROCESS_CALL = 0xd,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_sbs_device_addr {
    ACPI_SBS_CHARGER = 0x9,
    ACPI_SBS_MANAGER = 0xa,
    ACPI_SBS_BATTERY = 0xb,
}

extern "C" {
    pub fn void(context: *mut *mut smbus_alarm_callback)(void) -> typedef;
}
extern "C" {
    pub fn acpi_smbus_unregister_callback(hc: *mut acpi_smb_hc) -> c_int;
}
