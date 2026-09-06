//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/maple.h
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

// Maple Bus command and response codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum maple_code {
    MAPLE_RESPONSE_FILEERR =	-5,
    MAPLE_RESPONSE_AGAIN,	/* retransmit */
    MAPLE_RESPONSE_BADCMD,
    MAPLE_RESPONSE_BADFUNC,
    MAPLE_RESPONSE_NONE,	/* unit didn't respond*/
    MAPLE_COMMAND_DEVINFO =		1,
    MAPLE_COMMAND_ALLINFO,
    MAPLE_COMMAND_RESET,
    MAPLE_COMMAND_KILL,
    MAPLE_RESPONSE_DEVINFO,
    MAPLE_RESPONSE_ALLINFO,
    MAPLE_RESPONSE_OK,
    MAPLE_RESPONSE_DATATRF,
    MAPLE_COMMAND_GETCOND,
    MAPLE_COMMAND_GETMINFO,
    MAPLE_COMMAND_BREAD,
    MAPLE_COMMAND_BWRITE,
    MAPLE_COMMAND_BSYNC,
    MAPLE_COMMAND_SETCOND,
    MAPLE_COMMAND_MICCONTROL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum maple_file_errors {
    MAPLE_FILEERR_INVALID_PARTITION =	0x01000000,
    MAPLE_FILEERR_PHASE_ERROR =		0x02000000,
    MAPLE_FILEERR_INVALID_BLOCK =		0x04000000,
    MAPLE_FILEERR_WRITE_ERROR =		0x08000000,
    MAPLE_FILEERR_INVALID_WRITE_LENGTH =	0x10000000,
    MAPLE_FILEERR_BAD_CRC = 		0x20000000
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maple_buffer {
    pub bufx: [c_char; 0x400],
    pub buf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mapleq {
    pub list: list_head,
    pub dev: *mut maple_device,
    pub recvbuf: *mut maple_buffer,
    pub recvbuf_p2: *mut *mut void sendbuf,,
    pub length: c_uchar,
    pub command: maple_code,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maple_devinfo {
    pub function: c_ulong,
    pub function_data: [c_ulong; 3],
    pub area_code: c_uchar,
    pub connector_direction: c_uchar,
    pub product_name: [c_char; 31],
    pub product_licence: [c_char; 61],
    pub standby_power: c_ushort,
    pub max_power: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maple_device {
    pub driver: *mut maple_driver,
    pub mq: *mut mapleq,
    pub mq): *mut *mut *mut void (callback) (struct mapleq,
    pub recvbuf): *mut *mut *mut void (fileerr_handler)(struct maple_device mdev, void,
    pub mdev): *mut *mut int (can_unload)(struct maple_device,
    pub function: unsigned long when, interval,,
    pub devinfo: maple_devinfo,
    pub unit: unsigned char port,,
    pub product_name: [c_char; 32],
    pub product_licence: [c_char; 64],
    pub busy: core::sync::atomic::AtomicI32,
    pub maple_wait: wait_queue_head_t,
    pub dev: device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maple_driver {
    pub function: c_ulong,
    pub drv: device_driver,
}

extern "C" {
    pub fn maple_driver_register(: *mut maple_driver) -> c_int;
}
extern "C" {
    pub fn maple_driver_unregister(: *mut maple_driver);
}
extern "C" {
    pub fn maple_clear_dev(mdev: *mut maple_device);
}

