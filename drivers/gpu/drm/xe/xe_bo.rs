//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_bo.h
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
// Copyright © 2021 Intel Corporation
//

// The bits below need to be contiguous, or things break

// --

// this one is trigger internally only

pub const XE_PTE_SHIFT: c_int = 12;

pub const XE_64K_PTE_SHIFT: c_int = 16;

//
// enum xe_madv_purgeable_state - Buffer object purgeable state enumeration
//
// This enum defines the possible purgeable states for a buffer object,
// allowing userspace to provide memory usage hints to the kernel for
// better memory management under pressure.
//
// @XE_MADV_PURGEABLE_WILLNEED: The buffer object is needed and should not be purged.
// This is the default state.
// @XE_MADV_PURGEABLE_DONTNEED: The buffer object is not currently needed and can be
// purged by the kernel under memory pressure.
// @XE_MADV_PURGEABLE_PURGED: The buffer object has been purged by the kernel.
//
// Accessing a purged buffer will result in an error. Per i915 semantics,
// once purged, a BO remains permanently invalid and must be destroyed and recreated.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_madv_purgeable_state {
    XE_MADV_PURGEABLE_WILLNEED,
    XE_MADV_PURGEABLE_DONTNEED,
    XE_MADV_PURGEABLE_PURGED,
}

extern "C" {
    pub fn xe_bo_free(bo: *mut xe_bo);
}
extern "C" {
    pub fn xe_managed_bo_unpin_map_no_vm(bo: *mut xe_bo);
}
extern "C" {
    pub fn xe_managed_bo_reinit_in_vram(xe: *mut xe_device, tile: *mut xe_tile, src: *mut xe_bo) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: bo, xe_bo: struct, _arg: ttm) -> return;
}
extern "C" {
    pub fn container_of(_arg: obj, xe_bo: struct, _arg: ttm.base) -> return;
}

extern "C" {
    pub fn xe_bo_put(bo: *mut xe_bo);
}
//
// xe_bo_get_unless_zero() - Conditionally obtain a GEM object refcount on an
// xe bo
// @bo: The bo for which we want to obtain a refcount.
//
// There is a short window between where the bo's GEM object refcount reaches
// zero and where we put the final ttm_bo reference. Code in the eviction- and
// shrinking path should therefore attempt to grab a gem object reference before
// trying to use members outside of the base class ttm object. This function is
// intended for that purpose. On successful return, this function must be paired
// with an xe_bo_put().
//
// Return: @bo on success, NULL on failure.
//
extern "C" {
    pub fn xe_bo_lock(bo: *mut xe_bo, intr: bool) -> c_int;
}
extern "C" {
    pub fn xe_bo_unlock(bo: *mut xe_bo);
}
extern "C" {
    pub fn xe_bo_pin_external(bo: *mut xe_bo, in_place: bool, exec: *mut drm_exec) -> c_int;
}
extern "C" {
    pub fn xe_bo_pin(bo: *mut xe_bo, exec: *mut drm_exec) -> c_int;
}
extern "C" {
    pub fn xe_bo_unpin_external(bo: *mut xe_bo);
}
extern "C" {
    pub fn xe_bo_unpin(bo: *mut xe_bo);
}
//
// xe_bo_is_purged() - Check if buffer object has been purged
// @bo: The buffer object to check
//
// Checks if the buffer object's backing store has been discarded by the
// kernel due to memory pressure after being marked as purgeable (DONTNEED).
// Once purged, the BO cannot be restored and any attempt to use it will fail.
//
// Context: Caller must hold the BO's dma-resv lock
// Return: true if the BO has been purged, false otherwise
//
// xe_bo_madv_is_dontneed() - Check if BO is marked as DONTNEED
// @bo: The buffer object to check
//
// Checks if userspace has marked this BO as DONTNEED (i.e., its contents
// are not currently needed and can be discarded under memory pressure).
// This is used internally to decide whether a BO is eligible for purging.
//
// Context: Caller must hold the BO's dma-resv lock
// Return: true if the BO is marked DONTNEED, false otherwise
//
extern "C" {
    pub fn xe_bo_set_purgeable_state(bo: *mut xe_bo, new_state: xe_madv_purgeable_state);
}
//
// xe_bo_willneed_get_locked() - Acquire a WILLNEED holder on a BO
// @bo: Buffer object
//
// Increments willneed_count and, on a 0->1 transition, promotes the BO
// from DONTNEED to WILLNEED. PURGED is terminal and is never modified.
//
// Caller must hold the BO's dma-resv lock.
//
// Imported BOs are owned externally; do not track purgeability.
//
// xe_bo_willneed_put_locked() - Release a WILLNEED holder on a BO
// @bo: Buffer object
//
// Decrements willneed_count and, on a 1->0 transition, marks the BO
// DONTNEED only if it still has VMAs (implying all active VMAs are
// DONTNEED). If the last VMA is being removed, preserve the current BO
// state to match the previous VMA-walk semantics.
//
// PURGED is terminal and the BO state is never modified.
//
// Caller must hold the BO's dma-resv lock.
//
// xe_bo_vma_count_inc_locked() - Account a new VMA on a BO
// @bo: Buffer object
//
// Increments vma_count.
//
// Caller must hold the BO's dma-resv lock.
//
// xe_bo_vma_count_dec_locked() - Account a VMA removal on a BO
// @bo: Buffer object
//
// Decrements vma_count.
//
// Caller must hold the BO's dma-resv lock.
//
extern "C" {
    pub fn xe_bo_is_xe_bo(bo: *mut ttm_buffer_object) -> bool;
}
extern "C" {
    pub fn __xe_bo_addr(bo: *mut xe_bo, offset: u64, page_size: usize) -> dma_addr_t;
}
extern "C" {
    pub fn xe_bo_addr(bo: *mut xe_bo, offset: u64, page_size: usize) -> dma_addr_t;
}
extern "C" {
    pub fn xe_bo_addr(_arg: bo, _arg: 0, _arg: page_size) -> return;
}
//
// xe_bo_size() - Xe BO size
// @bo: The bo object.
//
// Simple helper to return Xe BO's size.
//
// Return: Xe BO's size
//
extern "C" {
    pub fn __xe_bo_ggtt_addr(_arg: bo, _arg: bo->tile->id) -> return;
}
extern "C" {
    pub fn xe_bo_vmap(bo: *mut xe_bo) -> c_int;
}
extern "C" {
    pub fn xe_bo_vunmap(bo: *mut xe_bo);
}
extern "C" {
    pub fn xe_bo_read(bo: *mut xe_bo, offset: u64, dst: *mut c_void, size: c_int) -> c_int;
}
extern "C" {
    pub fn mem_type_is_vram(mem_type: u32) -> bool;
}
extern "C" {
    pub fn xe_bo_is_vram(bo: *mut xe_bo) -> bool;
}
extern "C" {
    pub fn xe_bo_is_visible_vram(bo: *mut xe_bo) -> bool;
}
extern "C" {
    pub fn xe_bo_is_stolen(bo: *mut xe_bo) -> bool;
}
extern "C" {
    pub fn xe_bo_is_stolen_devmem(bo: *mut xe_bo) -> bool;
}
extern "C" {
    pub fn xe_bo_is_vm_bound(bo: *mut xe_bo) -> bool;
}
extern "C" {
    pub fn xe_bo_has_single_placement(bo: *mut xe_bo) -> bool;
}
extern "C" {
    pub fn vram_region_gpu_offset(res: *mut ttm_resource) -> u64;
}
extern "C" {
    pub fn xe_bo_can_migrate(bo: *mut xe_bo, mem_type: u32) -> bool;
}
extern "C" {
    pub fn xe_bo_evict(bo: *mut xe_bo, exec: *mut drm_exec) -> c_int;
}
extern "C" {
    pub fn xe_bo_evict_pinned(bo: *mut xe_bo) -> c_int;
}
extern "C" {
    pub fn xe_bo_notifier_prepare_pinned(bo: *mut xe_bo) -> c_int;
}
extern "C" {
    pub fn xe_bo_notifier_unprepare_pinned(bo: *mut xe_bo) -> c_int;
}
extern "C" {
    pub fn xe_bo_restore_pinned(bo: *mut xe_bo) -> c_int;
}
extern "C" {
    pub fn xe_bo_dma_unmap_pinned(bo: *mut xe_bo) -> c_int;
}
extern "C" {
    pub fn xe_bo_runtime_pm_release_mmap_offset(bo: *mut xe_bo);
}
extern "C" {
    pub fn xe_bo_needs_ccs_pages(bo: *mut xe_bo) -> bool;
}
extern "C" {
    pub fn xe_bo_decompress(bo: *mut xe_bo) -> c_int;
}
extern "C" {
    pub fn PAGE_ALIGN(_arg: xe_bo_size(bo)) -> return;
}
//
// xe_bo_has_valid_ccs_bb - Check if CCS's BBs were setup for the BO.
// @bo: the &xe_bo to check
//
// The CCS's BBs should only be setup by the driver VF, but it is safe
// to call this function also by non-VF driver.
//
// Return: true iff the CCS's BBs are setup, false otherwise.
//
extern "C" {
    pub fn __xe_bo_release_dummy(kref: *mut kref);
}
//
// xe_bo_put_deferred() - Put a buffer object with delayed final freeing
// @bo: The bo to put.
// @deferred: List to which to add the buffer object if we cannot put, or
// NULL if the function is to put unconditionally.
//
// Since the final freeing of an object includes both sleeping and (!)
// memory allocation in the dma_resv individualization, it's not ok
// to put an object from atomic context nor from within a held lock
// tainted by reclaim. In such situations we want to defer the final
// freeing until we've exited the restricting context, or in the worst
// case to a workqueue.
// This function either puts the object if possible without the refcount
// reaching zero, or adds it to the @deferred list if that was not possible.
// The caller needs to follow up with a call to xe_bo_put_commit() to actually
// put the bo iff this function returns true. It's safe to always
// follow up with a call to xe_bo_put_commit().
// TODO: It's TTM that is the villain here. Perhaps TTM should add an
// interface like this.
//
// Return: true if @bo was the first object put on the @freed list,
// false otherwise.
//
extern "C" {
    pub fn llist_add(_arg: &bo->freed, _arg: deferred) -> return;
}
extern "C" {
    pub fn xe_bo_put_commit(deferred: *mut llist_head);
}
//
// xe_bo_put_async() - Put BO async
// @bo: The bo to put.
//
// Put BO async, the final put is deferred to a worker to exit an IRQ context.
//
extern "C" {
    pub fn xe_bo_dev_init(bo_device: *mut xe_bo_dev);
}
extern "C" {
    pub fn xe_bo_dev_fini(bo_device: *mut xe_bo_dev);
}
//
// xe_bo_sg_is_contiguous() - Check if a BO's DMA address space is contiguous.
// @bo: the BO to check (must have a valid sg table, i.e. !xe_bo_is_vram())
// @len: required contiguous length in bytes
//
// Returns true if the first @len bytes of the BO are mapped to a contiguous
// DMA address range.
//
// xe_sg_segment_size() - Provides upper limit for sg segment size.
// @dev: device pointer
//
// Returns the maximum segment size for the 'struct scatterlist'
// elements.
//
// The iommu_dma_map_sg() function ensures iova allocation doesn't
// cross dma segment boundary. It does so by padding some sg elements.
// This can cause overflow, ending up with sg->length being set to 0.
// Avoid this by ensuring maximum segment size is half of 'max'
// rounded down to PAGE_SIZE.
//
extern "C" {
    pub fn round_down(2: max /, _arg: PAGE_SIZE) -> return;
}
//
// struct xe_bo_shrink_flags - flags governing the shrink behaviour.
// @purge: Only purging allowed. Don't shrink if bo not purgeable.
// @writeback: Attempt to immediately move content to swap.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_bo_shrink_flags {
    pub 1: u32 purge :,
    pub 1: u32 writeback :,
}

//
// xe_bo_is_mem_type - Whether the bo currently resides in the given
// TTM memory type
// @bo: The bo to check.
// @mem_type: The TTM memory type.
//
// Return: true iff the bo resides in @mem_type, false otherwise.
//
