//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_pagemap.h
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


// SPDX-License-Identifier: MIT

//
// enum drm_interconnect_protocol - Used to identify an interconnect protocol.
//
// @DRM_INTERCONNECT_SYSTEM: DMA map is system pages
// @DRM_INTERCONNECT_DRIVER: DMA map is driver defined
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_interconnect_protocol {
    DRM_INTERCONNECT_SYSTEM,
    DRM_INTERCONNECT_DRIVER,
// A driver can add private values beyond DRM_INTERCONNECT_DRIVER
}

//
// struct drm_pagemap_addr - Address representation.
// @addr: The dma address or driver-defined address for driver private interconnects.
// @proto: The interconnect protocol.
// @order: The page order of the device mapping. (Size is PAGE_SIZE << order).
// @dir: The DMA direction.
//
// Note: There is room for improvement here. We should be able to pack into
// 64 bits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pagemap_addr {
    pub addr: dma_addr_t,
    pub 54: u64 proto :,
    pub 8: u64 order :,
    pub 2: u64 dir :,
}

//
// drm_pagemap_addr_encode() - Encode a dma address with metadata
// @addr: The dma address or driver-defined address for driver private interconnects.
// @proto: The interconnect protocol.
// @order: The page order of the dma mapping. (Size is PAGE_SIZE << order).
// @dir: The DMA direction.
//
// Return: A struct drm_pagemap_addr encoding the above information.
//
// struct drm_pagemap_ops: Ops for a drm-pagemap.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pagemap_ops {
//
// @device_map: Map for device access or provide a virtual address suitable for
//
// @dpagemap: The struct drm_pagemap for the page.
// @dev: The device mapper.
// @page: The page to map.
// @order: The page order of the device mapping. (Size is PAGE_SIZE << order).
// @dir: The transfer direction.
//
    pub dir): dma_data_direction,
//
// @device_unmap: Unmap a device address previously obtained using @device_map.
//
// @dpagemap: The struct drm_pagemap for the mapping.
// @dev: The device unmapper.
// @addr: The device address obtained when mapping.
//
    pub addr): *const drm_pagemap_addr,
//
// @populate_mm: Populate part of the mm with @dpagemap memory,
// migrating existing data.
// @dpagemap: The struct drm_pagemap managing the memory.
// @start: The virtual start address in @mm
// @end: The virtual end address in @mm
// @mm: Pointer to a live mm. The caller must have an mmget()
// reference.
//
// The caller will have the mm lock at least in read mode.
// Note that there is no guarantee that the memory is resident
// after the function returns, it's best effort only.
// When the mm is not using the memory anymore,
// it will be released. The struct drm_pagemap might have a
// mechanism in place to reclaim the memory and the data will
// then be migrated. Typically to system memory.
// The implementation should hold sufficient runtime power-
// references while pages are used in an address space and
// should ideally guard against hardware device unbind in
// a way such that device pages are migrated back to system
// followed by device page removal. The implementation should
// return -ENODEV after device removal.
//
// Return: 0 if successful. Negative error code on error.
//
    pub timeslice_ms): c_ulong,
//
// @destroy: Destroy the drm_pagemap and associated resources.
// @dpagemap: The drm_pagemap to destroy.
// @is_atomic_or_reclaim: The function may be called from
// atomic- or reclaim context.
//
// The implementation should take care not to attempt to
// destroy resources that may already have been destroyed
// using devm_ callbacks, since this function may be called
// after the underlying struct device has been unbound.
// If the implementation defers the execution to a work item
// to avoid locking issues, then it must make sure the work
// items are flushed before module exit. If the destroy call
// happens after the provider's pci_remove() callback has
// been executed, a module reference and drm device reference is
// held across the destroy callback.
//
    pub is_atomic_or_reclaim): bool,
}

//
// struct drm_pagemap: Additional information for a struct dev_pagemap
// used for device p2p handshaking.
// @ops: The struct drm_pagemap_ops.
// @ref: Reference count.
// @drm: The struct drm device owning the device-private memory.
// @pagemap: Pointer to the underlying dev_pagemap.
// @dev_hold: Pointer to a struct drm_pagemap_dev_hold for
// device referencing.
// @cache: Back-pointer to the &struct drm_pagemap_cache used for this
// &struct drm_pagemap. May be NULL if no cache is used.
// @shrink_link: Link into the shrinker's list of drm_pagemaps. Only
// used if also using a pagemap cache.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pagemap {
    pub ops: *const drm_pagemap_ops,
    pub ref: kref,
    pub drm: *mut drm_device,
    pub pagemap: *mut dev_pagemap,
    pub dev_hold: *mut drm_pagemap_dev_hold,
    pub cache: *mut drm_pagemap_cache,
    pub shrink_link: list_head,
}

//
// struct drm_pagemap_devmem_ops - Operations structure for GPU SVM device memory
//
// This structure defines the operations for GPU Shared Virtual Memory (SVM)
// device memory. These operations are provided by the GPU driver to manage device memory
// allocations and perform operations such as migration between device memory and system
// RAM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pagemap_devmem_ops {
//
// @devmem_release: Release device memory allocation (optional)
// @devmem_allocation: device memory allocation
//
// Release device memory allocation and drop a reference to device
// memory allocation.
//
    pub devmem_allocation): *mut *mut void (devmem_release)(struct drm_pagemap_devmem,
//
// @populate_devmem_pfn: Populate device memory PFN (required for migration)
// @devmem_allocation: device memory allocation
// @npages: Number of pages to populate
// @pfn: Array of page frame numbers to populate
//
// Populate device memory page frame numbers (PFN).
//
// Return: 0 on success, a negative error code on failure.
//
    pub pfn): *mut unsigned long npages, unsigned long,
//
// @copy_to_devmem: Copy to device memory (required for migration)
// @pages: Pointer to array of device memory pages (destination)
// @pagemap_addr: Pointer to array of DMA information (source)
// @npages: Number of pages to copy
// @pre_migrate_fence: dma-fence to wait for before migration start.
// May be NULL.
//
// Copy pages to device memory. If the order of a @pagemap_addr entry
// is greater than 0, the entry is populated but subsequent entries
// within the range of that order are not populated.
//
// Return: 0 on success, a negative error code on failure.
//
    pub pre_migrate_fence): *mut dma_fence,
//
// @copy_to_ram: Copy to system RAM (required for migration)
// @pages: Pointer to array of device memory pages (source)
// @pagemap_addr: Pointer to array of DMA information (destination)
// @npages: Number of pages to copy
// @pre_migrate_fence: dma-fence to wait for before migration start.
// May be NULL.
//
// Copy pages to system RAM. If the order of a @pagemap_addr entry
// is greater than 0, the entry is populated but subsequent entries
// within the range of that order are not populated.
//
// Return: 0 on success, a negative error code on failure.
//
    pub pre_migrate_fence): *mut dma_fence,
}

extern "C" {
    pub fn drm_pagemap_put(dpagemap: *mut drm_pagemap);
}

//
// drm_pagemap_get() - Obtain a reference on a struct drm_pagemap
// @dpagemap: Pointer to the struct drm_pagemap, or NULL.
//
// Return: Pointer to the struct drm_pagemap, or NULL.
//
// drm_pagemap_get_unless_zero() - Obtain a reference on a struct drm_pagemap
// unless the current reference count is zero.
// @dpagemap: Pointer to the drm_pagemap or NULL.
//
// Return: A pointer to @dpagemap if the reference count was successfully
// incremented. NULL if @dpagemap was NULL, or its refcount was 0.
//
// struct drm_pagemap_devmem - Structure representing a GPU SVM device memory allocation
//
// @dev: Pointer to the device structure which device memory allocation belongs to
// @mm: Pointer to the mm_struct for the address space
// @detached: device memory allocations is detached from device pages
// @ops: Pointer to the operations structure for GPU SVM device memory
// @dpagemap: The struct drm_pagemap of the pages this allocation belongs to.
// @size: Size of device memory allocation
// @timeslice_expiration: Timeslice expiration in jiffies
// @pre_migrate_fence: Fence to wait for or pipeline behind before migration starts.
// (May be NULL).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pagemap_devmem {
    pub dev: *mut device,
    pub mm: *mut mm_struct,
    pub detached: completion,
    pub ops: *const drm_pagemap_devmem_ops,
    pub dpagemap: *mut drm_pagemap,
    pub size: usize,
    pub timeslice_expiration: u64,
    pub pre_migrate_fence: *mut dma_fence,
}

//
// struct drm_pagemap_migrate_details - Details to govern migration.
// @timeslice_ms: The time requested for the migrated pagemap pages to
// be present in @mm before being allowed to be migrated back.
// @can_migrate_same_pagemap: Whether the copy function can migrate
// device pages within a single drm_pagemap.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pagemap_migrate_details {
    pub timeslice_ms: c_ulong,
    pub 1: u32 can_migrate_same_pagemap :,
}

extern "C" {
    pub fn drm_pagemap_evict_to_ram(devmem_allocation: *mut drm_pagemap_devmem) -> c_int;
}
extern "C" {
    pub fn drm_pagemap_destroy(dpagemap: *mut drm_pagemap, is_atomic_or_reclaim: bool);
}
extern "C" {
    pub fn drm_pagemap_reinit(dpagemap: *mut drm_pagemap) -> c_int;
}
//
// drm_pagemap_page_zone_device_data() - Page to zone_device_data
// @page: Pointer to the page
//
// Return: Page's zone_device_data
//

