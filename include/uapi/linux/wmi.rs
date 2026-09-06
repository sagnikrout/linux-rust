//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/wmi.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// User API methods for ACPI-WMI mapping driver
//
// Copyright (C) 2017 Dell, Inc.
//

// WMI bus will filter all WMI vendor driver requests through this IOC

// All ioctl requests through WMI should declare their size followed by
// relevant data objects
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ioctl_buffer {
    pub length: __u64,
    pub data: [__u8; ],
}

// This structure may be modified by the firmware when we enter
// system management mode through SMM, hence the volatiles
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct calling_interface_buffer {
    pub cmd_class: __u16,
    pub cmd_select: __u16,
    pub input: [volatile __u32; 4],
    pub output: [volatile __u32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dell_wmi_extensions {
    pub argattrib: __u32,
    pub blength: __u32,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dell_wmi_smbios_buffer {
    pub length: __u64,
    pub std: calling_interface_buffer,
    pub ext: dell_wmi_extensions,
    pub __packed: },
// Whitelisted smbios class/select commands
pub const CLASS_TOKEN_READ: c_int = 0;
pub const CLASS_TOKEN_WRITE: c_int = 1;
pub const SELECT_TOKEN_STD: c_int = 0;
pub const SELECT_TOKEN_BAT: c_int = 1;
pub const SELECT_TOKEN_AC: c_int = 2;
pub const CLASS_FLASH_INTERFACE: c_int = 7;
pub const SELECT_FLASH_INTERFACE: c_int = 3;
pub const CLASS_ADMIN_PROP: c_int = 10;
pub const SELECT_ADMIN_PROP: c_int = 3;
pub const CLASS_INFO: c_int = 17;
pub const SELECT_RFKILL: c_int = 11;
pub const SELECT_APP_REGISTRATION: c_int = 3;
pub const SELECT_DOCK: c_int = 22;
// whitelisted tokens
pub const CAPSULE_EN_TOKEN: c_uint = 0x0461;
pub const CAPSULE_DIS_TOKEN: c_uint = 0x0462;
pub const WSMT_EN_TOKEN: c_uint = 0x04EC;
pub const WSMT_DIS_TOKEN: c_uint = 0x04ED;
// Dell SMBIOS calling IOCTL command used by dell-smbios-wmi

