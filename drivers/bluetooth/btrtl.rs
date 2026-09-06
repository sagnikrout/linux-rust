//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bluetooth/btrtl.h
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
// Bluetooth support for Realtek devices
//
// Copyright (C) 2015 Endless Mobile, Inc.
//
pub const RTL_FRAG_LEN: c_int = 252;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_chip_type_evt {
    pub status: __u8,
    pub type: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_download_cmd {
    pub index: __u8,
    pub data: [__u8; RTL_FRAG_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_download_response {
    pub status: __u8,
    pub index: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_rom_version_evt {
    pub status: __u8,
    pub version: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_epatch_header {
    pub signature: [__u8; 8],
    pub fw_version: __le32,
    pub num_patches: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_vendor_config_entry {
    pub offset: __le16,
    pub len: __u8,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_vendor_config {
    pub signature: __le32,
    pub total_len: __le16,
    pub entry: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_epatch_header_v2 {
    pub signature: [__u8; 8],
    pub fw_version: [__u8; 8],
    pub num_sections: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_section {
    pub opcode: __le32,
    pub len: __le32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_section_hdr {
    pub num: __le16,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_common_subsec {
    pub eco: __u8,
    pub prio: __u8,
    pub cb: [__u8; 2],
    pub len: __le32,
    pub data: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_sec_hdr {
    pub eco: __u8,
    pub prio: __u8,
    pub key_id: __u8,
    pub reserved: __u8,
    pub len: __le32,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_subsection {
    pub list: list_head,
    pub opcode: u32,
    pub len: u32,
    pub prio: u8,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_iovec {
    pub data: *mut u8,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_vendor_cmd {
    pub param: [__u8; 5],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_dump_info {
    pub driver_name: *const c_char,
    pub controller: *mut c_char,
    pub fw_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrealtek_data {
    pub __REALTEK_NUM_FLAGS): DECLARE_BITMAP(flags,,
    pub rtl_dump: rtl_dump_info,
}

extern "C" {
    pub fn btrtl_free(btrtl_dev: *mut btrtl_device_info);
}
extern "C" {
    pub fn btrtl_setup_realtek(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn btrtl_shutdown_realtek(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn btrtl_set_driver_name(hdev: *mut hci_dev, driver_name: *const c_char);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
