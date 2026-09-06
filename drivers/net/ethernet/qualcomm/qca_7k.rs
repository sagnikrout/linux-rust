//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qualcomm/qca_7k.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright (c) 2011, 2012, Qualcomm Atheros Communications Inc.
// Copyright (c) 2014, I2SE GmbH
//
// Qualcomm Atheros SPI register definition.
//
// This module is designed to define the Qualcomm Atheros SPI
// register placeholders.
//

pub const QCASPI_CMD_LEN: c_int = 2;
pub const QCASPI_HW_PKT_LEN: c_int = 4;
pub const QCASPI_HW_BUF_LEN: c_uint = 0xC5B;
// SPI registers;
pub const SPI_REG_BFR_SIZE: c_uint = 0x0100;
pub const SPI_REG_WRBUF_SPC_AVA: c_uint = 0x0200;
pub const SPI_REG_RDBUF_BYTE_AVA: c_uint = 0x0300;
pub const SPI_REG_SPI_CONFIG: c_uint = 0x0400;
pub const SPI_REG_SPI_STATUS: c_uint = 0x0500;
pub const SPI_REG_INTR_CAUSE: c_uint = 0x0C00;
pub const SPI_REG_INTR_ENABLE: c_uint = 0x0D00;
pub const SPI_REG_RDBUF_WATERMARK: c_uint = 0x1200;
pub const SPI_REG_WRBUF_WATERMARK: c_uint = 0x1300;
pub const SPI_REG_SIGNATURE: c_uint = 0x1A00;
pub const SPI_REG_ACTION_CTRL: c_uint = 0x1B00;
// SPI_CONFIG register definition;

// INTR_CAUSE/ENABLE register definition.

extern "C" {
    pub fn qcaspi_spi_error(qca: *mut qcaspi);
}
extern "C" {
    pub fn qcaspi_read_register(qca: *mut qcaspi, reg: u16, result: *mut u16) -> c_int;
}
extern "C" {
    pub fn qcaspi_write_register(qca: *mut qcaspi, reg: u16, value: u16, retry: c_int) -> c_int;
}
