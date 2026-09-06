//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/spi-fsl-dspi.h
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
// Freescale DSPI controller driver
//
// Copyright (c) 2017 Angelo Dureghello <angelo@sysam.it>
//
// struct fsl_dspi_platform_data - platform data for the Freescale DSPI driver
// @bus_num: board specific identifier for this DSPI driver.
// @cs_num: number of chip selects supported by this DSPI driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_dspi_platform_data {
    pub cs_num: u32,
    pub bus_num: u32,
    pub sck_cs_delay: u32,
    pub cs_sck_delay: u32,
}
