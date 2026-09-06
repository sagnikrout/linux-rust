//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/spi/spi-fsl-cpm.h
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
// Freescale SPI controller driver cpm functions.
//
// Maintainer: Kumar Gala
//
// Copyright (C) 2006 Polycom, Inc.
// Copyright 2010 Freescale Semiconductor, Inc.
//
// CPM SPI and QE buffer descriptors mode support:
// Copyright (c) 2009  MontaVista Software, Inc.
// Author: Anton Vorontsov <avorontsov@ru.mvista.com>
//

extern "C" {
    pub fn fsl_spi_cpm_reinit_txrx(mspi: *mut mpc8xxx_spi);
}
extern "C" {
    pub fn fsl_spi_cpm_bufs_complete(mspi: *mut mpc8xxx_spi);
}
extern "C" {
    pub fn fsl_spi_cpm_irq(mspi: *mut mpc8xxx_spi, events: u32);
}
extern "C" {
    pub fn fsl_spi_cpm_init(mspi: *mut mpc8xxx_spi) -> c_int;
}
extern "C" {
    pub fn fsl_spi_cpm_free(mspi: *mut mpc8xxx_spi);
}

