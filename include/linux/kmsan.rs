//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kmsan.h
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
// KMSAN API for subsystems.
//
// Copyright (C) 2017-2022 Google LLC
// Author: Alexander Potapenko <glider@google.com>
//

//
// kmsan_task_create() - Initialize KMSAN state for the task.
// @task: task to initialize.
//
extern "C" {
    pub fn kmsan_task_create(task: *mut task_struct);
}
//
// kmsan_task_exit() - Notify KMSAN that a task has exited.
// @task: task about to finish.
//
extern "C" {
    pub fn kmsan_task_exit(task: *mut task_struct);
}
//
// kmsan_init_shadow() - Initialize KMSAN shadow at boot time.
//
// Allocate and initialize KMSAN metadata for early allocations.
//
extern "C" {
    pub fn kmsan_init_shadow() -> void __init;
}
//
// kmsan_init_runtime() - Initialize KMSAN state and enable KMSAN.
//
extern "C" {
    pub fn kmsan_init_runtime() -> void __init;
}
//
// kmsan_memblock_free_pages() - handle freeing of memblock pages.
// @page:	struct page to free.
// @order:	order of @page.
//
// Freed pages are either returned to buddy allocator or held back to be used
// as metadata pages.
//
// kmsan_alloc_page() - Notify KMSAN about an alloc_pages() call.
// @page:  struct page pointer returned by alloc_pages().
// @order: order of allocated struct page.
// @flags: GFP flags used by alloc_pages()
//
// KMSAN marks 1<<@order pages starting at @page as uninitialized, unless
// @flags contain __GFP_ZERO.
//
extern "C" {
    pub fn kmsan_alloc_page(page: *mut page, order: c_uint, flags: gfp_t);
}
//
// kmsan_free_page() - Notify KMSAN about a free_pages() call.
// @page:  struct page pointer passed to free_pages().
// @order: order of deallocated struct page.
//
// KMSAN marks freed memory as uninitialized.
//
extern "C" {
    pub fn kmsan_free_page(page: *mut page, order: c_uint);
}
//
// kmsan_copy_page_meta() - Copy KMSAN metadata between two pages.
// @dst: destination page.
// @src: source page.
//
// KMSAN copies the contents of metadata pages for @src into the metadata pages
// for @dst. If @dst has no associated metadata pages, nothing happens.
// If @src has no associated metadata pages, @dst metadata pages are unpoisoned.
//
extern "C" {
    pub fn kmsan_copy_page_meta(dst: *mut page, src: *mut page);
}
//
// kmsan_slab_alloc() - Notify KMSAN about a slab allocation.
// @s:      slab cache the object belongs to.
// @object: object pointer.
// @flags:  GFP flags passed to the allocator.
//
// Depending on cache flags and GFP flags, KMSAN sets up the metadata of the
// newly created object, marking it as initialized or uninitialized.
//
extern "C" {
    pub fn kmsan_slab_alloc(s: *mut kmem_cache, object: *mut c_void, flags: gfp_t);
}
//
// kmsan_slab_free() - Notify KMSAN about a slab deallocation.
// @s:      slab cache the object belongs to.
// @object: object pointer.
//
// KMSAN marks the freed object as uninitialized.
//
extern "C" {
    pub fn kmsan_slab_free(s: *mut kmem_cache, object: *mut c_void);
}
//
// kmsan_kmalloc_large() - Notify KMSAN about a large slab allocation.
// @ptr:   object pointer.
// @size:  object size.
// @flags: GFP flags passed to the allocator.
//
// Similar to kmsan_slab_alloc(), but for large allocations.
//
extern "C" {
    pub fn kmsan_kmalloc_large(ptr: *const c_void, size: usize, flags: gfp_t);
}
//
// kmsan_kfree_large() - Notify KMSAN about a large slab deallocation.
// @ptr: object pointer.
//
// Similar to kmsan_slab_free(), but for large allocations.
//
extern "C" {
    pub fn kmsan_kfree_large(ptr: *const c_void);
}
//
// kmsan_map_kernel_range_noflush() - Notify KMSAN about a vmap.
// @start:	start of vmapped range.
// @end:	end of vmapped range.
// @prot:	page protection flags used for vmap.
// @pages:	array of pages.
// @page_shift:	page_shift passed to vmap_range_noflush().
// @gfp_mask:	gfp_mask to use internally.
//
// KMSAN maps shadow and origin pages of @pages into contiguous ranges in
// vmalloc metadata address range. Returns 0 on success, callers must check
// for non-zero return value.
//
// kmsan_vunmap_kernel_range_noflush() - Notify KMSAN about a vunmap.
// @start: start of vunmapped range.
// @end:   end of vunmapped range.
//
// KMSAN unmaps the contiguous metadata ranges created by
// kmsan_map_kernel_range_noflush().
//
extern "C" {
    pub fn kmsan_vunmap_range_noflush(start: c_ulong, end: c_ulong);
}
//
// kmsan_ioremap_page_range() - Notify KMSAN about a ioremap_page_range() call.
// @addr:	range start.
// @end:	range end.
// @phys_addr:	physical range start.
// @prot:	page protection flags used for ioremap_page_range().
// @page_shift:	page_shift argument passed to vmap_range_noflush().
//
// KMSAN creates new metadata pages for the physical pages mapped into the
// virtual memory. Returns 0 on success, callers must check for non-zero return
// value.
//
// kmsan_iounmap_page_range() - Notify KMSAN about a iounmap_page_range() call.
// @start: range start.
// @end:   range end.
//
// KMSAN unmaps the metadata pages for the given range and, unlike for
// vunmap_page_range(), also deallocates them.
//
extern "C" {
    pub fn kmsan_iounmap_page_range(start: c_ulong, end: c_ulong);
}
//
// kmsan_handle_dma() - Handle a DMA data transfer.
// @phys:   physical address of the buffer.
// @size:   buffer size.
// @dir:    one of possible dma_data_direction values.
//
// Depending on @direction, KMSAN:
// * checks the buffer, if it is copied to device;
// * initializes the buffer, if it is copied from device;
// * does both, if this is a DMA_BIDIRECTIONAL transfer.
//
// kmsan_handle_dma_sg() - Handle a DMA transfer using scatterlist.
// @sg:    scatterlist holding DMA buffers.
// @nents: number of scatterlist entries.
// @dir:   one of possible dma_data_direction values.
//
// Depending on @direction, KMSAN:
// * checks the buffers in the scatterlist, if they are copied to device;
// * initializes the buffers, if they are copied from device;
// * does both, if this is a DMA_BIDIRECTIONAL transfer.
//
// kmsan_handle_urb() - Handle a USB data transfer.
// @urb:    struct urb pointer.
// @is_out: data transfer direction (true means output to hardware).
//
// If @is_out is true, KMSAN checks the transfer buffer of @urb. Otherwise,
// KMSAN initializes the transfer buffer.
//
extern "C" {
    pub fn kmsan_handle_urb(urb: *const urb, is_out: bool);
}
//
// kmsan_unpoison_entry_regs() - Handle pt_regs in low-level entry code.
// @regs:	struct pt_regs pointer received from assembly code.
//
// KMSAN unpoisons the contents of the passed pt_regs, preventing potential
// false positive reports. Unlike kmsan_unpoison_memory(),
// kmsan_unpoison_entry_regs() can be called from the regions where
// kmsan_in_runtime() returns true, which is the case in early entry code.
//
extern "C" {
    pub fn kmsan_unpoison_entry_regs(regs: *const pt_regs);
}
//
// kmsan_get_metadata() - Return a pointer to KMSAN shadow or origins.
// @addr:      kernel address.
// @is_origin: whether to return origins or shadow.
//
// Return NULL if metadata cannot be found.
//
// kmsan_enable_current(): Enable KMSAN for the current task.
//
// Each kmsan_enable_current() current call must be preceded by a
// kmsan_disable_current() call. These call pairs may be nested.
//
extern "C" {
    pub fn kmsan_enable_current();
}
//
// kmsan_disable_current(): Disable KMSAN for the current task.
//
// Each kmsan_disable_current() current call must be followed by a
// kmsan_enable_current() call. These call pairs may be nested.
//
extern "C" {
    pub fn kmsan_disable_current();
}
//
// memset_no_sanitize_memory(): Fill memory without KMSAN instrumentation.
// @s: address of kernel memory to fill.
// @c: constant byte to fill the memory with.
// @n: number of bytes to fill.
//
// This is like memset(), but without KMSAN instrumentation.
//
extern "C" {
    pub fn __memset(_arg: s, _arg: c, _arg: n) -> return;
}
//
// KMSAN performs a lot of consistency checks that are currently enabled by
// default. BUG_ON is normally discouraged in the kernel, unless used for
// debugging, but KMSAN itself is a debugging tool, so it makes little sense to
// recover if something goes wrong.
//

// Can't call panic() here because */ \
// of uaccess checks. */              \

extern "C" {
    pub fn memset(_arg: s, _arg: c, _arg: n) -> return;
}

