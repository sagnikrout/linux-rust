//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/iommufd/io_pagetable.h
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
// Copyright (c) 2021-2022, NVIDIA CORPORATION & AFFILIATES.
//

//
// Each io_pagetable is composed of intervals of areas which cover regions of
// the iova that are backed by something. iova not covered by areas is not
// populated in the page table. Each area is fully populated with pages.
//
// iovas are in byte units, but must be iopt->iova_alignment aligned.
//
// pages can be NULL, this means some other thread is still working on setting
// up or tearing down the area. When observed under the write side of the
// domain_rwsem a NULL pages must mean the area is still being setup and no
// domains are filled.
//
// storage_domain points at an arbitrary iommu_domain that is holding the PFNs
// for this area. It is locked by the pages->mutex. This simplifies the locking
// as the pages code can rely on the storage_domain without having to get the
// iopt->domains_rwsem.
//
// The io_pagetable::iova_rwsem protects node
// The iopt_pages::mutex protects pages_node
// iopt and iommu_prot are immutable
// The pages::mutex protects num_accesses
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iopt_area {
    pub node: interval_tree_node,
    pub pages_node: interval_tree_node,
    pub iopt: *mut io_pagetable,
    pub pages: *mut iopt_pages,
    pub storage_domain: *mut iommu_domain,
// How many bytes into the first page the area starts
    pub page_offset: c_uint,
// IOMMU_READ, IOMMU_WRITE, etc
    pub iommu_prot: c_int,
    pub 1: bool prevent_access :,
    pub num_accesses: c_uint,
    pub num_locks: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iopt_allowed {
    pub node: interval_tree_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iopt_reserved {
    pub node: interval_tree_node,
    pub owner: *mut c_void,
}

extern "C" {
    pub fn iopt_area_fill_domains(area: *mut iopt_area, pages: *mut iopt_pages) -> c_int;
}
extern "C" {
    pub fn iopt_area_unfill_domains(area: *mut iopt_area, pages: *mut iopt_pages);
}
extern "C" {
    pub fn iopt_area_fill_domain(area: *mut iopt_area, domain: *mut iommu_domain) -> c_int;
}
//
// Number of bytes from the start of the iopt_pages that the iova begins.
// iopt_area_start_byte() / PAGE_SIZE encodes the starting page index
// iopt_area_start_byte() % PAGE_SIZE encodes the offset within that page
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iopt_area_contig_iter {
    pub cur_iova: c_ulong,
    pub last_iova: c_ulong,
    pub area: *mut iopt_area,
}

//
// Iterate over a contiguous list of areas that span the iova,last_iova range.
// The caller must check iopt_area_contig_done() after the loop to see if
// contiguous areas existed.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iopt_address_type {
    IOPT_ADDRESS_USER = 0,
    IOPT_ADDRESS_FILE,
    IOPT_ADDRESS_DMABUF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iopt_pages_dmabuf_track {
    pub domain: *mut iommu_domain,
    pub area: *mut iopt_area,
    pub elm: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iopt_pages_dmabuf {
    pub attach: *mut dma_buf_attachment,
    pub phys: phys_vec,
// Always PAGE_SIZE aligned
    pub start: c_ulong,
    pub tracker: list_head,
}

//
// This holds a pinned page list for multiple areas of IO address space. The
// pages always originate from a linear chunk of userspace VA. Multiple
// io_pagetable's, through their iopt_area's, can share a single iopt_pages
// which avoids multi-pinning and double accounting of page consumption.
//
// indexes in this structure are measured in PAGE_SIZE units, are 0 based from
// the start of the uptr and extend to npages. pages are pinned dynamically
// according to the intervals in the access_itree and domains_itree, npinned
// records the current number of pages pinned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iopt_pages {
    pub kref: kref,
    pub mutex: mutex,
    pub npages: usize,
    pub npinned: usize,
    pub last_npinned: usize,
    pub source_task: *mut task_struct,
    pub source_mm: *mut mm_struct,
    pub source_user: *mut user_struct,
    pub type: iopt_address_type,
    pub /: *mut *mut *mut void __user uptr; / IOPT_ADDRESS_USER,
    pub file: *mut file,
    pub start: c_ulong,
}

// IOPT_ADDRESS_DMABUF
// Of iopt_pages_access::node
// Of iopt_area::pages_node
extern "C" {
    pub fn iopt_release_pages(kref: *mut kref);
}
//
// Each interval represents an active iopt_access_pages(), it acts as an
// interval lock that keeps the PFNs pinned and stored in the xarray.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iopt_pages_access {
    pub node: interval_tree_node,
    pub users: c_uint,
}
