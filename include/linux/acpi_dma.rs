//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/acpi_dma.h
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
// ACPI helpers for DMA request / controller
//
// Based on of_dma.h
//
// Copyright (C) 2013, Intel Corporation
// Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
//

//
// struct acpi_dma_spec - slave device DMA resources
// @chan_id:	channel unique id
// @slave_id:	request line unique id
// @dev:	struct device of the DMA controller to be used in the filter
// function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dma_spec {
    pub chan_id: c_int,
    pub slave_id: c_int,
    pub dev: *mut device,
}

//
// struct acpi_dma - representation of the registered DMAC
// @dma_controllers:	linked list node
// @dev:		struct device of this controller
// @acpi_dma_xlate:	callback function to find a suitable channel
// @data:		private data used by a callback function
// @base_request_line:	first supported request line (CSRT)
// @end_request_line:	last supported request line (CSRT)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dma {
    pub dma_controllers: list_head,
    pub dev: *mut device,
    pub ): *mut *mut (struct acpi_dma_spec , struct acpi_dma,
    pub data: *mut c_void,
    pub base_request_line: c_ushort,
    pub end_request_line: c_ushort,
}

// Used with acpi_dma_simple_xlate()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dma_filter_info {
    pub dma_cap: dma_cap_mask_t,
    pub filter_fn: dma_filter_fn,
}

extern "C" {
    pub fn acpi_dma_controller_free(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

