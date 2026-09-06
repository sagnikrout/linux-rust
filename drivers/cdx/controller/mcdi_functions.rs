//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cdx/controller/mcdi_functions.h
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
//
// Header file for MCDI FW interaction for CDX bus.
//
// Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
//

//
// cdx_mcdi_get_num_buses - Get the total number of buses on
// the controller.
// @cdx: pointer to MCDI interface.
//
// Return: total number of buses available on the controller,
// <0 on failure
//
extern "C" {
    pub fn cdx_mcdi_get_num_buses(cdx: *mut cdx_mcdi) -> c_int;
}
//
// cdx_mcdi_get_num_devs - Get the total number of devices on
// a particular bus of the controller.
// @cdx: pointer to MCDI interface.
// @bus_num: Bus number.
//
// Return: total number of devices available on the bus, <0 on failure
//
extern "C" {
    pub fn cdx_mcdi_get_num_devs(cdx: *mut cdx_mcdi, bus_num: c_int) -> c_int;
}
//
// cdx_mcdi_get_dev_config - Get configuration for a particular
// bus_num:dev_num
// @cdx: pointer to MCDI interface.
// @bus_num: Bus number.
// @dev_num: Device number.
// @dev_params: Pointer to cdx_dev_params, this is populated by this
// device with the configuration corresponding to the provided
// bus_num:dev_num.
//
// Return: 0 total number of devices available on the bus, <0 on failure
//
// cdx_mcdi_bus_enable - Enable CDX bus represented by bus_num
// @cdx: pointer to MCDI interface.
// @bus_num: Bus number.
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn cdx_mcdi_bus_enable(cdx: *mut cdx_mcdi, bus_num: u8) -> c_int;
}
//
// cdx_mcdi_bus_disable - Disable CDX bus represented by bus_num
// @cdx: pointer to MCDI interface.
// @bus_num: Bus number.
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn cdx_mcdi_bus_disable(cdx: *mut cdx_mcdi, bus_num: u8) -> c_int;
}
//
// cdx_mcdi_write_msi - Write MSI configuration for CDX device
// @cdx: pointer to MCDI interface.
// @bus_num: Bus number.
// @dev_num: Device number.
// @msi_vector: Device-relative MSI vector number.
// Must be < MSI_COUNT reported for the device.
// @msi_address: MSI address to be used by the hardware. Typically, on ARM
// systems this address is translated by the IOMMU (if enabled) and
// it is the responsibility of the entity managing the IOMMU (APU kernel)
// to supply the correct IOVA here.
// @msi_data: MSI data to be used by the hardware. On versal-net, only the
// lower 16-bits are used, the remaining bits are ignored and should be
// set to zero.
//
// Return: 0 on success, <0 on failure
//
// cdx_mcdi_reset_device - Reset cdx device represented by bus_num:dev_num
// @cdx: pointer to MCDI interface.
// @bus_num: Bus number.
// @dev_num: Device number.
//
// Return: 0 on success, <0 on failure
//
// cdx_mcdi_bus_master_enable - Set/Reset bus mastering for cdx device
// represented by bus_num:dev_num
// @cdx: pointer to MCDI interface.
// @bus_num: Bus number.
// @dev_num: Device number.
// @enable: Enable bus mastering if set, disable otherwise.
//
// Return: 0 on success, <0 on failure
//
// cdx_mcdi_msi_enable - Enable/Disable MSIs for cdx device represented
// by bus_num:dev_num
// @cdx: pointer to MCDI interface.
// @bus_num: Bus number.
// @dev_num: Device number.
// @enable: Enable msi's if set, disable otherwise.
//
// Return: 0 on success, <0 on failure
//
