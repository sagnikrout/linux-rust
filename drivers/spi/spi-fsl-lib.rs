//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/spi/spi-fsl-lib.h
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
// Freescale SPI/eSPI controller driver library.
//
// Maintainer: Kumar Gala
//
// Copyright 2010 Freescale Semiconductor, Inc.
// Copyright (C) 2006 Polycom, Inc.
//
// CPM SPI and QE buffer descriptors mode support:
// Copyright (c) 2009  MontaVista Software, Inc.
// Author: Anton Vorontsov <avorontsov@ru.mvista.com>
//

// SPI/eSPI Controller driver's private data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc8xxx_spi {
    pub dev: *mut device,
    pub reg_base: *mut void __iomem,
// rx & tx bufs from the spi_transfer
    pub tx: *const c_void,
    pub rx: *mut c_void,
    pub subblock: c_int,
    pub pram: *mut spi_pram __iomem,

    pub tx_bd: *mut cpm_buf_desc __iomem,
    pub rx_bd: *mut cpm_buf_desc __iomem,

    pub xfer_in_progress: *mut spi_transfer,
// dma addresses for CPM transfers
    pub tx_dma: dma_addr_t,
    pub rx_dma: dma_addr_t,
    pub map_tx_dma: bool,
    pub map_rx_dma: bool,
    pub dma_dummy_tx: dma_addr_t,
    pub dma_dummy_rx: dma_addr_t,
// functions to deal with different sized buffers
    pub ): *mut *mut void (get_rx) (u32 rx_data, struct mpc8xxx_spi,
    pub ): *mut *mut u32(get_tx) (struct mpc8xxx_spi,
    pub count: c_uint,
    pub irq: c_uint,
    pub /: *mut *mut unsigned nsecs; / (clock cycle time)/2,
    pub /: *mut *mut u32 spibrg; / SPIBRG input clock,
    pub /: *mut *mut u32 rx_shift; / RX data reg shift when in qe mode,
    pub /: *mut *mut u32 tx_shift; / TX data reg shift when in qe mode,
    pub flags: c_uint,

    pub type: c_int,
    pub native_chipselects: c_int,
    pub max_bits_per_word: u8,
    pub msb_first): int bits_per_word, int,

    pub done: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_mpc8xxx_cs {
// functions to deal with different sized buffers
    pub ): *mut *mut void (get_rx) (u32 rx_data, struct mpc8xxx_spi,
    pub ): *mut *mut u32 (get_tx) (struct mpc8xxx_spi,
    pub /: *mut *mut u32 rx_shift; / RX data reg shift when in qe mode,
    pub /: *mut *mut u32 tx_shift; / TX data reg shift when in qe mode,
    pub /: *mut *mut u32 hw_mode; / Holds HW mode register settings,
}

extern "C" {
    pub fn ioread32be(_arg: reg) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc8xxx_spi_probe_info {
    pub pdata: fsl_spi_platform_data,
    pub immr_spi_cs: *mut __be32 __iomem,
}

extern "C" {
    pub fn mpc8xxx_spi_tx_buf_u8(mpc8xxx_spi: *mut mpc8xxx_spi) -> u32;
}
extern "C" {
    pub fn mpc8xxx_spi_tx_buf_u16(mpc8xxx_spi: *mut mpc8xxx_spi) -> u32;
}
extern "C" {
    pub fn mpc8xxx_spi_tx_buf_u32(mpc8xxx_spi: *mut mpc8xxx_spi) -> u32;
}
extern "C" {
    pub fn mpc8xxx_spi_rx_buf_u8(data: u32, mpc8xxx_spi: *mut mpc8xxx_spi);
}
extern "C" {
    pub fn mpc8xxx_spi_rx_buf_u16(data: u32, mpc8xxx_spi: *mut mpc8xxx_spi);
}
extern "C" {
    pub fn mpc8xxx_spi_rx_buf_u32(data: u32, mpc8xxx_spi: *mut mpc8xxx_spi);
}
extern "C" {
    pub fn of_mpc8xxx_spi_probe(ofdev: *mut platform_device) -> c_int;
}
