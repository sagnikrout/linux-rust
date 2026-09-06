//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rio_drv.h
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
// RapidIO driver services
//
// Copyright 2005 MontaVista Software, Inc.
// Matt Porter <mporter@kernel.crashing.org>
//

//
// rio_local_read_config_32 - Read 32 bits from local configuration space
// @port: Master port
// @offset: Offset into local configuration space
// @data: Pointer to read data into
//
// Reads 32 bits of data from the specified offset within the local
// device's configuration space.
//
extern "C" {
    pub fn __rio_local_read_config_32(_arg: port, _arg: offset, _arg: data) -> return;
}
//
// rio_local_write_config_32 - Write 32 bits to local configuration space
// @port: Master port
// @offset: Offset into local configuration space
// @data: Data to be written
//
// Writes 32 bits of data to the specified offset within the local
// device's configuration space.
//
extern "C" {
    pub fn __rio_local_write_config_32(_arg: port, _arg: offset, _arg: data) -> return;
}
//
// rio_local_read_config_16 - Read 16 bits from local configuration space
// @port: Master port
// @offset: Offset into local configuration space
// @data: Pointer to read data into
//
// Reads 16 bits of data from the specified offset within the local
// device's configuration space.
//
extern "C" {
    pub fn __rio_local_read_config_16(_arg: port, _arg: offset, _arg: data) -> return;
}
//
// rio_local_write_config_16 - Write 16 bits to local configuration space
// @port: Master port
// @offset: Offset into local configuration space
// @data: Data to be written
//
// Writes 16 bits of data to the specified offset within the local
// device's configuration space.
//
extern "C" {
    pub fn __rio_local_write_config_16(_arg: port, _arg: offset, _arg: data) -> return;
}
//
// rio_local_read_config_8 - Read 8 bits from local configuration space
// @port: Master port
// @offset: Offset into local configuration space
// @data: Pointer to read data into
//
// Reads 8 bits of data from the specified offset within the local
// device's configuration space.
//
extern "C" {
    pub fn __rio_local_read_config_8(_arg: port, _arg: offset, _arg: data) -> return;
}
//
// rio_local_write_config_8 - Write 8 bits to local configuration space
// @port: Master port
// @offset: Offset into local configuration space
// @data: Data to be written
//
// Writes 8 bits of data to the specified offset within the local
// device's configuration space.
//
extern "C" {
    pub fn __rio_local_write_config_8(_arg: port, _arg: offset, _arg: data) -> return;
}
//
// rio_read_config_32 - Read 32 bits from configuration space
// @rdev: RIO device
// @offset: Offset into device configuration space
// @data: Pointer to read data into
//
// Reads 32 bits of data from the specified offset within the
// RIO device's configuration space.
//
// rio_write_config_32 - Write 32 bits to configuration space
// @rdev: RIO device
// @offset: Offset into device configuration space
// @data: Data to be written
//
// Writes 32 bits of data to the specified offset within the
// RIO device's configuration space.
//
// rio_read_config_16 - Read 16 bits from configuration space
// @rdev: RIO device
// @offset: Offset into device configuration space
// @data: Pointer to read data into
//
// Reads 16 bits of data from the specified offset within the
// RIO device's configuration space.
//
// rio_write_config_16 - Write 16 bits to configuration space
// @rdev: RIO device
// @offset: Offset into device configuration space
// @data: Data to be written
//
// Writes 16 bits of data to the specified offset within the
// RIO device's configuration space.
//
// rio_read_config_8 - Read 8 bits from configuration space
// @rdev: RIO device
// @offset: Offset into device configuration space
// @data: Pointer to read data into
//
// Reads 8 bits of data from the specified offset within the
// RIO device's configuration space.
//
// rio_write_config_8 - Write 8 bits to configuration space
// @rdev: RIO device
// @offset: Offset into device configuration space
// @data: Data to be written
//
// Writes 8 bits of data to the specified offset within the
// RIO device's configuration space.
//
// rio_send_doorbell - Send a doorbell message to a device
// @rdev: RIO device
// @data: Doorbell message data
//
// Send a doorbell message to a RIO device. The doorbell message
// has a 16-bit info field provided by the @data argument.
//
extern "C" {
    pub fn rio_mport_send_doorbell(_arg: rdev->net->hport, _arg: rdev->destid, _arg: data) -> return;
}
//
// rio_init_mbox_res - Initialize a RIO mailbox resource
// @res: resource struct
// @start: start of mailbox range
// @end: end of mailbox range
//
// This function is used to initialize the fields of a resource
// for use as a mailbox resource.  It initializes a range of
// mailboxes using the start and end arguments.
//
// rio_init_dbell_res - Initialize a RIO doorbell resource
// @res: resource struct
// @start: start of doorbell range
// @end: end of doorbell range
//
// This function is used to initialize the fields of a resource
// for use as a doorbell resource.  It initializes a range of
// doorbell messages using the start and end arguments.
//
// RIO_DEVICE - macro used to describe a specific RIO device
// @dev: the 16 bit RIO device ID
// @ven: the 16 bit RIO vendor ID
//
// This macro is used to create a struct rio_device_id that matches a
// specific device.  The assembly vendor and assembly device fields
// will be set to %RIO_ANY_ID.
//

// Mailbox management
extern "C" {
    pub fn rio_release_outb_mbox(: *mut rio_mport, _arg: c_int) -> c_int;
}
//
// rio_add_outb_message - Add RIO message to an outbound mailbox queue
// @mport: RIO master port containing the outbound queue
// @rdev: RIO device the message is be sent to
// @mbox: The outbound mailbox queue
// @buffer: Pointer to the message buffer
// @len: Length of the message buffer
//
// Adds a RIO message buffer to an outbound mailbox queue for
// transmission. Returns 0 on success.
//
extern "C" {
    pub fn rio_release_inb_mbox(: *mut rio_mport, _arg: c_int) -> c_int;
}
//
// rio_add_inb_buffer - Add buffer to an inbound mailbox queue
// @mport: Master port containing the inbound mailbox
// @mbox: The inbound mailbox number
// @buffer: Pointer to the message buffer
//
// Adds a buffer to an inbound mailbox queue for reception. Returns
// 0 on success.
//
// rio_get_inb_message - Get A RIO message from an inbound mailbox queue
// @mport: Master port containing the inbound mailbox
// @mbox: The inbound mailbox number
//
// Get a RIO message from an inbound mailbox queue. Returns 0 on success.
//
// Doorbell management
extern "C" {
    pub fn rio_release_inb_dbell(: *mut rio_mport, _arg: u16, _arg: u16) -> c_int;
}
extern "C" {
    pub fn rio_release_outb_dbell(: *mut rio_dev, : *mut resource) -> c_int;
}
// Memory region management
extern "C" {
    pub fn rio_claim_resource(: *mut rio_dev, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn rio_request_regions(: *mut rio_dev, : *mut c_char) -> c_int;
}
extern "C" {
    pub fn rio_release_regions(: *mut rio_dev);
}
extern "C" {
    pub fn rio_request_region(: *mut rio_dev, _arg: c_int, : *mut c_char) -> c_int;
}
extern "C" {
    pub fn rio_release_region(: *mut rio_dev, _arg: c_int);
}
// Memory mapping functions
extern "C" {
    pub fn rio_unmap_inb_region(mport: *mut rio_mport, lstart: dma_addr_t);
}
// Port-Write management
extern "C" {
    pub fn rio_release_inb_pwrite(: *mut rio_dev) -> c_int;
}
extern "C" {
    pub fn rio_pw_enable(mport: *mut rio_mport, enable: c_int);
}
// LDM support
extern "C" {
    pub fn rio_register_driver(: *mut rio_driver) -> c_int;
}
extern "C" {
    pub fn rio_unregister_driver(: *mut rio_driver);
}
extern "C" {
    pub fn rio_dev_put(: *mut rio_dev);
}

extern "C" {
    pub fn rio_release_dma(dchan: *mut dma_chan);
}

//
// rio_name - Get the unique RIO device identifier
// @rdev: RIO device
//
// Get the unique RIO device identifier. Returns the device
// identifier string.
//
extern "C" {
    pub fn dev_name(_arg: &rdev->dev) -> return;
}
//
// rio_get_drvdata - Get RIO driver specific data
// @rdev: RIO device
//
// Get RIO driver specific data. Returns a pointer to the
// driver specific data.
//
extern "C" {
    pub fn dev_get_drvdata(_arg: &rdev->dev) -> return;
}
//
// rio_set_drvdata - Set RIO driver specific data
// @rdev: RIO device
// @data: Pointer to driver specific data
//
// Set RIO driver specific data. device struct driver data pointer
// is set to the @data argument.
//
// Misc driver helpers
extern "C" {
    pub fn rio_local_get_device_id(port: *mut rio_mport) -> u16;
}
extern "C" {
    pub fn rio_local_set_device_id(port: *mut rio_mport, did: u16);
}
extern "C" {
    pub fn rio_init_mports() -> c_int;
}
