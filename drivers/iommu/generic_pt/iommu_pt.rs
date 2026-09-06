//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/generic_pt/iommu_pt.h
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
// "Templated C code" for implementing the iommu operations for page tables.
// This is compiled multiple times, over all the page table formats to pick up
// the per-format definitions.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommupt_pending_gather {
    pub iotlb_gather: *mut iommu_iotlb_gather,
    pub free_list: iommu_pages_list,
    pub leaf_levels_bitmap: u8,
    pub table_levels_bitmap: u8,
}

//
// If running in DMA-FQ mode then the unmap will be followed by an IOTLB
// flush all so we need to optimize by never flushing the IOTLB here.
//
// For NO_GAPS the user gets to pick if flushing all or doing micro
// flushes is better for their work load by choosing DMA vs DMA-FQ
// operation. Drivers should also see shadow_on_flush.
//
// Note that the sync frees the gather's free list, so
// we must not have any pages on that list that are
// covered by iova/len
//

// range = pt_make_range(common, iova, last);
extern "C" {
    pub fn make_range_ul(_arg: common, _arg: range, _arg: iova, _arg: len) -> return;
}
//
// Some APIs use unsigned long, while othersuse dma_addr_t as the type. Dispatch
// to the correct validation based on the type.
//

//
// The page size is limited by the domain's bitmap. This allows the core
// code to reduce the supported page sizes by changing the bitmap.
//
extern "C" {
    pub fn pt_descend(_arg: &pts, _arg: arg, _arg: descend_fn) -> return;
}
// res = pt_entry_oa_exact(&pts);
//
// iova_to_phys() - Return the output address for the given IOVA
// @domain: Table to query
// @iova: IO virtual address to query
//
// Determine the output address from the given IOVA. @iova may have any
// alignment, the returned physical will be adjusted with any sub page offset.
//
// Context: The caller must hold a read range lock that includes @iova.
//
// Return: 0 if there is no translation for the given iova.
//
// PHYS_ADDR_MAX would be a better error code
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu_dirty_args {
    pub dirty: *mut iommu_dirty_bitmap,
    pub flags: c_uint,
}

// Adjust for being contained inside a contiguous page
//
// No write log required because DMA incoherence and atomic
// dirty tracking bits can't work together
//
// read_and_clear_dirty() - Manipulate the HW set write dirty state
// @domain: Domain to manipulate
// @iova: IO virtual address to start
// @size: Length of the IOVA
// @flags: A bitmap of IOMMU_DIRTY_NO_CLEAR
// @dirty: Place to store the dirty bits
//
// Iterate over all the entries in the mapped range and record their write dirty
// status in iommu_dirty_bitmap. If IOMMU_DIRTY_NO_CLEAR is not specified then
// the entries will be left dirty, otherwise they are returned to being not
// write dirty.
//
// Context: The caller must hold a read range lock that includes @iova.
//
// Returns: -ERRNO on failure, 0 on success.
//

extern "C" {
    pub fn pt_descend(_arg: &pts, _arg: arg, _arg: __set_dirty) -> return;
}
//
// Note: There is no locking here yet, if the test suite races this it
// can crash. It should use RCU locking eventually.
//
extern "C" {
    pub fn pt_walk_range(_arg: &range, _arg: __set_dirty, _arg: NULL) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_iommu_collect_args {
    pub pending: iommupt_pending_gather,
// Fail if any OAs are within the range
    pub 1: u8 check_mapped :,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum alloc_mode {

// Allocate a table, the empty table will be ready to be installed.
    static inline struct pt_table_p *_table_alloc(struct pt_common *common,
    size_t lg2sz, gfp_t gfp,
    enum alloc_mode mode)
    {
    struct pt_iommu *iommu_table = iommu_from_common(common);
    struct pt_table_p *table_mem;

    table_mem = iommu_alloc_pages_node_sz(iommu_table->nid, gfp,
    log2_to_int(lg2sz));
    if (!table_mem)
    return ERR_PTR(-ENOMEM);

    if (pt_feature(common, PT_FEAT_DMA_INCOHERENT) &&
    mode == ALLOC_NORMAL) {
    int ret = iommu_pages_start_incoherent(
    table_mem, iommu_table->iommu_device);
    if (ret) {
    iommu_free_pages(table_mem);
    return ERR_PTR(ret);
    }
    }
    return table_mem;
    }

    static inline struct pt_table_p *table_alloc_top(struct pt_common *common,
    uintptr_t top_of_table,
    gfp_t gfp,
    enum alloc_mode mode)
    {
//
// Top doesn't need the free list or otherwise, so it technically
// doesn't need to use iommu pages. Use the API anyhow as the top is
// usually not smaller than PAGE_SIZE to keep things simple.
//
    return _table_alloc(common, pt_top_memsize_lg2(common, top_of_table),
    gfp, mode);
    }

// Allocate an interior table
    static inline struct pt_table_p *table_alloc(const struct pt_state *parent_pts,
    gfp_t gfp, enum alloc_mode mode)
    {
    struct pt_state child_pts =
    pt_init(parent_pts->range, parent_pts->level - 1, NULL);

    return _table_alloc(parent_pts->range->common,
    pt_num_items_lg2(&child_pts) +
    ilog2(PT_ITEM_WORD_SIZE),
    gfp, mode);
    }

    static inline int pt_iommu_new_table(struct pt_state *pts,
    struct pt_write_attrs *attrs)
    {
    struct pt_table_p *table_mem;
    phys_addr_t phys;

// Given PA/VA/length can't be represented
    if (PT_WARN_ON(!pt_can_have_table(pts)))
    return -ENXIO;

    table_mem = table_alloc(pts, attrs->gfp, ALLOC_NORMAL);
    if (IS_ERR(table_mem))
    return PTR_ERR(table_mem);

    phys = virt_to_phys(table_mem);
    if (!pt_install_table(pts, phys, attrs)) {
    iommu_pages_free_incoherent(
    table_mem,
    iommu_from_common(pts->range->common)->iommu_device);
    return -EAGAIN;
    }

    if (pts_feature(pts, PT_FEAT_DMA_INCOHERENT)) {
    flush_writes_item(pts);
    pt_set_sw_bit_release(pts, SW_BIT_CACHE_FLUSH_DONE);
    }

    if (IS_ENABLED(CONFIG_DEBUG_GENERIC_PT)) {
//
// The underlying table can't store the physical table address.
// This happens when kunit testing tables outside their normal
// environment where a CPU might be limited.
//
    pt_load_single_entry(pts);
    if (PT_WARN_ON(pt_table_pa(pts) != phys)) {
    pt_clear_entries(pts, ilog2(1));
    iommu_pages_free_incoherent(
    table_mem, iommu_from_common(pts->range->common)
    ->iommu_device);
    return -EINVAL;
    }
    }

    pts->table_lower = table_mem;
    return 0;
    }

    struct pt_iommu_map_args {
    struct iommu_iotlb_gather *iotlb_gather;
    struct pt_write_attrs attrs;
    pt_oaddr_t oa;
    unsigned int leaf_pgsize_lg2;
    unsigned int leaf_level;
    pt_vaddr_t num_leaves;
}

//
// This will recursively check any tables in the block to validate they are
// empty and then free them through the gather.
//
// The table item must be cleared before we can update
// the gather
//
// Need to stop in the middle of the table to change sizes
// range->va is not valid if we reached the end of the table
//
// Reached a point where the page size changed, compute the new
// parameters.
//
// Didn't finish this table level, caller will repeat it
// Descend to a child table
// EAGAIN on a race will loop again
//
// Racing with a shared pt_iommu_new_table()? The other
// thread is still flushing the cache, so we have to
// also flush it to ensure that when our thread's map
// completes all the table items leading to our mapping
// are visible.
//
// This requires the pt_set_bit_release() to be a
// release of the cache flush so that this can acquire
// visibility at the iommu.
//
// The already present table can possibly be shared with another
// concurrent map.
//
// This level is currently running __map_range_leaf() which is
// not correct if the target level has been updated to this
// level. Have the caller invoke __map_range_leaf.
//
// Fast path for the easy case of mapping a 4k page to an already allocated
// table. This is a common workload. If it returns EAGAIN run the full algorithm
// instead.
//
// No flush, not used when incoherent
extern "C" {
    pub fn pt_descend(_arg: &pts, _arg: arg, _arg: descend_fn) -> return;
}
// Something else, use the slow path
//
// Add a table to the top, increasing the top level as much as necessary to
// encompass range.
//
// The new table links to the lower table always at index 0
//
// Avoid double flushing, flush it once after all pt_install_table()
//
// top_of_table is write locked by the spinlock, but readers can use
// READ_ONCE() to get the value. Since we encode both the level and the
// pointer in one quanta the lockless reader will always see something
// valid. The HW must be updated to the new level under the spinlock
// before top_of_table is updated so that concurrent readers don't map
// into the new level until it is fully functional. If another thread
// already updated it while we were working then throw everything away
// and try again.
//
// We do not issue any flushes for change_top on the expectation that
// any walk cache will not become a problem by adding another layer to
// the tree. Misses will rewalk from the updated top pointer, hits
// continue to be correct. Negative caching is fine too since all the
// new IOVA added by the new top is non-present.
//
// Reload the new top
// range = pt_make_range(common, range->va, range->last_va);
//
// The __map_single_page() fast path does not support DMA_INCOHERENT
// flushing to keep its .text small.
//
// EAGAIN falls through to the full path
// Check the paddr doesn't exceed what the table can store
// Calculate target page size and level for the leaves
//
// Table levels were freed and replaced with large items, flush any walk
// cache that may refer to the freed levels.
//
// Bytes successfully mapped
// mapped += map.oa - paddr;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_unmap_args {
    pub pending: iommupt_pending_gather,
    pub unmapped: pt_vaddr_t,
}

//
// A starting index is in the middle of a contiguous entry
//
// The IOMMU API does not require drivers to support unmapping parts of
// large pages. Long ago VFIO would try to split maps but the current
// version never does.
//
// Instead when unmap reaches a partial unmap of the start of a large
// IOPTE it should remove the entire IOPTE and return that size to the
// caller.
//
// Micro optimization
//
// If the unmapping range fully covers the table then we
// can free it as well. The clear is delayed until we
// succeed in clearing the lower table levels.
//
// If the caller requested an last that falls within a
// single entry then the entire entry is unmapped and
// the length returned will be larger than requested.
//
// Hide page sizes larger than the maximum. -1 because a whole table
// pgsize is not allowed
//
// The driver has to already have fenced the HW access to the page table
// and invalidated any caching referring to this memory.
//

// Requested features must match features compiled into this format
//
// Check if the top level of the page table is too small to hold the
// specified maxvasz.
//
// A 64-bit high address space table on a 32-bit system cannot work.
//
// The aperture is limited to what the API can do after considering all
// the different types dma_addr_t/unsigned long/pt_vaddr_t that are used
// to store a VA. Set the aperture to something that is valid for all
// cases. Saturate instead of truncate the end if the types are smaller
// than the top range. aperture_end should be called aperture_last.
//
// The caller can initialize some of these values

extern "C" {
    pub fn PTR_ERR(_arg: table_mem) -> return;
}
// Must be last, see pt_iommu_deinit()

// For iommu_dirty_bitmap_record()
