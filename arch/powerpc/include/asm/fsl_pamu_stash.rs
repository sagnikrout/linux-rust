//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/fsl_pamu_stash.h
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
// Copyright (C) 2013 Freescale Semiconductor, Inc.
//
// cache stash targets
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pamu_stash_target {
    PAMU_ATTR_CACHE_L1 = 1,
    PAMU_ATTR_CACHE_L2,
    PAMU_ATTR_CACHE_L3,
}

extern "C" {
    pub fn fsl_pamu_configure_l1_stash(domain: *mut iommu_domain, cpu: u32) -> c_int;
}
