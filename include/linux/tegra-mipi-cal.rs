//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tegra-mipi-cal.h
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


// SPDX-License-Identifier: GPL-2.0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mipi_device {
    pub ops: *const tegra_mipi_ops,
    pub pdev: *mut platform_device,
    pub pads: c_ulong,
}

//
// Operations for Tegra MIPI calibration device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mipi_ops {
//
// @enable:
//
// Enable MIPI calibration device
//
    pub device): *mut *mut int (enable)(struct tegra_mipi_device,
//
// @disable:
//
// Disable MIPI calibration device
//
    pub device): *mut *mut int (disable)(struct tegra_mipi_device,
//
// @start_calibration:
//
// Start MIPI calibration
//
    pub device): *mut *mut int (start_calibration)(struct tegra_mipi_device,
//
// @finish_calibration:
//
// Finish MIPI calibration
//
    pub device): *mut *mut int (finish_calibration)(struct tegra_mipi_device,
}

extern "C" {
    pub fn tegra_mipi_free(device: *mut tegra_mipi_device);
}
extern "C" {
    pub fn tegra_mipi_enable(device: *mut tegra_mipi_device) -> c_int;
}
extern "C" {
    pub fn tegra_mipi_disable(device: *mut tegra_mipi_device) -> c_int;
}
extern "C" {
    pub fn tegra_mipi_start_calibration(device: *mut tegra_mipi_device) -> c_int;
}
extern "C" {
    pub fn tegra_mipi_finish_calibration(device: *mut tegra_mipi_device) -> c_int;
}
