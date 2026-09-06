//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/spi/spi-amd.h
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
// AMD SPI controller driver common stuff
//
// Copyright (c) 2025, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: Krishnamoorthi M <krishnamoorthi.m@amd.com>
//
// enum amd_spi_versions - SPI controller versions
// @AMD_SPI_V1:         AMDI0061 hardware version
// @AMD_SPI_V2:         AMDI0062 hardware version
// @AMD_HID2_SPI:       AMDI0063 hardware version
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_spi_versions {
    AMD_SPI_V1 = 1,
    AMD_SPI_V2,
    AMD_HID2_SPI,
}

//
// struct amd_spi - SPI driver instance
// @io_remap_addr:      Start address of the SPI controller registers
// @phy_dma_buf:        Physical address of DMA buffer
// @dma_virt_addr:      Virtual address of DMA buffer
// @version:            SPI controller hardware version
// @speed_hz:           Device frequency
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_spi {
    pub io_remap_addr: *mut void __iomem,
    pub phy_dma_buf: dma_addr_t,
    pub dma_virt_addr: *mut c_void,
    pub version: amd_spi_versions,
    pub speed_hz: c_uint,
}

extern "C" {
    pub fn amd_spi_probe_common(dev: *mut device, host: *mut spi_controller) -> c_int;
}
