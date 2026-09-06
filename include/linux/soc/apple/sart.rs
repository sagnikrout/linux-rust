//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/apple/sart.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple SART device driver
// Copyright (C) The Asahi Linux Contributors
//
// Apple SART is a simple address filter for DMA transactions.
// Regions of physical memory must be added to the SART's allow
// list before any DMA can target these. Unlike a proper
// IOMMU no remapping can be done.
//

//
// Get a reference to the SART attached to dev.
//
// Looks for the phandle reference in apple,sart and returns a pointer
// to the corresponding apple_sart struct to be used with
// apple_sart_add_allowed_region and apple_sart_remove_allowed_region.
//
// Adds the region [paddr, paddr+size] to the DMA allow list.
//
// @sart: SART reference
// @paddr: Start address of the region to be used for DMA
// @size: Size of the region to be used for DMA.
//
// Removes the region [paddr, paddr+size] from the DMA allow list.
//
// Note that exact same paddr and size used for apple_sart_add_allowed_region
// have to be passed.
//
// @sart: SART reference
// @paddr: Start address of the region no longer used for DMA
// @size: Size of the region no longer used for DMA.
//
