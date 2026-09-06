//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/mei/mkhi.h
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
//
// Copyright (c) 2003-2022, Intel Corporation. All rights reserved.
// Intel Management Engine Interface (Intel MEI) Linux driver
//

pub const MKHI_FEATURE_PTT: c_uint = 0x10;
pub const MKHI_FWCAPS_GROUP_ID: c_uint = 0x3;
pub const MKHI_FWCAPS_SET_OS_VER_APP_RULE_CMD: c_int = 6;
pub const MKHI_GEN_GROUP_ID: c_uint = 0xFF;
pub const MKHI_GEN_GET_FW_VERSION_CMD: c_uint = 0x2;
pub const MKHI_GROUP_ID_GFX: c_uint = 0x30;
pub const MKHI_GFX_RESET_WARN_CMD_REQ: c_uint = 0x0;
pub const MKHI_GFX_MEMORY_READY_CMD_REQ: c_uint = 0x1;
// Allow transition to PXP mode without approval
pub const MKHI_GFX_MEM_READY_PXP_ALLOWED: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mkhi_rule_id {
    pub rule_type: __le16,
    pub feature_id: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mkhi_fwcaps {
    pub id: mkhi_rule_id,
    pub len: u8,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mkhi_msg_hdr {
    pub group_id: u8,
    pub command: u8,
    pub reserved: u8,
    pub result: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mkhi_msg {
    pub hdr: mkhi_msg_hdr,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mkhi_gfx_mem_ready {
    pub hdr: mkhi_msg_hdr,
    pub flags: u32,
    pub __packed: },
