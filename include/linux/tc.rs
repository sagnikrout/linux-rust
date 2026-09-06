//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tc.h
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


//
// Interface to the TURBOchannel related routines.
//
// Copyright (c) 1998  Harald Koerfgen
// Copyright (c) 2005  James Simmons
// Copyright (c) 2006  Maciej W. Rozycki
//
// Based on:
//
// "TURBOchannel Firmware Specification", EK-TCAAD-FS-004
//
// from Digital Equipment Corporation.
//
// This file is subject to the terms and conditions of the GNU
// General Public License.  See the file "COPYING" in the main
// directory of this archive for more details.
//

//
// Offsets for the ROM header locations for TURBOchannel cards.
//
pub const TC_OLDCARD: c_uint = 0x3c0000;
pub const TC_NEWCARD: c_uint = 0x000000;
pub const TC_ROM_WIDTH: c_uint = 0x3e0;
pub const TC_ROM_STRIDE: c_uint = 0x3e4;
pub const TC_ROM_SIZE: c_uint = 0x3e8;
pub const TC_SLOT_SIZE: c_uint = 0x3ec;
pub const TC_PATTERN0: c_uint = 0x3f0;
pub const TC_PATTERN1: c_uint = 0x3f4;
pub const TC_PATTERN2: c_uint = 0x3f8;
pub const TC_PATTERN3: c_uint = 0x3fc;
pub const TC_FIRM_VER: c_uint = 0x400;
pub const TC_VENDOR: c_uint = 0x420;
pub const TC_MODULE: c_uint = 0x440;
pub const TC_FIRM_TYPE: c_uint = 0x460;
pub const TC_FLAGS: c_uint = 0x470;
pub const TC_ROM_OBJECTS: c_uint = 0x480;
//
// Information obtained through the get_tcinfo() PROM call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcinfo {
    pub /: *mut *mut s32 revision; / Hardware revision level.,
    pub /: *mut *mut s32 clk_period; / Clock period in nanoseconds.,
    pub /: *mut *mut s32 slot_size; / Slot size in megabytes.,
    pub /: *mut *mut s32 io_timeout; / I/O timeout in cycles.,
    pub /: *mut *mut s32 dma_range; / DMA address range in megabytes.,
    pub /: *mut *mut s32 max_dma_burst; / Maximum DMA burst length.,
    pub /: *mut *mut s32 parity; / System module supports TC parity.,
    pub reserved: [i32; 4],
}

//
// TURBOchannel bus.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_bus {
    pub /: *mut *mut list_head devices; / List of devices on this bus.,
    pub /: *mut *mut resource resource[2]; / Address space routed to this bus.,
    pub dev: device,
    pub name: [c_char; 13],
    pub slot_base: resource_size_t,
    pub ext_slot_base: resource_size_t,
    pub ext_slot_size: resource_size_t,
    pub num_tcslots: c_int,
    pub info: tcinfo,
}

//
// TURBOchannel device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_dev {
    pub /: *mut *mut list_head node; / Node in list of all TC devices.,
    pub /: *mut *mut *mut tc_bus bus; / Bus this device is on.,
    pub this: *mut *mut *mut tc_driver driver; / Which driver has allocated,
    pub /: *mut *mut device dev; / Generic device interface.,
    pub /: *mut *mut resource resource; / Address space of this device.,
    pub /: *mut *mut u64 dma_mask; / DMA addressable range.,
    pub vendor: [c_char; 9],
    pub name: [c_char; 9],
    pub firmware: [c_char; 9],
    pub interrupt: c_int,
    pub slot: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_device_id {
    pub vendor: [c_char; 9],
    pub name: [c_char; 9],
}

//
// TURBOchannel driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_driver {
    pub node: list_head,
    pub id_table: *const tc_device_id,
    pub driver: device_driver,
}

//
// Return TURBOchannel clock frequency in Hz.
//

extern "C" {
    pub fn tc_register_driver(tdrv: *mut tc_driver) -> c_int;
}
extern "C" {
    pub fn tc_unregister_driver(tdrv: *mut tc_driver);
}

//
// These have to be provided by the architecture.
//
extern "C" {
    pub fn tc_preadb(valp: *mut u8, addr: *mut void __iomem) -> c_int;
}
extern "C" {
    pub fn tc_bus_get_info(tbus: *mut tc_bus) -> c_int;
}
extern "C" {
    pub fn tc_device_get_irq(tdev: *mut tc_dev);
}
