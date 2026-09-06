//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/sdhci-uhs2.h
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
// Header file for Host Controller UHS2 related registers.
//
// Copyright (C) 2014 Intel Corp, All Rights Reserved.
//

// SDHCI Category C registers : UHS2 usage
pub const SDHCI_UHS2_CM_TRAN_RESP: c_uint = 0x10;
pub const SDHCI_UHS2_SD_TRAN_RESP: c_uint = 0x18;
pub const SDHCI_UHS2_SD_TRAN_RESP_1: c_uint = 0x1C;
// SDHCI Category B registers : UHS2 only
pub const SDHCI_UHS2_BLOCK_SIZE: c_uint = 0x80;

pub const SDHCI_UHS2_BLOCK_COUNT: c_uint = 0x84;
pub const SDHCI_UHS2_CMD_PACKET: c_uint = 0x88;
pub const SDHCI_UHS2_CMD_PACK_MAX_LEN: c_int = 20;
pub const SDHCI_UHS2_TRANS_MODE: c_uint = 0x9C;

pub const SDHCI_UHS2_CMD: c_uint = 0x9E;

pub const SDHCI_UHS2_RESPONSE: c_uint = 0xA0;
pub const SDHCI_UHS2_RESPONSE_MAX_LEN: c_int = 20;
pub const SDHCI_UHS2_MSG_SELECT: c_uint = 0xB4;
pub const SDHCI_UHS2_MSG_SELECT_CURR: c_uint = 0x0;
pub const SDHCI_UHS2_MSG_SELECT_ONE: c_uint = 0x1;
pub const SDHCI_UHS2_MSG_SELECT_TWO: c_uint = 0x2;
pub const SDHCI_UHS2_MSG_SELECT_THREE: c_uint = 0x3;
pub const SDHCI_UHS2_MSG: c_uint = 0xB8;
pub const SDHCI_UHS2_DEV_INT_STATUS: c_uint = 0xBC;
pub const SDHCI_UHS2_DEV_SELECT: c_uint = 0xBE;

pub const SDHCI_UHS2_DEV_INT_CODE: c_uint = 0xBF;
pub const SDHCI_UHS2_SW_RESET: c_uint = 0xC0;

pub const SDHCI_UHS2_TIMER_CTRL: c_uint = 0xC2;

pub const SDHCI_UHS2_INT_STATUS: c_uint = 0xC4;
pub const SDHCI_UHS2_INT_STATUS_ENABLE: c_uint = 0xC8;
pub const SDHCI_UHS2_INT_SIGNAL_ENABLE: c_uint = 0xCC;

// CRC Error occurs during a packet receiving

pub const SDHCI_UHS2_SETTINGS_PTR: c_uint = 0xE0;

pub const SDHCI_UHS2_FD_OR_2L_HD: c_uint = 0x0 /* 2 lanes */;
pub const SDHCI_UHS2_2D1U_FD: c_uint = 0x2 /* 3 lanes, 2 down, 1 up, full duplex */;
pub const SDHCI_UHS2_1D2U_FD: c_uint = 0x3 /* 3 lanes, 1 down, 2 up, full duplex */;
pub const SDHCI_UHS2_2D2U_FD: c_uint = 0x4 /* 4 lanes, 2 down, 2 up, full duplex */;

pub const SDHCI_UHS2_CAPS_PTR: c_uint = 0xE2;
pub const SDHCI_UHS2_CAPS_OFFSET: c_int = 0;

pub const SDHCI_UHS2_CAPS_2L_HD_FD: c_int = 1;
pub const SDHCI_UHS2_CAPS_2D1U_FD: c_int = 2;
pub const SDHCI_UHS2_CAPS_1D2U_FD: c_int = 4;
pub const SDHCI_UHS2_CAPS_2D2U_FD: c_int = 8;

pub const SDHCI_UHS2_CAPS_DEV_TYPE_RMV: c_int = 0;
pub const SDHCI_UHS2_CAPS_DEV_TYPE_EMB: c_int = 1;
pub const SDHCI_UHS2_CAPS_DEV_TYPE_EMB_RMV: c_int = 2;

pub const SDHCI_UHS2_CAPS_BUS_TOPO_SHIFT: c_int = 22;
pub const SDHCI_UHS2_CAPS_BUS_TOPO_P2P: c_int = 0;
pub const SDHCI_UHS2_CAPS_BUS_TOPO_RING: c_int = 1;
pub const SDHCI_UHS2_CAPS_BUS_TOPO_HUB: c_int = 2;
pub const SDHCI_UHS2_CAPS_BUS_TOPO_HUB_RING: c_int = 3;
pub const SDHCI_UHS2_CAPS_PHY_OFFSET: c_int = 4;

pub const SDHCI_UHS2_CAPS_PHY_RANGE_A: c_int = 0;
pub const SDHCI_UHS2_CAPS_PHY_RANGE_B: c_int = 1;

pub const SDHCI_UHS2_CAPS_TRAN_OFFSET: c_int = 8;

pub const SDHCI_UHS2_CAPS_TRAN_1_OFFSET: c_int = 12;

pub const SDHCI_UHS2_EMBED_CTRL_PTR: c_uint = 0xE6;
pub const SDHCI_UHS2_VENDOR_PTR: c_uint = 0xE8;
extern "C" {
    pub fn sdhci_uhs2_dump_regs(host: *mut sdhci_host);
}
extern "C" {
    pub fn sdhci_uhs2_reset(host: *mut sdhci_host, mask: u16);
}
extern "C" {
    pub fn sdhci_uhs2_set_power(host: *mut sdhci_host, mode: c_uchar, vdd: c_ushort);
}
extern "C" {
    pub fn sdhci_uhs2_set_timeout(host: *mut sdhci_host, cmd: *mut mmc_command);
}
extern "C" {
    pub fn sdhci_uhs2_add_host(host: *mut sdhci_host) -> c_int;
}
extern "C" {
    pub fn sdhci_uhs2_remove_host(host: *mut sdhci_host, dead: c_int);
}
extern "C" {
    pub fn sdhci_uhs2_clear_set_irqs(host: *mut sdhci_host, clear: u32, set: u32);
}
extern "C" {
    pub fn sdhci_uhs2_irq(host: *mut sdhci_host, intmask: u32) -> u32;
}
