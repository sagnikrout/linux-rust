//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/b2c2/flexcop-usb.h
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
// Linux driver for digital TV devices equipped with B2C2 FlexcopII(b)/III
// flexcop-usb.h - header file for the USB part
// see flexcop.c for copyright information
//

// transfer parameters
pub const B2C2_USB_FRAMES_PER_ISO: c_int = 4;
pub const B2C2_USB_NUM_ISO_URB: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flexcop_usb {
    pub udev: *mut usb_device,
    pub uintf: *mut usb_interface,
    pub iso_buffer: *mut u8,
    pub buffer_size: c_int,
    pub dma_addr: dma_addr_t,
    pub iso_urb: [*mut urb; B2C2_USB_NUM_ISO_URB],
    pub fc_dev: *mut flexcop_device,
    pub tmp_buffer: [u8; 1023+190],
    pub tmp_buffer_length: c_int,
// for URB control messages
    pub data: [u8; 80],
    pub data_mutex: mutex,
}

// request types TODO What is its use?

// request
// function definition for I2C_REQUEST
// DKT 020208 - add this to support special case of DiSEqC
// function definition for UTILITY request 0x12
// DKT 020304 - new utility function
// DKT 020326 - add function for v1.14
pub const B2C2_WAIT_FOR_OPERATION_RW: c_int = 1000;
pub const B2C2_WAIT_FOR_OPERATION_RDW: c_int = 3000;
pub const B2C2_WAIT_FOR_OPERATION_WDW: c_int = 1000;
pub const B2C2_WAIT_FOR_OPERATION_V8READ: c_int = 3000;
pub const B2C2_WAIT_FOR_OPERATION_V8WRITE: c_int = 3000;
pub const B2C2_WAIT_FOR_OPERATION_V8FLASH: c_int = 3000;

pub const USB_MEM_READ_MAX: c_int = 32;
pub const USB_MEM_WRITE_MAX: c_int = 1;
pub const USB_FLASH_MAX: c_int = 8;
pub const V8_MEMORY_PAGE_SIZE: c_uint = 0x8000 /* 32K */;
pub const V8_MEMORY_PAGE_MASK: c_uint = 0x7FFF;
