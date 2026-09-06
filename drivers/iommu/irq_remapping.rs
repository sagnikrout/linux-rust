//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/irq_remapping.h
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
// Copyright (C) 2012 Advanced Micro Devices, Inc.
// Author: Joerg Roedel <jroedel@suse.de>
//
// This header file contains stuff that is shared between different interrupt
// remapping drivers but with no need to be visible outside of the IOMMU layer.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_remap_ops {
// The supported capabilities
    pub capability: c_int,
// Initializes hardware and makes it ready for remapping interrupts
    pub (*prepare)(void): *mut c_int,
// Enables the remapping hardware
    pub (*enable)(void): *mut c_int,
// Disables the remapping hardware
    pub (*disable)(void): *mut c_void,
// Reenables the remapping hardware
    pub (*reenable)(int): *mut c_int,
// Enable fault handling
    pub int): *mut *mut int (enable_faulting)(unsigned,
}

pub const irq_remapping_enabled: c_int = 0;
pub const irq_remap_broken: c_int = 0;
pub const disable_irq_post: c_int = 1;

