//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_gem.h
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


//
// GEM Graphics Execution Manager Driver Interfaces
//
// Copyright 1999 Precision Insight, Inc., Cedar Park, Texas.
// Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
// Copyright (c) 2009-2010, Code Aurora Forum.
// All rights reserved.
// Copyright © 2014 Intel Corporation
// Daniel Vetter <daniel.vetter@ffwll.ch>
//
// Author: Rickard E. (Rik) Faith <faith@valinux.com>
// Author: Gareth Hughes <gareth@valinux.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// VA LINUX SYSTEMS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

//
// enum drm_gem_object_status - bitmask of object state for fdinfo reporting
// @DRM_GEM_OBJECT_RESIDENT: object is resident in memory (ie. not unpinned)
// @DRM_GEM_OBJECT_PURGEABLE: object marked as purgeable by userspace
// @DRM_GEM_OBJECT_ACTIVE: object is currently used by an active submission
//
// Bitmask of status used for fdinfo memory stats, see &drm_gem_object_funcs.status
// and drm_show_fdinfo().  Note that an object can report DRM_GEM_OBJECT_PURGEABLE
// and be active or not resident, in which case drm_show_fdinfo() will not
// account for it as purgeable.  So drivers do not need to check if the buffer
// is idle and resident to return this bit, i.e. userspace can mark a buffer as
// purgeable even while it is still busy on the GPU. It will not get reported in
// the puregeable stats until it becomes idle.  The status gem object func does
// not need to consider this.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_gem_object_status {
    DRM_GEM_OBJECT_RESIDENT  = BIT(0),
    DRM_GEM_OBJECT_PURGEABLE = BIT(1),
    DRM_GEM_OBJECT_ACTIVE    = BIT(2),
}

//
// struct drm_gem_object_funcs - GEM object functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_object_funcs {
//
// @free:
//
// Deconstructor for drm_gem_objects.
//
// This callback is mandatory.
//
    pub obj): *mut *mut void (free)(struct drm_gem_object,
//
// @open:
//
// Called upon GEM handle creation.
//
// This callback is optional.
//
    pub file): *mut *mut *mut int (open)(struct drm_gem_object obj, struct drm_file,
//
// @close:
//
// Called upon GEM handle release.
//
// This callback is optional.
//
    pub file): *mut *mut *mut void (close)(struct drm_gem_object obj, struct drm_file,
//
// @print_info:
//
// If driver subclasses struct &drm_gem_object, it can implement this
// optional hook for printing additional driver specific info.
//
// drm_printf_indent() should be used in the callback passing it the
// indent argument.
//
// This callback is called from drm_gem_print_info().
//
// This callback is optional.
//
    pub obj): *const drm_gem_object,
//
// @export:
//
// Export backing buffer as a &dma_buf.
// If this is not set drm_gem_prime_export() is used.
//
// This callback is optional.
//
    pub flags): *mut *mut *mut *mut dma_buf (export)(drm_gem_object obj, int,
//
// @pin:
//
// Pin backing buffer in memory, such that dma-buf importers can
// access it. Used by the drm_gem_map_attach() helper.
//
// This callback is optional.
//
    pub obj): *mut *mut int (pin)(struct drm_gem_object,
//
// @unpin:
//
// Unpin backing buffer. Used by the drm_gem_map_detach() helper.
//
// This callback is optional.
//
    pub obj): *mut *mut void (unpin)(struct drm_gem_object,
//
// @get_sg_table:
//
// Returns a Scatter-Gather table representation of the buffer.
// Used when exporting a buffer by the drm_gem_map_dma_buf() helper.
// Releasing is done by calling dma_unmap_sg_attrs() and sg_free_table()
// in drm_gem_unmap_buf(), therefore these helpers and this callback
// here cannot be used for sg tables pointing at driver private memory
// ranges.
//
// See also drm_prime_pages_to_sg().
//
    pub obj): *mut *mut *mut sg_table (get_sg_table)(drm_gem_object,
//
// @vmap:
//
// Returns a virtual address for the buffer. Used by the
// drm_gem_dmabuf_vmap() helper. Called with a held GEM reservation
// lock.
//
// This callback is optional.
//
    pub map): *mut *mut *mut int (vmap)(struct drm_gem_object obj, struct iosys_map,
//
// @vunmap:
//
// Releases the address previously returned by @vmap. Used by the
// drm_gem_dmabuf_vunmap() helper. Called with a held GEM reservation
// lock.
//
// This callback is optional.
//
    pub map): *mut *mut *mut void (vunmap)(struct drm_gem_object obj, struct iosys_map,
//
// @mmap:
//
// Handle mmap() of the gem object, setup vma accordingly.
//
// This callback is optional.
//
// The callback is used by both drm_gem_mmap_obj() and
// drm_gem_prime_mmap().  When @mmap is present @vm_ops is not
// used, the @mmap callback must set vma->vm_ops instead.
//
    pub vma): *mut *mut *mut int (mmap)(struct drm_gem_object obj, struct vm_area_struct,
//
// @evict:
//
// Evicts gem object out from memory. Used by the drm_gem_object_evict()
// helper. Returns 0 on success, -errno otherwise. Called with a held
// GEM reservation lock.
//
// This callback is optional.
//
    pub obj): *mut *mut int (evict)(struct drm_gem_object,
//
// @status:
//
// The optional status callback can return additional object state
// which determines which stats the object is counted against.  The
// callback is called under table_lock.  Racing against object status
// change is "harmless", and the callback can expect to not race
// against object destruction.
//
// Called by drm_show_memory_stats().
//
    pub obj): *mut *mut drm_gem_object_status (status)(struct drm_gem_object,
//
// @rss:
//
// Return resident size of the object in physical memory.
//
// Called by drm_show_memory_stats().
//
    pub obj): *mut *mut size_t (rss)(struct drm_gem_object,
//
// @vm_ops:
//
// Virtual memory operations used with mmap.
//
// This is optional but necessary for mmap support.
//
    pub vm_ops: *const vm_operations_struct,
}

//
// struct drm_gem_lru - A simple LRU helper
//
// A helper for tracking GEM objects in a given state, to aid in
// driver's shrinker implementation.  Tracks the count of pages
// for lockless &shrinker.count_objects, and provides
// &drm_gem_lru_scan for driver's &shrinker.scan_objects
// implementation.
//
// Any access to this kind of object must be done with
// drm_device::gem_lru_mutex held.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_lru {
//
// @count:
//
// The total number of backing pages of the GEM objects in
// this LRU.
//
    pub count: c_long,
//
// @list:
//
// The LRU list.
//
    pub list: list_head,
}

//
// struct drm_gem_object - GEM buffer object
//
// This structure defines the generic parts for GEM buffer objects, which are
// mostly around handling mmap and userspace handles.
//
// Buffer objects are often abbreviated to BO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_object {
//
// @refcount:
//
// Reference count of this object
//
// Please use drm_gem_object_get() to acquire and drm_gem_object_put_locked()
// or drm_gem_object_put() to release a reference to a GEM
// buffer object.
//
    pub refcount: kref,
//
// @handle_count:
//
// This is the GEM file_priv handle count of this object.
//
// Each handle also holds a reference. Note that when the handle_count
// drops to 0 any global names (e.g. the id in the flink namespace) will
// be cleared.
//
// Protected by &drm_device.object_name_lock.
//
    pub handle_count: unsigned,
//
// @dev: DRM dev this object belongs to.
//
    pub dev: *mut drm_device,
//
// @filp:
//
// SHMEM file node used as backing storage for swappable buffer objects.
// GEM also supports driver private objects with driver-specific backing
// storage (contiguous DMA memory, special reserved blocks). In this
// case @filp is NULL.
//
    pub filp: *mut file,
//
// @vma_node:
//
// Mapping info for this object to support mmap. Drivers are supposed to
// allocate the mmap offset using drm_gem_create_mmap_offset(). The
// offset itself can be retrieved using drm_vma_node_offset_addr().
//
// Memory mapping itself is handled by drm_gem_mmap(), which also checks
// that userspace is allowed to access the object.
//
    pub vma_node: drm_vma_offset_node,
//
// @size:
//
// Size of the object, in bytes.  Immutable over the object's
// lifetime.
//
    pub size: usize,
//
// @name:
//
// Global name for this object, starts at 1. 0 means unnamed.
// Access is covered by &drm_device.object_name_lock. This is used by
// the GEM_FLINK and GEM_OPEN ioctls.
//
    pub name: c_int,
//
// @dma_buf:
//
// dma-buf associated with this GEM object.
//
// Pointer to the dma-buf associated with this gem object (either
// through importing or exporting). We break the resulting reference
// loop when the last gem handle for this object is released.
//
// Protected by &drm_device.object_name_lock.
//
    pub dma_buf: *mut dma_buf,
//
// @import_attach:
//
// dma-buf attachment backing this object.
//
// Any foreign dma_buf imported as a gem object has this set to the
// attachment point for the device. This is invariant over the lifetime
// of a gem object.
//
// The &drm_gem_object_funcs.free callback is responsible for
// cleaning up the dma_buf attachment and references acquired at import
// time.
//
// Note that the drm gem/prime core does not depend upon drivers setting
// this field any more. So for drivers where this doesn't make sense
// (e.g. virtual devices or a displaylink behind an usb bus) they can
// simply leave it as NULL.
//
    pub import_attach: *mut dma_buf_attachment,
//
// @resv:
//
// Pointer to reservation object associated with the this GEM object.
//
// Normally (@resv == &@_resv) except for imported GEM objects.
//
    pub resv: *mut dma_resv,
//
// @_resv:
//
// A reservation object for this GEM object.
//
// This is unused for imported GEM objects.
//
    pub _resv: dma_resv,
//
// @gpuva: Fields used by GPUVM to manage mappings pointing to this GEM object.
//
// When DRM_GPUVM_IMMEDIATE_MODE is set, this list is protected by the
// mutex. Otherwise, the list is protected by the GEMs &dma_resv lock.
//
// Note that all entries in this list must agree on whether
// DRM_GPUVM_IMMEDIATE_MODE is set.
//
// @gpuva.list: list of GPUVM mappings attached to this GEM object.
//
// Drivers should lock list accesses with either the GEMs
// &dma_resv lock (&drm_gem_object.resv) or the
// &drm_gem_object.gpuva.lock mutex.
//
    pub list: list_head,
//
// @gpuva.lock: lock protecting access to &drm_gem_object.gpuva.list
// when DRM_GPUVM_IMMEDIATE_MODE is used.
//
// Only used when DRM_GPUVM_IMMEDIATE_MODE is set. It should be
// safe to take this mutex during the fence signalling path, so
// do not allocate memory while holding this lock. Otherwise,
// the &dma_resv lock should be used.
//
    pub lock: mutex,
    pub gpuva: },
//
// @funcs:
//
// Optional GEM object functions. If this is set, it will be used instead of the
// corresponding &drm_driver GEM callbacks.
//
// New drivers should use this.
//
    pub funcs: *const drm_gem_object_funcs,
//
// @lru_node:
//
// List node in a &drm_gem_lru.
//
    pub lru_node: list_head,
//
// @lru:
//
// The current LRU list that the GEM object is on.
//
// Access to this field must be done with drm_device::gem_lru_mutex
// held.
//
    pub lru: *mut drm_gem_lru,
}

//
// DRM_GEM_FOPS - Default drm GEM file operations
//
// This macro provides a shorthand for setting the GEM file ops in the
// &file_operations structure.  If all you need are the default ops, use
// DEFINE_DRM_GEM_FOPS instead.
//

//
// DEFINE_DRM_GEM_FOPS() - macro to generate file operations for GEM drivers
// @name: name for the generated structure
//
// This macro autogenerates a suitable &struct file_operations for GEM based
// drivers, which can be assigned to &drm_driver.fops. Note that this structure
// cannot be shared between drivers, because it contains a reference to the
// current module using THIS_MODULE.
//
// Note that the declaration is already marked as static - if you need a
// non-static version of this you're probably doing it wrong and will break the
// THIS_MODULE reference by accident.
//

extern "C" {
    pub fn drm_gem_huge_mnt_create(dev: *mut drm_device, value: *const c_char) -> c_int;
}

//
// drm_gem_get_huge_mnt - Get the huge tmpfs mountpoint used by a DRM device
// @dev: DRM device
//
// This function gets the huge tmpfs mountpoint used by DRM device @dev. A huge
// tmpfs mountpoint is used instead of `shm_mnt` after a successful call to
// drm_gem_huge_mnt_create() when CONFIG_TRANSPARENT_HUGEPAGE is enabled.
//
// Returns:
// The huge tmpfs mountpoint in use, NULL otherwise.
//

extern "C" {
    pub fn drm_gem_object_release(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn drm_gem_object_free(kref: *mut kref);
}
extern "C" {
    pub fn drm_gem_private_object_fini(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn drm_gem_vm_open(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn drm_gem_vm_close(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn drm_gem_mmap(filp: *mut file, vma: *mut vm_area_struct) -> c_int;
}

//
// drm_gem_object_get - acquire a GEM buffer object reference
// @obj: GEM buffer object
//
// This function acquires an additional reference to @obj. It is illegal to
// call this without already holding a reference. No locks required.
//
// drm_gem_object_put - drop a GEM buffer object reference
// @obj: GEM buffer object
//
// This releases a reference to @obj.
//
extern "C" {
    pub fn drm_gem_handle_delete(filp: *mut drm_file, handle: u32) -> c_int;
}
extern "C" {
    pub fn drm_gem_free_mmap_offset(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn drm_gem_create_mmap_offset(obj: *mut drm_gem_object) -> c_int;
}
extern "C" {
    pub fn drm_gem_create_mmap_offset_size(obj: *mut drm_gem_object, size: usize) -> c_int;
}
extern "C" {
    pub fn drm_gem_lock(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn drm_gem_unlock(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn drm_gem_vmap(obj: *mut drm_gem_object, map: *mut iosys_map) -> c_int;
}
extern "C" {
    pub fn drm_gem_vunmap(obj: *mut drm_gem_object, map: *mut iosys_map);
}
extern "C" {
    pub fn drm_gem_lru_init(lru: *mut drm_gem_lru);
}
extern "C" {
    pub fn drm_gem_lru_remove(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn drm_gem_lru_move_tail_locked(lru: *mut drm_gem_lru, obj: *mut drm_gem_object);
}
extern "C" {
    pub fn drm_gem_lru_move_tail(lru: *mut drm_gem_lru, obj: *mut drm_gem_object);
}
extern "C" {
    pub fn drm_gem_evict_locked(obj: *mut drm_gem_object) -> c_int;
}
//
// drm_gem_object_is_shared_for_memory_stats - helper for shared memory stats
//
// This helper should only be used for fdinfo shared memory stats to determine
// if a GEM object is shared.
//
// @obj: obj in question
//
// drm_gem_is_imported() - Tests if GEM object's buffer has been imported
// @obj: the GEM object
//
// Returns:
// True if the GEM object's buffer has been imported, false otherwise
//

//
// drm_gem_gpuva_init() - initialize the gpuva list of a GEM object
// @obj: the &drm_gem_object
//
// This initializes the &drm_gem_object's &drm_gpuvm_bo list.
//
// See also drm_gem_gpuva_set_lock().
//
// drm_gem_for_each_gpuvm_bo() - iterator to walk over a list of &drm_gpuvm_bo
// @entry__: &drm_gpuvm_bo structure to assign to in each iteration step
// @obj__: the &drm_gem_object the &drm_gpuvm_bo to walk are associated with
//
// This iterator walks over all &drm_gpuvm_bo structures associated with the
// &drm_gem_object.
//

//
// drm_gem_for_each_gpuvm_bo_safe() - iterator to safely walk over a list of
// &drm_gpuvm_bo
// @entry__: &drm_gpuvm_bostructure to assign to in each iteration step
// @next__: &next &drm_gpuvm_bo to store the next step
// @obj__: the &drm_gem_object the &drm_gpuvm_bo to walk are associated with
//
// This iterator walks over all &drm_gpuvm_bo structures associated with the
// &drm_gem_object. It is implemented with list_for_each_entry_safe(), hence
// it is save against removal of elements.
//

