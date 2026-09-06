//! Automatically rewritten from C Header to Rust Module
//! Source: tools/virtio/linux/scatterlist.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scatterlist {
    pub page_link: c_ulong,
    pub offset: c_uint,
    pub length: c_uint,
    pub dma_address: dma_addr_t,
}

// Scatterlist helpers, stolen from linux/scatterlist.h

//
// sg_assign_page - Assign a given page to an SG entry
// @sg:		    SG entry
// @page:	    The page
//
// Description:
// Assign page to sg entry. Also see sg_set_page(), the most commonly used
// variant.
//
// In order for the low bit stealing approach to work, pages
// must be aligned at a 32-bit boundary as a minimum.
//

//
// sg_set_page - Set sg entry to point at given page
// @sg:		 SG entry
// @page:	 The page
// @len:	 Length of data
// @offset:	 Offset into page
//
// Description:
// Use this function to set an sg entry pointing at a page, never assign
// the page directly. We encode sg table information in the lower bits
// of the page pointer. See sg_page() for looking up the page belonging
// to an sg entry.
//

//
// Loop over each sg element, following the pointer to a new list if necessary
//

//
// sg_chain - Chain two sglists together
// @prv:	First scatterlist
// @prv_nents:	Number of entries in prv
// @sgl:	Second scatterlist
//
// Description:
// Links @prv@ and @sgl@ together, to form a longer scatterlist.
//
// offset and length are unused for chain entry.  Clear them.
//
// Set lowest bit to indicate a link pointer, and make sure to clear
// the termination bit if it happens to be set.
//
// sg_mark_end - Mark the end of the scatterlist
// @sg:		 SG entryScatterlist
//
// Description:
// Marks the passed in sg entry as the termination point for the sg
// table. A call to sg_next() on this entry will return NULL.
//
// Set termination bit, clear potential chain bit
//
// sg_unmark_end - Undo setting the end of the scatterlist
// @sg:		 SG entryScatterlist
//
// Description:
// Removes the termination marker from the given entry of the scatterlist.
//
