//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/uio.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Berkeley style UIO structures	-	Alan Cox 1994.
//

pub type iov_iter_extraction_t = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvec {
    pub /: *mut *mut *mut *mut *mut void iov_base; / and that should never hold a userland pointer,
    pub iov_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iter_type {
// iter types
    ITER_UBUF,
    ITER_IOVEC,
    ITER_BVEC,
    ITER_KVEC,
    ITER_FOLIOQ,
    ITER_XARRAY,
    ITER_DISCARD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iov_iter_state {
    pub iov_offset: usize,
    pub count: usize,
    pub nr_segs: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iov_iter {
    pub iter_type: u8,
    pub nofault: bool,
    pub data_source: bool,
    pub iov_offset: usize,
//
// Hack alert: overlay ubuf_iovec with iovec + count, so
// that the members resolve correctly regardless of the type
// of iterator used. This means that you can use:
//
// &iter->__ubuf_iovec or iter->__iov
//
// interchangably for the user_backed cases, hence simplifying
// some of the cases that need to deal with both.
//
// This really should be a const, but we cannot do that without
// also modifying any of the zero-filling iter init functions.
// Leave it non-const for now, but it should be treated as such.
//
    pub __ubuf_iovec: iovec,
// use iter_iov() to get the current vec
    pub __iov: *const iovec,
    pub kvec: *const kvec,
    pub bvec: *const bio_vec,
    pub folioq: *const folio_queue,
    pub xarray: *mut xarray,
    pub ubuf: *mut void __user,
}

pub type uio_meta_flags_t = __u16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uio_meta {
    pub flags: uio_meta_flags_t,
    pub app_tag: u16,
    pub seed: u64,
    pub iter: iov_iter,
}

extern "C" {
    pub fn iter_is_ubuf(iter_is_iovec(i: i) ||) -> return;
}
//
// Total number of bytes covered by an iovec.
//
// NOTE that it is not safe to use this function until all the iovec's
// segment lengths have been validated.  Because the individual lengths can
// overflow a size_t when added together.
//
extern "C" {
    pub fn iov_iter_advance(i: *mut iov_iter, bytes: usize);
}
extern "C" {
    pub fn iov_iter_revert(i: *mut iov_iter, bytes: usize);
}
extern "C" {
    pub fn fault_in_iov_iter_readable(i: *const iov_iter, bytes: usize) -> usize;
}
extern "C" {
    pub fn fault_in_iov_iter_writeable(i: *const iov_iter, bytes: usize) -> usize;
}
extern "C" {
    pub fn iov_iter_single_seg_count(i: *const iov_iter) -> usize;
}
extern "C" {
    pub fn _copy_to_iter(addr: *const c_void, bytes: usize, i: *mut iov_iter) -> usize;
}
extern "C" {
    pub fn _copy_from_iter(addr: *mut c_void, bytes: usize, i: *mut iov_iter) -> usize;
}
extern "C" {
    pub fn _copy_from_iter_nocache(addr: *mut c_void, bytes: usize, i: *mut iov_iter) -> usize;
}
extern "C" {
    pub fn copy_page_to_iter(_arg: &folio->page, _arg: offset, _arg: bytes, _arg: i) -> return;
}
extern "C" {
    pub fn copy_page_from_iter(_arg: &folio->page, _arg: offset, _arg: bytes, _arg: i) -> return;
}
extern "C" {
    pub fn _copy_to_iter(_arg: addr, _arg: bytes, _arg: i) -> return;
}
extern "C" {
    pub fn _copy_from_iter(_arg: addr, _arg: bytes, _arg: i) -> return;
}
extern "C" {
    pub fn _copy_from_iter_nocache(_arg: addr, _arg: bytes, _arg: i) -> return;
}

//
// Note, users like pmem that depend on the stricter semantics of
// _copy_from_iter_flushcache() than _copy_from_iter_nocache() must check for
// IS_ENABLED(CONFIG_ARCH_HAS_UACCESS_FLUSHCACHE) before assuming that the
// destination is flushed from the cache on return.
//
extern "C" {
    pub fn _copy_from_iter_flushcache(addr: *mut c_void, bytes: usize, i: *mut iov_iter) -> usize;
}

extern "C" {
    pub fn _copy_mc_to_iter(addr: *const c_void, bytes: usize, i: *mut iov_iter) -> usize;
}

extern "C" {
    pub fn iov_iter_zero(bytes: usize, : *mut iov_iter) -> usize;
}
extern "C" {
    pub fn iov_iter_alignment(i: *const iov_iter) -> c_ulong;
}
extern "C" {
    pub fn iov_iter_gap_alignment(i: *const iov_iter) -> c_ulong;
}
extern "C" {
    pub fn iov_iter_discard(i: *mut iov_iter, direction: c_uint, count: usize);
}
extern "C" {
    pub fn iov_iter_npages(i: *const iov_iter, maxpages: c_int) -> c_int;
}
extern "C" {
    pub fn iov_iter_restore(i: *mut iov_iter, state: *mut iov_iter_state);
}
//
// Cap the iov_iter by given limit; note that the second argument is
// *not* the new size - it's upper limit for such.  Passing it a value
// greater than the amount of data in iov_iter is fine - it'll just do
// nothing in that case.
//
// count doesn't have to fit in size_t - comparison extends both
// operands to u64 here and any value that would be truncated by
// conversion in assignement is by definition greater than all
// values of size_t, including old i->count.
//
// reexpand a previously truncated iterator; count must be no more than how much
// we had shrunk it.
//
extern "C" {
    pub fn import_ubuf(type: c_int, buf: *mut void __user, len: usize, i: *mut iov_iter) -> c_int;
}
// i = (struct iov_iter) {
// Flags for iov_iter_get/extract_pages*()
// Allow P2PDMA on the extracted pages

//
// Block-layer consumers (e.g. bio_iov_iter_get_pages()) require that the
// segments of an ITER_BVEC iterator are already aligned to the target device's
// DMA alignment, and forward them as-is.  In-kernel users that build their own
// bvecs must not create sub-aligned segments; iov_iter_extract_bvecs() enforces
// the same for the segments it extracts via @mem_align_mask.
//
// iov_iter_extract_will_pin - Indicate how pages from the iterator will be retained
// @iter: The iterator
//
// Examine the iterator and indicate by returning true or false as to how, if
// at all, pages extracted from the iterator will be retained by the extraction
// function.
//
// %true indicates that the pages will have a pin placed in them that the
// caller must unpin.  This is must be done for DMA/async DIO to force fork()
// to forcibly copy a page for the child (the parent must retain the original
// page).
//
// %false indicates that no measures are taken and that it's up to the caller
// to retain the pages.
//
extern "C" {
    pub fn user_backed_iter(_arg: iter) -> return;
}
