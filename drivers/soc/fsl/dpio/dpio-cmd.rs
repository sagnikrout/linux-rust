//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/fsl/dpio/dpio-cmd.h
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
//
// Copyright 2013-2016 Freescale Semiconductor Inc.
// Copyright 2016 NXP
//
// DPIO Version
pub const DPIO_VER_MAJOR: c_int = 4;
pub const DPIO_VER_MINOR: c_int = 2;
// Command Versioning
pub const DPIO_CMD_ID_OFFSET: c_int = 4;
pub const DPIO_CMD_BASE_VERSION: c_int = 1;

// Command IDs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpio_cmd_open {
    pub dpio_id: __le32,
}

pub const DPIO_CHANNEL_MODE_MASK: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpio_rsp_get_attr {
// cmd word 0
    pub id: __le32,
    pub qbman_portal_id: __le16,
    pub num_priorities: u8,
    pub channel_mode: u8,
// cmd word 1
    pub qbman_portal_ce_addr: __le64,
// cmd word 2
    pub qbman_portal_ci_addr: __le64,
// cmd word 3
    pub qbman_version: __le32,
    pub pad1: __le32,
// cmd word 4
    pub clk: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpio_stashing_dest {
    pub sdest: u8,
}
