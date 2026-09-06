//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/iommu.h
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
// Copyright (C) 2001 Mike Corrigan & Dave Engebretsen, IBM Corporation
// Rewrite, cleanup:
// Copyright (C) 2004 Olof Johansson <olof@lixom.net>, IBM Corporation
//

pub const IOMMU_PAGE_SHIFT_4K: c_int = 12;

// Boot time flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_table_ops {
//
// When called with direction==DMA_NONE, it is equal to clear().
// uaddr is a linear map address.
//
    pub attrs): c_ulong,

//
// Exchanges existing TCE with new TCE plus direction bits;
// returns old TCE and DMA direction mask.
// @tce is a physical address.
//
    pub direction): *mut dma_data_direction,
    pub pages): c_ulong,
    pub alloc): *mut *mut *mut *mut __be64 (useraddrptr)(struct iommu_table tbl, long index, bool,

    pub npages): long index, long,
// get() returns a physical address
    pub index): *mut *mut *mut unsigned long (get)(struct iommu_table tbl, long,
    pub tbl): *mut *mut void (flush)(struct iommu_table,
    pub tbl): *mut *mut void (free)(struct iommu_table,
}

// These are used by VIO
//
// IOMAP_MAX_ORDER defines the largest contiguous block
// of dma space we can get.  IOMAP_MAX_ORDER = 13
// allows up to 2**12 pages (4096 * 4096) = 16 MB
//
pub const IOMAP_MAX_ORDER: c_int = 13;
pub const IOMMU_POOL_HASHBITS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_pool {
    pub start: c_ulong,
    pub end: c_ulong,
    pub hint: c_ulong,
    pub lock: spinlock_t,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_table {
    pub /: *mut *mut unsigned long it_busno; / Bus number this table belongs to,
    pub /: *mut *mut unsigned long it_size; / Size of iommu table in entries,
    pub it_indirect_levels: c_ulong,
    pub it_level_size: c_ulong,
    pub it_allocated_size: c_ulong,
    pub /: *mut *mut unsigned long it_offset; / Offset into global table,
    pub /: *mut *mut unsigned long it_base; / mapped address of tce table,
    pub /: *mut *mut unsigned long it_index; / which iommu table this is,
    pub /: *mut *mut unsigned long it_type; / type: PCI or Virtual Bus,
    pub /: *mut *mut unsigned long it_blocksize; / Entries in each block (cacheline),
    pub poolsize: c_ulong,
    pub nr_pools: c_ulong,
    pub large_pool: iommu_pool,
    pub pools: [iommu_pool; IOMMU_NR_POOLS],
    pub /: *mut *mut *mut unsigned long it_map; / A simple allocation bitmap for now,
    pub /: *mut *mut unsigned long it_page_shift;/ table iommu page size,
    pub /: *mut *mut list_head it_group_list;/ List of iommu_table_group_link,
    pub /: *mut *mut *mut __be64 it_userspace; / userspace view of the table,
    pub it_ops: *mut iommu_table_ops,
    pub it_kref: kref,
    pub it_nid: c_int,
    pub /: *mut *mut unsigned long it_reserved_start; / Start of not-DMA-able (MMIO) area,
    pub it_reserved_end: c_ulong,
}

// Pure 2^n version of get_order

extern "C" {
    pub fn dma_iommu_dma_supported(dev: *mut device, mask: u64) -> c_int;
}
extern "C" {
    pub fn iommu_tce_table_put(tbl: *mut iommu_table) -> c_int;
}
// Initializes an iommu_table based in values set in the passed-in
// structure
//
extern "C" {
    pub fn iommu_table_in_use(tbl: *mut iommu_table) -> bool;
}
extern "C" {
    pub fn iommu_table_clear(tbl: *mut iommu_table);
}
pub const IOMMU_TABLE_GROUP_MAX_TABLES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_table_group_ops {
    pub levels): __u32,
    pub ptbl): *mut iommu_table,
    pub tblnew): *mut iommu_table,
    pub num): c_int,
// Switch ownership from platform code to external user (e.g. VFIO)
    pub dev): *mut *mut *mut long (take_ownership)(struct iommu_table_group table_group, struct device,
// Switch ownership from external user (e.g. VFIO) back to core
    pub dev): *mut *mut *mut void (release_ownership)(struct iommu_table_group table_group, struct device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_table_group_link {
    pub next: list_head,
    pub rcu: rcu_head,
    pub table_group: *mut iommu_table_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_table_group {
// IOMMU properties
    pub tce32_start: __u32,
    pub tce32_size: __u32,
    pub /: *mut *mut __u64 pgsizes; / Bitmap of supported page sizes,
    pub max_dynamic_windows_supported: __u32,
    pub max_levels: __u32,
    pub group: *mut iommu_group,
    pub tables: [*mut iommu_table; IOMMU_TABLE_GROUP_MAX_TABLES],
    pub ops: *mut iommu_table_group_ops,
}

extern "C" {
    pub fn dev_has_iommu_table(dev: *mut device, data: *mut c_void) -> c_int;
}

extern "C" {
    pub fn dma_iommu_get_required_mask(dev: *mut device) -> u64;
}

extern "C" {
    pub fn iommu_init_early_pSeries() -> void __init;
}
extern "C" {
    pub fn iommu_init_early_dart(controller_ops: *mut pci_controller_ops);
}
extern "C" {
    pub fn iommu_init_early_pasemi();
}

// The API to support IOMMU operations for VFIO

extern "C" {
    pub fn iommu_flush_tce(tbl: *mut iommu_table);
}
extern "C" {
    pub fn iommu_tce_direction(tce: c_ulong) -> dma_data_direction;
}
extern "C" {
    pub fn iommu_direction_to_tce_perm(dir: dma_data_direction) -> c_ulong;
}

