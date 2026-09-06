//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/ttm/ttm_tt.h
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
// Copyright (c) 2006-2009 Vmware, Inc., Palo Alto, CA., USA
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

//
// struct ttm_tt - This is a structure holding the pages, caching- and aperture
// binding status for a buffer object that isn't backed by fixed (VRAM / AGP)
// memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_tt {
// @pages: Array of pages backing the data.
    pub pages: *mut page,
//
// @page_flags: The page flags.
//
// Supported values:
//
// TTM_TT_FLAG_SWAPPED: Set by TTM when the pages have been unpopulated
// and swapped out by TTM.  Calling ttm_tt_populate() will then swap the
// pages back in, and unset the flag. Drivers should in general never
// need to touch this.
//
// TTM_TT_FLAG_ZERO_ALLOC: Set if the pages will be zeroed on
// allocation.
//
// TTM_TT_FLAG_EXTERNAL: Set if the underlying pages were allocated
// externally, like with dma-buf or userptr. This effectively disables
// TTM swapping out such pages.  Also important is to prevent TTM from
// ever directly mapping these pages.
//
// Note that enum ttm_bo_type.ttm_bo_type_sg objects will always enable
// this flag.
//
// TTM_TT_FLAG_EXTERNAL_MAPPABLE: Same behaviour as
// TTM_TT_FLAG_EXTERNAL, but with the reduced restriction that it is
// still valid to use TTM to map the pages directly. This is useful when
// implementing a ttm_tt backend which still allocates driver owned
// pages underneath(say with shmem).
//
// Note that since this also implies TTM_TT_FLAG_EXTERNAL, the usage
// here should always be:
//
// page_flags = TTM_TT_FLAG_EXTERNAL |
// TTM_TT_FLAG_EXTERNAL_MAPPABLE;
//
// TTM_TT_FLAG_DECRYPTED: The mapped ttm pages should be marked as
// not encrypted. The framework will try to match what the dma layer
// is doing, but note that it is a little fragile because ttm page
// fault handling abuses the DMA api a bit and dma_map_attrs can't be
// used to assure pgprot always matches.
//
// TTM_TT_FLAG_BACKED_UP: TTM internal only. This is set if the
// struct ttm_tt has been (possibly partially) backed up.
//
// TTM_TT_FLAG_PRIV_POPULATED: TTM internal only. DO NOT USE. This is
// set by TTM after ttm_tt_populate() has successfully returned, and is
// then unset when TTM calls ttm_tt_unpopulate().
//

    pub page_flags: u32,
// @num_pages: Number of pages in the page array.
    pub num_pages: u32,
// @sg: for SG objects via dma-buf.
    pub sg: *mut sg_table,
// @dma_address: The DMA (bus) addresses of the pages.
    pub dma_address: *mut dma_addr_t,
// @swap_storage: Pointer to shmem struct file for swap storage.
    pub swap_storage: *mut file,
//
// @backup: Pointer to backup struct for backed up tts.
// Could be unified with @swap_storage. Meanwhile, the driver's
// ttm_tt_create() callback is responsible for assigning
// this field.
//
    pub backup: *mut file,
//
// @caching: The current caching state of the pages, see enum
// ttm_caching.
//
    pub caching: ttm_caching,
// @restore: Partial restoration from backup state. TTM private
    pub restore: *mut ttm_pool_tt_restore,
}

//
// struct ttm_kmap_iter_tt - Specialization of a mappig iterator for a tt.
// @base: Embedded struct ttm_kmap_iter providing the usage interface
// @tt: Cached struct ttm_tt.
// @prot: Cached page protection for mapping.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_kmap_iter_tt {
    pub base: ttm_kmap_iter,
    pub tt: *mut ttm_tt,
    pub prot: pgprot_t,
}

//
// ttm_tt_is_swapped() - Whether the ttm_tt is swapped out or backed up
// @tt: The struct ttm_tt.
//
// Return: true if swapped or backed up, false otherwise.
//
// ttm_tt_is_backed_up() - Whether the ttm_tt backed up
// @tt: The struct ttm_tt.
//
// Return: true if swapped or backed up, false otherwise.
//
// ttm_tt_clear_backed_up() - Clear the ttm_tt backed-up status
// @tt: The struct ttm_tt.
//
// Drivers can use this functionto clear the backed-up status,
// for example before destroying or re-validating a purged tt.
//
// ttm_tt_create
//
// @bo: pointer to a struct ttm_buffer_object
// @zero_alloc: true if allocated pages needs to be zeroed
//
// Make sure we have a TTM structure allocated for the given BO.
// No pages are actually allocated.
//
extern "C" {
    pub fn ttm_tt_create(bo: *mut ttm_buffer_object, zero_alloc: bool) -> c_int;
}
//
// ttm_tt_init
//
// @ttm: The struct ttm_tt.
// @bo: The buffer object we create the ttm for.
// @page_flags: Page flags as identified by TTM_TT_FLAG_XX flags.
// @caching: the desired caching state of the pages
// @extra_pages: Extra pages needed for the driver.
//
// Create a struct ttm_tt to back data with system memory pages.
// No pages are actually allocated.
// Returns:
// NULL: Out of memory.
//
// ttm_tt_fini
//
// @ttm: the ttm_tt structure.
//
// Free memory of ttm_tt structure
//
extern "C" {
    pub fn ttm_tt_fini(ttm: *mut ttm_tt);
}
//
// ttm_tt_destroy:
//
// @bdev: the ttm_device this object belongs to
// @ttm: The struct ttm_tt.
//
// Unbind, unpopulate and destroy common struct ttm_tt.
//
extern "C" {
    pub fn ttm_tt_destroy(bdev: *mut ttm_device, ttm: *mut ttm_tt);
}
//
// ttm_tt_swapin:
//
// @ttm: The struct ttm_tt.
//
// Swap in a previously swap out ttm_tt.
//
extern "C" {
    pub fn ttm_tt_swapin(ttm: *mut ttm_tt) -> c_int;
}
//
// ttm_tt_populate - allocate pages for a ttm
//
// @bdev: the ttm_device this object belongs to
// @ttm: Pointer to the ttm_tt structure
// @ctx: operation context for populating the tt object.
//
// Calls the driver method to allocate pages for a ttm
//
// ttm_tt_unpopulate - free pages from a ttm
//
// @bdev: the ttm_device this object belongs to
// @ttm: Pointer to the ttm_tt structure
//
// Calls the driver method to free all pages from a ttm
//
extern "C" {
    pub fn ttm_tt_unpopulate(bdev: *mut ttm_device, ttm: *mut ttm_tt);
}
//
// ttm_tt_mark_for_clear - Mark pages for clearing on populate.
//
// @ttm: Pointer to the ttm_tt structure
//
// Marks pages for clearing so that the next time the page vector is
// populated, the pages will be cleared.
//
extern "C" {
    pub fn ttm_tt_mgr_init(num_pages: c_ulong, num_dma32_pages: c_ulong);
}
extern "C" {
    pub fn ttm_tt_pages_limit() -> c_ulong;
}
//
// struct ttm_backup_flags - Flags to govern backup behaviour.
// @purge: Free pages without backing up. Bypass pools.
// @writeback: Attempt to copy contents directly to swap space, even
// if that means blocking on writes to external memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_backup_flags {
    pub 1: u32 purge :,
    pub 1: u32 writeback :,
}

extern "C" {
    pub fn ttm_tt_setup_backup(tt: *mut ttm_tt) -> c_int;
}

//
// ttm_agp_tt_create
//
// @bo: Buffer object we allocate the ttm for.
// @bridge: The agp bridge this device is sitting on.
// @page_flags: Page flags as identified by TTM_TT_FLAG_XX flags.
//
// Create a TTM backend that uses the indicated AGP bridge as an aperture
// for TT memory. This function uses the linux agpgart interface to
// bind and unbind memory backing a ttm_tt.
//
extern "C" {
    pub fn ttm_agp_bind(ttm: *mut ttm_tt, bo_mem: *mut ttm_resource) -> c_int;
}
extern "C" {
    pub fn ttm_agp_unbind(ttm: *mut ttm_tt);
}
extern "C" {
    pub fn ttm_agp_destroy(ttm: *mut ttm_tt);
}
extern "C" {
    pub fn ttm_agp_is_bound(ttm: *mut ttm_tt) -> bool;
}

