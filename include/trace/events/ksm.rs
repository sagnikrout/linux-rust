//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/ksm.h
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
// ksm_scan_template - called for start / stop scan
//
// @seq:		sequence number of scan
// @rmap_entries:	actual number of rmap entries
//
// Allows to trace the start / stop of a ksm scan.
//
// ksm_start_scan - called after a new ksm scan is started
//
// @seq:		sequence number of scan
// @rmap_entries:	actual number of rmap entries
//
// Allows to trace the start of a ksm scan.
//
// ksm_stop_scan - called after a new ksm scan has completed
//
// @seq:		sequence number of scan
// @rmap_entries:	actual number of rmap entries
//
// Allows to trace the completion of a ksm scan.
//
// ksm_enter - called after a new process has been added / removed from ksm
//
// @mm:			address of the mm object of the process
//
// Allows to trace the when a process has been added or removed from ksm.
//
// ksm_enter - called after a new process has been added to ksm
//
// @mm:			address of the mm object of the process
//
// Allows to trace the when a process has been added to ksm.
//
// ksm_exit - called after a new process has been removed from ksm
//
// @mm:			address of the mm object of the process
//
// Allows to trace the when a process has been removed from ksm.
//
// ksm_merge_one_page - called after a page has been merged
//
// @pfn:		page frame number of ksm page
// @rmap_item:		address of rmap_item  object
// @mm:			address of the process mm struct
// @err:		success
//
// Allows to trace the ksm merging of individual pages.
//
// ksm_merge_with_ksm_page - called after a page has been merged with a ksm page
//
// @ksm_page:		address ksm page
// @pfn:		page frame number of ksm page
// @rmap_item:		address of rmap_item  object
// @mm:			address of the mm object of the process
// @err:		success
//
// Allows to trace the merging of a page with a ksm page.
//
// ksm_remove_ksm_page - called after a ksm page has been removed
//
// @pfn:		page frame number of ksm page
//
// Allows to trace the removing of stable ksm pages.
//
// ksm_remove_rmap_item - called after a rmap_item has been removed from the
// stable tree
//
// @pfn:		page frame number of ksm page
// @rmap_item:		address of rmap_item  object
// @mm:			address of the process mm struct
//
// Allows to trace the removal of pages from the stable tree list.
//
// ksm_advisor - called after the advisor has run
//
// @scan_time:		scan time in seconds
// @pages_to_scan:	new pages_to_scan value
// @cpu_percent:	cpu usage in percent
//
// Allows to trace the ksm advisor.
//

// This part must be outside protection
