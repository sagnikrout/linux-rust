//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/omap-iopgtable.h
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
// omap iommu: pagetable definitions
//
// Copyright (C) 2008-2010 Nokia Corporation
//
// Written by Hiroshi DOYU <Hiroshi.DOYU@nokia.com>
//

//
// "L2 table" address mask and size definitions.
//
pub const IOPGD_SHIFT: c_int = 20;

//
// "section" address mask and size definitions.
//
pub const IOSECTION_SHIFT: c_int = 20;

//
// "supersection" address mask and size definitions.
//
pub const IOSUPER_SHIFT: c_int = 24;

//
// "small page" address mask and size definitions.
//
pub const IOPTE_SHIFT: c_int = 12;

//
// "large page" address mask and size definitions.
//
pub const IOLARGE_SHIFT: c_int = 16;

//
// omap_iommu_translate() - va to pa translation
// @d:		omap iommu descriptor
// @va:		virtual address
// @mask:	omap iommu descriptor mask
//
// va to pa translation
//
// some descriptor attributes.
//

// to find an entry in a page-table-directory

// to find an entry in the second-level page table.

