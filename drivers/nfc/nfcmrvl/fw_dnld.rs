//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nfc/nfcmrvl/fw_dnld.h
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
// Marvell NFC driver: Firmware downloader
//
// Copyright (C) 2015, Marvell International Ltd.
//

pub const NFCMRVL_FW_MAGIC: c_uint = 0x88888888;
pub const NCI_OP_PROP_BOOT_CMD: c_uint = 0x3A;
pub const NCI_CORE_LC_PROP_FW_DL: c_uint = 0xFD;
pub const NCI_CORE_LC_CONNID_PROP_FW_DL: c_uint = 0x02;
pub const HELPER_CMD_ENTRY_POINT: c_uint = 0x04;
pub const HELPER_CMD_PACKET_FORMAT: c_uint = 0xA5;
pub const HELPER_ACK_PACKET_FORMAT: c_uint = 0x5A;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfcmrvl_fw_uart_config {
    pub flow_control: u8,
    pub baudrate: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfcmrvl_fw_i2c_config {
    pub clk: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfcmrvl_fw_spi_config {
    pub clk: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfcmrvl_fw_binary_config {
    pub offset: u32,
    pub config: *mut c_void,
    pub uart: nfcmrvl_fw_uart_config,
    pub i2c: nfcmrvl_fw_i2c_config,
    pub spi: nfcmrvl_fw_spi_config,
    pub reserved: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfcmrvl_fw {
    pub magic: u32,
    pub ref_clock: u32,
    pub phy: u32,
    pub bootrom: nfcmrvl_fw_binary_config,
    pub helper: nfcmrvl_fw_binary_config,
    pub firmware: nfcmrvl_fw_binary_config,
    pub reserved: [u8; 64],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfcmrvl_fw_dnld {
    pub 1]: char name[NFC_FIRMWARE_NAME_MAXSIZE +,
    pub fw: *const firmware,
    pub header: *const nfcmrvl_fw,
    pub binary_config: *const nfcmrvl_fw_binary_config,
    pub state: c_int,
    pub substate: c_int,
    pub offset: c_int,
    pub chunk_len: c_int,
    pub rx_wq: *mut workqueue_struct,
    pub rx_work: work_struct,
    pub rx_q: sk_buff_head,
    pub timer: timer_list,
}

extern "C" {
    pub fn nfcmrvl_fw_dnld_init(priv: *mut nfcmrvl_private) -> c_int;
}
extern "C" {
    pub fn nfcmrvl_fw_dnld_deinit(priv: *mut nfcmrvl_private);
}
extern "C" {
    pub fn nfcmrvl_fw_dnld_abort(priv: *mut nfcmrvl_private);
}
extern "C" {
    pub fn nfcmrvl_fw_dnld_start(ndev: *mut nci_dev, firmware_name: *const c_char) -> c_int;
}
