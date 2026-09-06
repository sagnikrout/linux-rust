//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/amba/bus.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/include/amba/bus.h
//
// This device type deals with ARM PrimeCells and anything else that
// presents a proper CID (0xB105F00D) at the end of the I/O register
// region or that is derived from a PrimeCell.
//
// Copyright (C) 2003 Deep Blue Solutions Ltd, All Rights Reserved.
//

pub const AMBA_NR_IRQS: c_int = 9;
pub const AMBA_CID: c_uint = 0xb105f00d;
pub const CORESIGHT_CID: c_uint = 0xb105900d;
//
// CoreSight Architecture specification updates the ID specification
// for components on the AMBA bus. (ARM IHI 0029E)
//
// Bits 15:12 of the CID are the device class.
//
// Class 0xF remains for PrimeCell and legacy components. (AMBA_CID above)
// Class 0x9 defines the component as CoreSight (CORESIGHT_CID above)
// Class 0x0, 0x1, 0xB, 0xE define components that do not have driver support
// at present.
// Class 0x2-0x8,0xA and 0xD-0xD are presently reserved.
//
// Remaining CID bits stay as 0xb105-00d
//
// Class 0x9 components use additional values to form a Unique Component
// Identifier (UCI), where peripheral ID values are identical for different
// components. Passed to the amba bus code from the component driver via
// the amba_id->data pointer.
// @devarch	: coresight devarch register value
// @devarch_mask: mask bits used for matching. 0 indicates UCI not used.
// @devtype	: coresight device type value
// @data	: additional driver data. As we have usurped the original
// pointer some devices may still need additional data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amba_cs_uci_id {
    pub devarch: c_uint,
    pub devarch_mask: c_uint,
    pub devtype: c_uint,
    pub data: *mut c_void,
}

// define offsets for registers used by UCI
pub const UCI_REG_DEVTYPE_OFFSET: c_uint = 0xFCC;
pub const UCI_REG_DEVARCH_OFFSET: c_uint = 0xFBC;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amba_device {
    pub dev: device,
    pub res: resource,
    pub pclk: *mut clk,
    pub dma_parms: device_dma_parameters,
    pub periphid: c_uint,
    pub periphid_lock: mutex,
    pub cid: c_uint,
    pub uci: amba_cs_uci_id,
    pub irq: [c_uint; AMBA_NR_IRQS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amba_driver {
    pub drv: device_driver,
    pub ): *const *const *const int (probe)(struct amba_device , struct amba_id,
    pub ): *mut *mut void (remove)(struct amba_device,
    pub ): *mut *mut void (shutdown)(struct amba_device,
    pub id_table: *const amba_id,
//
// For most device drivers, no need to care about this flag as long as
// all DMAs are handled through the kernel DMA API. For some special
// ones, for example VFIO drivers, they know how to manage the DMA
// themselves and set this flag so that the IOMMU layer will allow them
// to setup and manage their own I/O address space.
//
    pub driver_managed_dma: bool,
}

//
// Constants for the designer field of the Peripheral ID register. When bit 7
// is set to '1', bits [6:0] should be the JEP106 manufacturer identity code.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amba_vendor {
    AMBA_VENDOR_ARM = 0x41,
    AMBA_VENDOR_ST = 0x80,
    AMBA_VENDOR_QCOM = 0x51,
    AMBA_VENDOR_LSI = 0xb6,
}

//
// use a macro to avoid include chaining to get THIS_MODULE
//

extern "C" {
    pub fn __amba_driver_register(: *mut amba_driver, : *mut module) -> c_int;
}
extern "C" {
    pub fn amba_driver_unregister(: *mut amba_driver);
}
extern "C" {
    pub fn dev_is_amba(dev: *const device) -> bool;
}

extern "C" {
    pub fn amba_device_put(: *mut amba_device);
}
extern "C" {
    pub fn amba_device_add(: *mut amba_device, : *mut resource) -> c_int;
}
extern "C" {
    pub fn amba_device_register(: *mut amba_device, : *mut resource) -> c_int;
}
extern "C" {
    pub fn amba_device_unregister(: *mut amba_device);
}
extern "C" {
    pub fn amba_request_regions(: *mut amba_device, : *const c_char) -> c_int;
}
extern "C" {
    pub fn amba_release_regions(: *mut amba_device);
}
// Some drivers don't use the struct amba_device

//
// APB devices do not themselves have the ability to address memory,
// so DMA masks should be zero (much like USB peripheral devices.)
// The DMA controller DMA masks should be used instead (much like
// USB host controllers in conventional PCs.)
//

//
// AHB devices are DMA capable, so set their DMA masks
//

//
// module_amba_driver() - Helper macro for drivers that don't do anything
// special in module init/exit.  This eliminates a lot of boilerplate.  Each
// module may only use this macro once, and calling it replaces module_init()
// and module_exit()
//

//
// builtin_amba_driver() - Helper macro for drivers that don't do anything
// special in driver initcall.  This eliminates a lot of boilerplate.  Each
// driver may only use this macro once, and calling it replaces the instance
// device_initcall().
//

