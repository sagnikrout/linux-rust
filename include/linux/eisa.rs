//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/eisa.h
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

pub const EISA_MAX_SLOTS: c_int = 8;
pub const EISA_MAX_RESOURCES: c_int = 4;
// A few EISA constants/offsets...
pub const EISA_DMA1_STATUS: c_int = 8;
pub const EISA_INT1_CTRL: c_uint = 0x20;
pub const EISA_INT1_MASK: c_uint = 0x21;
pub const EISA_INT2_CTRL: c_uint = 0xA0;
pub const EISA_INT2_MASK: c_uint = 0xA1;
pub const EISA_DMA2_STATUS: c_uint = 0xD0;
pub const EISA_DMA2_WRITE_SINGLE: c_uint = 0xD4;
pub const EISA_EXT_NMI_RESET_CTRL: c_uint = 0x461;
pub const EISA_INT1_EDGE_LEVEL: c_uint = 0x4D0;
pub const EISA_INT2_EDGE_LEVEL: c_uint = 0x4D1;
pub const EISA_VENDOR_ID_OFFSET: c_uint = 0xC80;
pub const EISA_CONFIG_OFFSET: c_uint = 0xC84;
pub const EISA_CONFIG_ENABLED: c_int = 1;
pub const EISA_CONFIG_FORCED: c_int = 2;
// Chosen to hold the longest string in eisa.ids.
pub const EISA_DEVICE_INFO_NAME_SIZE: c_int = 74;
// There is not much we can say about an EISA device, apart from
// signature, slot number, and base address. dma_mask is set by
// default to parent device mask..
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eisa_device {
    pub id: eisa_device_id,
    pub slot: c_int,
    pub state: c_int,
    pub base_addr: c_ulong,
    pub res: [resource; EISA_MAX_RESOURCES],
    pub dma_mask: u64,
    pub /: *mut *mut device dev; / generic device,
    pub pretty_name: [c_char; EISA_DEVICE_INFO_NAME_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eisa_driver {
    pub id_table: *const eisa_device_id,
    pub driver: device_driver,
}

// These external functions are only available when EISA support is enabled.

extern "C" {
    pub fn eisa_driver_register(edrv: *mut eisa_driver) -> c_int;
}
extern "C" {
    pub fn eisa_driver_unregister(edrv: *mut eisa_driver);
}

// Mimics pci.h...
extern "C" {
    pub fn dev_get_drvdata(_arg: &edev->dev) -> return;
}
// The EISA root device. There's rumours about machines with multiple
// busses (PA-RISC ?), so we try to handle that.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eisa_root_device {
    pub /: *mut *mut *mut device dev; / Pointer to bridge device,
    pub res: *mut resource,
    pub bus_base_addr: c_ulong,
    pub /: *mut *mut int slots; / Max slot number,
    pub /: *mut *mut int force_probe; / Probe even when no slot 0,
    pub /: *mut *mut u64 dma_mask; / from bridge device,
    pub /: *mut *mut int bus_nr; / Set by eisa_root_register,
    pub /: *mut *mut resource eisa_root_res; / ditto,
}

extern "C" {
    pub fn eisa_root_register(root: *mut eisa_root_device) -> c_int;
}

