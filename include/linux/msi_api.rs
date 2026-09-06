//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/msi_api.h
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
// APIs which are relevant for device driver code for allocating and
// freeing MSI interrupts and querying the associations between
// hardware/software MSI indices and the Linux interrupt number.
//
// Per device interrupt domain related constants.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msi_domain_ids {
    MSI_DEFAULT_DOMAIN,
    MSI_MAX_DEVICE_IRQDOMAINS,
}

//
// union msi_instance_cookie - MSI instance cookie
// @value:	u64 value store
// @ptr:	Pointer to usage site specific data
//
// This cookie is handed to the IMS allocation function and stored in the
// MSI descriptor for the interrupt chip callbacks.
//
// The content of this cookie is MSI domain implementation defined.  For
// PCI/IMS implementations this could be a PASID or a pointer to queue
// memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union msi_instance_cookie {
    pub value: u64,
    pub ptr: *mut c_void,
}

//
// msi_map - Mapping between MSI index and Linux interrupt number
// @index:	The MSI index, e.g. slot in the MSI-X table or
// a software managed index if >= 0. If negative
// the allocation function failed and it contains
// the error code.
// @virq:	The associated Linux interrupt number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_map {
    pub index: c_int,
    pub virq: c_int,
}

//
// Constant to be used for dynamic allocations when the allocation is any
// free MSI index, which is either an entry in a hardware table or a
// software managed index.
//

extern "C" {
    pub fn msi_domain_get_virq(dev: *mut device, domid: c_uint, index: c_uint) -> c_uint;
}
//
// msi_get_virq - Lookup the Linux interrupt number for a MSI index on the default interrupt domain
// @dev:	Device for which the lookup happens
// @index:	The MSI index to lookup
//
// Return: The Linux interrupt number on success (> 0), 0 if not found
//
extern "C" {
    pub fn msi_domain_get_virq(_arg: dev, _arg: MSI_DEFAULT_DOMAIN, _arg: index) -> return;
}
