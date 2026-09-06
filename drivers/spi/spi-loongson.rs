//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/spi/spi-loongson.h
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


// SPDX-License-Identifier: GPL-2.0+
// Header File for Loongson SPI Driver.
// Copyright (C) 2023 Loongson Technology Corporation Limited

pub const LOONGSON_SPI_SPCR_REG: c_uint = 0x00;
pub const LOONGSON_SPI_SPSR_REG: c_uint = 0x01;
pub const LOONGSON_SPI_FIFO_REG: c_uint = 0x02;
pub const LOONGSON_SPI_SPER_REG: c_uint = 0x03;
pub const LOONGSON_SPI_PARA_REG: c_uint = 0x04;
pub const LOONGSON_SPI_SFCS_REG: c_uint = 0x05;
pub const LOONGSON_SPI_TIMI_REG: c_uint = 0x06;
// Bits definition for Loongson SPI register

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson_spi {
    pub controller: *mut spi_controller,
    pub base: *mut void __iomem,
    pub cs_active: c_int,
    pub hz: c_uint,
    pub spcr: c_uchar,
    pub sper: c_uchar,
    pub spsr: c_uchar,
    pub para: c_uchar,
    pub sfcs: c_uchar,
    pub timi: c_uchar,
    pub mode: c_uint,
    pub clk_rate: u64,
}

extern "C" {
    pub fn loongson_spi_init_controller(dev: *mut device, reg: *mut void __iomem) -> c_int;
}
