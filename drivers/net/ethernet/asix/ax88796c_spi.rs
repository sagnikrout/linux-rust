//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/asix/ax88796c_spi.h
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
// Copyright (c) 2010 ASIX Electronics Corporation
// Copyright (c) 2020 Samsung Electronics Co., Ltd.
//
// ASIX AX88796C SPI Fast Ethernet Linux driver
//

// Definition of SPI command
pub const AX_SPICMD_WRITE_TXQ: c_uint = 0x02;
pub const AX_SPICMD_READ_REG: c_uint = 0x03;
pub const AX_SPICMD_READ_STATUS: c_uint = 0x05;
pub const AX_SPICMD_READ_RXQ: c_uint = 0x0B;
pub const AX_SPICMD_BIDIR_WRQ: c_uint = 0xB2;
pub const AX_SPICMD_WRITE_REG: c_uint = 0xD8;
pub const AX_SPICMD_EXIT_PWD: c_uint = 0xAB;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct axspi_data {
    pub spi: *mut spi_device,
    pub rx_msg: spi_message,
    pub spi_rx_xfer: [spi_transfer; 2],
    pub cmd_buf: [u8; 6],
    pub rx_buf: [u8; 6],
    pub comp: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_status {
    pub isr: u16,
    pub status: u8,

}

extern "C" {
    pub fn axspi_read_rxq(ax_spi: *mut axspi_data, data: *mut c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn axspi_write_txq(ax_spi: *const axspi_data, data: *mut c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn axspi_read_reg(ax_spi: *mut axspi_data, reg: u8) -> u16;
}
extern "C" {
    pub fn axspi_write_reg(ax_spi: *mut axspi_data, reg: u8, value: u16) -> c_int;
}
extern "C" {
    pub fn axspi_read_status(ax_spi: *mut axspi_data, status: *mut spi_status) -> c_int;
}
extern "C" {
    pub fn axspi_wakeup(ax_spi: *mut axspi_data) -> c_int;
}
extern "C" {
    pub fn axspi_read_reg(_arg: ax_spi, _arg: offset) -> return;
}
extern "C" {
    pub fn axspi_write_reg(_arg: ax_spi, _arg: offset, _arg: value) -> return;
}
extern "C" {
    pub fn axspi_read_status(_arg: ax_spi, _arg: status) -> return;
}
extern "C" {
    pub fn axspi_wakeup(_arg: ax_spi) -> return;
}
