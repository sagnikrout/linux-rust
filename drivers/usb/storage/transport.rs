//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/storage/transport.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Driver for USB Mass Storage compliant devices
// Transport Functions Header File
//
// Current development and maintenance by:
// (c) 1999, 2000 Matthew Dharm (mdharm-usb@one-eyed-alien.net)
//
// This driver is based on the 'USB Mass Storage Class' document. This
// describes in detail the protocol used to communicate with such
// devices.  Clearly, the designers had SCSI and ATAPI commands in
// mind when they created this document.  The commands are all very
// similar to commands in the SCSI-II and ATAPI specifications.
//
// It is important to note that in a number of cases this class
// exhibits class-specific exemptions from the USB specification.
// Notably the usage of NAK, STALL and ACK differs from the norm, in
// that they are used to communicate wait, failed and OK on commands.
//
// Also, for certain devices, the interrupt endpoint is used to convey
// status of a command.
//

//
// usb_stor_bulk_transfer_xxx() return codes, in order of severity
//

//
// Transport return codes
//

//
// We used to have USB_STOR_XFER_ABORTED and USB_STOR_TRANSPORT_ABORTED
// return codes.  But now the transport and low-level transfer routines
// treat an abort as just another error (-ENOENT for a cancelled URB).
// It is up to the invoke_transport() function to test for aborts and
// distinguish them from genuine communication errors.
//
// CBI accept device specific command
//
pub const US_CBI_ADSC: c_int = 0;
extern "C" {
    pub fn usb_stor_CB_transport(: *mut scsi_cmnd, us_data*: *mut struct) -> c_int;
}
extern "C" {
    pub fn usb_stor_CB_reset(us_data*: *mut struct) -> c_int;
}
extern "C" {
    pub fn usb_stor_Bulk_transport(: *mut scsi_cmnd, us_data*: *mut struct) -> c_int;
}
extern "C" {
    pub fn usb_stor_Bulk_max_lun(us_data*: *mut struct) -> c_int;
}
extern "C" {
    pub fn usb_stor_Bulk_reset(us_data*: *mut struct) -> c_int;
}
extern "C" {
    pub fn usb_stor_invoke_transport(: *mut scsi_cmnd, us_data*: *mut struct);
}
extern "C" {
    pub fn usb_stor_stop_transport(us_data*: *mut struct);
}
extern "C" {
    pub fn usb_stor_clear_halt(us: *mut us_data, pipe: c_uint) -> c_int;
}
extern "C" {
    pub fn usb_stor_port_reset(us: *mut us_data) -> c_int;
}
