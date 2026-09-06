//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/highmem.h
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
// kmap - Map a page for long term usage
// @page:	Pointer to the page to be mapped
//
// Returns: The virtual address of the mapping
//
// Can only be invoked from preemptible task context because on 32bit
// systems with CONFIG_HIGHMEM enabled this function might sleep.
//
// For systems with CONFIG_HIGHMEM=n and for pages in the low memory area
// this returns the virtual address of the direct kernel mapping.
//
// The returned virtual address is globally visible and valid up to the
// point where it is unmapped via kunmap(). The pointer can be handed to
// other contexts.
//
// For highmem pages on 32bit systems this can be slow as the mapping space
// is limited and protected by a global lock. In case that there is no
// mapping slot available the function blocks until a slot is released via
// kunmap().
//
// kunmap - Unmap the virtual address mapped by kmap()
// @page:	Pointer to the page which was mapped by kmap()
//
// Counterpart to kmap(). A NOOP for CONFIG_HIGHMEM=n and for mappings of
// pages in the low memory area.
//
extern "C" {
    pub fn kunmap(page: *const page);
}
//
// kmap_to_page - Get the page for a kmap'ed address
// @addr:	The address to look up
//
// Returns: The page which is mapped to @addr.
//
// kmap_flush_unused - Flush all unused kmap mappings in order to
// remove stray mappings
//
extern "C" {
    pub fn kmap_flush_unused();
}
//
// kmap_local_page - Map a page for temporary usage
// @page: Pointer to the page to be mapped
//
// Returns: The virtual address of the mapping
//
// Can be invoked from any context, including interrupts.
//
// Requires careful handling when nesting multiple mappings because the map
// management is stack based. The unmap has to be in the reverse order of
// the map operation:
//
// addr1 = kmap_local_page(page1);
// addr2 = kmap_local_page(page2);
// ...
// kunmap_local(addr2);
// kunmap_local(addr1);
//
// Unmapping addr1 before addr2 is invalid and causes malfunction.
//
// Contrary to kmap() mappings the mapping is only valid in the context of
// the caller and cannot be handed to other contexts.
//
// On CONFIG_HIGHMEM=n kernels and for low memory pages this returns the
// virtual address of the direct mapping. Only real highmem pages are
// temporarily mapped.
//
// While kmap_local_page() is significantly faster than kmap() for the highmem
// case it comes with restrictions about the pointer validity.
//
// On HIGHMEM enabled systems mapping a highmem page has the side effect of
// disabling migration in order to keep the virtual address stable across
// preemption. No caller of kmap_local_page() can rely on this side effect.
//
// kmap_local_folio - Map a page in this folio for temporary usage
// @folio: The folio containing the page.
// @offset: The byte offset within the folio which identifies the page.
//
// Requires careful handling when nesting multiple mappings because the map
// management is stack based. The unmap has to be in the reverse order of
// the map operation::
//
// addr1 = kmap_local_folio(folio1, offset1);
// addr2 = kmap_local_folio(folio2, offset2);
// ...
// kunmap_local(addr2);
// kunmap_local(addr1);
//
// Unmapping addr1 before addr2 is invalid and causes malfunction.
//
// Contrary to kmap() mappings the mapping is only valid in the context of
// the caller and cannot be handed to other contexts.
//
// On CONFIG_HIGHMEM=n kernels and for low memory pages this returns the
// virtual address of the direct mapping. Only real highmem pages are
// temporarily mapped.
//
// While it is significantly faster than kmap() for the highmem case it
// comes with restrictions about the pointer validity.
//
// On HIGHMEM enabled systems mapping a highmem page has the side effect of
// disabling migration in order to keep the virtual address stable across
// preemption. No caller of kmap_local_folio() can rely on this side effect.
//
// Context: Can be invoked from any context.
// Return: The virtual address of @offset.
//
// kmap_atomic - Atomically map a page for temporary usage - Deprecated!
// @page:	Pointer to the page to be mapped
//
// Returns: The virtual address of the mapping
//
// In fact a wrapper around kmap_local_page() which also disables pagefaults
// and, depending on PREEMPT_RT configuration, also CPU migration and
// preemption. Therefore users should not count on the latter two side effects.
//
// Mappings should always be released by kunmap_atomic().
//
// Do not use in new code. Use kmap_local_page() instead.
//
// It is used in atomic context when code wants to access the contents of a
// page that might be allocated from high memory (see __GFP_HIGHMEM), for
// example a page in the pagecache.  The API has two functions, and they
// can be used in a manner similar to the following::
//
// // Find the page of interest.
// struct page *page = find_get_page(mapping, offset);
//
// // Gain access to the contents of that page.
// void *vaddr = kmap_atomic(page);
//
// // Do something to the contents of that page.
// memset(vaddr, 0, PAGE_SIZE);
//
// // Unmap that page.
// kunmap_atomic(vaddr);
//
// Note that the kunmap_atomic() call takes the result of the kmap_atomic()
// call, not the argument.
//
// If you need to map two pages because you want to copy from one page to
// another you need to keep the kmap_atomic calls strictly nested, like:
//
// vaddr1 = kmap_atomic(page1);
// vaddr2 = kmap_atomic(page2);
//
// memcpy(vaddr1, vaddr2, PAGE_SIZE);
//
// kunmap_atomic(vaddr2);
// kunmap_atomic(vaddr1);
//
// Highmem related interfaces for management code
extern "C" {
    pub fn nr_free_highpages() -> c_ulong;
}
extern "C" {
    pub fn totalhigh_pages() -> c_ulong;
}

//
// clear_user_page() - clear a page to be mapped to user space
// @addr: the address of the page
// @vaddr: the address of the user mapping
// @page: the page
//
// We condition the definition of clear_user_page() on the architecture
// not having a custom clear_user_highpage(). That's because if there
// is some special flushing needed for clear_user_highpage() then it
// is likely that clear_user_page() also needs some magic. And, since
// our only caller is the generic clear_user_highpage(), not defining
// is not much of a loss.
//

//
// clear_user_pages() - clear a page range to be mapped to user space
// @addr: start address
// @vaddr: start address of the user mapping
// @page: start page
// @npages: number of pages
//
// Assumes that the region (@addr, +@npages) has been validated
// already so this does no exception handling.
//
// If the architecture provides a clear_user_page(), use that;
// otherwise, we can safely use clear_pages().
//

//
// Prefer clear_pages() to allow for architectural optimizations
// when operating on contiguous page ranges.
//

//
// clear_user_highpage() - clear a page to be mapped to user space
// @page: start page
// @vaddr: start address of the user mapping
//
// With !CONFIG_HIGHMEM this (and the copy_user_highpage() below) will
// be plain clear_user_page() (and copy_user_page()).
//

//
// clear_user_highpages() - clear a page range to be mapped to user space
// @page: start page
// @vaddr: start address of the user mapping
// @npages: number of pages
//
// Assumes that all the pages in the region (@page, +@npages) are valid
// so this does no exception handling.
//

//
// An architecture defined clear_user_highpage() implies special
// handling is needed.
//
// So we use that or, the generic variant if CONFIG_HIGHMEM is
// enabled.
//

//
// Prefer clear_user_pages() to allow for architectural optimizations
// when operating on contiguous page ranges.
//

//
// vma_alloc_zeroed_movable_folio - Allocate a zeroed page for a VMA.
// @vma: The VMA the page is to be allocated for.
// @vaddr: The virtual address the page will be inserted into.
//
// This function will allocate a page suitable for inserting into this
// VMA at this virtual address.  It may be allocated from highmem or
// the movable zone.  An architecture may provide its own implementation.
//
// Return: A folio containing one allocated and zeroed page or NULL if
// we are out of memory.
//

// Returns true if the caller has to initialize the pages

//
// If we pass in a base or tail page, we can zero up to PAGE_SIZE.
// If we pass in a head page, we can zero up to the size of the compound page.
//

//
// If architecture supports machine check exception handling, define the
// #MC versions of copy_user_highpage and copy_highpage. They copy a memory
// page with #MC in source page (@from) handled, and return the number
// of bytes not copied if there was a #MC, otherwise 0 for success.
//

//
// memcpy_from_folio - Copy a range of bytes from a folio.
// @to: The memory to copy to.
// @folio: The folio to read from.
// @offset: The first byte in the folio to read.
// @len: The number of bytes to copy.
//
// memcpy_to_folio - Copy a range of bytes to a folio.
// @folio: The folio to write to.
// @offset: The first byte in the folio to store to.
// @from: The memory to copy from.
// @len: The number of bytes to copy.
//
// folio_zero_tail - Zero the tail of a folio.
// @folio: The folio to zero.
// @offset: The byte offset in the folio to start zeroing at.
// @kaddr: The address the folio is currently mapped to.
//
// If you have already used kmap_local_folio() to map a folio, written
// some data to it and now need to zero the end of the folio (and flush
// the dcache), you can use this function.  If you do not have the
// folio kmapped (eg the folio has been partially populated by DMA),
// use folio_zero_range() or folio_zero_segment() instead.
//
// Return: An address which can be passed to kunmap_local().
//
// folio_fill_tail - Copy some data to a folio and pad with zeroes.
// @folio: The destination folio.
// @offset: The offset into @folio at which to start copying.
// @from: The data to copy.
// @len: How many bytes of data to copy.
//
// This function is most useful for filesystems which support inline data.
// When they want to copy data from the inode into the page cache, this
// function does everything for them.  It supports large folios even on
// HIGHMEM configurations.
//
// memcpy_from_file_folio - Copy some bytes from a file folio.
// @to: The destination buffer.
// @folio: The folio to copy from.
// @pos: The position in the file.
// @len: The maximum number of bytes to copy.
//
// Copy up to @len bytes from this folio.  This may be limited by PAGE_SIZE
// if the folio comes from HIGHMEM, and by the size of the folio.
//
// Return: The number of bytes copied from the folio.
//
// folio_zero_segments() - Zero two byte ranges in a folio.
// @folio: The folio to write to.
// @start1: The first byte to zero.
// @xend1: One more than the last byte in the first range.
// @start2: The first byte to zero in the second range.
// @xend2: One more than the last byte in the second range.
//
// folio_zero_segment() - Zero a byte range in a folio.
// @folio: The folio to write to.
// @start: The first byte to zero.
// @xend: One more than the last byte to zero.
//
// folio_zero_range() - Zero a byte range in a folio.
// @folio: The folio to write to.
// @start: The first byte to zero.
// @length: The number of bytes to zero.
//
// folio_release_kmap - Unmap a folio and drop a refcount.
// @folio: The folio to release.
// @addr: The address previously returned by a call to kmap_local_folio().
//
// It is common, eg in directory handling to kmap a folio.  This function
// unmaps the folio and drops the refcount that was being held to keep the
// folio alive while we accessed it.
//
