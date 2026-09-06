//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/ttm/ttm_bo.h
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
// Copyright (c) 2006-2009 VMware, Inc., Palo Alto, CA., USA
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Thomas Hellstrom <thellstrom-at-vmware-dot-com>
//

// Default number of pre-faulted pages in the TTM fault handler
pub const TTM_BO_VM_NUM_PREFAULT: c_int = 16;
//
// enum ttm_bo_type
//
// @ttm_bo_type_device:	These are 'normal' buffers that can
// be mmapped by user space. Each of these bos occupy a slot in the
// device address space, that can be used for normal vm operations.
//
// @ttm_bo_type_kernel: These buffers are like ttm_bo_type_device buffers,
// but they cannot be accessed from user-space. For kernel-only use.
//
// @ttm_bo_type_sg: Buffer made from dmabuf sg table shared with another
// driver.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ttm_bo_type {
    ttm_bo_type_device,
    ttm_bo_type_kernel,
    ttm_bo_type_sg
}

//
// struct ttm_buffer_object
//
// @base: drm_gem_object superclass data.
// @bdev: Pointer to the buffer object device structure.
// @type: The bo type.
// @page_alignment: Page alignment.
// @destroy: Destruction function. If NULL, kfree is used.
// @kref: Reference count of this buffer object. When this refcount reaches
// zero, the object is destroyed or put on the delayed delete list.
// @resource: structure describing current placement.
// @ttm: TTM structure holding system pages.
// @deleted: True if the object is only a zombie and already deleted.
// @bulk_move: The bulk move object.
// @priority: Priority for LRU, BOs with lower priority are evicted first.
// @pin_count: Pin count.
//
// Base class for TTM buffer object, that deals with data placement and CPU
// mappings. GPU mappings are really up to the driver, but for simpler GPUs
// the driver can usually use the placement offset @offset directly as the
// GPU virtual address. For drivers implementing multiple
// GPU memory manager contexts, the driver should manage the address space
// in these contexts separately and use these objects to get the correct
// placement and caching for these GPU maps. This makes it possible to use
// these objects for even quite elaborate memory management schemes.
// The destroy member, the API visibility of this object makes it possible
// to derive driver specific types.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_buffer_object {
    pub base: drm_gem_object,
//
// Members constant at init.
//
    pub bdev: *mut ttm_device,
    pub type: ttm_bo_type,
    pub page_alignment: u32,
    pub ): *mut *mut void (destroy) (struct ttm_buffer_object,
//
// Members not needing protection.
//
    pub kref: kref,
//
// Members protected by the bo::resv::reserved lock.
//
    pub resource: *mut ttm_resource,
    pub ttm: *mut ttm_tt,
    pub deleted: bool,
    pub bulk_move: *mut ttm_lru_bulk_move,
    pub priority: unsigned,
    pub pin_count: unsigned,
//
// @delayed_delete: Work item used when we can't delete the BO
// immediately
//
    pub delayed_delete: work_struct,
//
// @sg: external source of pages and DMA addresses, protected by the
// reservation lock.
//
    pub sg: *mut sg_table,
}

pub const TTM_BO_MAP_IOMEM_MASK: c_uint = 0x80;
//
// struct ttm_bo_kmap_obj
//
// @virtual: The current kernel virtual address.
// @page: The page when kmap'ing a single page.
// @bo_kmap_type: Type of bo_kmap.
// @bo: The TTM BO.
//
// Object describing a kernel mapping. Since a TTM bo may be located
// in various memory types with various caching policies, the
// mapping can either be an ioremap, a vmap, a kmap or part of a
// premapped region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_bo_kmap_obj {
    pub virtual: *mut c_void,
    pub page: *mut page,
    pub bo_kmap_type: },
    pub bo: *mut ttm_buffer_object,
}

//
// struct ttm_operation_ctx
//
// Context for TTM operations like changing buffer placement or general memory
// allocation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_operation_ctx {
// @interruptible: Sleep interruptible if sleeping.
    pub interruptible: bool,
// @no_wait_gpu: Return immediately if the GPU is busy.
    pub no_wait_gpu: bool,
//
// @gfp_retry_mayfail: Use __GFP_RETRY_MAYFAIL | __GFP_NOWARN
// when allocation pages. This is to avoid invoking the OOM
// killer when populating a buffer object, in order to
// forward the error for it to be dealt with.
//
    pub gfp_retry_mayfail: bool,
//
// @allow_res_evict: Allow eviction of reserved BOs. Can be used
// when multiple BOs share the same reservation object @resv.
//
    pub allow_res_evict: bool,
//
// @resv: Reservation object to be used together with
// @allow_res_evict.
//
    pub resv: *mut dma_resv,
//
// @bytes_moved: Statistics on how many bytes have been moved.
//
    pub bytes_moved: u64,
}

// struct ttm_lru_walk_ops - Operations for a LRU walk.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_lru_walk_ops {
//
// process_bo - Process this bo.
// @walk: struct ttm_lru_walk describing the walk.
// @bo: A locked and referenced buffer object.
//
// Return: Negative error code on error, User-defined positive value
// (typically, but not always, size of the processed bo) on success.
// On success, the returned values are summed by the walk and the
// walk exits when its target is met.
// 0 also indicates success, -EBUSY means this bo was skipped.
//
    pub bo): *mut *mut *mut s64 (process_bo)(struct ttm_lru_walk walk, struct ttm_buffer_object,
}

//
// struct ttm_lru_walk_arg - Common part for the variants of BO LRU walk.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_lru_walk_arg {
// @ctx: Pointer to the struct ttm_operation_ctx.
    pub ctx: *mut ttm_operation_ctx,
// @ticket: The struct ww_acquire_ctx if any.
    pub ticket: *mut ww_acquire_ctx,
// @trylock_only: Only use trylock for locking.
    pub trylock_only: bool,
//
// @sleeping_lock: Use sleeping locks even with %NULL @ticket.
// @trylock_only has precedence over this field.
//
    pub sleeping_lock: bool,
}

//
// struct ttm_lru_walk - Structure describing a LRU walk.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_lru_walk {
// @ops: Pointer to the ops structure.
    pub ops: *const ttm_lru_walk_ops,
// @arg: Common bo LRU walk arguments.
    pub arg: ttm_lru_walk_arg,
}

//
// struct ttm_bo_shrink_flags - flags to govern the bo shrinking behaviour
// @purge: Purge the content rather than backing it up.
// @writeback: Attempt to immediately write content to swap space.
// @allow_move: Allow moving to system before shrinking. This is typically
// not desired for zombie- or ghost objects (with zombie object meaning
// objects with a zero gem object refcount)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_bo_shrink_flags {
    pub 1: u32 purge :,
    pub 1: u32 writeback :,
    pub 1: u32 allow_move :,
}

extern "C" {
    pub fn ttm_bo_shrink_suitable(bo: *mut ttm_buffer_object, ctx: *mut ttm_operation_ctx) -> bool;
}
extern "C" {
    pub fn ttm_bo_shrink_avoid_wait() -> bool;
}
//
// ttm_bo_reserve:
//
// @bo: A pointer to a struct ttm_buffer_object.
// @interruptible: Sleep interruptible if waiting.
// @no_wait: Don't sleep while trying to reserve, rather return -EBUSY.
// @ticket: ticket used to acquire the ww_mutex.
//
// Locks a buffer object for validation. (Or prevents other processes from
// locking it for validation), while taking a number of measures to prevent
// deadlocks.
//
// Returns:
// -EDEADLK: The reservation may cause a deadlock.
// Release all buffer reservations, wait for @bo to become unreserved and
// try again.
// -ERESTARTSYS: A wait for the buffer to become unreserved was interrupted by
// a signal. Release all buffer reservations and return to user-space.
// -EBUSY: The function needed to sleep, but @no_wait was true
// -EALREADY: Bo already reserved using @ticket. This error code will only
// be returned if @use_ticket is set to true.
//
// ttm_bo_reserve_slowpath:
// @bo: A pointer to a struct ttm_buffer_object.
// @interruptible: Sleep interruptible if waiting.
// @ticket: Ticket used to acquire the ww_mutex.
//
// This is called after ttm_bo_reserve returns -EAGAIN and we backed off
// from all our other reservations. Because there are no other reservations
// held by us, this function cannot deadlock any more.
//
extern "C" {
    pub fn ttm_bo_move_to_lru_tail(bo: *mut ttm_buffer_object);
}
//
// ttm_bo_move_null - assign memory for a buffer object.
// @bo: The bo to assign the memory to
// @new_mem: The memory to be assigned.
//
// Assign the memory from new_mem to the memory of the buffer object bo.
//
// ttm_bo_unreserve
//
// @bo: A pointer to a struct ttm_buffer_object.
//
// Unreserve a previous reservation of @bo.
//
// ttm_kmap_obj_virtual
//
// @map: A struct ttm_bo_kmap_obj returned from ttm_bo_kmap.
// @is_iomem: Pointer to an integer that on return indicates 1 if the
// virtual map is io memory, 0 if normal memory.
//
// Returns the virtual address of a buffer object area mapped by ttm_bo_kmap.
// If *is_iomem is 1 on return, the virtual address points to an io memory area,
// that should strictly be accessed by the iowriteXX() and similar functions.
//
// is_iomem = !!(map->bo_kmap_type & TTM_BO_MAP_IOMEM_MASK);
extern "C" {
    pub fn ttm_bo_fini(bo: *mut ttm_buffer_object);
}
extern "C" {
    pub fn ttm_bo_kunmap(map: *mut ttm_bo_kmap_obj);
}
extern "C" {
    pub fn ttm_bo_vmap(bo: *mut ttm_buffer_object, map: *mut iosys_map) -> c_int;
}
extern "C" {
    pub fn ttm_bo_vunmap(bo: *mut ttm_buffer_object, map: *mut iosys_map);
}
extern "C" {
    pub fn ttm_bo_mmap_obj(vma: *mut vm_area_struct, bo: *mut ttm_buffer_object) -> c_int;
}
extern "C" {
    pub fn ttm_bo_pin(bo: *mut ttm_buffer_object);
}
extern "C" {
    pub fn ttm_bo_unpin(bo: *mut ttm_buffer_object);
}
extern "C" {
    pub fn ttm_bo_vm_fault(vmf: *mut vm_fault) -> vm_fault_t;
}
extern "C" {
    pub fn ttm_bo_vm_open(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn ttm_bo_vm_close(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn ttm_bo_vm_dummy_page(vmf: *mut vm_fault, prot: pgprot_t) -> vm_fault_t;
}
extern "C" {
    pub fn ttm_bo_unmap_virtual(bo: *mut ttm_buffer_object);
}
//
// ttm_bo_util.c
//
extern "C" {
    pub fn ttm_bo_pipeline_gutting(bo: *mut ttm_buffer_object) -> c_int;
}
extern "C" {
    pub fn ttm_bo_tt_destroy(bo: *mut ttm_buffer_object);
}
// Driver LRU walk helpers initially targeted for shrinking.
//
// struct ttm_bo_lru_cursor - Iterator cursor for TTM LRU list looping
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_bo_lru_cursor {
// @res_curs: Embedded struct ttm_resource_cursor.
    pub res_curs: ttm_resource_cursor,
//
// @bo: Buffer object pointer if a buffer object is refcounted,
// NULL otherwise.
//
    pub bo: *mut ttm_buffer_object,
//
// @needs_unlock: Valid iff @bo != NULL. The bo resv needs
// unlock before the next iteration or after loop exit.
//
    pub needs_unlock: bool,
// @arg: Pointer to common BO LRU walk arguments.
    pub arg: *mut ttm_lru_walk_arg,
}

extern "C" {
    pub fn ttm_bo_lru_cursor_fini(curs: *mut ttm_bo_lru_cursor);
}
//
// Defines needed to use autocleanup (linux/cleanup.h) with struct ttm_bo_lru_cursor.
//

//
// ttm_bo_lru_for_each_reserved_guarded() - Iterate over buffer objects owning
// resources on LRU lists.
// @_cursor: struct ttm_bo_lru_cursor to use for the iteration.
// @_man: The resource manager whose LRU lists to iterate over.
// @_arg: The struct ttm_lru_walk_arg to govern the LRU walk.
// @_bo: The struct ttm_buffer_object pointer pointing to the buffer object
// for the current iteration.
//
// Iterate over all resources of @_man and for each resource, attempt to
// reference and lock (using the locking mode detailed in @_ctx) the buffer
// object it points to. If successful, assign @_bo to the address of the
// buffer object and update @_cursor. The iteration is guarded in the
// sense that @_cursor will be initialized before looping start and cleaned
// up at looping termination, even if terminated prematurely by, for
// example a return or break statement. Exiting the loop will also unlock
// (if needed) and unreference @_bo.
//
// Return: If locking of a bo returns an error, then iteration is terminated
// and @_bo is set to a corresponding error pointer. It's illegal to
// dereference @_bo after loop exit.
//

