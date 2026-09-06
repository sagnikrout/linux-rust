//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/storage/usb.h
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
// Main Header File
//
// Current development and maintenance by:
// (c) 1999-2002 Matthew Dharm (mdharm-usb@one-eyed-alien.net)
//
// Initial work by:
// (c) 1999 Michael Gee (michael@linuxspecific.com)
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
// Unusual device list definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct us_unusual_dev {
    pub vendorName: *const *const c_char,
    pub productName: *const *const c_char,
    pub useProtocol: __u8,
    pub useTransport: __u8,
    pub ): *mut *mut int (initFunction)(struct us_data,
}

// Dynamic bitflag definitions (us->dflags): used in set_bit() etc.

pub const USB_STOR_STRING_LEN: c_int = 32;
//
// We provide a DMA-mapped I/O buffer for use with small USB transfers.
// It turns out that CB[I] needs a 12-byte buffer and Bulk-only needs a
// 31-byte buffer.  But Freecom needs a 64-byte buffer, so that's the
// size we'll allocate.
//

extern "C" {
    pub fn int(: *mut *mut trans_cmnd)(struct scsi_cmnd, us_data*: *mut struct) -> typedef;
}
extern "C" {
    pub fn int(us_data*: *mut *mut trans_reset)(struct) -> typedef;
}
extern "C" {
    pub fn void(scsi_cmnd*: *mut *mut proto_cmnd)(struct, us_data*: *mut struct) -> typedef;
}
pub const US_SUSPEND: c_int = 0;
pub const US_RESUME: c_int = 1;
// we allocate one of these for every device that we remember
#[repr(C)]
#[derive(Copy, Clone)]
pub struct us_data {
//
// The device we're working with
// It's important to note:
// (o) you must hold dev_mutex to change pusb_dev
//
    pub /: *mut *mut mutex dev_mutex; / protect pusb_dev,
    pub /: *mut *mut *mut usb_device pusb_dev; / this usb_device,
    pub /: *mut *mut *mut usb_interface pusb_intf; / this interface,
    pub unusual_dev: *const us_unusual_dev,
// device-filter entry
    pub /: *mut *mut u64 fflags; / fixed flags from filter,
    pub /: *mut *mut unsigned long dflags; / dynamic atomic bitflags,
    pub /: *mut *mut unsigned int send_bulk_pipe; / cached pipe values,
    pub recv_bulk_pipe: c_uint,
    pub send_ctrl_pipe: c_uint,
    pub recv_ctrl_pipe: c_uint,
    pub recv_intr_pipe: c_uint,
// information about the device
    pub transport_name: *mut c_char,
    pub protocol_name: *mut c_char,
    pub bcs_signature: __le32,
    pub subclass: u8,
    pub protocol: u8,
    pub max_lun: u8,
    pub /: *mut *mut u8 ifnum; / interface number,
    pub /: *mut *mut u8 ep_bInterval; / interrupt interval,
// function pointers for this device
    pub /: *mut *mut trans_cmnd transport; / transport function,
    pub /: *mut *mut trans_reset transport_reset; / transport device reset,
    pub /: *mut *mut proto_cmnd proto_handler; / protocol handler,
// SCSI interfaces
    pub /: *mut *mut *mut scsi_cmnd srb; / current srb,
    pub /: *mut *mut unsigned int tag; / current dCBWTag,
    pub /: *mut *mut char scsi_name[32]; / scsi_host name,
// control and bulk communications data
    pub /: *mut *mut *mut urb current_urb; / USB requests,
    pub /: *mut *mut *mut usb_ctrlrequest cr; / control requests,
    pub /: *mut *mut usb_sg_request current_sg; / scatter-gather req.,
    pub /: *mut *mut *mut unsigned char iobuf; / I/O buffer,
    pub /: *mut *mut dma_addr_t iobuf_dma; / buffer DMA addresses,
    pub /: *mut *mut *mut task_ctl_thread; / the control thread,
// mutual exclusion and synchronization structures
    pub /: *mut *mut completion cmnd_ready; / to sleep thread on,
    pub /: *mut *mut completion notify; / thread begin/end,
    pub /: *mut *mut wait_queue_head_t delay_wait; / wait during reset,
    pub /: *mut *mut delayed_work scan_dwork; / for async scanning,
// subdriver information
    pub /: *mut *mut *mut void extra; / Any extra data,
    pub /: *mut *mut extra_data_destructor extra_destructor;/ extra data destructor,

    pub suspend_resume_hook: pm_hook,

// hacks for READ CAPACITY bug handling
    pub use_last_sector_hacks: c_int,
    pub last_sector_retries: c_int,
}

// Convert between us_data and the corresponding Scsi_Host
extern "C" {
    pub fn container_of(us: *mut *mut (void ), Scsi_Host: struct, _arg: hostdata) -> return;
}
// Function to fill an inquiry response. See usb.c for details
//
// The scsi_lock() and scsi_unlock() macros protect the sm_state and the
// single queue element srb for write access
//

// General routines provided by the usb-storage standard core

extern "C" {
    pub fn usb_stor_suspend(iface: *mut usb_interface, message: pm_message_t) -> c_int;
}
extern "C" {
    pub fn usb_stor_resume(iface: *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn usb_stor_reset_resume(iface: *mut usb_interface) -> c_int;
}

extern "C" {
    pub fn usb_stor_pre_reset(iface: *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn usb_stor_post_reset(iface: *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn usb_stor_probe2(us: *mut us_data) -> c_int;
}
extern "C" {
    pub fn usb_stor_disconnect(intf: *mut usb_interface);
}

