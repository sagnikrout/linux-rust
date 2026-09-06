//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/grant_table.h
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
// grant_table.h
//
// Two sets of functionality:
// 1. Granting foreign access to our memory reservation.
// 2. Accessing others' memory reservations via grant references.
// (i.e., mechanisms for both sender and recipient of grant references)
//
// Copyright (c) 2004-2005, K A Fraser
// Copyright (c) 2005, Christopher Clark
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation; or, when distributed
// separately from the Linux kernel or incorporated into other
// software packages, subject to the following license:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this source file (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

//
// Technically there's no reliably invalid grant reference or grant handle,
// so pick the value that is the most unlikely one to be observed valid.
//

// NR_GRANT_FRAMES must be less than or equal to that configured in Xen
pub const NR_GRANT_FRAMES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_free_callback {
    pub next: *mut gnttab_free_callback,
    pub ): *mut *mut void (fn)(void,
    pub arg: *mut c_void,
    pub count: u16,
}

extern "C" {
    pub fn void(result: *mut *mut gnttab_unmap_refs_done)(int, data: *mut gntab_unmap_queue_data) -> typedef;
}
extern "C" {
    pub fn gnttab_init() -> c_int;
}

extern "C" {
    pub fn gnttab_suspend() -> c_int;
}
extern "C" {
    pub fn gnttab_resume() -> c_int;
}

//
// End access through the given grant reference, iff the grant entry is no
// longer in use.  Return 1 if the grant entry was freed, 0 if it is still in
// use.
//
extern "C" {
    pub fn gnttab_end_foreign_access_ref(ref: grant_ref_t) -> c_int;
}
//
// Eventually end access through the given grant reference, and once that
// access has been ended, free the given page too.  Access will be ended
// immediately iff the grant entry is not in use, otherwise it will happen
// some time later.  page may be NULL, in which case no freeing will occur.
// Note that the granted page might still be accessed (read or write) by the
// other side after gnttab_end_foreign_access() returns, so even if page was
// specified as NULL it is not allowed to just reuse the page for other
// purposes immediately. gnttab_end_foreign_access() will take an additional
// reference to the granted page in this case, which is dropped only after
// the grant is no longer in use.
// This requires that multi page allocations for areas subject to
// gnttab_end_foreign_access() are done via alloc_pages_exact() (and freeing
// via free_pages_exact()) in order to avoid high order pages.
//
extern "C" {
    pub fn gnttab_end_foreign_access(ref: grant_ref_t, page: *mut page);
}
//
// End access through the given grant reference, iff the grant entry is
// no longer in use.  In case of success ending foreign access, the
// grant reference is deallocated.
// Return 1 if the grant entry was freed, 0 if it is still in use.
//
extern "C" {
    pub fn gnttab_try_end_foreign_access(ref: grant_ref_t) -> c_int;
}
//
// operations on reserved batches of grant references
//
extern "C" {
    pub fn gnttab_alloc_grant_references(count: u16, pprivate_head: *mut grant_ref_t) -> c_int;
}
extern "C" {
    pub fn gnttab_alloc_grant_reference_seq(count: c_uint, first: *mut grant_ref_t) -> c_int;
}
extern "C" {
    pub fn gnttab_free_grant_reference(ref: grant_ref_t);
}
extern "C" {
    pub fn gnttab_free_grant_references(head: grant_ref_t);
}
extern "C" {
    pub fn gnttab_free_grant_reference_seq(head: grant_ref_t, count: c_uint);
}
extern "C" {
    pub fn gnttab_empty_grant_references(pprivate_head: *const grant_ref_t) -> c_int;
}
extern "C" {
    pub fn gnttab_claim_grant_reference(pprivate_head: *mut grant_ref_t) -> c_int;
}
extern "C" {
    pub fn gnttab_cancel_free_callback(callback: *mut gnttab_free_callback);
}
// Give access to the first 4K of the page
extern "C" {
    pub fn arch_gnttab_init(nr_shared: c_ulong, nr_status: c_ulong) -> c_int;
}
extern "C" {
    pub fn arch_gnttab_unmap(shared: *mut c_void, nr_gframes: c_ulong);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grant_frames {
    pub pfn: *mut xen_pfn_t,
    pub count: c_uint,
    pub vaddr: *mut c_void,
}

extern "C" {
    pub fn gnttab_max_grant_frames() -> c_uint;
}
extern "C" {
    pub fn gnttab_setup_auto_xlat_frames(addr: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn gnttab_free_auto_xlat_frames();
}

extern "C" {
    pub fn gnttab_alloc_pages(nr_pages: c_int, pages: *mut page) -> c_int;
}
extern "C" {
    pub fn gnttab_free_pages(nr_pages: c_int, pages: *mut page);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_page_cache {
    pub lock: spinlock_t,

    pub pages: *mut page,

    pub pages: list_head,

    pub num_pages: c_uint,
}

extern "C" {
    pub fn gnttab_page_cache_init(cache: *mut gnttab_page_cache);
}
extern "C" {
    pub fn gnttab_page_cache_get(cache: *mut gnttab_page_cache, page: *mut page) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_dma_alloc_args {
// Device for which DMA memory will be/was allocated.
    pub dev: *mut device,
// If set then DMA buffer is coherent and write-combine otherwise.
    pub coherent: bool,
    pub nr_pages: c_int,
    pub pages: *mut page,
    pub frames: *mut xen_pfn_t,
    pub vaddr: *mut c_void,
    pub dev_bus_addr: dma_addr_t,
}

extern "C" {
    pub fn gnttab_dma_alloc_pages(args: *mut gnttab_dma_alloc_args) -> c_int;
}
extern "C" {
    pub fn gnttab_dma_free_pages(args: *mut gnttab_dma_alloc_args) -> c_int;
}

extern "C" {
    pub fn gnttab_pages_set_private(nr_pages: c_int, pages: *mut page) -> c_int;
}
extern "C" {
    pub fn gnttab_pages_clear_private(nr_pages: c_int, pages: *mut page);
}
extern "C" {
    pub fn gnttab_unmap_refs_async(item: *mut *mut gntab_unmap_queue_data);
}
extern "C" {
    pub fn gnttab_unmap_refs_sync(item: *mut gntab_unmap_queue_data) -> c_int;
}
// Perform a batch of grant map/copy operations. Retry every batch slot
// for which the hypervisor returns GNTST_eagain. This is typically due
// to paged out target frames.
//
// Will retry for 1, 2, ... 255 ms, i.e. 256 times during 32 seconds.
//
// Return value in each iand every status field of the batch guaranteed
// to not be GNTST_eagain.
//
extern "C" {
    pub fn gnttab_batch_map(batch: *mut gnttab_map_grant_ref, count: unsigned);
}
extern "C" {
    pub fn gnttab_batch_copy(batch: *mut gnttab_copy, count: unsigned);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_page_foreign {
    pub domid: domid_t,
    pub gref: grant_ref_t,
}

// Split Linux page in chunk of the size of the grant and call fn
//
// Parameters of fn:
// gfn: guest frame number
// offset: offset in the grant
// len: length of the data in the grant.
// data: internal information
//
// Helper to get to call fn only on the first "grant chunk"
// The first request is limited to the size of one grant
// Get @nr_grefs grants from an array of page and call fn for each grant
// Get the number of grant in a specified region
//
// start: Offset from the beginning of the first page
// len: total length of data (can cross multiple page)
//
extern "C" {
    pub fn XEN_PFN_UP(len: xen_offset_in_page(start) +) -> return;
}
