//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/pwrseqcmd.h
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
// Copyright(c) 2009-2012  Realtek Corporation.

// ---------------------------------------------
// 3 The value of cmd: 4 bits
// ---------------------------------------------
//
pub const PWR_CMD_READ: c_uint = 0x00;
pub const PWR_CMD_WRITE: c_uint = 0x01;
pub const PWR_CMD_POLLING: c_uint = 0x02;
pub const PWR_CMD_DELAY: c_uint = 0x03;
pub const PWR_CMD_END: c_uint = 0x04;
// define the base address of each block
pub const PWR_BASEADDR_MAC: c_uint = 0x00;
pub const PWR_BASEADDR_USB: c_uint = 0x01;
pub const PWR_BASEADDR_PCIE: c_uint = 0x02;
pub const PWR_BASEADDR_SDIO: c_uint = 0x03;

pub const PWR_CUT_ALL_MSK: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pwrseq_delay_unit {
    PWRSEQ_DELAY_US,
    PWRSEQ_DELAY_MS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlan_pwr_cfg {
    pub offset: u16,
    pub cut_msk: u8,
    pub fab_msk:4: u8,
    pub interface_msk:4: u8,
    pub base:4: u8,
    pub cmd:4: u8,
    pub msk: u8,
    pub value: u8,
}

