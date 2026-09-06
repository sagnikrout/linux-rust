//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma-mapping.h
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
// List of possible attributes associated with a DMA mapping. The semantics
// of each attribute should be defined in Documentation/core-api/dma-attributes.rst.
//
// DMA_ATTR_WEAK_ORDERING: Specifies that reads and writes to the mapping
// may be weakly ordered, that is that reads and writes may pass each other.
//

//
// DMA_ATTR_WRITE_COMBINE: Specifies that writes to the mapping may be
// buffered to improve performance.
//

//
// DMA_ATTR_NO_KERNEL_MAPPING: Lets the platform to avoid creating a kernel
// virtual mapping for the allocated buffer.
//

//
// DMA_ATTR_SKIP_CPU_SYNC: Allows platform code to skip synchronization of
// the CPU cache for the given buffer assuming that it has been already
// transferred to 'device' domain.
//

//
// DMA_ATTR_FORCE_CONTIGUOUS: Forces contiguous allocation of the buffer
// in physical memory.
//

//
// DMA_ATTR_ALLOC_SINGLE_PAGES: This is a hint to the DMA-mapping subsystem
// that it's probably not worth the time to try to allocate memory to in a way
// that gives better TLB efficiency.
//

//
// DMA_ATTR_NO_WARN: This tells the DMA-mapping subsystem to suppress
// allocation failure reports (similarly to __GFP_NOWARN).
//

//
// DMA_ATTR_PRIVILEGED: used to indicate that the buffer is fully
// accessible at an elevated privilege level (and ideally inaccessible or
// at least read-only at lesser-privileged levels).
//

//
// DMA_ATTR_MMIO - Indicates memory-mapped I/O (MMIO) region for DMA mapping
//
// This attribute indicates the physical address is not normal system
// memory. It may not be used with kmap*()/phys_to_virt()/phys_to_page()
// functions, it may not be cacheable, and access using CPU load/store
// instructions may not be allowed.
//
// Usually this will be used to describe MMIO addresses, or other non-cacheable
// register addresses. When DMA mapping this sort of address we call
// the operation Peer to Peer as a one device is DMA'ing to another device.
// For PCI devices the p2pdma APIs must be used to determine if DMA_ATTR_MMIO
// is appropriate.
//
// For architectures that require cache flushing for DMA coherence
// DMA_ATTR_MMIO will not perform any cache flushing. The address
// provided must never be mapped cacheable into the CPU.
//

//
// DMA_ATTR_DEBUGGING_IGNORE_CACHELINES: Indicates the CPU cache line can be
// overlapped. All mappings sharing a cacheline must have this attribute for
// this to be considered safe.
//

//
// DMA_ATTR_REQUIRE_COHERENT: Indicates that DMA coherency is required.
// All mappings that carry this attribute can't work with SWIOTLB and cache
// flushing.
//

//
// DMA_ATTR_CC_SHARED: Indicates the DMA mapping is shared (decrypted) for
// confidential computing guests. For normal system memory the caller must have
// called set_memory_decrypted(), and pgprot_decrypted must be used when
// creating CPU PTEs for the mapping. The same shared semantic may be passed
// to the vIOMMU when it sets up the IOPTE. For MMIO use together with
// DMA_ATTR_MMIO to indicate shared MMIO. Unless DMA_ATTR_MMIO is provided
// a struct page is required.
//

//
// __DMA_ATTR_ALLOC_CC_SHARED: Internal DMA-mapping attribute used by
// allocation paths that create shared (decrypted) backing pages for
// confidential computing guests. Drivers must not pass this attribute to
// dma_alloc_attrs().
//

//
// A dma_addr_t can hold any valid DMA or bus address for the platform.  It can
// be given to a device to use as a DMA source or target.  It is specific to a
// given device and there may be a translation between the CPU physical address
// space and the bus address space.
//
// DMA_MAPPING_ERROR is the magic error code if a mapping failed.  It should not
// be used directly in drivers, but checked for using dma_mapping_error()
// instead.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_iova_state {
    pub addr: dma_addr_t,
    pub __size: u64,
}

//
// Use the high bit to mark if we used swiotlb for one or more ranges.
//

// Casting is needed for 32-bits systems

extern "C" {
    pub fn debug_dma_mapping_error(dev: *mut device, dma_addr: dma_addr_t);
}

extern "C" {
    pub fn dma_can_mmap(dev: *mut device) -> bool;
}
extern "C" {
    pub fn dma_pci_p2pdma_supported(dev: *mut device) -> bool;
}
extern "C" {
    pub fn dma_set_mask(dev: *mut device, mask: u64) -> c_int;
}
extern "C" {
    pub fn dma_set_coherent_mask(dev: *mut device, mask: u64) -> c_int;
}
extern "C" {
    pub fn dma_get_required_mask(dev: *mut device) -> u64;
}
extern "C" {
    pub fn dma_addressing_limited(dev: *mut device) -> bool;
}
extern "C" {
    pub fn dma_max_mapping_size(dev: *mut device) -> usize;
}
extern "C" {
    pub fn dma_opt_mapping_size(dev: *mut device) -> usize;
}
extern "C" {
    pub fn dma_get_merge_boundary(dev: *mut device) -> c_ulong;
}
extern "C" {
    pub fn dma_vunmap_noncontiguous(dev: *mut device, vaddr: *mut c_void);
}

//
// dma_use_iova - check if the IOVA API is used for this state
// @state: IOVA state
//
// Return %true if the DMA transfers uses the dma_iova_*() calls or %false if
// they can't be used.
//
extern "C" {
    pub fn dma_iova_free(dev: *mut device, state: *mut dma_iova_state);
}

extern "C" {
    pub fn __dma_need_sync(dev: *mut device, dma_addr: dma_addr_t) -> bool;
}
// Always call DMA sync operations when debugging is enabled
extern "C" {
    pub fn dma_need_unmap(dev: *mut device) -> bool;
}

// DMA must never operate on areas that might be remapped.
extern "C" {
    pub fn dma_unmap_page_attrs(_arg: dev, _arg: addr, _arg: size, _arg: dir, _arg: attrs) -> return;
}
extern "C" {
    pub fn dma_sync_single_for_cpu(_arg: dev, offset: addr +, _arg: size, _arg: dir) -> return;
}
extern "C" {
    pub fn dma_sync_single_for_device(_arg: dev, offset: addr +, _arg: size, _arg: dir) -> return;
}
//
// dma_unmap_sgtable - Unmap the given buffer for DMA
// @dev:	The device for which to perform the DMA operation
// @sgt:	The sg_table object describing the buffer
// @dir:	DMA direction
// @attrs:	Optional DMA attributes for the unmap operation
//
// Unmaps a buffer described by a scatterlist stored in the given sg_table
// object for the @dir DMA operation by the @dev device. After this function
// the ownership of the buffer is transferred back to the CPU domain.
//
// dma_sync_sgtable_for_cpu - Synchronize the given buffer for CPU access
// @dev:	The device for which to perform the DMA operation
// @sgt:	The sg_table object describing the buffer
// @dir:	DMA direction
//
// Performs the needed cache synchronization and moves the ownership of the
// buffer back to the CPU domain, so it is safe to perform any access to it
// by the CPU. Before doing any further DMA operations, one has to transfer
// the ownership of the buffer back to the DMA domain by calling the
// dma_sync_sgtable_for_device().
//
// dma_sync_sgtable_for_device - Synchronize the given buffer for DMA
// @dev:	The device for which to perform the DMA operation
// @sgt:	The sg_table object describing the buffer
// @dir:	DMA direction
//
// Performs the needed cache synchronization and moves the ownership of the
// buffer back to the DMA domain, so it is safe to perform the DMA operation.
// Once finished, one has to call dma_sync_sgtable_for_cpu() or
// dma_unmap_sgtable().
//

extern "C" {
    pub fn dma_coherent_ok(dev: *mut device, phys: phys_addr_t, size: usize) -> bool;
}
extern "C" {
    pub fn dma_free_attrs(_arg: dev, _arg: size, _arg: cpu_addr, _arg: dma_handle, _arg: 0) -> return;
}
extern "C" {
    pub fn DMA_BIT_MASK(_arg: 32) -> return;
}
//
// Set both the DMA mask and the coherent DMA mask to the same thing.
// Note that we don't check the return value from dma_set_coherent_mask()
// as the DMA API guarantees that the coherent DMA mask can be set to
// the same or smaller than the streaming DMA mask.
//
// Similar to the above, except it deals with the case where the device
// does not have dev->dma_mask appropriately setup.
//
extern "C" {
    pub fn dma_set_mask_and_coherent(_arg: dev, _arg: mask) -> return;
}
//
// dma_get_seg_boundary_nr_pages - return the segment boundary in "page" units
// @dev: device to guery the boundary for
// @page_shift: ilog() of the IOMMU page size
//
// Return the segment boundary in IOMMU page units (which may be different from
// the CPU page size) for the passed in device.
//
// If @dev is NULL a boundary of U32_MAX is assumed, this case is just for
// non-DMA API callers.
//

// Macro flag: #define ____dma_from_device_aligned

// Mark start of DMA buffer

// Mark end of DMA buffer

extern "C" {
    pub fn dma_alloc_attrs(_arg: dev, _arg: size, _arg: dma_addr, _arg: gfp, _arg: attrs) -> return;
}

// Macro flag: #define DEFINE_DMA_UNMAP_ADDR(ADDR_NAME)
// Macro flag: #define DEFINE_DMA_UNMAP_LEN(LEN_NAME)

