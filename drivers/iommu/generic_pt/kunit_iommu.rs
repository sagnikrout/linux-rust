//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/generic_pt/kunit_iommu.h
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
// Copyright (c) 2024-2025, NVIDIA CORPORATION & AFFILIATES
//
pub const GENERIC_PT_KUNIT: c_int = 1;

// The format can provide a list of configurations it would like to test

//
// When the test is run on a 32 bit system unsigned long can be 32 bits. This
// cause the iommu op signatures to be restricted to 32 bits. Meaning the test
// has to be mindful not to create any VA's over the 32 bit limit. Reduce the
// scope of the testing as the main purpose of checking on full 32 bit is to
// look for 32bitism in the core code. Run the test on i386 with X86_PAE=y to
// get the full coverage when dma_addr_t & phys_addr_t are 8 bytes
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_iommu_priv {
    pub domain: iommu_domain,
    pub fmt_table: pt_iommu_table,
}

// Enough so the memory allocator works
extern "C" {
    pub fn PTR_ERR(_arg: priv->dummy_dev) -> return;
}

//
// The format can set a list of features that the kunit_fmt_cfgs
// controls, other features are default to on.
//

// Defaults, for the kunit
//
// size_t is used to pass the mapping length, it can be 32 bit, truncate
// the pagesizes so we don't use large sizes.
//
// We run out of VA space if the mappings get too big, make something
// smaller that can safely pass through dma_addr_t API.
//
