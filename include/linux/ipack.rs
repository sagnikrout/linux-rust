//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ipack.h
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
// Industry-pack bus.
//
// Copyright (C) 2011-2012 CERN (www.cern.ch)
// Author: Samuel Iglesias Gonsalvez <siglesias@igalia.com>
//

pub const IPACK_IDPROM_OFFSET_I: c_uint = 0x01;
pub const IPACK_IDPROM_OFFSET_P: c_uint = 0x03;
pub const IPACK_IDPROM_OFFSET_A: c_uint = 0x05;
pub const IPACK_IDPROM_OFFSET_C: c_uint = 0x07;
pub const IPACK_IDPROM_OFFSET_MANUFACTURER_ID: c_uint = 0x09;
pub const IPACK_IDPROM_OFFSET_MODEL: c_uint = 0x0B;
pub const IPACK_IDPROM_OFFSET_REVISION: c_uint = 0x0D;
pub const IPACK_IDPROM_OFFSET_RESERVED: c_uint = 0x0F;
pub const IPACK_IDPROM_OFFSET_DRIVER_ID_L: c_uint = 0x11;
pub const IPACK_IDPROM_OFFSET_DRIVER_ID_H: c_uint = 0x13;
pub const IPACK_IDPROM_OFFSET_NUM_BYTES: c_uint = 0x15;
pub const IPACK_IDPROM_OFFSET_CRC: c_uint = 0x17;
//
// IndustryPack Fromat, Vendor and Device IDs.
//
// ID section format versions
pub const IPACK_ID_VERSION_INVALID: c_uint = 0x00;
pub const IPACK_ID_VERSION_1: c_uint = 0x01;
pub const IPACK_ID_VERSION_2: c_uint = 0x02;
// Vendors and devices. Sort key: vendor first, device next.
pub const IPACK1_VENDOR_ID_RESERVED1: c_uint = 0x00;
pub const IPACK1_VENDOR_ID_RESERVED2: c_uint = 0xFF;
pub const IPACK1_VENDOR_ID_UNREGISTRED01: c_uint = 0x01;
pub const IPACK1_VENDOR_ID_UNREGISTRED02: c_uint = 0x02;
pub const IPACK1_VENDOR_ID_UNREGISTRED03: c_uint = 0x03;
pub const IPACK1_VENDOR_ID_UNREGISTRED04: c_uint = 0x04;
pub const IPACK1_VENDOR_ID_UNREGISTRED05: c_uint = 0x05;
pub const IPACK1_VENDOR_ID_UNREGISTRED06: c_uint = 0x06;
pub const IPACK1_VENDOR_ID_UNREGISTRED07: c_uint = 0x07;
pub const IPACK1_VENDOR_ID_UNREGISTRED08: c_uint = 0x08;
pub const IPACK1_VENDOR_ID_UNREGISTRED09: c_uint = 0x09;
pub const IPACK1_VENDOR_ID_UNREGISTRED10: c_uint = 0x0A;
pub const IPACK1_VENDOR_ID_UNREGISTRED11: c_uint = 0x0B;
pub const IPACK1_VENDOR_ID_UNREGISTRED12: c_uint = 0x0C;
pub const IPACK1_VENDOR_ID_UNREGISTRED13: c_uint = 0x0D;
pub const IPACK1_VENDOR_ID_UNREGISTRED14: c_uint = 0x0E;
pub const IPACK1_VENDOR_ID_UNREGISTRED15: c_uint = 0x0F;
pub const IPACK1_VENDOR_ID_SBS: c_uint = 0xF0;
pub const IPACK1_DEVICE_ID_SBS_OCTAL_232: c_uint = 0x22;
pub const IPACK1_DEVICE_ID_SBS_OCTAL_422: c_uint = 0x2A;
pub const IPACK1_DEVICE_ID_SBS_OCTAL_485: c_uint = 0x48;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipack_space {
    IPACK_IO_SPACE    = 0,
    IPACK_ID_SPACE,
    IPACK_INT_SPACE,
    IPACK_MEM8_SPACE,
    IPACK_MEM16_SPACE,
// Dummy for counting the number of entries.  Must remain the last
// entry
    IPACK_SPACE_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipack_region {
    pub start: phys_addr_t,
    pub size: usize,
}

//
// struct ipack_device - subsystem representation of an IPack device
//
// @slot: Slot where the device is plugged in the carrier board
// @bus: ipack_bus_device where the device is plugged to.
// @id_space: Virtual address to ID space.
// @io_space: Virtual address to IO space.
// @mem_space: Virtual address to MEM space.
// @dev: device in kernel representation.
//
// Warning: Direct access to mapped memory is possible but the endianness
// is not the same with PCI carrier or VME carrier. The endianness is managed
// by the carrier board through bus->ops.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipack_device {
    pub slot: c_uint,
    pub bus: *mut ipack_bus_device,
    pub dev: device,
    pub dev): *mut *mut void (release) (struct ipack_device,
    pub region: [ipack_region; IPACK_SPACE_COUNT],
    pub id: *mut u8,
    pub id_avail: usize,
    pub id_vendor: u32,
    pub id_device: u32,
    pub id_format: u8,
    pub id_crc_correct:1: c_uint,
    pub speed_8mhz:1: c_uint,
    pub speed_32mhz:1: c_uint,
}

//
// struct ipack_driver_ops -- Callbacks to IPack device driver
//
// @probe:  Probe function
// @remove: Prepare imminent removal of the device.  Services provided by the
// device should be revoked.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipack_driver_ops {
    pub dev): *mut *mut int (probe) (struct ipack_device,
    pub dev): *mut *mut void (remove) (struct ipack_device,
}

//
// struct ipack_driver -- Specific data to each ipack device driver
//
// @driver: Device driver kernel representation
// @id_table: Device ID table for this driver
// @ops:    Callbacks provided by the IPack device driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipack_driver {
    pub driver: device_driver,
    pub id_table: *const ipack_device_id,
    pub ops: *const ipack_driver_ops,
}

//
// struct ipack_bus_ops - available operations on a bridge module
//
// @map_space: map IP address space
// @unmap_space: unmap IP address space
// @request_irq: request IRQ
// @free_irq: free IRQ
// @get_clockrate: Returns the clockrate the carrier is currently
// communicating with the device at.
// @set_clockrate: Sets the clock-rate for carrier / module communication.
// Should return -EINVAL if the requested speed is not supported.
// @get_error: Returns the error state for the slot the device is attached
// to.
// @get_timeout: Returns 1 if the communication with the device has
// previously timed out.
// @reset_timeout: Resets the state returned by get_timeout.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipack_bus_ops {
    pub arg): *mut *mut *mut irqreturn_t (handler)(void ), void,
    pub dev): *mut *mut int (free_irq) (struct ipack_device,
    pub dev): *mut *mut int (get_clockrate) (struct ipack_device,
    pub mherz): *mut *mut *mut int (set_clockrate) (struct ipack_device dev, int,
    pub dev): *mut *mut int (get_error) (struct ipack_device,
    pub dev): *mut *mut int (get_timeout) (struct ipack_device,
    pub dev): *mut *mut int (reset_timeout) (struct ipack_device,
}

//
// struct ipack_bus_device - IPack bus representation
//
// @dev: pointer to carrier device
// @slots: number of slots available
// @bus_nr: ipack bus number
// @ops: bus operations for the mezzanine drivers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipack_bus_device {
    pub owner: *mut module,
    pub parent: *mut device,
    pub slots: c_int,
    pub bus_nr: c_int,
    pub ops: *const ipack_bus_ops,
}

//
// ipack_bus_register -- register a new ipack bus
//
// @parent: pointer to the parent device, if any.
// @slots: number of slots available in the bus device.
// @ops: bus operations for the mezzanine drivers.
//
// The carrier board device should call this function to register itself as
// available bus device in ipack.
//
// Return: %NULL on error or &struct ipack_bus_device on success
//
// ipack_bus_unregister -- unregister an ipack bus
//
// Return: %0
//
extern "C" {
    pub fn ipack_bus_unregister(bus: *mut ipack_bus_device) -> c_int;
}
//
// ipack_driver_register -- Register a new ipack device driver
//
// Called by a ipack driver to register itself as a driver
// that can manage ipack devices.
//
// Return: zero on success or error code on failure.
//
extern "C" {
    pub fn ipack_driver_unregister(edrv: *mut ipack_driver);
}
//
// ipack_device_init -- initialize an IPack device
// @dev: the new device to initialize.
//
// Initialize a new IPack device ("module" in IndustryPack jargon). The call
// is done by the carrier driver.  The carrier should populate the fields
// bus and slot as well as the region array of @dev prior to calling this
// function.  The rest of the fields will be allocated and populated
// during initalization.
//
// Return: zero on success or error code on failure.
//
// NOTE: _Never_ directly free @dev after calling this function, even
// if it returned an error! Always use ipack_put_device() to give up the
// reference initialized in this function instead.
//
extern "C" {
    pub fn ipack_device_init(dev: *mut ipack_device) -> c_int;
}
//
// ipack_device_add -- Add an IPack device
// @dev: the new device to add.
//
// Add a new IPack device. The call is done by the carrier driver
// after calling ipack_device_init().
//
// Return: zero on success or error code on failure.
//
// NOTE: _Never_ directly free @dev after calling this function, even
// if it returned an error! Always use ipack_put_device() to give up the
// reference initialized in this function instead.
//
extern "C" {
    pub fn ipack_device_add(dev: *mut ipack_device) -> c_int;
}
extern "C" {
    pub fn ipack_device_del(dev: *mut ipack_device);
}
extern "C" {
    pub fn ipack_get_device(dev: *mut ipack_device);
}
extern "C" {
    pub fn ipack_put_device(dev: *mut ipack_device);
}
//
// DEFINE_IPACK_DEVICE_TABLE - macro used to describe a IndustryPack table
// @_table: device table name
//
// This macro is used to create a struct ipack_device_id array (a device table)
// in a generic manner.
//

//
// IPACK_DEVICE - macro used to describe a specific IndustryPack device
// @_format: the format version (currently either 1 or 2, 8 bit value)
// @vend:    the 8 or 24 bit IndustryPack Vendor ID
// @dev:     the 8 or 16  bit IndustryPack Device ID
//
// This macro is used to create a struct ipack_device_id that matches a specific
// device.
//

//
// ipack_get_carrier - try to increase the carrier ref. counter of
// the carrier module
// @dev: mezzanine device which wants to get the carrier
//
// Return: true on success.
//
extern "C" {
    pub fn try_module_get(_arg: dev->bus->owner) -> return;
}
//
// ipack_get_carrier - it decrease the carrier ref. counter of
// the carrier module
// @dev: mezzanine device which wants to get the carrier
//
