//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/vio.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// IBM PowerPC Virtual I/O Infrastructure Support.
//
// Copyright (c) 2003 IBM Corp.
// Dave Engebretsen engebret@us.ibm.com
// Santiago Leon santil@us.ibm.com
//

//
// Architecture-specific constants for drivers to
// extract attributes of the device using vio_get_attribute()
//

// End architecture-specific constants

//
// VIO CMO minimum entitlement for all devices and spare entitlement
//
pub const VIO_CMO_MIN_ENT: c_int = 1562624;
//
// Platform Facilities Option (PFO)-specific data
//
// Starting unit address for PFO devices on the VIO BUS
pub const VIO_BASE_PFO_UA: c_uint = 0x50000000;
//
// vio_pfo_op - PFO operation parameters
//
// @flags: h_call subfunctions and modifiers
// @in: Input data block logical real address
// @inlen: If non-negative, the length of the input data block.  If negative,
// the length of the input data descriptor list in bytes.
// @out: Output data block logical real address
// @outlen: If non-negative, the length of the input data block.  If negative,
// the length of the input data descriptor list in bytes.
// @csbcpb: Logical real address of the 4k naturally-aligned storage block
// containing the CSB & optional FC field specific CPB
// @timeout: # of milliseconds to retry h_call, 0 for no timeout.
// @hcall_err: pointer to return the h_call return value, else NULL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vio_pfo_op {
    pub flags: u64,
    pub in: i64,
    pub inlen: i64,
    pub out: i64,
    pub outlen: i64,
    pub csbcpb: u64,
    pub done: *mut c_void,
    pub handle: c_ulong,
    pub timeout: c_uint,
    pub hcall_err: c_long,
}

// End PFO specific data
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vio_dev_family {
    VDEVICE,	/* The OF node is a child of /vdevice */
    PFO,		/* The OF node is a child of /ibm,platform-facilities */
}

//
// vio_dev - This structure is used to describe virtual I/O devices.
//
// @desired: set from return of driver's get_desired_dma() function
// @entitled: bytes of IO data that has been reserved for this device.
// @allocated: bytes of IO data currently in use by the device.
// @allocs_failed: number of DMA failures due to insufficient entitlement.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vio_dev {
    pub name: *const c_char,
    pub type: *const c_char,
    pub unit_address: u32,
    pub resource_id: u32,
    pub irq: c_uint,
    pub desired: usize,
    pub entitled: usize,
    pub allocated: usize,
    pub allocs_failed: core::sync::atomic::AtomicI32,
    pub cmo: },
    pub family: vio_dev_family,
    pub dev: device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vio_driver {
    pub name: *const c_char,
    pub id_table: *const vio_device_id,
    pub id): *const *const *const int (probe)(struct vio_dev dev, struct vio_device_id,
    pub dev): *mut *mut void (remove)(struct vio_dev,
    pub dev): *mut *mut void (shutdown)(struct vio_dev,
// A driver must have a get_desired_dma() function to
// be loaded in a CMO environment if it uses DMA.
//
    pub dev): *mut *mut unsigned long (get_desired_dma)(struct vio_dev,
    pub pm: *const dev_pm_ops,
    pub driver: device_driver,
}

//
// vio_register_driver must be a macro so that KBUILD_MODNAME can be expanded
//

extern "C" {
    pub fn vio_unregister_driver(drv: *mut vio_driver);
}
extern "C" {
    pub fn vio_cmo_entitlement_update(_arg: usize) -> c_int;
}
extern "C" {
    pub fn vio_cmo_set_dev_desired(viodev: *mut vio_dev, desired: usize);
}
extern "C" {
    pub fn vio_unregister_device(dev: *mut vio_dev);
}
extern "C" {
    pub fn vio_h_cop_sync(vdev: *mut vio_dev, op: *mut vio_pfo_op) -> c_int;
}

extern "C" {
    pub fn vio_enable_interrupts(dev: *mut vio_dev) -> c_int;
}
extern "C" {
    pub fn vio_disable_interrupts(dev: *mut vio_dev) -> c_int;
}

