//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/scatterwalk.h
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
// Cryptographic scatter and gather helpers.
//
// Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
// Copyright (c) 2002 Adam J. Richter <adam@yggdrasil.com>
// Copyright (c) 2004 Jean-Luc Cooke <jlcooke@certainkey.com>
// Copyright (c) 2007 Herbert Xu <herbert@gondor.apana.org.au>
//

//
// This is equivalent to scatterwalk_start(walk, sg) followed by
// scatterwalk_skip(walk, pos).
//
// HIGHMEM case: the page may have to be mapped into memory.  To avoid
// the complexity of having to map multiple pages at once per sg entry,
// clamp the returned length to not cross a page boundary.
//
// !HIGHMEM case: no mapping is needed; all pages of the sg entry are
// already mapped contiguously in the kernel's direct map.  For improved
// performance, allow the walker to return data segments that cross a
// page boundary.  Do still cap the length to PAGE_SIZE, since some
// users rely on that to avoid disabling preemption for too long when
// using SIMD.  It's also needed for when skcipher_walk uses a bounce
// page due to the data not being aligned to the algorithm's alignmask.
//
extern "C" {
    pub fn min3(_arg: nbytes, _arg: len_this_sg, _arg: limit) -> return;
}
//
// Create a scatterlist that represents the remaining data in a walk.  Uses
// chaining to reference the original scatterlist, so this uses at most two
// entries in @sg_out regardless of the number of entries in the original list.
// Assumes that sg_init_table() was already done.
//
// When !HIGHMEM we allow the walker to return segments that
// span a page boundary; see scatterwalk_clamp().  To make it
// clear that in this case we're working in the linear buffer of
// the whole sg entry in the kernel's direct map rather than
// within the mapped buffer of a single page, compute the
// address as an offset from the page_address() of the first
// page of the sg entry.  Either way the result is the address
// in the direct map, but this makes it clearer what is really
// going on.
//
// scatterwalk_next() - Get the next data buffer in a scatterlist walk
// @walk: the scatter_walk
// @total: the total number of bytes remaining, > 0
//
// A virtual address for the next segment of data from the scatterlist will
// be placed into @walk->addr.  The caller must call scatterwalk_done_src()
// or scatterwalk_done_dst() when it is done using this virtual address.
//
// Returns: the next number of bytes available, <= @total
//
// scatterwalk_done_src() - Finish one step of a walk of source scatterlist
// @walk: the scatter_walk
// @nbytes: the number of bytes processed this step, less than or equal to the
// number of bytes that scatterwalk_next() returned.
//
// Use this if the mapped address was not written to, i.e. it is source data.
//
// Flush the dcache of any pages that overlap the region
// [offset, offset + nbytes) relative to base_page.
//
// This should be called only when ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE, to ensure
// that all relevant code (including the call to sg_page() in the caller, if
// applicable) gets fully optimized out when !ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE.
//
// This is an overflow-safe version of
// num_pages = DIV_ROUND_UP(offset + nbytes, PAGE_SIZE).
//
// scatterwalk_done_dst() - Finish one step of a walk of destination scatterlist
// @walk: the scatter_walk
// @nbytes: the number of bytes processed this step, less than or equal to the
// number of bytes that scatterwalk_next() returned.
//
// Use this if the mapped address may have been written to, i.e. it is
// destination data.
//
extern "C" {
    pub fn scatterwalk_skip(walk: *mut scatter_walk, nbytes: c_uint);
}
// In new code, please use memcpy_{from,to}_sglist() directly instead.
