//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bluetooth/btmrvl_sdio.h
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
// Marvell BT-over-SDIO driver: SDIO interface related definitions
//
// Copyright (C) 2009, Marvell International Ltd.
//
pub const SDIO_HEADER_LEN: c_int = 4;
// SD block size can not bigger than 64 due to buf size limit in firmware
// define SD block size for data Tx/Rx
pub const SDIO_BLOCK_SIZE: c_int = 64;
// Number of blocks for firmware transfer
pub const FIRMWARE_TRANSFER_NBLOCK: c_int = 2;
// This is for firmware specific length
pub const FW_EXTRA_LEN: c_int = 36;

// SDIO_BLOCK_SIZE)
// The number of times to try when polling for status
pub const MAX_POLL_TRIES: c_int = 100;
// Max retry number of CMD53 write
pub const MAX_WRITE_IOMEM_RETRY: c_int = 2;
// register bitmasks

pub const HIM_DISABLE: c_uint = 0xff;

pub const FIRMWARE_READY: c_uint = 0xfedc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmrvl_plt_wake_cfg {
    pub irq_bt: c_int,
    pub wake_by_bt: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmrvl_sdio_card_reg {
    pub cfg: u8,
    pub host_int_mask: u8,
    pub host_intstatus: u8,
    pub card_status: u8,
    pub sq_read_base_addr_a0: u8,
    pub sq_read_base_addr_a1: u8,
    pub card_revision: u8,
    pub card_fw_status0: u8,
    pub card_fw_status1: u8,
    pub card_rx_len: u8,
    pub card_rx_unit: u8,
    pub io_port_0: u8,
    pub io_port_1: u8,
    pub io_port_2: u8,
    pub int_read_to_clear: bool,
    pub host_int_rsr: u8,
    pub card_misc_cfg: u8,
    pub fw_dump_ctrl: u8,
    pub fw_dump_start: u8,
    pub fw_dump_end: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmrvl_sdio_card {
    pub func: *mut sdio_func,
    pub ioport: u32,
    pub helper: *const c_char,
    pub firmware: *const c_char,
    pub reg: *const btmrvl_sdio_card_reg,
    pub support_pscan_win_report: bool,
    pub supports_fw_dump: bool,
    pub sd_blksz_fw_dl: u16,
    pub rx_unit: u8,
    pub priv: *mut btmrvl_private,
    pub plt_of_node: *mut device_node,
    pub plt_wake_cfg: *mut btmrvl_plt_wake_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmrvl_sdio_device {
    pub helper: *const c_char,
    pub firmware: *const c_char,
    pub reg: *const btmrvl_sdio_card_reg,
    pub support_pscan_win_report: bool,
    pub sd_blksz_fw_dl: u16,
    pub supports_fw_dump: bool,
}

// Platform specific DMA alignment
pub const BTSDIO_DMA_ALIGN: c_int = 8;
// Macros for Data Alignment : size

// Macros for Data Alignment : address
