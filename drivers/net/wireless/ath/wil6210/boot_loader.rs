//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/wil6210/boot_loader.h
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
// Copyright (c) 2015 Qualcomm Atheros, Inc.
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
//
// This file contains the definitions for the boot loader
// for the Qualcomm "Sparrow" 60 Gigabit wireless solution.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bl_dedicated_registers_v1 {
    pub poll: *mut *mut __le32 boot_loader_ready; / 0x880A3C driver will,
// this Dword until BL will
// set it to 1 (initial value
// should be 0)
//
    pub /: *mut *mut __le32 boot_loader_struct_version; / 0x880A40 BL struct ver.,
    pub /: *mut *mut __le16 rf_type; / 0x880A44 connected RF ID,
    pub status,: *mut *mut __le16 rf_status; / 0x880A46 RF,
// 0 is OK else error
//
    pub /: *mut *mut __le32 baseband_type; / 0x880A48 board type ID,
    pub /: *mut *mut u8 mac_address[6]; / 0x880A4c BL mac address,
    pub /: *mut *mut u8 bl_version_major; / 0x880A52 BL ver. major,
    pub /: *mut *mut u8 bl_version_minor; / 0x880A53 BL ver. minor,
    pub /: *mut *mut __le16 bl_version_subminor; / 0x880A54 BL ver. subminor,
    pub /: *mut *mut __le16 bl_version_build; / 0x880A56 BL ver. build,
// valid only for version 2 and above
    pub /: *mut *mut __le32 bl_assert_code; / 0x880A58 BL Assert code,
    pub /: *mut *mut __le32 bl_assert_blink; / 0x880A5C BL Assert Branch,
    pub /: *mut *mut __le32 bl_shutdown_handshake; / 0x880A60 BL cleaner shutdown,
    pub /: *mut *mut __le32 bl_reserved[21]; / 0x880A64 - 0x880AB4,
    pub /: *mut *mut __le32 bl_magic_number; / 0x880AB8 BL Magic number,
    pub __packed: },
// the following struct is the version 0 struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bl_dedicated_registers_v0 {
    pub poll: *mut *mut __le32 boot_loader_ready; / 0x880A3C driver will,
// this Dword until BL will
// set it to 1 (initial value
// should be 0)
//

    pub /: *mut *mut __le32 boot_loader_struct_version; / 0x880A40 BL struct ver.,
    pub /: *mut *mut __le32 rf_type; / 0x880A44 connected RF ID,
    pub /: *mut *mut __le32 baseband_type; / 0x880A48 board type ID,
    pub /: *mut *mut u8 mac_address[6]; / 0x880A4c BL mac address,
    pub __packed: },
// bits for bl_shutdown_handshake

