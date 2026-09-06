//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cdx/cdx.h
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
// Header file for the CDX Bus
//
// Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
//

//
// struct cdx_dev_params - CDX device parameters
// @cdx: CDX controller associated with the device
// @parent: Associated CDX Bus device
// @vendor: Vendor ID for CDX device
// @device: Device ID for CDX device
// @subsys_vendor: Sub vendor ID for CDX device
// @subsys_device: Sub device ID for CDX device
// @bus_num: Bus number for this CDX device
// @dev_num: Device number for this device
// @res: array of MMIO region entries
// @res_count: number of valid MMIO regions
// @req_id: Requestor ID associated with CDX device
// @class: Class of the CDX Device
// @revision: Revision of the CDX device
// @msi_dev_id: MSI device ID associated with CDX device
// @num_msi: Number of MSI's supported by the device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_dev_params {
    pub cdx: *mut cdx_controller,
    pub parent: *mut device,
    pub vendor: u16,
    pub device: u16,
    pub subsys_vendor: u16,
    pub subsys_device: u16,
    pub bus_num: u8,
    pub dev_num: u8,
    pub res: [resource; MAX_CDX_DEV_RESOURCES],
    pub res_count: u8,
    pub req_id: u32,
    pub class: u32,
    pub revision: u8,
    pub msi_dev_id: u32,
    pub num_msi: u32,
}

//
// cdx_register_controller - Register a CDX controller and its ports
// on the CDX bus.
// @cdx: The CDX controller to register
//
// Return: -errno on failure, 0 on success.
//
extern "C" {
    pub fn cdx_register_controller(cdx: *mut cdx_controller) -> c_int;
}
//
// cdx_unregister_controller - Unregister a CDX controller
// @cdx: The CDX controller to unregister
//
extern "C" {
    pub fn cdx_unregister_controller(cdx: *mut cdx_controller);
}
//
// cdx_device_add - Add a CDX device. This function adds a CDX device
// on the CDX bus as per the device parameters provided
// by caller. It also creates and registers an associated
// Linux generic device.
// @dev_params: device parameters associated with the device to be created.
//
// Return: -errno on failure, 0 on success.
//
extern "C" {
    pub fn cdx_device_add(dev_params: *mut cdx_dev_params) -> c_int;
}
//
// cdx_bus_add - Add a CDX bus. This function adds a bus on the CDX bus
// subsystem. It creates a CDX device for the corresponding bus and
// also registers an associated Linux generic device.
// @cdx: Associated CDX controller
// @us_num: Bus number
//
// Return: associated Linux generic device pointer on success or NULL on failure.
//
// cdx_msi_domain_init - Init the CDX bus MSI domain.
// @dev: Device of the CDX bus controller
//
// Return: CDX MSI domain, NULL on failure
//
