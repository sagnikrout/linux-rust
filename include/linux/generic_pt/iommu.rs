//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/generic_pt/iommu.h
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

//
// DOC: IOMMU Radix Page Table
//
// The IOMMU implementation of the Generic Page Table provides an ops struct
// that is useful to go with an iommu_domain to serve the DMA API, IOMMUFD and
// the generic map/unmap interface.
//
// This interface uses a caller provided locking approach. The caller must have
// a VA range lock concept that prevents concurrent threads from calling ops on
// the same VA. Generally the range lock must be at least as large as a single
// map call.
//
// struct pt_iommu - Base structure for IOMMU page tables
//
// The format-specific struct will include this as the first member.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu {
//
// @domain: The core IOMMU domain. The driver should use a union to
// overlay this memory with its previously existing domain struct to
// create an alias.
//
    pub domain: iommu_domain,
//
// @ops: Function pointers to access the API
//
    pub ops: *const pt_iommu_ops,
//
// @driver_ops: Function pointers provided by the HW driver to help
// manage HW details like caches.
//
    pub driver_ops: *const pt_iommu_driver_ops,
//
// @nid: Node ID to use for table memory allocations. The IOMMU driver
// may want to set the NID to the device's NID, if there are multiple
// table walkers.
//
    pub nid: c_int,
//
// @iommu_device: Device pointer used for any DMA cache flushing when
// PT_FEAT_DMA_INCOHERENT. This is the iommu device that created the
// page table which must have dma ops that perform cache flushing.
//
    pub iommu_device: *mut device,
}

extern "C" {
    pub fn container_of(_arg: domain, pt_iommu: struct, _arg: domain) -> return;
}
//
// struct pt_iommu_info - Details about the IOMMU page table
//
// Returned from pt_iommu_ops->get_info()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu_info {
//
// @pgsize_bitmap: A bitmask where each set bit indicates
// a page size that can be natively stored in the page table.
//
    pub pgsize_bitmap: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu_ops {
//
// @map_range: Install translation for an IOVA range
// @iommu_table: Table to manipulate
// @iova: IO virtual address to start
// @paddr: Physical/Output address to start
// @len: Length of the range starting from @iova
// @prot: A bitmap of IOMMU_READ/WRITE/CACHE/NOEXEC/MMIO
// @gfp: GFP flags for any memory allocations
//
// The range starting at IOVA will have paddr installed into it. The
// rage is automatically segmented into optimally sized table entries,
// and can have any valid alignment.
//
// On error the caller will probably want to invoke unmap on the range
// from iova up to the amount indicated by @mapped to return the table
// back to an unchanged state.
//
// Context: The caller must hold a write range lock that includes
// the whole range.
//
// Returns: -ERRNO on failure, 0 on success. The number of bytes of VA
// that were mapped are added to @mapped, @mapped is not zerod first.
//
    pub mapped): *mut gfp_t gfp, size_t,
//
// @unmap_range: Make a range of IOVA empty/not present
// @iommu_table: Table to manipulate
// @iova: IO virtual address to start
// @len: Length of the range starting from @iova
// @iotlb_gather: Gather struct that must be flushed on return
//
// unmap_range() will remove a translation created by map_range(). It
// cannot subdivide a mapping created by map_range(), so it should be
// called with IOVA ranges that match those passed to map_pages. The
// IOVA range can aggregate contiguous map_range() calls so long as no
// individual range is split.
//
// Context: The caller must hold a write range lock that includes
// the whole range.
//
// Returns: Number of bytes of VA unmapped. iova + res will be the
// point unmapping stopped.
//
    pub iotlb_gather): *mut iommu_iotlb_gather,
//
// @set_dirty: Make the iova write dirty
// @iommu_table: Table to manipulate
// @iova: IO virtual address to start
//
// This is only used by iommufd testing. It makes the iova dirty so that
// read_and_clear_dirty() will see it as dirty. Unlike all the other ops
// this one is safe to call without holding any locking. It may return
// -EAGAIN if there is a race.
//
    pub iova): *mut *mut *mut int (set_dirty)(struct pt_iommu iommu_table, dma_addr_t,
//
// @get_info: Return the pt_iommu_info structure
// @iommu_table: Table to query
//
// Return some basic static information about the page table.
//
    pub info): *mut pt_iommu_info,
//
// @deinit: Undo a format specific init operation
// @iommu_table: Table to destroy
//
// Release all of the memory. The caller must have already removed the
// table from all HW access and all caches.
//
    pub iommu_table): *mut *mut void (deinit)(struct pt_iommu,
}

//
// struct pt_iommu_driver_ops - HW IOTLB cache flushing operations
//
// The IOMMU driver should implement these using container_of(iommu_table) to
// get to it's iommu_domain derived structure. All ops can be called in atomic
// contexts as they are buried under DMA API calls.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu_driver_ops {
//
// @change_top: Update the top of table pointer
// @iommu_table: Table to operate on
// @top_paddr: New CPU physical address of the top pointer
// @top_level: IOMMU PT level of the new top
//
// Called under the get_top_lock() spinlock. The driver must update all
// HW references to this domain with a new top address and
// configuration. On return mappings placed in the new top must be
// reachable by the HW.
//
// top_level encodes the level in IOMMU PT format, level 0 is the
// smallest page size increasing from there. This has to be translated
// to any HW specific format. During this call the new top will not be
// visible to any other API.
//
// This op is only used by PT_FEAT_DYNAMIC_TOP, and is required if
// enabled.
//
    pub top_level): c_uint,
//
// @get_top_lock: lock to hold when changing the table top
// @iommu_table: Table to operate on
//
// Return a lock to hold when changing the table top page table from
// being stored in HW. The lock will be held prior to calling
// change_top() and released once the top is fully visible.
//
// Typically this would be a lock that protects the iommu_domain's
// attachment list.
//
// This op is only used by PT_FEAT_DYNAMIC_TOP, and is required if
// enabled.
//
    pub iommu_table): *mut *mut *mut spinlock_t (get_top_lock)(struct pt_iommu,
}

//
// It is safe to call pt_iommu_deinit() before an init, or if init
// fails. The ops pointer will only become non-NULL if deinit needs to be
// run.
//
// struct pt_iommu_cfg - Common configuration values for all formats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu_cfg {
//
// @features: Features required. Only these features will be turned on.
// The feature list should reflect what the IOMMU HW is capable of.
//
    pub features: c_uint,
//
// @hw_max_vasz_lg2: Maximum VA the IOMMU HW can support. This will
// imply the top level of the table.
//
    pub hw_max_vasz_lg2: u8,
//
// @hw_max_oasz_lg2: Maximum OA the IOMMU HW can support. The format
// might select a lower maximum OA.
//
    pub hw_max_oasz_lg2: u8,
}

// Generate the exported function signatures from iommu_pt.h

//
// A driver uses IOMMU_PT_DOMAIN_OPS to populate the iommu_domain_ops for the
// iommu_pt
//

//
// The driver should setup its domain struct like
// union {
// struct iommu_domain domain;
// struct pt_iommu_xxx xx;
// };
// PT_IOMMU_CHECK_DOMAIN(struct mock_iommu_domain, xx.iommu, domain);
//
// Which creates an alias between driver_domain.domain and
// driver_domain.xx.iommu.domain. This is to avoid a mass rename of existing
// driver_domain.domain users.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu_amdv1_cfg {
    pub common: pt_iommu_cfg,
    pub starting_level: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu_amdv1_hw_info {
    pub host_pt_root: u64,
    pub mode: u8,
}

// amdv1_mock is used by the iommufd selftest

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu_vtdss_cfg {
    pub common: pt_iommu_cfg,
// 4 is a 57 bit 5 level table
    pub top_level: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu_vtdss_hw_info {
    pub ssptptr: u64,
    pub aw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu_riscv_64_cfg {
    pub common: pt_iommu_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu_riscv_64_hw_info {
    pub ppn: u64,
    pub fsc_iosatp_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu_x86_64_cfg {
    pub common: pt_iommu_cfg,
// 4 is a 57 bit 5 level table
    pub top_level: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu_x86_64_hw_info {
    pub gcr3_pt: u64,
    pub levels: u8,
}

