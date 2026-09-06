//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mcb/mcb-internal.h
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

pub const PCI_VENDOR_ID_MEN: c_uint = 0x1a88;
pub const PCI_DEVICE_ID_MEN_CHAMELEON: c_uint = 0x4d45;
pub const CHAMELEONV2_MAGIC: c_uint = 0xabce;
pub const CHAM_HEADER_SIZE: c_uint = 0x200;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chameleon_descriptor_type {
    CHAMELEON_DTYPE_GENERAL = 0x0,
    CHAMELEON_DTYPE_BRIDGE = 0x1,
    CHAMELEON_DTYPE_CPU = 0x2,
    CHAMELEON_DTYPE_BAR = 0x3,
    CHAMELEON_DTYPE_END = 0xf,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chameleon_bus_type {
    CHAMELEON_BUS_WISHBONE,
    CHAMELEON_BUS_AVALON,
    CHAMELEON_BUS_LPC,
    CHAMELEON_BUS_ISA,
}

//
// struct chameleon_fpga_header
//
// @revision:	Revison of Chameleon table in FPGA
// @model:	Chameleon table model ASCII char
// @minor:	Revision minor
// @bus_type:	Bus type (usually %CHAMELEON_BUS_WISHBONE)
// @magic:	Chameleon header magic number (0xabce for version 2)
// @reserved:	Reserved
// @filename:	Filename of FPGA bitstream
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chameleon_fpga_header {
    pub revision: u8,
    pub model: c_char,
    pub minor: u8,
    pub bus_type: u8,
    pub magic: u16,
    pub reserved: u16,
// This one has no '\0' at the end!!!
    pub filename: [c_char; CHAMELEON_FILENAME_LEN],
    pub __packed: },
pub const HEADER_MAGIC_OFFSET: c_uint = 0x4;
//
// struct chameleon_gdd - Chameleon General Device Descriptor
//
// @irq:	the position in the FPGA's IRQ controller vector
// @rev:	the revision of the variant's implementation
// @var:	the variant of the IP core
// @dev:	the device  the IP core is
// @dtype:	device descriptor type
// @bar:	BAR offset that must be added to module offset
// @inst:	the instance number of the device, 0 is first instance
// @group:	the group the device belongs to (0 = no group)
// @reserved:	reserved
// @offset:	beginning of the address window of desired module
// @size:	size of the module's address window
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chameleon_gdd {
    pub reg1: __le32,
    pub reg2: __le32,
    pub offset: __le32,
    pub size: __le32,
    pub __packed: },
// GDD Register 1 fields

// GDD Register 2 fields

//
// struct chameleon_bdd - Chameleon Bridge Device Descriptor
//
// @irq:	the position in the FPGA's IRQ controller vector
// @rev:	the revision of the variant's implementation
// @var:	the variant of the IP core
// @dev:	the device  the IP core is
// @dtype:	device descriptor type
// @bar:	BAR offset that must be added to module offset
// @inst:	the instance number of the device, 0 is first instance
// @dbar:	destination bar from the bus _behind_ the bridge
// @chamoff:	offset within the BAR of the source bus
// @offset:
// @size:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chameleon_bdd {
    pub irq:6: c_uint,
    pub rev:6: c_uint,
    pub var:6: c_uint,
    pub dev:10: c_uint,
    pub dtype:4: c_uint,
    pub bar:3: c_uint,
    pub inst:6: c_uint,
    pub dbar:3: c_uint,
    pub group:6: c_uint,
    pub reserved:14: c_uint,
    pub chamoff: u32,
    pub offset: u32,
    pub size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chameleon_bar {
    pub addr: u32,
    pub size: u32,
}

pub const CHAMELEON_BAR_MAX: c_int = 6;

