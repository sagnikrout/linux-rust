//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mucse/rnpgbe/rnpgbe.h
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
// Copyright(c) 2020 - 2025 Mucse Corporation.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rnpgbe_boards {
    board_n500,
    board_n210
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mucse_mbx_info {
    pub timeout_us: u32,
    pub delay_us: u32,
    pub fw_req: u16,
    pub fw_ack: u16,
// lock for only one use mbx
    pub lock: mutex,
// fw <--> pf mbx
    pub fwpf_shm_base: u32,
    pub pf2fw_mbx_ctrl: u32,
    pub fwpf_mbx_mask: u32,
    pub fwpf_ctrl_base: u32,
}

// Enum for firmware notification modes,
// more modes (e.g., portup, link_report) will be added in future
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mucse_hw {
    pub hw_addr: *mut void __iomem,
    pub pdev: *mut pci_dev,
    pub mbx: mucse_mbx_info,
    pub port: c_int,
    pub pfvfnum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mucse_stats {
    pub tx_dropped: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mucse {
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub hw: mucse_hw,
    pub stats: mucse_stats,
}

extern "C" {
    pub fn rnpgbe_get_permanent_mac(hw: *mut mucse_hw, perm_addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn rnpgbe_reset_hw(hw: *mut mucse_hw) -> c_int;
}
extern "C" {
    pub fn rnpgbe_init_hw(hw: *mut mucse_hw, board_type: c_int) -> c_int;
}
// Device IDs
pub const PCI_VENDOR_ID_MUCSE: c_uint = 0x8848;
pub const RNPGBE_DEVICE_ID_N500_QUAD_PORT: c_uint = 0x8308;
pub const RNPGBE_DEVICE_ID_N500_DUAL_PORT: c_uint = 0x8318;
pub const RNPGBE_DEVICE_ID_N210: c_uint = 0x8208;
pub const RNPGBE_DEVICE_ID_N210L: c_uint = 0x820a;

