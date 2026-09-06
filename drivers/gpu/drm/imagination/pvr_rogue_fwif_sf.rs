//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_rogue_fwif_sf.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.
//
// *DO*NOT* rearrange or delete lines in rogue_fw_log_sfgroups or stid_fmts
// WILL BREAK fw tracing message compatibility with previous
// fw versions. Only add new ones, if so required.
//
// Available log groups.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fw_log_sfgroups {
    ROGUE_FW_GROUP_NULL,
    ROGUE_FW_GROUP_MAIN,
    ROGUE_FW_GROUP_CLEANUP,
    ROGUE_FW_GROUP_CSW,
    ROGUE_FW_GROUP_PM,
    ROGUE_FW_GROUP_RTD,
    ROGUE_FW_GROUP_SPM,
    ROGUE_FW_GROUP_MTS,
    ROGUE_FW_GROUP_BIF,
    ROGUE_FW_GROUP_MISC,
    ROGUE_FW_GROUP_POW,
    ROGUE_FW_GROUP_HWR,
    ROGUE_FW_GROUP_HWP,
    ROGUE_FW_GROUP_RPM,
    ROGUE_FW_GROUP_DMA,
    ROGUE_FW_GROUP_DBG,
}

// pair of string format id and string formats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fw_stid_fmt {
    pub id: u32,
    pub name: [c_char; PVR_SF_STRING_MAX_SIZE],
}

//
// The symbolic names found in the table above are assigned an u32 value of
// the following format:
// 31 30 28 27       20   19  16    15  12      11            0   bits
// -   ---   ---- ----     ----      ----        ---- ---- ----
// 0-11: id number
// 12-15: group id number
// 16-19: number of parameters
// 20-27: unused
// 28-30: active: identify SF packet, otherwise regular int32
// 31: reserved for signed/unsigned compatibility
//
// The following macro assigns those values to the enum generated SF ids list.
//

// Return the group id that the given (enum generated) id belongs to

// Returns how many arguments the SF(string format) for the given (enum generated) id requires

// pair of string format id and string formats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_km_stid_fmt {
    pub id: u32,
    pub name: *const c_char,
}

