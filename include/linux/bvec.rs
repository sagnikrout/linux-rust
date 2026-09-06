//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bvec.h
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
// bvec iterator
//
// Copyright (C) 2001 Ming Lei <ming.lei@canonical.com>
//

//
// struct bio_vec - a contiguous range of physical memory addresses
// @bv_page:   First page associated with the address range.
// @bv_len:    Number of bytes in the address range.
// @bv_offset: Start of the address range relative to the start of @bv_page.
//
// All pages within a bio_vec starting from @bv_page are contiguous and
// can simply be iterated (see bvec_advance()).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio_vec {
    pub bv_page: *mut page,
    pub bv_len: c_uint,
    pub bv_offset: c_uint,
}

//
// bvec_set_page - initialize a bvec based off a struct page
// @bv:		bvec to initialize
// @page:	page the bvec should point to
// @len:	length of the bvec
// @offset:	offset into the page
//
// bvec_set_folio - initialize a bvec based off a struct folio
// @bv:		bvec to initialize
// @folio:	folio the bvec should point to
// @len:	length of the bvec
// @offset:	offset into the folio
//
// bvec_set_virt - initialize a bvec based on a virtual address
// @bv:		bvec to initialize
// @vaddr:	virtual address to set the bvec to
// @len:	length of the bvec
//
// bvec_folio - Return the first folio referenced by this bvec
// @bv: bvec to access
//
// A bvec can contain non-folio memory, so this should only be called by
// the creator of the bvec; drivers have no business looking at the owner
// of the memory.  It may not even be the right interface for the caller
// to use as a bvec can span multiple folios.  You may be better off using
// something like bio_for_each_folio_all() which iterates over all folios.
//
extern "C" {
    pub fn page_folio(_arg: bv->bv_page) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bvec_iter {
//
// Current device address in 512 byte sectors. Only updated by the bio
// iter wrappers and not the bvec iterator helpers themselves.
//
    pub bi_sector: sector_t,
//
// Remaining size in bytes.
//
    pub bi_size: c_uint,
//
// Current index into the bvec array. This indexes into `bi_io_vec` when
// iterating a bvec array that is part of a `bio`.
//
    pub bi_idx: c_uint,
//
// Current offset in the bvec entry pointed to by `bi_idx`.
//
    pub bi_offset: c_uint,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bvec_iter_all {
    pub bv: bio_vec,
    pub idx: c_int,
    pub done: unsigned,
}

// multi-page (mp_bvec) helpers
// For building single-page bvec in flight
//
// A simpler version of bvec_iter_advance(), @bytes should not span
// across multiple bvec entries, i.e. bytes <= bv[i->bi_idx].bv_len
//

//
// bvec_kmap_local - map a bvec into the kernel virtual address space
// @bvec: bvec to map
//
// Must be called on single-page bvecs only.  Call kunmap_local on the returned
// address to unmap.
//
// memcpy_from_bvec - copy data from a bvec
// @to: Kernel virtual address to copy to.
// @bvec: bvec to copy from
//
// Must be called on single-page bvecs only.
//
// memcpy_to_bvec - copy data to a bvec
// @bvec: bvec to copy to
// @from: Kernel virtual address to copy from.
//
// Must be called on single-page bvecs only.
//
// memzero_bvec - zero all data in a bvec
// @bvec: bvec to zero
//
// Must be called on single-page bvecs only.
//
// bvec_virt - return the virtual address for a bvec
// @bvec: bvec to return the virtual address for
//
// Note: the caller must ensure that @bvec->bv_page is not a highmem page.
//
// bvec_phys - return the physical address for a bvec
// @bvec: bvec to return the physical address for
//
