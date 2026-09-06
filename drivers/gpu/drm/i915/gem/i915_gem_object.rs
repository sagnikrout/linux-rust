//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gem/i915_gem_object.h
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
// Copyright © 2016 Intel Corporation
//

extern "C" {
    pub fn i915_gem_init__objects(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn i915_objects_module_exit();
}
extern "C" {
    pub fn i915_objects_module_init() -> c_int;
}
extern "C" {
    pub fn i915_gem_object_free(obj: *mut drm_i915_gem_object);
}
extern "C" {
    pub fn __i915_gem_object_fini(obj: *mut drm_i915_gem_object);
}
extern "C" {
    pub fn i915_gem_object_attach_phys(obj: *mut drm_i915_gem_object, align: c_int) -> c_int;
}
extern "C" {
    pub fn i915_gem_flush_free_objects(i915: *mut drm_i915_private);
}
//
// i915_gem_object_lookup_rcu - look up a temporary GEM object from its handle
// @file: DRM file private date
// @handle: userspace handle
//
// Returns:
// A pointer to the object named by the handle if such exists on @filp, NULL
// otherwise. This object is only valid whilst under the RCU read lock, and
// note carefully the object may be in the process of being destroyed.
//

extern "C" {
    pub fn idr_find(_arg: &file->object_idr, _arg: handle) -> return;
}

//
// If more than one potential simultaneous locker, assert held.
//
// Note mm list lookup is protected by
// kref_get_unless_zero().
//
extern "C" {
    pub fn __i915_gem_object_lock(_arg: obj, _arg: ww, ww->intr: ww &&) -> return;
}
extern "C" {
    pub fn __i915_gem_object_lock(_arg: obj, _arg: ww, _arg: true) -> return;
}
extern "C" {
    pub fn dma_resv_trylock(_arg: obj->base.resv) -> return;
}
extern "C" {
    pub fn ww_mutex_trylock(_arg: &obj->base.resv->lock, _arg: &ww->ctx) -> return;
}
extern "C" {
    pub fn test_bit(_arg: I915_TILING_QUIRK_BIT, _arg: &obj->flags) -> return;
}
extern "C" {
    pub fn i915_gem_object_has_struct_page(obj: *const drm_i915_gem_object) -> bool;
}
extern "C" {
    pub fn i915_gem_object_has_iomem(obj: *const drm_i915_gem_object) -> bool;
}
extern "C" {
    pub fn i915_gem_object_type_has(_arg: obj, _arg: I915_GEM_OBJECT_IS_SHRINKABLE) -> return;
}
extern "C" {
    pub fn i915_gem_object_type_has(_arg: obj, _arg: I915_GEM_OBJECT_SELF_MANAGED_SHRINK_LIST) -> return;
}
extern "C" {
    pub fn i915_gem_object_type_has(_arg: obj, _arg: I915_GEM_OBJECT_IS_PROXY) -> return;
}
extern "C" {
    pub fn i915_gem_object_type_has(_arg: obj, _arg: I915_GEM_OBJECT_NO_MMAP) -> return;
}
extern "C" {
    pub fn i915_gem_tile_height(_arg: i915_gem_object_get_tiling(obj)) -> return;
}
//
// __i915_gem_object_page_iter_get_sg - helper to find the target scatterlist
// pointer and the target page position using pgoff_t n input argument and
// i915_gem_object_page_iter
// @obj: i915 GEM buffer object
// @iter: i915 GEM buffer object page iterator
// @n: page offset
// @offset: searched physical offset,
// it will be used for returning physical page offset value
//
// Context: Takes and releases the mutex lock of the i915_gem_object_page_iter.
// Takes and releases the RCU lock to search the radix_tree of
// i915_gem_object_page_iter.
//
// Returns:
// The target scatterlist pointer and the target page position.
//
// Recommended to use wrapper macro: i915_gem_object_page_iter_get_sg()
//
// i915_gem_object_page_iter_get_sg - wrapper macro for
// __i915_gem_object_page_iter_get_sg()
// @obj: i915 GEM buffer object
// @it: i915 GEM buffer object page iterator
// @n: page offset
// @offset: searched physical offset,
// it will be used for returning physical page offset value
//
// Context: Takes and releases the mutex lock of the i915_gem_object_page_iter.
// Takes and releases the RCU lock to search the radix_tree of
// i915_gem_object_page_iter.
//
// Returns:
// The target scatterlist pointer and the target page position.
//
// In order to avoid the truncation of the input parameter, it checks the page
// offset n's type from the input parameter before calling
// __i915_gem_object_page_iter_get_sg().
//

//
// __i915_gem_object_get_sg - helper to find the target scatterlist
// pointer and the target page position using pgoff_t n input argument and
// drm_i915_gem_object. It uses an internal shmem scatterlist lookup function.
// @obj: i915 GEM buffer object
// @n: page offset
// @offset: searched physical offset,
// it will be used for returning physical page offset value
//
// It uses drm_i915_gem_object's internal shmem scatterlist lookup function as
// i915_gem_object_page_iter and calls __i915_gem_object_page_iter_get_sg().
//
// Returns:
// The target scatterlist pointer and the target page position.
//
// Recommended to use wrapper macro: i915_gem_object_get_sg()
// See also __i915_gem_object_page_iter_get_sg()
//
extern "C" {
    pub fn __i915_gem_object_page_iter_get_sg(_arg: obj, _arg: &obj->mm.get_page, _arg: n, _arg: offset) -> return;
}
//
// i915_gem_object_get_sg - wrapper macro for __i915_gem_object_get_sg()
// @obj: i915 GEM buffer object
// @n: page offset
// @offset: searched physical offset,
// it will be used for returning physical page offset value
//
// Returns:
// The target scatterlist pointer and the target page position.
//
// In order to avoid the truncation of the input parameter, it checks the page
// offset n's type from the input parameter before calling
// __i915_gem_object_get_sg().
// See also __i915_gem_object_page_iter_get_sg()
//

//
// __i915_gem_object_get_sg_dma - helper to find the target scatterlist
// pointer and the target page position using pgoff_t n input argument and
// drm_i915_gem_object. It uses an internal DMA mapped scatterlist lookup function
// @obj: i915 GEM buffer object
// @n: page offset
// @offset: searched physical offset,
// it will be used for returning physical page offset value
//
// It uses drm_i915_gem_object's internal DMA mapped scatterlist lookup function
// as i915_gem_object_page_iter and calls __i915_gem_object_page_iter_get_sg().
//
// Returns:
// The target scatterlist pointer and the target page position.
//
// Recommended to use wrapper macro: i915_gem_object_get_sg_dma()
// See also __i915_gem_object_page_iter_get_sg()
//
extern "C" {
    pub fn __i915_gem_object_page_iter_get_sg(_arg: obj, _arg: &obj->mm.get_dma_page, _arg: n, _arg: offset) -> return;
}
//
// i915_gem_object_get_sg_dma - wrapper macro for __i915_gem_object_get_sg_dma()
// @obj: i915 GEM buffer object
// @n: page offset
// @offset: searched physical offset,
// it will be used for returning physical page offset value
//
// Returns:
// The target scatterlist pointer and the target page position.
//
// In order to avoid the truncation of the input parameter, it checks the page
// offset n's type from the input parameter before calling
// __i915_gem_object_get_sg_dma().
// See also __i915_gem_object_page_iter_get_sg()
//

//
// __i915_gem_object_get_page - helper to find the target page with a page offset
// @obj: i915 GEM buffer object
// @n: page offset
//
// It uses drm_i915_gem_object's internal shmem scatterlist lookup function as
// i915_gem_object_page_iter and calls __i915_gem_object_page_iter_get_sg()
// internally.
//
// Returns:
// The target page pointer.
//
// Recommended to use wrapper macro: i915_gem_object_get_page()
// See also __i915_gem_object_page_iter_get_sg()
//
// i915_gem_object_get_page - wrapper macro for __i915_gem_object_get_page
// @obj: i915 GEM buffer object
// @n: page offset
//
// Returns:
// The target page pointer.
//
// In order to avoid the truncation of the input parameter, it checks the page
// offset n's type from the input parameter before calling
// __i915_gem_object_get_page().
// See also __i915_gem_object_page_iter_get_sg()
//

//
// __i915_gem_object_get_dirty_page - helper to find the target page with a page
// offset
// @obj: i915 GEM buffer object
// @n: page offset
//
// It works like i915_gem_object_get_page(), but it marks the returned page dirty.
//
// Returns:
// The target page pointer.
//
// Recommended to use wrapper macro: i915_gem_object_get_dirty_page()
// See also __i915_gem_object_page_iter_get_sg() and __i915_gem_object_get_page()
//
// i915_gem_object_get_dirty_page - wrapper macro for __i915_gem_object_get_dirty_page
// @obj: i915 GEM buffer object
// @n: page offset
//
// Returns:
// The target page pointer.
//
// In order to avoid the truncation of the input parameter, it checks the page
// offset n's type from the input parameter before calling
// __i915_gem_object_get_dirty_page().
// See also __i915_gem_object_page_iter_get_sg() and __i915_gem_object_get_page()
//

//
// __i915_gem_object_get_dma_address_len - helper to get bus addresses of
// targeted DMA mapped scatterlist from i915 GEM buffer object and it's length
// @obj: i915 GEM buffer object
// @n: page offset
// @len: DMA mapped scatterlist's DMA bus addresses length to return
//
// Returns:
// Bus addresses of targeted DMA mapped scatterlist
//
// Recommended to use wrapper macro: i915_gem_object_get_dma_address_len()
// See also __i915_gem_object_page_iter_get_sg() and __i915_gem_object_get_sg_dma()
//
// i915_gem_object_get_dma_address_len - wrapper macro for
// __i915_gem_object_get_dma_address_len
// @obj: i915 GEM buffer object
// @n: page offset
// @len: DMA mapped scatterlist's DMA bus addresses length to return
//
// Returns:
// Bus addresses of targeted DMA mapped scatterlist
//
// In order to avoid the truncation of the input parameter, it checks the page
// offset n's type from the input parameter before calling
// __i915_gem_object_get_dma_address_len().
// See also __i915_gem_object_page_iter_get_sg() and
// __i915_gem_object_get_dma_address_len()
//

//
// __i915_gem_object_get_dma_address - helper to get bus addresses of
// targeted DMA mapped scatterlist from i915 GEM buffer object
// @obj: i915 GEM buffer object
// @n: page offset
//
// Returns:
// Bus addresses of targeted DMA mapped scatterlis
//
// Recommended to use wrapper macro: i915_gem_object_get_dma_address()
// See also __i915_gem_object_page_iter_get_sg() and __i915_gem_object_get_sg_dma()
//
// i915_gem_object_get_dma_address - wrapper macro for
// __i915_gem_object_get_dma_address
// @obj: i915 GEM buffer object
// @n: page offset
//
// Returns:
// Bus addresses of targeted DMA mapped scatterlist
//
// In order to avoid the truncation of the input parameter, it checks the page
// offset n's type from the input parameter before calling
// __i915_gem_object_get_dma_address().
// See also __i915_gem_object_page_iter_get_sg() and
// __i915_gem_object_get_dma_address()
//

extern "C" {
    pub fn ____i915_gem_object_get_pages(obj: *mut drm_i915_gem_object) -> c_int;
}
extern "C" {
    pub fn __i915_gem_object_get_pages(obj: *mut drm_i915_gem_object) -> c_int;
}
extern "C" {
    pub fn __i915_gem_object_get_pages(_arg: obj) -> return;
}
extern "C" {
    pub fn i915_gem_object_pin_pages_unlocked(obj: *mut drm_i915_gem_object) -> c_int;
}
extern "C" {
    pub fn atomic_read(_arg: &obj->mm.pages_pin_count) -> return;
}
extern "C" {
    pub fn __i915_gem_object_put_pages(obj: *mut drm_i915_gem_object) -> c_int;
}
extern "C" {
    pub fn i915_gem_object_truncate(obj: *mut drm_i915_gem_object) -> c_int;
}
//
// i915_gem_object_pin_map - return a contiguous mapping of the entire object
// @obj: the object to map into kernel address space
// @type: the type of mapping, used to select pgprot_t
//
// Calls i915_gem_object_pin_pages() to prevent reaping of the object's
// pages and then returns a contiguous mapping of the backing storage into
// the kernel address space. Based on the @type of mapping, the PTE will be
// set to either WriteBack or WriteCombine (via pgprot_t).
//
// The caller is responsible for calling i915_gem_object_unpin_map() when the
// mapping is no longer required.
//
// Returns the pointer through which to access the mapped object, or an
// ERR_PTR() on error.
//
// i915_gem_object_unpin_map - releases an earlier mapping
// @obj: the object to unmap
//
// After pinning the object and mapping its pages, once you are finished
// with your access, call i915_gem_object_unpin_map() to release the pin
// upon the mapping. Once the pin count reaches zero, that mapping may be
// removed.
//
extern "C" {
    pub fn __i915_gem_object_release_map(obj: *mut drm_i915_gem_object);
}

extern "C" {
    pub fn i915_gem_object_has_unknown_state(obj: *mut drm_i915_gem_object) -> bool;
}
extern "C" {
    pub fn i915_gem_object_can_bypass_llc(obj: *mut drm_i915_gem_object) -> bool;
}
extern "C" {
    pub fn i915_gem_object_flush_if_display(obj: *mut drm_i915_gem_object);
}
extern "C" {
    pub fn i915_gem_object_flush_if_display_locked(obj: *mut drm_i915_gem_object);
}
extern "C" {
    pub fn i915_gem_cpu_write_needs_clflush(obj: *mut drm_i915_gem_object) -> bool;
}
extern "C" {
    pub fn i915_gem_object_make_unshrinkable(obj: *mut drm_i915_gem_object);
}
extern "C" {
    pub fn i915_gem_object_make_shrinkable(obj: *mut drm_i915_gem_object);
}
extern "C" {
    pub fn __i915_gem_object_make_shrinkable(obj: *mut drm_i915_gem_object);
}
extern "C" {
    pub fn __i915_gem_object_make_purgeable(obj: *mut drm_i915_gem_object);
}
extern "C" {
    pub fn i915_gem_object_make_purgeable(obj: *mut drm_i915_gem_object);
}
extern "C" {
    pub fn i915_gem_fence_wait_priority_display(fence: *mut dma_fence);
}
extern "C" {
    pub fn i915_gem_object_read_from_page(obj: *mut drm_i915_gem_object, offset: u64, dst: *mut c_void, size: c_int) -> c_int;
}
extern "C" {
    pub fn i915_gem_object_is_shmem(obj: *const drm_i915_gem_object) -> bool;
}
extern "C" {
    pub fn __i915_gem_free_object_rcu(head: *mut rcu_head);
}
extern "C" {
    pub fn __i915_gem_object_pages_fini(obj: *mut drm_i915_gem_object);
}
extern "C" {
    pub fn __i915_gem_free_object(obj: *mut drm_i915_gem_object);
}
extern "C" {
    pub fn i915_gem_object_evictable(obj: *mut drm_i915_gem_object) -> bool;
}
extern "C" {
    pub fn i915_gem_object_migratable(obj: *mut drm_i915_gem_object) -> bool;
}
extern "C" {
    pub fn i915_gem_object_needs_ccs_pages(obj: *mut drm_i915_gem_object) -> bool;
}
extern "C" {
    pub fn __shmem_writeback(size: usize, mapping: *mut address_space);
}

extern "C" {
    pub fn i915_gem_object_userptr_submit_init(obj: *mut drm_i915_gem_object) -> c_int;
}
extern "C" {
    pub fn i915_gem_object_userptr_submit_done(obj: *mut drm_i915_gem_object) -> c_int;
}
extern "C" {
    pub fn i915_gem_object_userptr_validate(obj: *mut drm_i915_gem_object) -> c_int;
}

