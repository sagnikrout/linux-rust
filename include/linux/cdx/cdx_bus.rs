//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cdx/cdx_bus.h
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
// CDX bus public interface
//
// Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
//

pub const MAX_CDX_DEV_RESOURCES: c_int = 4;
pub const CDX_CONTROLLER_ID_SHIFT: c_int = 4;
pub const CDX_BUS_NUM_MASK: c_uint = 0xF;
// Forward declaration for CDX controller
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_msi_config {
    pub addr: u64,
    pub data: u32,
    pub msi_index: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_device_config {
    pub type: u8,
    pub msi: cdx_msi_config,
    pub bus_master_enable: bool,
    pub msi_enable: bool,
}

extern "C" {
    pub fn int(cdx: *mut *mut cdx_bus_enable_cb)(struct cdx_controller, bus_num: u8) -> typedef;
}
extern "C" {
    pub fn int(cdx: *mut *mut cdx_bus_disable_cb)(struct cdx_controller, bus_num: u8) -> typedef;
}
extern "C" {
    pub fn int(cdx: *mut *mut cdx_scan_cb)(struct cdx_controller) -> typedef;
}
//
// CDX_DEVICE - macro used to describe a specific CDX device
// @vend: the 16 bit CDX Vendor ID
// @dev: the 16 bit CDX Device ID
//
// This macro is used to create a struct cdx_device_id that matches a
// specific device. The subvendor and subdevice fields will be set to
// CDX_ANY_ID.
//

//
// CDX_DEVICE_DRIVER_OVERRIDE - macro used to describe a CDX device with
// override_only flags.
// @vend: the 16 bit CDX Vendor ID
// @dev: the 16 bit CDX Device ID
// @driver_override: the 32 bit CDX Device override_only
//
// This macro is used to create a struct cdx_device_id that matches only a
// driver_override device. The subvendor and subdevice fields will be set to
// CDX_ANY_ID.
//

//
// struct cdx_ops - Callbacks supported by CDX controller.
// @bus_enable: enable bus on the controller
// @bus_disable: disable bus on the controller
// @scan: scan the devices on the controller
// @dev_configure: configuration like reset, master_enable,
// msi_config etc for a CDX device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_ops {
    pub bus_enable: cdx_bus_enable_cb,
    pub bus_disable: cdx_bus_disable_cb,
    pub scan: cdx_scan_cb,
    pub dev_configure: cdx_dev_configure_cb,
}

//
// struct cdx_controller: CDX controller object
// @dev: Linux device associated with the CDX controller.
// @priv: private data
// @msi_domain: MSI domain
// @id: Controller ID
// @controller_registered: controller registered with bus
// @ops: CDX controller ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_controller {
    pub dev: *mut device,
    pub priv: *mut c_void,
    pub msi_domain: *mut irq_domain,
    pub id: u32,
    pub controller_registered: bool,
    pub ops: *mut cdx_ops,
}

//
// struct cdx_device - CDX device object
// @dev: Linux driver model device object
// @cdx: CDX controller associated with the device
// @vendor: Vendor ID for CDX device
// @device: Device ID for CDX device
// @subsystem_vendor: Subsystem Vendor ID for CDX device
// @subsystem_device: Subsystem Device ID for CDX device
// @class: Class for the CDX device
// @revision: Revision of the CDX device
// @bus_num: Bus number for this CDX device
// @dev_num: Device number for this device
// @res: array of MMIO region entries
// @res_attr: resource binary attribute
// @debugfs_dir: debugfs directory for this device
// @res_count: number of valid MMIO regions
// @dma_mask: Default DMA mask
// @flags: CDX device flags
// @req_id: Requestor ID associated with CDX device
// @is_bus: Is this bus device
// @enabled: is this bus enabled
// @msi_dev_id: MSI Device ID associated with CDX device
// @num_msi: Number of MSI's supported by the device
// @irqchip_lock: lock to synchronize irq/msi configuration
// @msi_write_pending: MSI write pending for this device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_device {
    pub dev: device,
    pub cdx: *mut cdx_controller,
    pub vendor: u16,
    pub device: u16,
    pub subsystem_vendor: u16,
    pub subsystem_device: u16,
    pub class: u32,
    pub revision: u8,
    pub bus_num: u8,
    pub dev_num: u8,
    pub res: [resource; MAX_CDX_DEV_RESOURCES],
    pub res_attr: [*mut bin_attribute; MAX_CDX_DEV_RESOURCES],
    pub debugfs_dir: *mut dentry,
    pub res_count: u8,
    pub dma_mask: u64,
    pub flags: u16,
    pub req_id: u32,
    pub is_bus: bool,
    pub enabled: bool,
    pub msi_dev_id: u32,
    pub num_msi: u32,
    pub irqchip_lock: mutex,
    pub msi_write_pending: bool,
}

//
// struct cdx_driver - CDX device driver
// @driver: Generic device driver
// @match_id_table: table of supported device matching Ids
// @probe: Function called when a device is added
// @remove: Function called when a device is removed
// @shutdown: Function called at shutdown time to quiesce the device
// @reset_prepare: Function called before is reset to notify driver
// @reset_done: Function called after reset is complete to notify driver
// @driver_managed_dma: Device driver doesn't use kernel DMA API for DMA.
// For most device drivers, no need to care about this flag
// as long as all DMAs are handled through the kernel DMA API.
// For some special ones, for example VFIO drivers, they know
// how to manage the DMA themselves and set this flag so that
// the IOMMU layer will allow them to setup and manage their
// own I/O address space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_driver {
    pub driver: device_driver,
    pub match_id_table: *const cdx_device_id,
    pub dev): *mut *mut int (probe)(struct cdx_device,
    pub dev): *mut *mut int (remove)(struct cdx_device,
    pub dev): *mut *mut void (shutdown)(struct cdx_device,
    pub dev): *mut *mut void (reset_prepare)(struct cdx_device,
    pub dev): *mut *mut void (reset_done)(struct cdx_device,
    pub driver_managed_dma: bool,
}

// Macro to avoid include chaining to get THIS_MODULE

//
// __cdx_driver_register - registers a CDX device driver
// @cdx_driver: CDX driver to register
// @owner: module owner
//
// Return: -errno on failure, 0 on success.
//
// cdx_driver_unregister - unregisters a device driver from the
// CDX bus.
// @cdx_driver: CDX driver to register
//
extern "C" {
    pub fn cdx_driver_unregister(cdx_driver: *mut cdx_driver);
}
//
// cdx_dev_reset - Reset CDX device
// @dev: device pointer
//
// Return: 0 for success, -errno on failure
//
extern "C" {
    pub fn cdx_dev_reset(dev: *mut device) -> c_int;
}
//
// cdx_set_master - enables bus-mastering for CDX device
// @cdx_dev: the CDX device to enable
//
// Return: 0 for success, -errno on failure
//
extern "C" {
    pub fn cdx_set_master(cdx_dev: *mut cdx_device) -> c_int;
}
//
// cdx_clear_master - disables bus-mastering for CDX device
// @cdx_dev: the CDX device to disable
//
// Return: 0 for success, -errno on failure
//
extern "C" {
    pub fn cdx_clear_master(cdx_dev: *mut cdx_device) -> c_int;
}

//
// cdx_enable_msi - Enable MSI for the CDX device.
// @cdx_dev: device pointer
//
// Return: 0 for success, -errno on failure
//
extern "C" {
    pub fn cdx_enable_msi(cdx_dev: *mut cdx_device) -> c_int;
}
//
// cdx_disable_msi - Disable MSI for the CDX device.
// @cdx_dev: device pointer
//
extern "C" {
    pub fn cdx_disable_msi(cdx_dev: *mut cdx_device);
}

