//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/scatterlist.h
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

    pub dma_length: c_uint,

    pub dma_flags: c_uint,

}

//
// These macros should be used after a dma_map_sg call has been done
// to get bus addresses of each of the SG entries and their lengths.
// You should only work with the number of sg entries dma_map_sg
// returns, or alternatively stop on the first sg_dma_len(sg) which
// is 0.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_table {
    pub /: *mut *mut *mut scatterlist sgl; / the list,
    pub /: *mut *mut unsigned int nents; / number of mapped entries,
    pub /: *mut *mut unsigned int orig_nents; / original size of list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_append_table {
    pub /: *mut *mut sg_table sgt; / The scatter list table,
    pub /: *mut *mut *mut scatterlist prv; / last populated sge in the table,
    pub /: *mut *mut unsigned int total_nents; / Total entries in the table,
}

//
// Notes on SG table design.
//
// We use the unsigned long page_link field in the scatterlist struct to place
// the page pointer AND encode information about the sg table as well. The two
// lower bits are reserved for this information.
//
// If bit 0 is set, then the page_link contains a pointer to the next sg
// table list. Otherwise the next entry is at sg + 1.
//
// If bit 1 is set, then this sg entry is the last element in a list.
//
// See sg_next().
//
pub const SG_CHAIN: c_uint = 0x01UL;
pub const SG_END: c_uint = 0x02UL;
//
// We overload the LSB of the page pointer to indicate whether it's
// a valid sg entry, or whether it points to the start of a new scatterlist.
// Those low bits are there for everyone! (thanks mason :-)
//

//
// sg_next - return the next scatterlist entry in a list
// @sg:		The current sg entry
//
// Description:
// Usually the next entry will be @sg + 1, but if this sg element is part
// of a chained scatterlist, it could jump to the start of a new
// scatterlist array.
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
// sg_set_folio - Set sg entry to point at given folio
// @sg:		 SG entry
// @folio:	 The folio
// @len:	 Length of data
// @offset:	 Offset into folio
//
// Description:
// Use this function to set an sg entry pointing at a folio, never assign
// the folio directly. We encode sg table information in the lower bits
// of the folio pointer. See sg_page() for looking up the page belonging
// to an sg entry.
//

//
// sg_set_buf - Set sg entry to point at given data
// @sg:		 SG entry
// @buf:	 Data
// @buflen:	 Data length
//

//
// Loop over each sg element, following the pointer to a new list if necessary
//

//
// Loop over each sg element in the given sg_table object.
//

//
// Loop over each sg element in the given *DMA mapped* sg_table object.
// Please use sg_dma_address(sg) and sg_dma_len(sg) to extract DMA addresses
// of the each element.
//

//
// offset and length are unused for chain entry. Clear them.
//
// Set lowest bit to indicate a link pointer, and make sure to clear
// the termination bit if it happens to be set.
//
// sg_chain - Chain two sglists together
// @prv:	First scatterlist
// @prv_nents:	Number of entries in prv
// @sgl:	Second scatterlist
//
// Description:
// Links @prv and @sgl together, to form a longer scatterlist.
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
// On 64-bit architectures there is a 4-byte padding in struct scatterlist
// (assuming also CONFIG_NEED_SG_DMA_LENGTH is set). Use this padding for DMA
// flags bits to indicate when a specific dma address is a bus address or the
// buffer may have been bounced via SWIOTLB.
//

//
// sg_dma_is_bus_address - Return whether a given segment was marked
// as a bus address
// @sg:		 SG entry
//
// Description:
// Returns true if sg_dma_mark_bus_address() has been called on
// this segment.
//
// sg_dma_mark_bus_address - Mark the scatterlist entry as a bus address
// @sg:		 SG entry
//
// Description:
// Marks the passed in sg entry to indicate that the dma_address is
// a bus address and doesn't need to be unmapped. This should only be
// used by dma_map_sg() implementations to mark bus addresses
// so they can be properly cleaned up in dma_unmap_sg().
//
// sg_dma_unmark_bus_address - Unmark the scatterlist entry as a bus address
// @sg:		 SG entry
//
// Description:
// Clears the bus address mark.
//
// sg_dma_is_swiotlb - Return whether the scatterlist was marked for SWIOTLB
// bouncing
// @sg:		SG entry
//
// Description:
// Returns true if the scatterlist was marked for SWIOTLB bouncing. Not all
// elements may have been bounced, so the caller would have to check
// individual SG entries with swiotlb_find_pool().
//
// sg_dma_mark_swiotlb - Mark the scatterlist for SWIOTLB bouncing
// @sg:		SG entry
//
// Description:
// Marks a a scatterlist for SWIOTLB bounce. Not all SG entries may be
// bounced.
//

//
// sg_phys - Return physical address of an sg entry
// @sg:	     SG entry
//
// Description:
// This calls page_to_phys() on the page in this sg entry, and adds the
// sg offset. The caller must know that it is legal to call page_to_phys()
// on the sg page.
//
// sg_virt - Return virtual address of an sg entry
// @sg:      SG entry
//
// Description:
// This calls page_address() on the page in this sg entry, and adds the
// sg offset. The caller must know that the sg page has a valid virtual
// mapping.
//
// sg_init_marker - Initialize markers in sg table
// @sgl:	   The SG table
// @nents:	   Number of entries in table
//
extern "C" {
    pub fn sg_nents(sg: *mut scatterlist) -> c_int;
}
extern "C" {
    pub fn sg_nents_for_len(sg: *mut scatterlist, len: u64) -> c_int;
}
extern "C" {
    pub fn sg_nents_for_dma(sgl: *mut scatterlist, sglen: c_uint, len: usize) -> c_int;
}
extern "C" {
    pub fn sg_init_table(: *mut scatterlist, int: unsigned);
}
extern "C" {
    pub fn sg_init_one(: *mut scatterlist, : *const c_void, int: unsigned);
}
extern "C" {
    pub fn void(: *mut sg_free_fn)(struct scatterlist, int: unsigned) -> typedef;
}
extern "C" {
    pub fn sg_free_table(: *mut sg_table);
}
extern "C" {
    pub fn sg_free_append_table(sgt: *mut sg_append_table);
}
extern "C" {
    pub fn sg_alloc_table(: *mut sg_table, int: unsigned, _arg: gfp_t) -> c_int;
}
//
// sg_alloc_table_from_pages - Allocate and initialize an sg table from
// an array of pages
// @sgt:	 The sg table header to use
// @pages:	 Pointer to an array of page pointers
// @n_pages:	 Number of pages in the pages array
// @offset:      Offset from start of the first page to the start of a buffer
// @size:        Number of valid bytes in the buffer (after offset)
// @gfp_mask:	 GFP allocation mask
//
// Description:
// Allocate and initialize an sg table from a list of pages. Contiguous
// ranges of the pages are squashed into a single scatterlist node. A user
// may provide an offset at a start and a size of valid data in a buffer
// specified by the page array. The returned sg table is released by
// sg_free_table.
//
// Returns:
// 0 on success, negative error on failure
//

extern "C" {
    pub fn sgl_free_n_order(sgl: *mut scatterlist, nents: c_int, order: c_int);
}
extern "C" {
    pub fn sgl_free_order(sgl: *mut scatterlist, order: c_int);
}
extern "C" {
    pub fn sgl_free(sgl: *mut scatterlist);
}

//
// Maximum number of entries that will be allocated in one piece, if
// a list larger than this is required then chaining will be utilized.
//

//
// The maximum number of SG segments that we will put inside a
// scatterlist (unless chaining is used). Should ideally fit inside a
// single page, to avoid a higher order allocation.  We could define this
// to SG_MAX_SINGLE_ALLOC to pack correctly at the highest order.  The
// minimum value is 32
//
pub const SG_CHUNK_SIZE: c_int = 128;
//
// Like SG_CHUNK_SIZE, but for archs that have sg chaining. This limit
// is totally arbitrary, a setting of 2048 will get you at least 8mb ios.
//

pub const SG_MAX_SEGMENTS: c_int = 2048;

//
// sg page iterator
//
// Iterates over sg entries page-by-page.  On each successful iteration, you
// can call sg_page_iter_page(@piter) to get the current page.
// @piter->sg will point to the sg holding this page and @piter->sg_pgoffset to
// the page's page offset within the sg. The iteration will stop either when a
// maximum number of sg entries was reached or a terminating sg
// (sg_last(sg) == true) was reached.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_page_iter {
    pub /: *mut *mut *mut scatterlist sg; / sg holding the page,
    pub /: *mut *mut unsigned int sg_pgoffset; / page offset within the sg,
// these are internal states, keep away
    pub /: *mut *mut unsigned int __nents; / remaining sg entries,
    pub the: *mut *mut int __pg_advance; / nr pages to advance at,
// next step
}

//
// sg page iterator for DMA addresses
//
// This is the same as sg_page_iter however you can call
// sg_page_iter_dma_address(@dma_iter) to get the page's DMA
// address. sg_page_iter_page() cannot be called on this iterator.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_dma_page_iter {
    pub base: sg_page_iter,
}

extern "C" {
    pub fn __sg_page_iter_next(piter: *mut sg_page_iter) -> bool;
}
extern "C" {
    pub fn __sg_page_iter_dma_next(dma_iter: *mut sg_dma_page_iter) -> bool;
}
//
// sg_page_iter_page - get the current page held by the page iterator
// @piter:	page iterator holding the page
//
// sg_page_iter_dma_address - get the dma address of the current page held by
// the page iterator.
// @dma_iter:	page iterator holding the page
//
// for_each_sg_page - iterate over the pages of the given sg list
// @sglist:	sglist to iterate over
// @piter:	page iterator to hold current page, sg, sg_pgoffset
// @nents:	maximum number of sg entries to iterate over
// @pgoffset:	starting page offset (in pages)
//
// Callers may use sg_page_iter_page() to get each page pointer.
// In each loop it operates on PAGE_SIZE unit.
//

//
// for_each_sg_dma_page - iterate over the pages of the given sg list
// @sglist:	sglist to iterate over
// @dma_iter:	DMA page iterator to hold current page
// @dma_nents:	maximum number of sg entries to iterate over, this is the value
// returned from dma_map_sg
// @pgoffset:	starting page offset (in pages)
//
// Callers may use sg_page_iter_dma_address() to get each page's DMA address.
// In each loop it operates on PAGE_SIZE unit.
//

//
// for_each_sgtable_page - iterate over all pages in the sg_table object
// @sgt:	sg_table object to iterate over
// @piter:	page iterator to hold current page
// @pgoffset:	starting page offset (in pages)
//
// Iterates over the all memory pages in the buffer described by
// a scatterlist stored in the given sg_table object.
// See also for_each_sg_page(). In each loop it operates on PAGE_SIZE unit.
//

//
// for_each_sgtable_dma_page - iterate over the DMA mapped sg_table object
// @sgt:	sg_table object to iterate over
// @dma_iter:	DMA page iterator to hold current page
// @pgoffset:	starting page offset (in pages)
//
// Iterates over the all DMA mapped pages in the buffer described by
// a scatterlist stored in the given sg_table object.
// See also for_each_sg_dma_page(). In each loop it operates on PAGE_SIZE
// unit.
//

//
// Mapping sg iterator
//
// Iterates over sg entries mapping page-by-page.  On each successful
// iteration, @miter->page points to the mapped page and
// @miter->length bytes of data can be accessed at @miter->addr.  As
// long as an iteration is enclosed between start and stop, the user
// is free to choose control structure and when to stop.
//
// @miter->consumed is set to @miter->length on each iteration.  It
// can be adjusted if the user can't consume all the bytes in one go.
// Also, a stopped iteration can be resumed by calling next on it.
// This is useful when iteration needs to release all resources and
// continue later (e.g. at the next interrupt).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_mapping_iter {
// the following three fields can be accessed directly
    pub /: *mut *mut *mut page page; / currently mapped page,
    pub /: *mut *mut *mut void addr; / pointer to the mapped area,
    pub /: *mut *mut size_t length; / length of the mapped area,
    pub /: *mut *mut size_t consumed; / number of consumed bytes,
    pub /: *mut *mut sg_page_iter piter; / page iterator,
// these are internal states, keep away
    pub /: *mut *mut unsigned int __offset; / offset within page,
    pub /: *mut *mut unsigned int __remaining; / remaining bytes on page,
    pub __flags: c_uint,
}

extern "C" {
    pub fn sg_miter_skip(miter: *mut sg_mapping_iter, offset: off_t) -> bool;
}
extern "C" {
    pub fn sg_miter_next(miter: *mut sg_mapping_iter) -> bool;
}
extern "C" {
    pub fn sg_miter_stop(miter: *mut sg_mapping_iter);
}
