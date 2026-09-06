//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/morsemicro/mm81x/hw.h
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
//
// Copyright (c) 2017-2026 Morse Micro
//

// This should be at a fixed location for a family of chipset
pub const MM8108_REG_CHIP_ID: c_uint = 0x00002d20;

pub const MM81X_CONFIG_ACCESS_1BYTE: c_int = 0;
pub const MM81X_CONFIG_ACCESS_2BYTE: c_int = 1;
pub const MM81X_CONFIG_ACCESS_4BYTE: c_int = 2;

// Bit 17 to 24 reserved for the beacon VIF 0 to 7 interrupts

// PV0 NDP probe interrupts (VIF 0 and 1).

// Bit 27 Chip to Host stop notify

// Chip IDs
pub const CHIP_ID_MM8108: c_uint = 0x809;
//
// Minimum time we must wait between attempting to reload the HW after a
// stop notification
//
pub const HW_RELOAD_AFTER_STOP_WINDOW: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_table_firmware_flags {
    MM81X_FW_FLAGS_SUPPORT_S1G = BIT(0),
    MM81X_FW_FLAGS_BUSY_ACTIVE_LOW = BIT(1),
    MM81X_FW_FLAGS_REPORTS_TX_BEACON_COMPLETION = BIT(2),
    MM81X_FW_FLAGS_SUPPORT_HW_SCAN = BIT(3),
    MM81X_FW_FLAGS_SUPPORT_CHIP_HALT_IRQ = BIT(4),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_table {
    pub magic_number: __le32,
    pub fw_version_number: __le32,
    pub host_flags: __le32,
    pub fw_flags: __le32,
    pub memcmd_cmd_addr: __le32,
    pub memcmd_resp_addr: __le32,
    pub ext_host_tbl_addr: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_regs {
    pub chip_id_address: u32,
    pub irq_base_address: u32,
    pub trgr_base_address: u32,
    pub cpu_reset_address: u32,
    pub cpu_reset_value: u32,
    pub msi_address: u32,
    pub msi_value: u32,
    pub manifest_ptr_address: u32,
    pub magic_num_value: u32,
    pub clk_ctrl_address: u32,
    pub clk_ctrl_value: u32,
    pub early_clk_ctrl_value: u32,
    pub boot_address: u32,
    pub boot_value: u32,
    pub pager_base_address: u32,
    pub aon_latch: u32,
    pub aon_latch_mask: u32,
    pub aon_reset_usb_value: u32,
    pub aon: u32,
    pub aon_count: u8,
}

extern "C" {
    pub fn mm81x_hw_otp_get_board_type(mors: *mut mm81x) -> c_int;
}
extern "C" {
    pub fn mm81x_hw_otp_valid_board_type(board_type: u32) -> bool;
}
extern "C" {
    pub fn mm81x_hw_otp_get_mac_addr(mors: *mut mm81x) -> c_int;
}
extern "C" {
    pub fn mm81x_hw_irq_enable(mors: *mut mm81x, irq: u32, enable: bool);
}
extern "C" {
    pub fn mm81x_hw_irq_handle(mors: *mut mm81x) -> c_int;
}
extern "C" {
    pub fn mm81x_hw_irq_clear(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_hw_toggle_aon_latch(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_hw_enable_burst_mode(mors: *mut mm81x, burst_mode: u8);
}
extern "C" {
    pub fn mm81x_hw_digital_reset(mors: *mut mm81x) -> c_int;
}
extern "C" {
    pub fn mm81x_hw_pre_firmware_ndr_hook(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_hw_post_firmware_ndr_hook(mors: *mut mm81x);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdio_burst_mode {
    SDIO_WORD_BURST_DISABLE =
    0, /* Intentionally duplicate to make it clear it's disabled */
    SDIO_WORD_BURST_SIZE_0 = 0, /* 000: no bursting (single 32bit word) */
    SDIO_WORD_BURST_SIZE_2 = 1, /* 001: bursts of 2 words */
    SDIO_WORD_BURST_SIZE_4 = 2, /* 010: bursts of 4 words */
    SDIO_WORD_BURST_SIZE_8 = 3, /* 011: bursts of 8 words */
    SDIO_WORD_BURST_SIZE_16 = 4, /* 100: bursts of 16 words */
    SDIO_WORD_BURST_MASK = 7,
}

extern "C" {
    pub fn mm81x_hw_enable_stop_notifications(mors: *mut mm81x, enable: bool);
}
