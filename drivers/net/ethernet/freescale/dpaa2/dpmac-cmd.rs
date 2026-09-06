//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/dpaa2/dpmac-cmd.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
// Copyright 2013-2016 Freescale Semiconductor Inc.
// Copyright 2019, 2024-2026 NXP
//
// DPMAC Version
pub const DPMAC_VER_MAJOR: c_int = 4;
pub const DPMAC_VER_MINOR: c_int = 4;
pub const DPMAC_CMD_BASE_VERSION: c_int = 1;
pub const DPMAC_CMD_2ND_VERSION: c_int = 2;
pub const DPMAC_CMD_ID_OFFSET: c_int = 4;

// Command IDs

// Macros for accessing command fields smaller than 1byte

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmac_cmd_open {
    pub dpmac_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmac_rsp_get_attributes {
    pub eth_if: u8,
    pub link_type: u8,
    pub id: __le16,
    pub max_rate: __le32,
}

pub const DPMAC_STATE_SIZE: c_int = 1;
pub const DPMAC_STATE_SHIFT: c_int = 0;
pub const DPMAC_STATE_VALID_SIZE: c_int = 1;
pub const DPMAC_STATE_VALID_SHIFT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmac_cmd_set_link_state {
    pub options: __le64,
    pub rate: __le32,
    pub pad0: __le32,
// from lsb: up:1, state_valid:1
    pub state: u8,
    pub pad1: [u8; 7],
    pub supported: __le64,
    pub advertising: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmac_cmd_get_counter {
    pub id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmac_rsp_get_counter {
    pub pad: __le64,
    pub counter: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmac_rsp_get_api_version {
    pub major: __le16,
    pub minor: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmac_cmd_set_protocol {
    pub eth_if: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmac_cmd_get_statistics {
    pub iova_cnt: __le64,
    pub iova_values: __le64,
    pub num_cnt: __le32,
}
