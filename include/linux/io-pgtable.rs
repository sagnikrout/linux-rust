//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/io-pgtable.h
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
// Public API for use by IOMMU drivers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum io_pgtable_fmt {
    ARM_32_LPAE_S1,
    ARM_32_LPAE_S2,
    ARM_64_LPAE_S1,
    ARM_64_LPAE_S2,
    ARM_V7S,
    ARM_MALI_LPAE,
    APPLE_DART,
    APPLE_DART2,
    IO_PGTABLE_NUM_FMTS,
}

//
// struct iommu_flush_ops - IOMMU callbacks for TLB and page table management.
//
// @tlb_flush_all:  Synchronously invalidate the entire TLB context.
// @tlb_flush_walk: Synchronously invalidate all intermediate TLB state
// (sometimes referred to as the "walk cache") for a virtual
// address range.
// @tlb_add_page:   Optional callback to queue up leaf TLB invalidation for a
// single page.  IOMMUs that cannot batch TLB invalidation
// operations efficiently will typically issue them here, but
// others may decide to update the iommu_iotlb_gather structure
// and defer the invalidation until iommu_iotlb_sync() instead.
//
// Note that these can all be called in atomic context and must therefore
// not block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_flush_ops {
    pub cookie): *mut *mut void (tlb_flush_all)(void,
    pub cookie): *mut c_void,
    pub cookie): *mut unsigned long iova, size_t granule, void,
}

//
// struct io_pgtable_cfg - Configuration data for a set of page tables.
//
// @quirks:        A bitmap of hardware quirks that require some special
// action by the low-level page table allocator.
// @pgsize_bitmap: A bitmap of page sizes supported by this set of page
// tables.
// @ias:           Input address (iova) size, in bits.
// @oas:           Output address (paddr) size, in bits.
// @coherent_walk: A flag to indicate whether or not page table walks made
// by the IOMMU are coherent with the CPU caches.
// @tlb:           TLB management callbacks for this set of tables.
// @iommu_dev:     The device representing the DMA configuration for the
// page table walker.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_pgtable_cfg {
//
// IO_PGTABLE_QUIRK_ARM_NS: (ARM formats) Set NS and NSTABLE bits in
// stage 1 PTEs, for hardware which insists on validating them
// even in	non-secure state where they should normally be ignored.
//
// IO_PGTABLE_QUIRK_NO_PERMS: Ignore the IOMMU_READ, IOMMU_WRITE and
// IOMMU_NOEXEC flags and map everything with full access, for
// hardware which does not implement the permissions of a given
// format, and/or requires some format-specific default value.
//
// IO_PGTABLE_QUIRK_ARM_MTK_EXT: (ARM v7s format) MediaTek IOMMUs extend
// to support up to 35 bits PA where the bit32, bit33 and bit34 are
// encoded in the bit9, bit4 and bit5 of the PTE respectively.
//
// IO_PGTABLE_QUIRK_ARM_MTK_TTBR_EXT: (ARM v7s format) MediaTek IOMMUs
// extend the translation table base support up to 35 bits PA, the
// encoding format is same with IO_PGTABLE_QUIRK_ARM_MTK_EXT.
//
// IO_PGTABLE_QUIRK_ARM_TTBR1: (ARM LPAE format) Configure the table
// for use in the upper half of a split address space.
//
// IO_PGTABLE_QUIRK_ARM_OUTER_WBWA: Override the outer-cacheability
// attributes set in the TCR for a non-coherent page-table walker.
//
// IO_PGTABLE_QUIRK_ARM_HD: Enables dirty tracking in stage 1 pagetable.
// IO_PGTABLE_QUIRK_ARM_S2FWB: Use the FWB format for the MemAttrs bits
//
// IO_PGTABLE_QUIRK_NO_WARN: Do not WARN_ON() on conflicting
// mappings, but silently return -EEXISTS.  Normally an attempt
// to map over an existing mapping would indicate some sort of
// kernel bug, which would justify the WARN_ON().  But for GPU
// drivers, this could be under control of userspace.  Which
// deserves an error return, but not to spam dmesg.
//

    pub quirks: c_ulong,
    pub pgsize_bitmap: c_ulong,
    pub ias: c_uint,
    pub oas: c_uint,
    pub coherent_walk: bool,
    pub tlb: *const iommu_flush_ops,
    pub iommu_dev: *mut device,
//
// @alloc: Custom page allocator.
//
// Optional hook used to allocate page tables. If this function is NULL,
// @free must be NULL too.
//
// Memory returned should be zeroed and suitable for dma_map_single() and
// virt_to_phys().
//
// Not all formats support custom page allocators. Before considering
// passing a non-NULL value, make sure the chosen page format supports
// this feature.
//
    pub gfp): *mut *mut *mut *mut void (alloc)(void cookie, size_t size, gfp_t,
//
// @free: Custom page de-allocator.
//
// Optional hook used to free page tables allocated with the @alloc
// hook. Must be non-NULL if @alloc is not NULL, must be NULL
// otherwise.
//
    pub size): *mut *mut *mut *mut void (free)(void cookie, void pages, size_t,
// Low-level data specific to the table format
// private:
    pub ttbr: u64,
    pub ips:3: u32,
    pub tg:2: u32,
    pub sh:2: u32,
    pub orgn:2: u32,
    pub irgn:2: u32,
    pub tsz:6: u32,
    pub tcr: },
    pub mair: u64,
    pub arm_lpae_s1_cfg: },
    pub vttbr: u64,
    pub ps:3: u32,
    pub tg:2: u32,
    pub sh:2: u32,
    pub orgn:2: u32,
    pub irgn:2: u32,
    pub sl:2: u32,
    pub tsz:6: u32,
    pub vtcr: },
    pub arm_lpae_s2_cfg: },
    pub ttbr: u32,
    pub tcr: u32,
    pub nmrr: u32,
    pub prrr: u32,
    pub arm_v7s_cfg: },
    pub transtab: u64,
    pub memattr: u64,
    pub arm_mali_lpae_cfg: },
    pub ttbr: [u64; 4],
    pub n_ttbrs: u32,
    pub n_levels: u32,
    pub apple_dart_cfg: },
    pub nid: c_int,
    pub amd: },
}

//
// struct arm_lpae_io_pgtable_walk_data - information from a pgtable walk
//
// @ptes:     The recorded PTE values from the walk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_lpae_io_pgtable_walk_data {
    pub ptes: [u64; 4],
}

//
// struct io_pgtable_ops - Page table manipulation API for IOMMU drivers.
//
// @map_pages:    Map a physically contiguous range of pages of the same size.
// @unmap_pages:  Unmap a range of virtually contiguous pages of the same size.
// @iova_to_phys: Translate iova to physical address.
// @pgtable_walk: (optional) Perform a page table walk for a given iova.
// @read_and_clear_dirty: Record dirty info per IOVA. If an IOVA is dirty,
// clear its dirty state from the PTE unless the
// IOMMU_DIRTY_NO_CLEAR flag is passed in.
//
// These functions map directly onto the iommu_ops member functions with
// the same names.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_pgtable_ops {
    pub mapped): *mut int prot, gfp_t gfp, size_t,
    pub gather): *mut iommu_iotlb_gather,
    pub iova): c_ulong,
    pub wd): *mut *mut *mut int (pgtable_walk)(struct io_pgtable_ops ops, unsigned long iova, void,
    pub dirty): *mut iommu_dirty_bitmap,
}

//
// alloc_io_pgtable_ops() - Allocate a page table allocator for use by an IOMMU.
//
// @fmt:    The page table format.
// @cfg:    The page table configuration. This will be modified to represent
// the configuration actually provided by the allocator (e.g. the
// pgsize_bitmap may be restricted).
// @cookie: An opaque token provided by the IOMMU driver and passed back to
// the callback routines.
//
// Returns: Pointer to the &struct io_pgtable_ops for this set of page tables.
//
// free_io_pgtable_ops() - Free an io_pgtable_ops structure. The caller
// *must* ensure that the page table is no longer
// live, but the TLB can be dirty.
//
// @ops: The ops returned from alloc_io_pgtable_ops.
//
extern "C" {
    pub fn free_io_pgtable_ops(ops: *mut io_pgtable_ops);
}
//
// Internal structures for page table allocator implementations.
//
// struct io_pgtable - Internal structure describing a set of page tables.
//
// @fmt:    The page table format.
// @cookie: An opaque token provided by the IOMMU driver and passed back to
// any callback routines.
// @cfg:    A copy of the page table configuration.
// @ops:    The page table operations in use for this set of page tables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_pgtable {
    pub fmt: io_pgtable_fmt,
    pub cookie: *mut c_void,
    pub cfg: io_pgtable_cfg,
    pub ops: io_pgtable_ops,
}

//
// enum io_pgtable_caps - IO page table backend capabilities.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum io_pgtable_caps {
// @IO_PGTABLE_CAP_CUSTOM_ALLOCATOR: Backend accepts custom page table allocators.
    IO_PGTABLE_CAP_CUSTOM_ALLOCATOR = BIT(0),
}

//
// struct io_pgtable_init_fns - Alloc/free a set of page tables for a
// particular format.
//
// @alloc: Allocate a set of page tables described by cfg.
// @free:  Free the page tables associated with iop.
// @caps:  Combination of @io_pgtable_caps flags encoding the backend capabilities.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_pgtable_init_fns {
    pub cookie): *mut *mut *mut *mut io_pgtable (alloc)(io_pgtable_cfg cfg, void,
    pub iop): *mut *mut void (free)(struct io_pgtable,
    pub caps: u32,
}
