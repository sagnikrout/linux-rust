//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ralink/rt2x00/rt2x00usb.h
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

//
// For USB vendor requests we need to pass a timeout time in ms, for this we
// use the REGISTER_TIMEOUT, however when loading firmware or read EEPROM
// a higher value is required. In that case we use the REGISTER_TIMEOUT_FIRMWARE
// and EEPROM_TIMEOUT.
//
pub const REGISTER_TIMEOUT: c_int = 100;
pub const REGISTER_TIMEOUT_FIRMWARE: c_int = 1000;
pub const EEPROM_TIMEOUT: c_int = 2000;
//
// Cache size
//
pub const CSR_CACHE_SIZE: c_int = 64;
//
// USB request types.
//

//
// enum rt2x00usb_vendor_request: USB vendor commands.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt2x00usb_vendor_request {
    USB_DEVICE_MODE = 1,
    USB_SINGLE_WRITE = 2,
    USB_SINGLE_READ = 3,
    USB_MULTI_WRITE = 6,
    USB_MULTI_READ = 7,
    USB_EEPROM_WRITE = 8,
    USB_EEPROM_READ = 9,
    USB_LED_CONTROL = 10, /* RT73USB */
    USB_RX_CONTROL = 12,
}

//
// enum rt2x00usb_mode_offset: Device modes offset.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt2x00usb_mode_offset {
    USB_MODE_RESET = 1,
    USB_MODE_UNPLUG = 2,
    USB_MODE_FUNCTION = 3,
    USB_MODE_TEST = 4,
    USB_MODE_SLEEP = 7,	/* RT73USB */
    USB_MODE_FIRMWARE = 8,	/* RT73USB */
    USB_MODE_WAKEUP = 9,	/* RT73USB */
    USB_MODE_AUTORUN = 17, /* RT2800USB */
}

//
// rt2x00usb_vendor_request - Send register command to device
// @rt2x00dev: Pointer to &struct rt2x00_dev
// @request: USB vendor command (See &enum rt2x00usb_vendor_request)
// @requesttype: Request type &USB_VENDOR_REQUEST_
// @offset: Register offset to perform action on
// @value: Value to write to device
// @buffer: Buffer where information will be read/written to by device
// @buffer_length: Size of &buffer
// @timeout: Operation timeout
//
// This is the main function to communicate with the device,
// the &buffer argument _must_ either be NULL or point to
// a buffer allocated by kmalloc. Failure to do so can lead
// to unexpected behavior depending on the architecture.
//
// rt2x00usb_vendor_request_buff - Send register command to device (buffered)
// @rt2x00dev: Pointer to &struct rt2x00_dev
// @request: USB vendor command (See &enum rt2x00usb_vendor_request)
// @requesttype: Request type &USB_VENDOR_REQUEST_
// @offset: Register offset to perform action on
// @buffer: Buffer where information will be read/written to by device
// @buffer_length: Size of &buffer
//
// This function will use a previously with kmalloc allocated cache
// to communicate with the device. The contents of the buffer pointer
// will be copied to this cache when writing, or read from the cache
// when reading.
// Buffers send to &rt2x00usb_vendor_request _must_ be allocated with
// kmalloc. Hence the reason for using a previously allocated cache
// which has been allocated properly.
//
// rt2x00usb_vendor_request_buff - Send register command to device (buffered)
// @rt2x00dev: Pointer to &struct rt2x00_dev
// @request: USB vendor command (See &enum rt2x00usb_vendor_request)
// @requesttype: Request type &USB_VENDOR_REQUEST_
// @offset: Register offset to perform action on
// @buffer: Buffer where information will be read/written to by device
// @buffer_length: Size of &buffer
// @timeout: Operation timeout
//
// A version of &rt2x00usb_vendor_request_buff which must be called
// if the usb_cache_mutex is already held.
//
// rt2x00usb_vendor_request_sw - Send single register command to device
// @rt2x00dev: Pointer to &struct rt2x00_dev
// @request: USB vendor command (See &enum rt2x00usb_vendor_request)
// @offset: Register offset to perform action on
// @value: Value to write to device
// @timeout: Operation timeout
//
// Simple wrapper around rt2x00usb_vendor_request to write a single
// command to the device. Since we don't use the buffer argument we
// don't have to worry about kmalloc here.
//
// rt2x00usb_eeprom_read - Read eeprom from device
// @rt2x00dev: Pointer to &struct rt2x00_dev
// @eeprom: Pointer to eeprom array to store the information in
// @length: Number of bytes to read from the eeprom
//
// Simple wrapper around rt2x00usb_vendor_request to read the eeprom
// from the device. Note that the eeprom argument _must_ be allocated using
// kmalloc for correct handling inside the kernel USB layer.
//
// rt2x00usb_register_read - Read 32bit register word
// @rt2x00dev: Device pointer, see &struct rt2x00_dev.
// @offset: Register offset
//
// This function is a simple wrapper for 32bit register access
// through rt2x00usb_vendor_request_buff().
//
extern "C" {
    pub fn le32_to_cpu(_arg: reg) -> return;
}
//
// rt2x00usb_register_read_lock - Read 32bit register word
// @rt2x00dev: Device pointer, see &struct rt2x00_dev.
// @offset: Register offset
//
// This function is a simple wrapper for 32bit register access
// through rt2x00usb_vendor_req_buff_lock().
//
extern "C" {
    pub fn le32_to_cpu(_arg: reg) -> return;
}
//
// rt2x00usb_register_multiread - Read 32bit register words
// @rt2x00dev: Device pointer, see &struct rt2x00_dev.
// @offset: Register offset
// @value: Pointer to where register contents should be stored
// @length: Length of the data
//
// This function is a simple wrapper for 32bit register access
// through rt2x00usb_vendor_request_buff().
//
// rt2x00usb_register_write - Write 32bit register word
// @rt2x00dev: Device pointer, see &struct rt2x00_dev.
// @offset: Register offset
// @value: Data which should be written
//
// This function is a simple wrapper for 32bit register access
// through rt2x00usb_vendor_request_buff().
//
// rt2x00usb_register_write_lock - Write 32bit register word
// @rt2x00dev: Device pointer, see &struct rt2x00_dev.
// @offset: Register offset
// @value: Data which should be written
//
// This function is a simple wrapper for 32bit register access
// through rt2x00usb_vendor_req_buff_lock().
//
// rt2x00usb_register_multiwrite - Write 32bit register words
// @rt2x00dev: Device pointer, see &struct rt2x00_dev.
// @offset: Register offset
// @value: Data which should be written
// @length: Length of the data
//
// This function is a simple wrapper for 32bit register access
// through rt2x00usb_vendor_request_buff().
//
// rt2x00usb_regbusy_read - Read from register with busy check
// @rt2x00dev: Device pointer, see &struct rt2x00_dev.
// @offset: Register offset
// @field: Field to check if register is busy
// @reg: Pointer to where register contents should be stored
//
// This function will read the given register, and checks if the
// register is busy. If it is, it will sleep for a couple of
// microseconds before reading the register again. If the register
// is not read after a certain timeout, this function will return
// FALSE.
//
// rt2x00usb_register_read_async - Asynchronously read 32bit register word
// @rt2x00dev: Device pointer, see &struct rt2x00_dev.
// @offset: Register offset
// @callback: Functon to call when read completes.
//
// Submit a control URB to read a 32bit register. This safe to
// be called from atomic context.  The callback will be called
// when the URB completes. Otherwise the function is similar
// to rt2x00usb_register_read().
// When the callback function returns false, the memory will be cleaned up,
// when it returns true, the urb will be fired again.
//
// Radio handlers
//
extern "C" {
    pub fn rt2x00usb_disable_radio(rt2x00dev: *mut rt2x00_dev);
}
//
// struct queue_entry_priv_usb: Per entry USB specific information
//
// @urb: Urb structure used for device communication.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct queue_entry_priv_usb {
    pub urb: *mut urb,
}

//
// struct queue_entry_priv_usb_bcn: Per TX entry USB specific information
//
// The first section should match &struct queue_entry_priv_usb exactly.
// rt2500usb can use this structure to send a guardian byte when working
// with beacons.
//
// @urb: Urb structure used for device communication.
// @guardian_data: Set to 0, used for sending the guardian data.
// @guardian_urb: Urb structure used to send the guardian data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct queue_entry_priv_usb_bcn {
    pub urb: *mut urb,
    pub guardian_data: c_uint,
    pub guardian_urb: *mut urb,
}

//
// rt2x00usb_kick_queue - Kick data queue
// @queue: Data queue to kick
//
// This will walk through all entries of the queue and push all pending
// frames to the hardware as a single burst.
//
extern "C" {
    pub fn rt2x00usb_kick_queue(queue: *mut data_queue);
}
//
// rt2x00usb_flush_queue - Flush data queue
// @queue: Data queue to stop
// @drop: True to drop all pending frames.
//
// This will walk through all entries of the queue and will optionally
// kill all URB's which were send to the device, or at least wait until
// they have been returned from the device..
//
extern "C" {
    pub fn rt2x00usb_flush_queue(queue: *mut data_queue, drop: bool);
}
//
// rt2x00usb_watchdog - Watchdog for USB communication
// @rt2x00dev: Pointer to &struct rt2x00_dev
//
// Check the health of the USB communication and determine
// if timeouts have occurred. If this is the case, this function
// will reset all communication to restore functionality again.
//
extern "C" {
    pub fn rt2x00usb_watchdog(rt2x00dev: *mut rt2x00_dev);
}
//
// Device initialization handlers.
//
extern "C" {
    pub fn rt2x00usb_clear_entry(entry: *mut queue_entry);
}
extern "C" {
    pub fn rt2x00usb_initialize(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2x00usb_uninitialize(rt2x00dev: *mut rt2x00_dev);
}
//
// USB driver handlers.
//
extern "C" {
    pub fn rt2x00usb_disconnect(usb_intf: *mut usb_interface);
}

extern "C" {
    pub fn rt2x00usb_suspend(usb_intf: *mut usb_interface, state: pm_message_t) -> c_int;
}
extern "C" {
    pub fn rt2x00usb_resume(usb_intf: *mut usb_interface) -> c_int;
}

