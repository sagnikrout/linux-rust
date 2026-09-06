//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/ni_usb/ni_usb_gpib.h
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
// copyright            : (C) 2004 by Frank Mori Hess
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ni_usb_device {
    NIUSB_SUBDEV_TNT4882 = 1,
    NIUSB_SUBDEV_UNKNOWN2 = 2,
    NIUSB_SUBDEV_UNKNOWN3 = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum endpoint_addresses {
    NIUSB_B_BULK_OUT_ENDPOINT = 0x2,
    NIUSB_B_BULK_IN_ENDPOINT = 0x2,
    NIUSB_B_BULK_IN_ALT_ENDPOINT = 0x6,
    NIUSB_B_INTERRUPT_IN_ENDPOINT = 0x4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hs_enpoint_addresses {
    NIUSB_HS_BULK_OUT_ENDPOINT = 0x2,
    NIUSB_HS_BULK_OUT_ALT_ENDPOINT = 0x6,
    NIUSB_HS_BULK_IN_ENDPOINT = 0x4,
    NIUSB_HS_BULK_IN_ALT_ENDPOINT = 0x8,
    NIUSB_HS_INTERRUPT_IN_ENDPOINT = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hs_plus_endpoint_addresses {
    NIUSB_HS_PLUS_BULK_OUT_ENDPOINT = 0x1,
    NIUSB_HS_PLUS_BULK_OUT_ALT_ENDPOINT = 0x4,
    NIUSB_HS_PLUS_BULK_IN_ENDPOINT = 0x2,
    NIUSB_HS_PLUS_BULK_IN_ALT_ENDPOINT = 0x5,
    NIUSB_HS_PLUS_INTERRUPT_IN_ENDPOINT = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_usb_urb_ctx {
    pub complete: completion,
    pub 1: unsigned timed_out :,
}

// struct which defines private_data for ni_usb devices
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_usb_priv {
    pub bus_interface: *mut usb_interface,
    pub bulk_out_endpoint: c_int,
    pub bulk_in_endpoint: c_int,
    pub interrupt_in_endpoint: c_int,
    pub eos_char: u8,
    pub eos_mode: c_ushort,
    pub monitored_ibsta_bits: c_uint,
    pub bulk_urb: *mut urb,
    pub interrupt_urb: *mut urb,
    pub interrupt_buffer: [u8; 0x11],
    pub lock: mutex addressed_transfer_lock; // protect transfer,
    pub sends: mutex bulk_transfer_lock; // protect bulk message,
    pub messages: mutex control_transfer_lock; // protect control,
    pub messages: mutex interrupt_transfer_lock; // protect interrupt,
    pub bulk_timer: timer_list,
    pub context: ni_usb_urb_ctx,
    pub product_id: c_int,
    pub ren_state: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_usb_status_block {
    pub id: c_short,
    pub ibsta: c_ushort,
    pub error_code: c_short,
    pub count: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_usb_register {
    pub device: ni_usb_device,
    pub address: c_short,
    pub value: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ni_usb_bulk_ids {
    NIUSB_IBCAC_ID = 0x1,
    NIUSB_UNKNOWN3_ID = 0x3, // device level function id?
    NIUSB_TERM_ID = 0x4,
    NIUSB_IBGTS_ID = 0x6,
    NIUSB_IBRPP_ID = 0x7,
    NIUSB_REG_READ_ID = 0x8,
    NIUSB_REG_WRITE_ID = 0x9,
    NIUSB_IBSIC_ID = 0xf,
    NIUSB_REGISTER_READ_DATA_START_ID = 0x34,
    NIUSB_REGISTER_READ_DATA_END_ID = 0x35,
    NIUSB_IBRD_DATA_ID = 0x36,
    NIUSB_IBRD_EXTENDED_DATA_ID = 0x37,
    NIUSB_IBRD_STATUS_ID = 0x38
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ni_usb_error_codes {
    NIUSB_NO_ERROR = 0,
//
// NIUSB_ABORTED_ERROR occurs when I/O is interrupted early by
// doing a NI_USB_STOP_REQUEST on the control endpoint.
//
    NIUSB_ABORTED_ERROR = 1,
//
// NIUSB_READ_ATN_ERROR occurs when you do a board read while
// ATN is set
//
    NIUSB_ATN_STATE_ERROR = 2,
//
// NIUSB_ADDRESSING_ERROR occurs when you do a board
// read/write as CIC but are not in LACS/TACS
//
    NIUSB_ADDRESSING_ERROR = 3,
//
// NIUSB_EOSMODE_ERROR occurs on reads if any eos mode or char
// bits are set when REOS is not set.
// Have also seen error 4 if you try to send more than 16
// command bytes at once on a usb-b.
//
    NIUSB_EOSMODE_ERROR = 4,
//
// NIUSB_NO_BUS_ERROR occurs when you try to write a command
// byte but there are no devices connected to the gpib bus
//
    NIUSB_NO_BUS_ERROR = 5,
//
// NIUSB_NO_LISTENER_ERROR occurs when you do a board write as
// CIC with no listener
//
    NIUSB_NO_LISTENER_ERROR = 8,
// get NIUSB_TIMEOUT_ERROR on board read/write timeout
    NIUSB_TIMEOUT_ERROR = 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ni_usb_control_requests {
    NI_USB_STOP_REQUEST = 0x20,
    NI_USB_WAIT_REQUEST = 0x21,
    NI_USB_POLL_READY_REQUEST = 0x40,
    NI_USB_SERIAL_NUMBER_REQUEST = 0x41,
    NI_USB_HS_PLUS_0x48_REQUEST = 0x48,
    NI_USB_HS_PLUS_LED_REQUEST = 0x4b,
    NI_USB_HS_PLUS_0xf8_REQUEST = 0xf8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ni_usb_unknown3_register {
    SERIAL_NUMBER_4_REG = 0x8,
    SERIAL_NUMBER_3_REG = 0x9,
    SERIAL_NUMBER_2_REG = 0xa,
    SERIAL_NUMBER_1_REG = 0xb,
}
