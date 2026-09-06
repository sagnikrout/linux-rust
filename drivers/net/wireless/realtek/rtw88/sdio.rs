//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/sdio.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (C) 2021 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
// Copyright (C) 2021 Jernej Skrabec <jernej.skrabec@gmail.com>
//
// I/O bus domain address mapping
pub const SDIO_LOCAL_OFFSET: c_uint = 0x10250000;
pub const WLAN_IOREG_OFFSET: c_uint = 0x10260000;
pub const FIRMWARE_FIFO_OFFSET: c_uint = 0x10270000;
pub const TX_HIQ_OFFSET: c_uint = 0x10310000;
pub const TX_MIQ_OFFSET: c_uint = 0x10320000;
pub const TX_LOQ_OFFSET: c_uint = 0x10330000;
pub const TX_EPQ_OFFSET: c_uint = 0x10350000;
pub const RX_RX0FF_OFFSET: c_uint = 0x10340000;
pub const RTW_SDIO_BUS_MSK: c_uint = 0xffff0000;
pub const SDIO_LOCAL_REG_MSK: c_uint = 0x00000fff;
pub const WLAN_IOREG_REG_MSK: c_uint = 0x0000ffff;
// SDIO Tx Control

// SDIO status timeout

// SDIO Host Interrupt Mask

// the following two are RTL8188 SDIO Specific

// SDIO Host Interrupt Service Routine

// the following two are RTL8188 SDIO Specific

// HCI Current Power Mode

// RXDMA Request Length

// OQT Free Page

// Free Tx Buffer Page

// HCI Current Power Mode 1

// HCI Current Power Mode 2

// Free Tx Page Sequence

// HTSF Information

// H2C

// HCI Request Power Mode 1

// HCI Request Power Mode 2

// HCI Power Save Clock

// SDIO HCI Suspend Control

// SDIO Host Extension Interrupt Mask Always

// SDIO Host Extension Interrupt Status Always

// Sdio Address for SDIO Local Reg, TRX FIFO, MAC Reg

pub const REG_SDIO_CMD_ADDR_SDIO_REG: c_int = 0;
pub const REG_SDIO_CMD_ADDR_MAC_REG: c_int = 8;
pub const REG_SDIO_CMD_ADDR_TXFF_HIGH: c_int = 4;
pub const REG_SDIO_CMD_ADDR_TXFF_LOW: c_int = 6;
pub const REG_SDIO_CMD_ADDR_TXFF_NORMAL: c_int = 5;
pub const REG_SDIO_CMD_ADDR_TXFF_EXTRA: c_int = 7;
pub const REG_SDIO_CMD_ADDR_RXFF: c_int = 7;
pub const RTW_SDIO_BLOCK_SIZE: c_int = 512;

pub const RTW_SDIO_DATA_PTR_ALIGN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_sdio_tx_data {
    pub sn: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_sdio_work_data {
    pub work: work_struct,
    pub rtwdev: *mut rtw_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_sdio {
    pub sdio_func: *mut sdio_func,
    pub irq_mask: u32,
    pub rx_addr: u8,
    pub sdio3_bus_mode: bool,
    pub irq_thread: *mut c_void,
    pub txwq: *mut workqueue_struct,
    pub tx_handler_data: *mut rtw_sdio_work_data,
    pub tx_queue: [sk_buff_head; RTK_MAX_TX_QUEUE_NUM],
}

extern "C" {
    pub fn rtw_sdio_remove(sdio_func: *mut sdio_func);
}
extern "C" {
    pub fn rtw_sdio_shutdown(sdio_func: *mut sdio_func);
}
