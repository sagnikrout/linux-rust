//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hmm.h
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
// Copyright 2013 Red Hat Inc.
//
// Authors: Jérôme Glisse <jglisse@redhat.com>
//
// See Documentation/mm/hmm.rst for reasons and overview of what HMM is.
//

//
// On output:
// 0             - The page is faultable and a future call with
// HMM_PFN_REQ_FAULT could succeed.
// HMM_PFN_VALID - the pfn field points to a valid PFN. This PFN is at
// least readable. If dev_private_owner is !NULL then this could
// point at a DEVICE_PRIVATE page.
// HMM_PFN_WRITE - if the page memory can be written to (requires HMM_PFN_VALID)
// HMM_PFN_ERROR - accessing the pfn is impossible and the device should
// fail. ie poisoned memory, special pages, no vma, etc
// HMM_PFN_P2PDMA - P2P page
// HMM_PFN_P2PDMA_BUS - Bus mapped P2P transfer
// HMM_PFN_DMA_MAPPED - Flag preserved on input-to-output transformation
// to mark that page is already DMA mapped
//
// On input:
// 0                 - Return the current state of the page, do not fault it.
// HMM_PFN_REQ_FAULT - The output must have HMM_PFN_VALID or hmm_range_fault()
// will fail
// HMM_PFN_REQ_WRITE - The output must have HMM_PFN_WRITE or hmm_range_fault()
// will fail. Must be combined with HMM_PFN_REQ_FAULT.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hmm_pfn_flags {
// Output fields and flags
    HMM_PFN_VALID = 1UL << (BITS_PER_LONG - 1),
    HMM_PFN_WRITE = 1UL << (BITS_PER_LONG - 2),
    HMM_PFN_ERROR = 1UL << (BITS_PER_LONG - 3),
//
// Sticky flags, carried from input to output,
// don't forget to update HMM_PFN_INOUT_FLAGS
//
    HMM_PFN_DMA_MAPPED = 1UL << (BITS_PER_LONG - 4),
    HMM_PFN_P2PDMA     = 1UL << (BITS_PER_LONG - 5),
    HMM_PFN_P2PDMA_BUS = 1UL << (BITS_PER_LONG - 6),

    HMM_PFN_ORDER_SHIFT = (BITS_PER_LONG - 11),

// Input flags
    HMM_PFN_REQ_FAULT = HMM_PFN_VALID,
    HMM_PFN_REQ_WRITE = HMM_PFN_WRITE,

    HMM_PFN_FLAGS = ~((1UL << HMM_PFN_ORDER_SHIFT) - 1),
}

//
// hmm_pfn_to_page() - return struct page pointed to by a device entry
//
// This must be called under the caller 'user_lock' after a successful
// mmu_interval_read_begin(). The caller must have tested for HMM_PFN_VALID
// already.
//
extern "C" {
    pub fn pfn_to_page(~HMM_PFN_FLAGS: hmm_pfn &) -> return;
}
//
// hmm_pfn_to_phys() - return physical address pointed to by a device entry
//
extern "C" {
    pub fn __pfn_to_phys(~HMM_PFN_FLAGS: hmm_pfn &) -> return;
}
//
// hmm_pfn_to_map_order() - return the CPU mapping size order
//
// This is optionally useful to optimize processing of the pfn result
// array. It indicates that the page starts at the order aligned VA and is
// 1<<order bytes long.  Every pfn within an high order page will have the
// same pfn flags, both access protections and the map_order.  The caller must
// be careful with edge cases as the start and end VA of the given page may
// extend past the range used with hmm_range_fault().
//
// This must be called under the caller 'user_lock' after a successful
// mmu_interval_read_begin(). The caller must have tested for HMM_PFN_VALID
// already.
//
// struct hmm_range - track invalidation lock on virtual address range
//
// @notifier: a mmu_interval_notifier that includes the start/end
// @notifier_seq: result of mmu_interval_read_begin()
// @start: range virtual start address (inclusive)
// @end: range virtual end address (exclusive)
// @hmm_pfns: array of pfns (big enough for the range)
// @default_flags: default flags for the range (write, read, ... see hmm doc)
// @pfn_flags_mask: allows to mask pfn flags so that only default_flags matter
// @dev_private_owner: owner of device private pages
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmm_range {
    pub notifier: *mut mmu_interval_notifier,
    pub notifier_seq: c_ulong,
    pub start: c_ulong,
    pub end: c_ulong,
    pub hmm_pfns: *mut c_ulong,
    pub default_flags: c_ulong,
    pub pfn_flags_mask: c_ulong,
    pub dev_private_owner: *mut c_void,
}

//
// Please see Documentation/mm/hmm.rst for how to use the range API.
//
extern "C" {
    pub fn hmm_range_fault(range: *mut hmm_range) -> c_int;
}
//
// HMM_RANGE_DEFAULT_TIMEOUT - default timeout (ms) when waiting for a range
//
// When waiting for mmu notifiers we need some kind of time out otherwise we
// could potentially wait for ever, 1000ms ie 1s sounds like a long time to
// wait already.
//
pub const HMM_RANGE_DEFAULT_TIMEOUT: c_int = 1000;
