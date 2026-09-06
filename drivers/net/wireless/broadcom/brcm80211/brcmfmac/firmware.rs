//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/firmware.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2013 Broadcom Corporation
//
pub const BRCMF_FW_REQF_OPTIONAL: c_uint = 0x0001;
pub const BRCMF_FW_NAME_LEN: c_int = 320;

pub const BRCMF_FW_MAX_BOARD_TYPES: c_int = 8;
//
// struct brcmf_firmware_mapping - Used to map chipid/revmask to firmware
// filename and nvram filename. Each bus type implementation should create
// a table of firmware mappings (using the macros defined below).
//
// @chipid: ID of chip.
// @revmask: bitmask of revisions, e.g. 0x10 means rev 4 only, 0xf means rev 0-3
// @fw: name of the firmware file.
// @nvram: name of nvram file.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_firmware_mapping {
    pub chipid: u32,
    pub revmask: u32,
    pub fw_base: *const c_char,
}

// Firmware and Country Local Matrix files

extern "C" {
    pub fn brcmf_fw_nvram_free(nvram: *mut c_void);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_fw_type {
    BRCMF_FW_TYPE_BINARY,
    BRCMF_FW_TYPE_NVRAM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fw_item {
    pub path: *const c_char,
    pub type: brcmf_fw_type,
    pub flags: u16,
    pub binary: *const firmware,
    pub data: *mut c_void,
    pub len: u32,
    pub nv_data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fw_request {
    pub domain_nr: u16,
    pub bus_nr: u16,
    pub n_items: u32,
    pub board_types: [*const c_char; BRCMF_FW_MAX_BOARD_TYPES],
    pub __counted_by(n_items): brcmf_fw_item items[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fw_name {
    pub extension: *const c_char,
    pub path: *mut c_char,
}

//
// Request firmware(s) asynchronously. When the asynchronous request
// fails it will not use the callback, but call device_release_driver()
// instead which will call the driver .remove() callback.
//
