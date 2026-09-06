//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/iommu-pages.h
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
// Copyright (c) 2024, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//

//
// struct ioptdesc - Memory descriptor for IOMMU page tables
// @iopt_freelist_elm: List element for a struct iommu_pages_list
//
// This struct overlays struct page for now. Do not modify without a good
// understanding of the issues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioptdesc {
    pub __page_flags: c_ulong,
    pub iopt_freelist_elm: list_head,
    pub __page_mapping: c_ulong,
    pub incoherent: u8,
    pub __index: pgoff_t,
}

extern "C" {
    pub fn folio_ioptdesc(_arg: virt_to_folio(virt)) -> return;
}
extern "C" {
    pub fn iommu_free_pages(virt: *mut c_void);
}
extern "C" {
    pub fn iommu_put_pages_list(list: *mut iommu_pages_list);
}
//
// iommu_pages_list_add - add the page to a iommu_pages_list
// @list: List to add the page to
// @virt: Address returned from iommu_alloc_pages_node_sz()
//
// iommu_pages_list_splice - Put all the pages in list from into list to
// @from: Source list of pages
// @to: Destination list of pages
//
// from must be re-initialized after calling this function if it is to be
// used again.
//
// iommu_pages_list_empty - True if the list is empty
// @list: List to check
//
extern "C" {
    pub fn list_empty(_arg: &list->pages) -> return;
}
//
// iommu_alloc_pages_sz - Allocate a zeroed page of a given size from
// specific NUMA node
// @nid: memory NUMA node id
// @gfp: buddy allocator flags
// @size: Memory size to allocate, this is rounded up to a power of 2
//
// Returns the virtual address of the allocated page.
//
extern "C" {
    pub fn iommu_alloc_pages_node_sz(_arg: NUMA_NO_NODE, _arg: gfp, _arg: size) -> return;
}
extern "C" {
    pub fn iommu_pages_start_incoherent(virt: *mut c_void, dma_dev: *mut device) -> c_int;
}

pub const IOMMU_PAGES_USE_DMA_API: c_int = 0;

//
// For performance leave the incoherent flag alone which turns this into
// a NOP. For X86 the rest of the stop/free flow ignores the flag.
//

pub const IOMMU_PAGES_USE_DMA_API: c_int = 1;

extern "C" {
    pub fn iommu_pages_free_incoherent(virt: *mut c_void, dma_dev: *mut device);
}

