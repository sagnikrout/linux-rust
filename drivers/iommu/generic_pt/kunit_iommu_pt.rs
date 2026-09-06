//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/generic_pt/kunit_iommu_pt.h
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
// Copyright (c) 2024, NVIDIA CORPORATION & AFFILIATES
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct count_valids {
    pub per_size: [u64; PT_VADDR_MAX_LG2],
}

//
// Number of valid table entries. This counts contiguous entries as a single
// valid.
//
// Only a single page size is present, count the number of valid entries
// Add every possible level to the max
// Map every reported page size
// The read interface reports that every page size was created
// Unmap works
//
// Test to convert a table pointer into an OA by mapping something small,
// unmapping it so as to leave behind a table pointer, then mapping something
// larger that will convert the table into an OA.
//
// Test unmapping a small page at the start of a large page. This always unmaps
// the large page.
//
// Make sure unmap doesn't keep going
//
// Randomly map and unmap ranges that can large physical pages. If a random
// range overlaps with existing ranges then unmap them. This hits all the
// special cases.
//
// Shrink the range so randomization is more likely to have
// intersections
//
// Try overmapping to test the failure handling
// See https://lore.kernel.org/r/b9b18a03-63a2-4065-a27e-d92dd5c860bc@amd.com
// See https://lore.kernel.org/r/20250826143816.38686-1-eugkoira@amazon.com
// 14 2M, 3 1G, 3 2M
//
// Look for memory leaks, assumes kunit is running isolated and nothing
// else is using secondary page tables.
//
