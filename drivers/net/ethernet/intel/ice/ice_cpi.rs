//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_cpi.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2026 Intel Corporation

pub const CPI0_PHY1_CMD_DATA: c_uint = 0x7FD028;
pub const CPI0_LM1_CMD_DATA: c_uint = 0x7FD024;
pub const CPI_RETRIES_COUNT: c_int = 10;
pub const CPI_RETRIES_CADENCE_MS: c_int = 100;
// CPI PHY CMD DATA register (CPI0_PHY1_CMD_DATA)

// CPI LM CMD DATA register (CPI0_LM1_CMD_DATA)

pub const CPI_OPCODE_PHY_CLK: c_uint = 0xF1;

pub const CPI_OPCODE_PHY_CLK_DISABLE: c_int = 1;
pub const CPI_OPCODE_PHY_CLK_ENABLE: c_int = 2;

pub const CPI_LM_CMD_REQ: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_cpi_cmd {
    pub port: u8,
    pub opcode: u8,
    pub data: u16,
    pub set: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_cpi_resp {
    pub port: u8,
    pub opcode: u8,
    pub data: u16,
}
