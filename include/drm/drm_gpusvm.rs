//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_gpusvm.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Copyright © 2024 Intel Corporation
//

//
// struct drm_gpusvm_ops - Operations structure for GPU SVM
//
// This structure defines the operations for GPU Shared Virtual Memory (SVM).
// These operations are provided by the GPU driver to manage SVM ranges and
// notifiers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpusvm_ops {
//
// @notifier_alloc: Allocate a GPU SVM notifier (optional)
//
// Allocate a GPU SVM notifier.
//
// Return: Pointer to the allocated GPU SVM notifier on success, NULL on failure.
//
    pub (*notifier_alloc)(void): *mut drm_gpusvm_notifier,
//
// @notifier_free: Free a GPU SVM notifier (optional)
// @notifier: Pointer to the GPU SVM notifier to be freed
//
// Free a GPU SVM notifier.
//
    pub notifier): *mut *mut void (notifier_free)(struct drm_gpusvm_notifier,
//
// @range_alloc: Allocate a GPU SVM range (optional)
// @gpusvm: Pointer to the GPU SVM
//
// Allocate a GPU SVM range.
//
// Return: Pointer to the allocated GPU SVM range on success, NULL on failure.
//
    pub gpusvm): *mut *mut *mut drm_gpusvm_range (range_alloc)(drm_gpusvm,
//
// @range_free: Free a GPU SVM range (optional)
// @range: Pointer to the GPU SVM range to be freed
//
// Free a GPU SVM range.
//
    pub range): *mut *mut void (range_free)(struct drm_gpusvm_range,
//
// @invalidate: Invalidate GPU SVM notifier (required)
// @gpusvm: Pointer to the GPU SVM
// @notifier: Pointer to the GPU SVM notifier
// @mmu_range: Pointer to the mmu_notifier_range structure
//
// Invalidate the GPU page tables. It can safely walk the notifier range
// RB tree/list in this function. Called while holding the notifier lock.
//
    pub mmu_range): *const mmu_notifier_range,
}

//
// struct drm_gpusvm_notifier - Structure representing a GPU SVM notifier
//
// @gpusvm: Pointer to the GPU SVM structure
// @notifier: MMU interval notifier
// @itree: Interval tree node for the notifier (inserted in GPU SVM)
// @entry: List entry to fast interval tree traversal
// @root: Cached root node of the RB tree containing ranges
// @range_list: List head containing of ranges in the same order they appear in
// interval tree. This is useful to keep iterating ranges while
// doing modifications to RB tree.
// @flags: Flags for notifier
// @flags.removed: Flag indicating whether the MMU interval notifier has been
// removed
//
// This structure represents a GPU SVM notifier.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpusvm_notifier {
    pub gpusvm: *mut drm_gpusvm,
    pub notifier: mmu_interval_notifier,
    pub itree: interval_tree_node,
    pub entry: list_head,
    pub root: rb_root_cached,
    pub range_list: list_head,
    pub 1: u32 removed :,
    pub flags: },
}

//
// struct drm_gpusvm_pages_flags - Structure representing a GPU SVM pages flags
//
// @unmapped: Flag indicating if the pages has been unmapped
// @has_devmem_pages: Flag indicating if the pages has devmem pages
// @has_dma_mapping: Flag indicating if the pages has a DMA mapping
// @__flags: Flags for pages in u16 form (used for READ_ONCE)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpusvm_pages_flags {
// All flags below must be set / cleared under notifier lock
    pub 1: u16 unmapped :,
    pub 1: u16 has_devmem_pages :,
    pub 1: u16 has_dma_mapping :,
}

//
// struct drm_gpusvm_pages - Structure representing a GPU SVM mapped pages
//
// @drm: The DRM device that owns the dma mappings
// @dma_addr: Device address array
// @dpagemap: The struct drm_pagemap of the device pages we're dma-mapping.
// Note this is assuming only one drm_pagemap per range is allowed.
// @state: DMA IOVA state for mapping.
// @state_offset: DMA IOVA offset for mapping.
// @notifier_seq: Notifier sequence number of the range's pages
// @flags: Flags for the range; see &struct drm_gpusvm_pages_flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpusvm_pages {
    pub drm: *mut drm_device,
    pub dma_addr: *mut drm_pagemap_addr,
    pub dpagemap: *mut drm_pagemap,
    pub state: dma_iova_state,
    pub state_offset: c_ulong,
    pub notifier_seq: c_ulong,
    pub flags: drm_gpusvm_pages_flags,
}

//
// struct drm_gpusvm_range_flags - Range-level GPU SVM flags
//
// @migrate_devmem: Flag indicating whether the range can be migrated to device memory
// @unmapped: Flag indicating if the range has been unmapped
// @partial_unmap: Flag indicating if the range has been partially unmapped
// @__flags: All flags in u16 form (used for READ_ONCE)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpusvm_range_flags {
// All flags below must be set upon creation
    pub 1: u16 migrate_devmem :,
// All flags below must be set / cleared under notifier lock
    pub 1: u16 unmapped :,
    pub 1: u16 partial_unmap :,
}

//
// struct drm_gpusvm_range - Structure representing a GPU SVM range
//
// @gpusvm: Pointer to the GPU SVM structure
// @notifier: Pointer to the GPU SVM notifier
// @refcount: Reference count for the range
// @itree: Interval tree node for the range (inserted in GPU SVM notifier)
// @entry: List entry to fast interval tree traversal
// @flags: Flags for range see &struct drm_gpusvm_range_flags
//
// This structure represents a GPU SVM range used for tracking memory ranges
// mapped in a DRM device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpusvm_range {
    pub gpusvm: *mut drm_gpusvm,
    pub notifier: *mut drm_gpusvm_notifier,
    pub refcount: kref,
    pub itree: interval_tree_node,
    pub entry: list_head,
    pub flags: drm_gpusvm_range_flags,
}

//
// struct drm_gpusvm - GPU SVM structure
//
// @name: Name of the GPU SVM
// @mm: Pointer to the mm_struct for the address space
// @mm_start: Start address of GPU SVM
// @mm_range: Range of the GPU SVM
// @notifier_size: Size of individual notifiers
// @ops: Pointer to the operations structure for GPU SVM
// @chunk_sizes: Pointer to the array of chunk sizes used in range allocation.
// Entries should be powers of 2 in descending order.
// @num_chunks: Number of chunks
// @notifier_lock: Read-write semaphore for protecting notifier operations
// @root: Cached root node of the Red-Black tree containing GPU SVM notifiers
// @notifier_list: list head containing of notifiers in the same order they
// appear in interval tree. This is useful to keep iterating
// notifiers while doing modifications to RB tree.
//
// This structure represents a GPU SVM (Shared Virtual Memory) used for tracking
// memory ranges mapped in a DRM (Direct Rendering Manager) device.
//
// No reference counting is provided, as this is expected to be embedded in the
// driver VM structure along with the struct drm_gpuvm, which handles reference
// counting.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpusvm {
    pub name: *const c_char,
    pub mm: *mut mm_struct,
    pub mm_start: c_ulong,
    pub mm_range: c_ulong,
    pub notifier_size: c_ulong,
    pub ops: *const drm_gpusvm_ops,
    pub chunk_sizes: *const c_ulong,
    pub num_chunks: c_int,
    pub notifier_lock: rw_semaphore,
    pub root: rb_root_cached,
    pub notifier_list: list_head,

//
// @lock_dep_map: Annotates drm_gpusvm_range_find_or_insert and
// drm_gpusvm_range_remove with a driver provided lock.
//
    pub lock_dep_map: *mut lockdep_map,

}

//
// struct drm_gpusvm_ctx - DRM GPU SVM context
//
// @device_private_page_owner: The device-private page owner to use for
// this operation
// @check_pages_threshold: Check CPU pages for present if chunk is less than or
// equal to threshold. If not present, reduce chunk
// size.
// @timeslice_ms: The timeslice MS which in minimum time a piece of memory
// remains with either exclusive GPU or CPU access.
// @in_notifier: entering from a MMU notifier
// @read_only: operating on read-only memory
// @devmem_possible: possible to use device memory
// @devmem_only: use only device memory
// @allow_mixed: Allow mixed mappings in get pages. Mixing between system and
// single dpagemap is supported, mixing between multiple dpagemap
// is unsupported.
//
// Context that is DRM GPUSVM is operating in (i.e. user arguments).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gpusvm_ctx {
    pub device_private_page_owner: *mut c_void,
    pub check_pages_threshold: c_ulong,
    pub timeslice_ms: c_ulong,
    pub :1: unsigned int in_notifier,
    pub :1: unsigned int read_only,
    pub :1: unsigned int devmem_possible,
    pub :1: unsigned int devmem_only,
    pub :1: unsigned int allow_mixed,
}

extern "C" {
    pub fn drm_gpusvm_fini(gpusvm: *mut drm_gpusvm);
}
extern "C" {
    pub fn drm_gpusvm_free(gpusvm: *mut drm_gpusvm);
}
extern "C" {
    pub fn drm_gpusvm_range_put(range: *mut drm_gpusvm_range);
}
//
// drm_gpusvm_init_pages() - Initialize a freshly allocated drm_gpusvm_pages
// @svm_pages: Pointer to the drm_gpusvm_pages to initialize.
// @drm: The DRM device that will own DMA mappings for this pages object.
//
// Drivers that embed one or more drm_gpusvm_pages in their own range
// structure must call this once on each pages instance after allocation,
// before the first drm_gpusvm_get_pages() / unmap / free.
//
// enum drm_gpusvm_scan_result - Scan result from the drm_gpusvm_scan_mm() function.
// @DRM_GPUSVM_SCAN_UNPOPULATED: At least one page was not present or inaccessible.
// @DRM_GPUSVM_SCAN_EQUAL: All pages belong to the struct dev_pagemap indicated as
// the @pagemap argument to the drm_gpusvm_scan_mm() function.
// @DRM_GPUSVM_SCAN_OTHER: All pages belong to exactly one dev_pagemap, which is
// *NOT* the @pagemap argument to the drm_gpusvm_scan_mm(). All pages belong to
// the same device private owner.
// @DRM_GPUSVM_SCAN_SYSTEM: All pages are present and system pages.
// @DRM_GPUSVM_SCAN_MIXED_DEVICE: All pages are device pages and belong to at least
// two different struct dev_pagemaps. All pages belong to the same device private
// owner.
// @DRM_GPUSVM_SCAN_MIXED: Pages are present and are a mix of system pages
// and device-private pages. All device-private pages belong to the same device
// private owner.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_gpusvm_scan_result {
    DRM_GPUSVM_SCAN_UNPOPULATED,
    DRM_GPUSVM_SCAN_EQUAL,
    DRM_GPUSVM_SCAN_OTHER,
    DRM_GPUSVM_SCAN_SYSTEM,
    DRM_GPUSVM_SCAN_MIXED_DEVICE,
    DRM_GPUSVM_SCAN_MIXED,
}

//
// drm_gpusvm_driver_set_lock() - Set the lock protecting accesses to GPU SVM
// @gpusvm: Pointer to the GPU SVM structure.
// @lock: the lock used to protect the gpuva list. The locking primitive
// must contain a dep_map field.
//
// Call this to annotate drm_gpusvm_range_find_or_insert and
// drm_gpusvm_range_remove.
//

//
// drm_gpusvm_notifier_lock() - Lock GPU SVM notifier
// @gpusvm__: Pointer to the GPU SVM structure.
//
// Abstract client usage GPU SVM notifier lock, take lock
//

//
// drm_gpusvm_notifier_unlock() - Unlock GPU SVM notifier
// @gpusvm__: Pointer to the GPU SVM structure.
//
// Abstract client usage GPU SVM notifier lock, drop lock
//

//
// drm_gpusvm_range_start() - GPU SVM range start address
// @range: Pointer to the GPU SVM range
//
// Return: GPU SVM range start address
//
// drm_gpusvm_range_end() - GPU SVM range end address
// @range: Pointer to the GPU SVM range
//
// Return: GPU SVM range end address
//
// drm_gpusvm_range_size() - GPU SVM range size
// @range: Pointer to the GPU SVM range
//
// Return: GPU SVM range size
//
extern "C" {
    pub fn drm_gpusvm_range_end(drm_gpusvm_range_start(range: range) -) -> return;
}
//
// drm_gpusvm_notifier_start() - GPU SVM notifier start address
// @notifier: Pointer to the GPU SVM notifier
//
// Return: GPU SVM notifier start address
//
// drm_gpusvm_notifier_end() - GPU SVM notifier end address
// @notifier: Pointer to the GPU SVM notifier
//
// Return: GPU SVM notifier end address
//
// drm_gpusvm_notifier_size() - GPU SVM notifier size
// @notifier: Pointer to the GPU SVM notifier
//
// Return: GPU SVM notifier size
//
// __drm_gpusvm_range_next() - Get the next GPU SVM range in the list
// @range: a pointer to the current GPU SVM range
//
// Return: A pointer to the next drm_gpusvm_range if available, or NULL if the
// current range is the last one or if the input range is NULL.
//
extern "C" {
    pub fn list_next_entry(_arg: range, _arg: entry) -> return;
}
//
// drm_gpusvm_for_each_range() - Iterate over GPU SVM ranges in a notifier
// @range__: Iterator variable for the ranges. If set, it indicates the start of
// the iterator. If NULL, call drm_gpusvm_range_find() to get the range.
// @notifier__: Pointer to the GPU SVM notifier
// @start__: Start address of the range
// @end__: End address of the range
//
// This macro is used to iterate over GPU SVM ranges in a notifier. It is safe
// to use while holding the driver SVM lock or the notifier lock.
//

//
// drm_gpusvm_for_each_range_safe() - Safely iterate over GPU SVM ranges in a notifier
// @range__: Iterator variable for the ranges
// @next__: Iterator variable for the ranges temporay storage
// @notifier__: Pointer to the GPU SVM notifier
// @start__: Start address of the range
// @end__: End address of the range
//
// This macro is used to iterate over GPU SVM ranges in a notifier while
// removing ranges from it.
//

//
// __drm_gpusvm_notifier_next() - get the next drm_gpusvm_notifier in the list
// @notifier: a pointer to the current drm_gpusvm_notifier
//
// Return: A pointer to the next drm_gpusvm_notifier if available, or NULL if
// the current notifier is the last one or if the input notifier is
// NULL.
//
extern "C" {
    pub fn list_next_entry(_arg: notifier, _arg: entry) -> return;
}
//
// drm_gpusvm_for_each_notifier() - Iterate over GPU SVM notifiers in a gpusvm
// @notifier__: Iterator variable for the notifiers
// @gpusvm__: Pointer to the GPU SVM notifier
// @start__: Start address of the notifier
// @end__: End address of the notifier
//
// This macro is used to iterate over GPU SVM notifiers in a gpusvm.
//

//
// drm_gpusvm_for_each_notifier_safe() - Safely iterate over GPU SVM notifiers in a gpusvm
// @notifier__: Iterator variable for the notifiers
// @next__: Iterator variable for the notifiers temporay storage
// @gpusvm__: Pointer to the GPU SVM notifier
// @start__: Start address of the notifier
// @end__: End address of the notifier
//
// This macro is used to iterate over GPU SVM notifiers in a gpusvm while
// removing notifiers from it.
//

