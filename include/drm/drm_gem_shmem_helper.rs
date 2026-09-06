//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_gem_shmem_helper.h
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
// struct drm_gem_shmem_object - GEM object backed by shmem
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_shmem_object {
//
// @base: Base GEM object
//
    pub base: drm_gem_object,
//
// @pages: Page table
//
    pub pages: *mut page,
//
// @pages_use_count:
//
// Reference count on the pages table.
// The pages are put when the count reaches zero.
//
    pub pages_use_count: refcount_t,
//
// @pages_pin_count:
//
// Reference count on the pinned pages table.
//
// Pages are hard-pinned and reside in memory if count
// greater than zero. Otherwise, when count is zero, the pages are
// allowed to be evicted and purged by memory shrinker.
//
    pub pages_pin_count: refcount_t,
//
// @madv: State for madvise
//
// 0 is active/inuse.
// A negative value is the object is purged.
// Positive values are driver specific and not used by the helpers.
//
    pub madv: c_int,
//
// @madv_list: List entry for madvise tracking
//
// Typically used by drivers to track purgeable objects
//
    pub madv_list: list_head,
//
// @sgt: Scatter/gather table for imported PRIME buffers
//
    pub sgt: *mut sg_table,
//
// @vaddr: Kernel virtual address of the backing memory
//
    pub vaddr: *mut c_void,
//
// @vmap_use_count:
//
// Reference count on the virtual address.
// The address are un-mapped when the count reaches zero.
//
    pub vmap_use_count: refcount_t,
//
// @pages_mark_dirty_on_put:
//
// Mark pages as dirty when they are put.
//
    pub 1: bool pages_mark_dirty_on_put :,
//
// @pages_mark_accessed_on_put:
//
// Mark pages as accessed when they are put.
//
    pub 1: bool pages_mark_accessed_on_put :,
//
// @map_wc: map object write-combined (instead of using shmem defaults).
//
    pub 1: bool map_wc :,
}

extern "C" {
    pub fn drm_gem_shmem_init(dev: *mut drm_device, shmem: *mut drm_gem_shmem_object, size: usize) -> c_int;
}
extern "C" {
    pub fn drm_gem_shmem_release(shmem: *mut drm_gem_shmem_object);
}
extern "C" {
    pub fn drm_gem_shmem_free(shmem: *mut drm_gem_shmem_object);
}
extern "C" {
    pub fn __drm_gem_shmem_free_sgt_locked(shmem: *mut drm_gem_shmem_object);
}
extern "C" {
    pub fn drm_gem_shmem_put_pages_locked(shmem: *mut drm_gem_shmem_object);
}
extern "C" {
    pub fn drm_gem_shmem_pin(shmem: *mut drm_gem_shmem_object) -> c_int;
}
extern "C" {
    pub fn drm_gem_shmem_unpin(shmem: *mut drm_gem_shmem_object);
}
extern "C" {
    pub fn drm_gem_shmem_mmap(shmem: *mut drm_gem_shmem_object, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn drm_gem_shmem_pin_locked(shmem: *mut drm_gem_shmem_object) -> c_int;
}
extern "C" {
    pub fn drm_gem_shmem_unpin_locked(shmem: *mut drm_gem_shmem_object);
}
extern "C" {
    pub fn drm_gem_shmem_madvise_locked(shmem: *mut drm_gem_shmem_object, madv: c_int) -> c_int;
}
extern "C" {
    pub fn drm_gem_shmem_purge_locked(shmem: *mut drm_gem_shmem_object);
}
//
// GEM object functions
//
// drm_gem_shmem_object_free - GEM object function for drm_gem_shmem_free()
// @obj: GEM object to free
//
// This function wraps drm_gem_shmem_free(). Drivers that employ the shmem helpers
// should use it as their &drm_gem_object_funcs.free handler.
//
// drm_gem_shmem_object_print_info() - Print &drm_gem_shmem_object info for debugfs
// @p: DRM printer
// @indent: Tab indentation level
// @obj: GEM object
//
// This function wraps drm_gem_shmem_print_info(). Drivers that employ the shmem helpers should
// use this function as their &drm_gem_object_funcs.print_info handler.
//
// drm_gem_shmem_object_pin - GEM object function for drm_gem_shmem_pin()
// @obj: GEM object
//
// This function wraps drm_gem_shmem_pin(). Drivers that employ the shmem helpers should
// use it as their &drm_gem_object_funcs.pin handler.
//
extern "C" {
    pub fn drm_gem_shmem_pin_locked(_arg: shmem) -> return;
}
//
// drm_gem_shmem_object_unpin - GEM object function for drm_gem_shmem_unpin()
// @obj: GEM object
//
// This function wraps drm_gem_shmem_unpin(). Drivers that employ the shmem helpers should
// use it as their &drm_gem_object_funcs.unpin handler.
//
// drm_gem_shmem_object_get_sg_table - GEM object function for drm_gem_shmem_get_sg_table()
// @obj: GEM object
//
// This function wraps drm_gem_shmem_get_sg_table(). Drivers that employ the shmem helpers should
// use it as their &drm_gem_object_funcs.get_sg_table handler.
//
// Returns:
// A pointer to the scatter/gather table of pinned pages or error pointer on failure.
//
extern "C" {
    pub fn drm_gem_shmem_get_sg_table(_arg: shmem) -> return;
}
//
// drm_gem_shmem_object_vmap - GEM object function for drm_gem_shmem_vmap_locked()
// @obj: GEM object
// @map: Returns the kernel virtual address of the SHMEM GEM object's backing store.
//
// This function wraps drm_gem_shmem_vmap_locked(). Drivers that employ the shmem
// helpers should use it as their &drm_gem_object_funcs.vmap handler.
//
// Returns:
// 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn drm_gem_shmem_vmap_locked(_arg: shmem, _arg: map) -> return;
}
//
// drm_gem_shmem_object_vunmap - GEM object function for drm_gem_shmem_vunmap()
// @obj: GEM object
// @map: Kernel virtual address where the SHMEM GEM object was mapped
//
// This function wraps drm_gem_shmem_vunmap_locked(). Drivers that employ the shmem
// helpers should use it as their &drm_gem_object_funcs.vunmap handler.
//
// drm_gem_shmem_object_mmap - GEM object function for drm_gem_shmem_mmap()
// @obj: GEM object
// @vma: VMA for the area to be mapped
//
// This function wraps drm_gem_shmem_mmap(). Drivers that employ the shmem helpers should
// use it as their &drm_gem_object_funcs.mmap handler.
//
// Returns:
// 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn drm_gem_shmem_mmap(_arg: shmem, _arg: vma) -> return;
}
//
// Driver ops
//
// DRM_GEM_SHMEM_DRIVER_OPS - Default shmem GEM operations
//
// This macro provides a shortcut for setting the shmem GEM operations
// in the &drm_driver structure. Drivers that do not require an s/g table
// for imported buffers should use this.
//

//
// Kunit helpers
//

extern "C" {
    pub fn drm_gem_shmem_vmap(shmem: *mut drm_gem_shmem_object, map: *mut iosys_map) -> c_int;
}
extern "C" {
    pub fn drm_gem_shmem_vunmap(shmem: *mut drm_gem_shmem_object, map: *mut iosys_map);
}
extern "C" {
    pub fn drm_gem_shmem_madvise(shmem: *mut drm_gem_shmem_object, madv: c_int) -> c_int;
}
extern "C" {
    pub fn drm_gem_shmem_purge(shmem: *mut drm_gem_shmem_object) -> c_int;
}

