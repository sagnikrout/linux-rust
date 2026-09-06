//! Automatically rewritten from C Header to Rust Module
//! Source: mm/damon/tests/vaddr-kunit.h
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
// Data Access Monitor Unit Tests
//

//
// Test __damon_va_three_regions() function
//
// In case of virtual memory address spaces monitoring, DAMON converts the
// complex and dynamic memory mappings of each target task to three
// discontiguous regions which cover every mapped areas.  However, the three
// regions should not include the two biggest unmapped areas in the original
// mapping, because the two biggest areas are normally the areas between 1)
// heap and the mmap()-ed regions, and 2) the mmap()-ed regions and stack.
// Because these two unmapped areas are very huge but obviously never accessed,
// covering the region is just a waste.
//
// '__damon_va_three_regions() receives an address space of a process.  It
// first identifies the start of mappings, end of mappings, and the two biggest
// unmapped areas.  After that, based on the information, it constructs the
// three regions and returns.  For more detail, refer to the comment of
// 'damon_init_regions_of()' function definition in 'mm/damon.c' file.
//
// For example, suppose virtual address ranges of 10-20, 20-25, 200-210,
// 210-220, 300-305, and 307-330 (Other comments represent this mappings in
// more short form: 10-20-25, 200-210-220, 300-305, 307-330) of a process are
// mapped.  To cover every mappings, the three regions should start with 10,
// and end with 305.  The process also has three unmapped areas, 25-200,
// 220-300, and 305-307.  Among those, 25-200 and 220-300 are the biggest two
// unmapped areas, and thus it should be converted to three regions of 10-25,
// 200-220, and 300-330.
//
// 10-20-25, 200-210-220, 300-305, 307-330
//
// Test 'damon_set_regions()'
//
// test			kunit object
// regions		an array containing start/end addresses of current
// monitoring target regions
// nr_regions		the number of the addresses in 'regions'
// three_regions	The three regions that need to be applied now
// expected		start/end addresses of monitoring target regions that
// 'three_regions' are applied
// nr_expected		the number of addresses in 'expected'
//
// The memory mapping of the target processes changes dynamically.  To follow
// the change, DAMON periodically reads the mappings, simplifies it to the
// three regions, and updates the monitoring target regions to fit in the three
// regions.  The update of current target regions is the role of
// 'damon_set_regions()'.
//
// This test passes the given target regions and the new three regions that
// need to be applied to the function and check whether it updates the regions
// as expected.
//
// This function test most common case where the three big regions are only
// slightly changed.  Target regions should adjust their boundary (10-20-30,
// 50-55, 70-80, 90-100) to fit with the new big regions or remove target
// regions (57-79) that now out of the three regions.
//
// 10-20-30, 50-55-57-59, 70-80-90-100
// 5-27, 45-55, 73-104
// 5-20-27, 45-55, 73-80-90-104
//
// Test slightly bigger change.  Similar to above, but the second big region
// now require two target regions (50-55, 57-59) to be removed.
//
// 10-20-30, 50-55-57-59, 70-80-90-100
// 5-27, 56-57, 65-104
// 5-20-27, 56-57, 65-80-90-104
//
// Test a big change.  The second big region has totally freed and mapped to
// different area (50-59 -> 61-63).  The target regions which were in the old
// second big region (50-55-57-59) should be removed and new target region
// covering the second big region (61-63) should be created.
//
// 10-20-30, 50-55-57-59, 70-80-90-100
// 5-27, 61-63, 65-104
// 5-20-27, 61-63, 65-80-90-104
//
// Test another big change.  Both of the second and third big regions (50-59
// and 70-100) has totally freed and mapped to different area (30-32 and
// 65-68).  The target regions which were in the old second and third big
// regions should now be removed and new target regions covering the new second
// and third big regions should be created.
//
// 10-20-30, 50-55-57-59, 70-80-90-100
// 5-7, 30-32, 65-68
// expect 5-7, 30-32, 65-68

