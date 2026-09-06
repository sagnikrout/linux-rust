//! Automatically rewritten from C Header to Rust Module
//! Source: mm/zpdesc.h
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
// zpdesc.h: zsmalloc pool memory descriptor
//
// Written by Alex Shi <alexs@kernel.org>
// Hyeonggon Yoo <42.hyeyoo@gmail.com>
//

//
// struct zpdesc -	Memory descriptor for zsmalloc pool memory.
// @flags:		Page flags, mostly unused by zsmalloc.
// @lru:		Indirectly used by page migration.
// @movable_ops:	Used by page migration.
// @next:		Next zpdesc in a zspage in zsmalloc pool.
// @handle:		For huge zspage in zsmalloc pool.
// @zspage:		Points to the zspage this zpdesc is a part of.
// @first_obj_offset:	First object offset in zsmalloc pool.
// @_refcount:		The number of references to this zpdesc.
//
// This struct overlays struct page for now. Do not modify without a good
// understanding of the issues. In particular, do not expand into the overlap
// with memcg_data.
//
// Page flags used:
// * PG_private identifies the first component page.
// * PG_locked is used by page migration code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpdesc {
    pub flags: c_ulong,
    pub lru: list_head,
    pub movable_ops: c_ulong,
    pub next: *mut zpdesc,
    pub handle: c_ulong,
}

//
// Only the lower 24 bits are available for offset, limiting a page
// to 16 MiB. The upper 8 bits are reserved for PGTY_zsmalloc.
//
// Do not access this field directly.
// Instead, use {get,set}_first_obj_offset() helpers.
//

//
// zpdesc_page - The first struct page allocated for a zpdesc
// @zp: The zpdesc.
//
// A convenience wrapper for converting zpdesc to the first struct page of the
// underlying folio, to communicate with code not yet converted to folio or
// struct zpdesc.
//

//
// zpdesc_folio - The folio allocated for a zpdesc
// @zp: The zpdesc.
//
// Zpdescs are descriptors for zsmalloc memory. The memory itself is allocated
// as folios that contain the zsmalloc objects, and zpdesc uses specific
// fields in the first struct page of the folio - those fields are now accessed
// by struct zpdesc.
//
// It is occasionally necessary convert to back to a folio in order to
// communicate with the rest of the mm. Please use this helper function
// instead of casting yourself, as the implementation may change in the future.
//

//
// page_zpdesc - Converts from first struct page to zpdesc.
// @p: The first (either head of compound or single) page of zpdesc.
//
// A temporary wrapper to convert struct page to struct zpdesc in situations
// where we know the page is the compound head, or single order-0 page.
//
// Long-term ideally everything would work with struct zpdesc directly or go
// through folio to struct zpdesc.
//
// Return: The zpdesc which contains this page
//

extern "C" {
    pub fn folio_trylock(_arg: zpdesc_folio(zpdesc)) -> return;
}
extern "C" {
    pub fn kmap_local_page(_arg: zpdesc_page(zpdesc)) -> return;
}
extern "C" {
    pub fn page_to_pfn(_arg: zpdesc_page(zpdesc)) -> return;
}
extern "C" {
    pub fn page_zpdesc(_arg: pfn_to_page(pfn)) -> return;
}
extern "C" {
    pub fn page_zone(_arg: zpdesc_page(zpdesc)) -> return;
}
extern "C" {
    pub fn folio_test_locked(_arg: zpdesc_folio(zpdesc)) -> return;
}
