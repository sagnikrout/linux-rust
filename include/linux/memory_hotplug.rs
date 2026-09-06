//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/memory_hotplug.h
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

// Types for control the zone type of onlined and offlined memory
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmop {
// Offline the memory.
    MMOP_OFFLINE = 0,
// Online the memory. Zone depends, see default_zone_for_pfn().
    MMOP_ONLINE,
// Online the memory to ZONE_NORMAL.
    MMOP_ONLINE_KERNEL,
// Online the memory to ZONE_MOVABLE.
    MMOP_ONLINE_MOVABLE,
}

// Flags for add_memory() and friends to specify memory hotplug details.
pub type mhp_t = int ;
// No special request

//
// Allow merging of the added System RAM resource with adjacent,
// mergeable resources. After a successful call to add_memory_resource()
// with this flag set, the resource pointer must no longer be used as it
// might be stale, or the resource might have changed.
//

//
// We want memmap (struct page array) to be self contained.
// To do so, we will use the beginning of the hot-added range to build
// the page tables for the memmap array that describes the entire range.
// Only selected architectures support it with SPARSE_VMEMMAP.
// This is only a hint, the core kernel can decide to not do this based on
// different alignment checks.
//

//
// The nid field specifies a memory group id (mgid) instead. The memory group
// implies the node id (nid).
//

//
// Extended parameters for memory hotplug:
// altmap: alternative allocator for memmap array (optional)
// pgprot: page protection flags to apply to newly created page tables
// (required)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhp_params {
    pub altmap: *mut vmem_altmap,
    pub pgprot: pgprot_t,
    pub pgmap: *mut dev_pagemap,
}

extern "C" {
    pub fn mhp_range_allowed(start: u64, size: u64, need_mapping: bool) -> bool;
}
extern "C" {
    pub fn mhp_get_pluggable_range(need_mapping: bool) -> range;
}
extern "C" {
    pub fn mhp_supports_memmap_on_memory() -> bool;
}
//
// Zone resizing functions
//
// Note: any attempt to resize a zone should has pgdat_resize_lock()
// zone_span_writelock() both held. This ensure the size of a zone
// can't be changed while pgdat_resize_lock() held.
//
extern "C" {
    pub fn read_seqbegin(_arg: &zone->span_seqlock) -> return;
}
extern "C" {
    pub fn read_seqretry(_arg: &zone->span_seqlock, _arg: iv) -> return;
}
// VM interface that may be used by firmware interface
extern "C" {
    pub fn mhp_deinit_memmap_on_memory(pfn: c_ulong, nr_pages: c_ulong);
}
extern "C" {
    pub fn void(page: *mut *mut online_page_callback_t)(struct page, order: c_uint) -> typedef;
}
extern "C" {
    pub fn generic_online_page(page: *mut page, order: c_uint);
}
extern "C" {
    pub fn set_online_page_callback(callback: online_page_callback_t) -> c_int;
}
extern "C" {
    pub fn restore_online_page_callback(callback: online_page_callback_t) -> c_int;
}
extern "C" {
    pub fn try_online_node(nid: c_int) -> c_int;
}
extern "C" {
    pub fn mhp_online_type_from_str(str: *const c_char) -> c_int;
}
// If movable_node boot option specified
// reasonably generic interface to expand the physical pages

extern "C" {
    pub fn __add_pages(_arg: nid, _arg: start_pfn, _arg: nr_pages, _arg: params) -> return;
}

extern "C" {
    pub fn get_online_mems();
}
extern "C" {
    pub fn put_online_mems();
}
extern "C" {
    pub fn mem_hotplug_begin();
}
extern "C" {
    pub fn mem_hotplug_done();
}
// See kswapd_is_running()

//
// Keep this declaration outside CONFIG_MEMORY_HOTPLUG as some
// platforms might override and use arch_get_mappable_range()
// for internal non memory hotplug purposes.
//
extern "C" {
    pub fn arch_get_mappable_range() -> range;
}

//
// pgdat resizing functions
//

//
// Stub functions for when hotplug is off
//

extern "C" {
    pub fn try_offline_node(nid: c_int);
}
extern "C" {
    pub fn remove_memory(start: u64, size: u64) -> c_int;
}
extern "C" {
    pub fn __remove_memory(start: u64, size: u64);
}
extern "C" {
    pub fn offline_and_remove_memory(start: u64, size: u64) -> c_int;
}

// Default online_type (MMOP_*) when new memory blocks are added.
extern "C" {
    pub fn mhp_get_default_online_type() -> mmop;
}
extern "C" {
    pub fn mhp_set_default_online_type(online_type: mmop);
}
extern "C" {
    pub fn free_area_init_core_hotplug(pgdat: *mut pglist_data) -> int __ref;
}
extern "C" {
    pub fn __add_memory(nid: c_int, start: u64, size: u64, mhp_flags: mhp_t) -> c_int;
}
extern "C" {
    pub fn add_memory(nid: c_int, start: u64, size: u64, mhp_flags: mhp_t) -> c_int;
}
extern "C" {
    pub fn arch_remove_linear_mapping(start: u64, size: u64);
}

