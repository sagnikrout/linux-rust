//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/usbip.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// usbip.h
//
// USBIP uapi defines and function prototypes etc.
//
// usbip device status - exported in usbip device sysfs status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usbip_device_status {
// sdev is available.
    SDEV_ST_AVAILABLE = 0x01,
// sdev is now used.
    SDEV_ST_USED,
// sdev is unusable because of a fatal error.
    SDEV_ST_ERROR,

// vdev does not connect a remote device.
    VDEV_ST_NULL,
// vdev is used, but the USB address is not assigned yet
    VDEV_ST_NOTASSIGNED,
    VDEV_ST_USED,
    VDEV_ST_ERROR
}

// USB URB Transfer flags:
//
// USBIP server and client (vchi) pack URBs in TCP packets. The following
// are the transfer type defines used in USBIP protocol.
//
pub const USBIP_URB_SHORT_NOT_OK: c_uint = 0x0001;
pub const USBIP_URB_ISO_ASAP: c_uint = 0x0002;
pub const USBIP_URB_NO_TRANSFER_DMA_MAP: c_uint = 0x0004;
pub const USBIP_URB_ZERO_PACKET: c_uint = 0x0040;
pub const USBIP_URB_NO_INTERRUPT: c_uint = 0x0080;
pub const USBIP_URB_FREE_BUFFER: c_uint = 0x0100;
pub const USBIP_URB_DIR_IN: c_uint = 0x0200;
pub const USBIP_URB_DIR_OUT: c_int = 0;

pub const USBIP_URB_DMA_MAP_SINGLE: c_uint = 0x00010000;
pub const USBIP_URB_DMA_MAP_PAGE: c_uint = 0x00020000;
pub const USBIP_URB_DMA_MAP_SG: c_uint = 0x00040000;
pub const USBIP_URB_MAP_LOCAL: c_uint = 0x00080000;
pub const USBIP_URB_SETUP_MAP_SINGLE: c_uint = 0x00100000;
pub const USBIP_URB_SETUP_MAP_LOCAL: c_uint = 0x00200000;
pub const USBIP_URB_DMA_SG_COMBINED: c_uint = 0x00400000;
pub const USBIP_URB_ALIGNED_TEMP_BUFFER: c_uint = 0x00800000;
