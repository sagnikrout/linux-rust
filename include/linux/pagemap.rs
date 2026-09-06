//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pagemap.h
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
// Copyright 1995 Linus Torvalds
//

extern "C" {
    pub fn invalidate_inode_pages2(mapping: *mut address_space) -> c_int;
}
extern "C" {
    pub fn kiocb_invalidate_pages(iocb: *mut kiocb, count: usize) -> c_int;
}
extern "C" {
    pub fn kiocb_invalidate_post_direct_write(iocb: *mut kiocb, count: usize);
}
extern "C" {
    pub fn write_inode_now(: *mut inode, sync: c_int) -> c_int;
}
extern "C" {
    pub fn filemap_fdatawrite(: *mut address_space) -> c_int;
}
extern "C" {
    pub fn filemap_flush(: *mut address_space) -> c_int;
}
extern "C" {
    pub fn filemap_flush_nr(mapping: *mut address_space, nr_to_write: *mut c_long) -> c_int;
}
extern "C" {
    pub fn filemap_fdatawait_keep_errors(mapping: *mut address_space) -> c_int;
}
extern "C" {
    pub fn filemap_fdatawait_range(: *mut address_space, lstart: loff_t, lend: loff_t) -> c_int;
}
extern "C" {
    pub fn filemap_fdatawait_range(_arg: mapping, _arg: 0, _arg: LLONG_MAX) -> return;
}
extern "C" {
    pub fn filemap_range_has_page(: *mut address_space, lstart: loff_t, lend: loff_t) -> bool;
}
extern "C" {
    pub fn filemap_check_errors(mapping: *mut address_space) -> c_int;
}
extern "C" {
    pub fn __filemap_set_wb_err(mapping: *mut address_space, err: c_int);
}
extern "C" {
    pub fn kiocb_write_and_wait(iocb: *mut kiocb, count: usize) -> c_int;
}
extern "C" {
    pub fn filemap_write_and_wait_range(_arg: mapping, _arg: 0, _arg: LLONG_MAX) -> return;
}
//
// filemap_set_wb_err - set a writeback error on an address_space
// @mapping: mapping in which to set writeback error
// @err: error to be set in mapping
//
// When writeback fails in some way, we must record that error so that
// userspace can be informed when fsync and the like are called.  We endeavor
// to report errors on any file that was open at the time of the error.  Some
// internal callers also need to know when writeback errors have occurred.
//
// When a writeback error occurs, most filesystems will want to call
// filemap_set_wb_err to record the error in the mapping so that it will be
// automatically reported whenever fsync is called on the file.
//
// Fastpath for common case of no error
//
// filemap_check_wb_err - has an error occurred since the mark was sampled?
// @mapping: mapping to check for writeback errors
// @since: previously-sampled errseq_t
//
// Grab the errseq_t value from the mapping, and see if it has changed "since"
// the given value was sampled.
//
// If it has then report the latest error set, otherwise return 0.
//
extern "C" {
    pub fn errseq_check(_arg: &mapping->wb_err, _arg: since) -> return;
}
//
// filemap_sample_wb_err - sample the current errseq_t to test for later errors
// @mapping: mapping to be sampled
//
// Writeback errors are always reported relative to a particular sample point
// in the past. This function provides those sample points.
//
extern "C" {
    pub fn errseq_sample(_arg: &mapping->wb_err) -> return;
}
//
// file_sample_sb_err - sample the current errseq_t to test for later errors
// @file: file pointer to be sampled
//
// Grab the most current superblock-level errseq_t value for the given
// struct file.
//
extern "C" {
    pub fn errseq_sample(_arg: &file->f_path.dentry->d_sb->s_wb_err) -> return;
}
//
// Flush file data before changing attributes.  Caller must hold any locks
// required to prevent further writes to this file until we're done setting
// flags.
//
extern "C" {
    pub fn filemap_write_and_wait(_arg: inode->i_mapping) -> return;
}
extern "C" {
    pub fn xa_empty(_arg: &mapping->i_pages) -> return;
}
//
// mapping_shrinkable - test if page cache state allows inode reclaim
// @mapping: the page cache mapping
//
// This checks the mapping's cache state for the pupose of inode
// reclaim and LRU management.
//
// The caller is expected to hold the i_lock, but is not required to
// hold the i_pages lock, which usually protects cache state. That's
// because the i_lock and the list_lru lock that protect the inode and
// its LRU state don't nest inside the irq-safe i_pages lock.
//
// Cache deletions are performed under the i_lock, which ensures that
// when an inode goes empty, it will reliably get queued on the LRU.
//
// Cache additions do not acquire the i_lock and may race with this
// check, in which case we'll report the inode as shrinkable when it
// has cache pages. This is okay: the shrinker also checks the
// refcount and the referenced bit, which will be elevated or set in
// the process of adding new cache pages to an inode.
//
// On highmem systems, there could be lowmem pressure from the
// inodes before there is highmem pressure from the page
// cache. Make inodes shrinkable regardless of cache state.
//
// Cache completely empty? Shrink away.
//
// The xarray stores single offset-0 entries directly in the
// head pointer, which allows non-resident page cache entries
// to escape the shadow shrinker's list of xarray nodes. The
// inode shrinker needs to pick them up under memory pressure.
//
// Bits in mapping->flags.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mapping_flags {
    AS_EIO		= 0,	/* IO error on async write */
    AS_ENOSPC	= 1,	/* ENOSPC on async write */
    AS_MM_ALL_LOCKS	= 2,	/* under mm_take_all_locks() */
    AS_UNEVICTABLE	= 3,	/* e.g., ramdisk, SHM_LOCK */
    AS_EXITING	= 4, 	/* final truncate in progress */
// writeback related tags are not used
    AS_NO_WRITEBACK_TAGS = 5,
    AS_RELEASE_ALWAYS = 6,	/* Call ->release_folio(), even if no private data */
    AS_STABLE_WRITES = 7,	/* must wait for writeback before modifying
    folio contents */
    AS_INACCESSIBLE = 8,	/* Do not attempt direct R/W access to the mapping */
    AS_WRITEBACK_MAY_DEADLOCK_ON_RECLAIM = 9,
    AS_KERNEL_FILE = 10,	/* mapping for a fake kernel file that shouldn't
    account usage to user cgroups */
// Bits 16-25 are used for FOLIO_ORDER
    AS_FOLIO_ORDER_BITS = 5,
    AS_FOLIO_ORDER_MIN = 16,
    AS_FOLIO_ORDER_MAX = AS_FOLIO_ORDER_MIN + AS_FOLIO_ORDER_BITS,
}

//
// mapping_set_error - record a writeback error in the address_space
// @mapping: the mapping in which an error should be set
// @error: the error to set in the mapping
//
// When writeback fails in some way, we must record that error so that
// userspace can be informed when fsync and the like are called.  We endeavor
// to report errors on any file that was open at the time of the error.  Some
// internal callers also need to know when writeback errors have occurred.
//
// When a writeback error occurs, most filesystems will want to call
// mapping_set_error to record the error in the mapping so that it can be
// reported when the application calls fsync(2).
//
// Record in wb_err for checkers using errseq_t based tracking
// Record it in superblock
// Record it in flags for now, for legacy callers
extern "C" {
    pub fn test_bit(_arg: AS_EXITING, _arg: &mapping->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: AS_RELEASE_ALWAYS, _arg: &mapping->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: AS_STABLE_WRITES, _arg: &mapping->flags) -> return;
}
//
// It's expected inaccessible mappings are also unevictable. Compaction
// migrate scanner (isolate_migratepages_block()) relies on this to
// reduce page locking.
//
extern "C" {
    pub fn test_bit(_arg: AS_INACCESSIBLE, _arg: &mapping->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: AS_WRITEBACK_MAY_DEADLOCK_ON_RECLAIM, _arg: &mapping->flags) -> return;
}
// Restricts the given gfp_mask to what the mapping allows.
//
// This is non-atomic.  Only to be used before the mapping is activated.
// Probably needs a barrier...
//
// There are some parts of the kernel which assume that PMD entries
// are exactly HPAGE_PMD_ORDER.  Those should be fixed, but until then,
// limit the maximum allocation order to PMD size.  I'm not aware of any
// assumptions about maximum order if THP are disabled, but 8 seems like
// a good order (that's 1MB if you're using 4kB pages)
//

pub const PREFERRED_MAX_PAGECACHE_ORDER: c_int = 8;

//
// xas_split_alloc() does not support arbitrary orders. This implies no
// 512MB THP on ARM64 with 64KB base page size.
//

//
// mapping_max_folio_size_supported() - Check the max folio size supported
//
// The filesystem should call this function at mount time if there is a
// requirement on the folio mapping size in the page cache.
//
// mapping_set_folio_order_range() - Set the orders supported by a file.
// @mapping: The address space of the file.
// @min: Minimum folio order (between 0-MAX_PAGECACHE_ORDER inclusive).
// @max: Maximum folio order (between @min-MAX_PAGECACHE_ORDER inclusive).
//
// The filesystem should call this function in its inode constructor to
// indicate which base size (min) and maximum size (max) of folio the VFS
// can use to cache the contents of the file.  This should only be used
// if the filesystem needs special handling of folio sizes (ie there is
// something the core cannot know).
// Do not tune it based on, eg, i_size.
//
// Context: This should not be called while the inode is active as it
// is non-atomic.
//
// mapping_set_large_folios() - Indicate the file supports large folios.
// @mapping: The address space of the file.
//
// The filesystem should call this function in its inode constructor to
// indicate that the VFS can use large folios to cache the contents of
// the file.
//
// Context: This should not be called while the inode is active as it
// is non-atomic.
//
// mapping_align_index() - Align index for this mapping.
// @mapping: The address_space.
// @index: The page index.
//
// The index of a folio must be naturally aligned.  If you are adding a
// new folio to the page cache and need to know what index to give it,
// call this function.
//
extern "C" {
    pub fn round_down(_arg: index, _arg: mapping_min_folio_nrpages(mapping)) -> return;
}
//
// Large folio support currently depends on THP.  These dependencies are
// being worked on but are not yet fixed.
//
// AS_FOLIO_ORDER is only reasonable for pagecache folios
//
// mapping_pmd_folio_support() - Check if a mapping supports PMD-sized folio
// @mapping: The address_space
//
// While some mappings support large folios, they might not support PMD-sized
// folios. This function checks whether a mapping supports PMD-sized folios.
// For example, khugepaged needs this information before attempting to
// collapsing THPs.
//
// Return: True if PMD-sized folios are supported, otherwise false.
//

// AS_FOLIO_ORDER is only reasonable for pagecache folios

// Return the maximum folio size for this pagecache mapping, in bytes.
//
// folio_flush_mapping - Find the file mapping this folio belongs to.
// @folio: The folio.
//
// For folios which are in the page cache, return the mapping that this
// page belongs to.  Anonymous folios return NULL, even if they're in
// the swap cache.  Other kinds of folio also return NULL.
//
// This is ONLY used by architecture cache flushing code.  If you aren't
// writing cache flushing code, you want either folio_mapping() or
// folio_file_mapping().
//
extern "C" {
    pub fn folio_mapping(_arg: folio) -> return;
}
//
// folio_inode - Get the host inode for this folio.
// @folio: The folio.
//
// For folios which are in the page cache, return the inode that this folio
// belongs to.
//
// Do not call this for folios which aren't in the page cache.
//
// folio_attach_private - Attach private data to a folio.
// @folio: Folio to attach data to.
// @data: Data to attach to folio.
//
// Attaching private data to a folio increments the page's reference count.
// The data must be detached before the folio will be freed.
//
// folio_change_private - Change private data on a folio.
// @folio: Folio to change the data on.
// @data: Data to set on the folio.
//
// Change the private data attached to a folio and return the old
// data.  The page must previously have had data attached and the data
// must be detached before the folio will be freed.
//
// Return: Data that was previously attached to the folio.
//
// folio_detach_private - Detach private data from a folio.
// @folio: Folio to detach data from.
//
// Removes the data that was previously attached to the folio and decrements
// the refcount on the page.
//
// Return: Data that was attached to the folio.
//
extern "C" {
    pub fn folio_detach_private(_arg: page_folio(page)) -> return;
}

extern "C" {
    pub fn folio_alloc_noprof(_arg: gfp, _arg: order) -> return;
}

extern "C" {
    pub fn filler_t(: *mut file, : *mut folio) -> typedef int;
}
//
// typedef fgf_t - Flags for getting folios from the page cache.
//
// Most users of the page cache will not need to use these flags;
// there are convenience functions such as filemap_get_folio() and
// filemap_lock_folio().  For users which need more control over exactly
// what is done with the folios, these flags to __filemap_get_folio()
// are available.
//
// * %FGP_ACCESSED - The folio will be marked accessed.
// * %FGP_LOCK - The folio is returned locked.
// * %FGP_CREAT - If no folio is present then a new folio is allocated,
// added to the page cache and the VM's LRU list.  The folio is
// returned locked.
// * %FGP_FOR_MMAP - The caller wants to do its own locking dance if the
// folio is already in cache.  If the folio was allocated, unlock it
// before returning so the caller can do the same dance.
// * %FGP_WRITE - The folio will be written to by the caller.
// * %FGP_NOFS - __GFP_FS will get cleared in gfp.
// * %FGP_NOWAIT - Don't block on the folio lock.
// * %FGP_STABLE - Wait for the folio to be stable (finished writeback)
// * %FGP_DONTCACHE - Uncached buffered IO
// * %FGP_WRITEBEGIN - The flags to use in a filesystem write_begin()
// implementation.
//
pub type fgf_t = u32;

//
// fgf_set_order - Encode a length in the fgf_t flags.
// @size: The suggested size of the folio to create.
//
// The caller of __filemap_get_folio() can use this to suggest a preferred
// size for the folio that is created.  If there is already a folio at
// the index, it will be returned, no matter what its size.  If a folio
// is freshly created, it may be of a different size than requested
// due to alignment constraints, memory pressure, or the presence of
// other folios at nearby indices.
//
extern "C" {
    pub fn __filemap_get_folio_mpol(_arg: mapping, _arg: index, _arg: fgf_flags, _arg: gfp, _arg: NULL) -> return;
}
//
// write_begin_get_folio - Get folio for write_begin with flags.
// @iocb: The kiocb passed from write_begin (may be NULL).
// @mapping: The address space to search.
// @index: The page cache index.
// @len: Length of data being written.
//
// This is a helper for filesystem write_begin() implementations.
// It wraps __filemap_get_folio(), setting appropriate flags in
// the write begin context.
//
// Return: A folio or an ERR_PTR.
//
// filemap_get_folio - Find and get a folio.
// @mapping: The address_space to search.
// @index: The page index.
//
// Looks up the page cache entry at @mapping & @index.  If a folio is
// present, it is returned with an increased refcount.
//
// Return: A folio or ERR_PTR(-ENOENT) if there is no folio in the cache for
// this index.  Will not return a shadow, swap or DAX entry.
//
extern "C" {
    pub fn __filemap_get_folio(_arg: mapping, _arg: index, _arg: 0, _arg: 0) -> return;
}
//
// filemap_lock_folio - Find and lock a folio.
// @mapping: The address_space to search.
// @index: The page index.
//
// Looks up the page cache entry at @mapping & @index.  If a folio is
// present, it is returned locked with an increased refcount.
//
// Context: May sleep.
// Return: A folio or ERR_PTR(-ENOENT) if there is no folio in the cache for
// this index.  Will not return a shadow, swap or DAX entry.
//
extern "C" {
    pub fn __filemap_get_folio(_arg: mapping, _arg: index, _arg: FGP_LOCK, _arg: 0) -> return;
}
//
// filemap_grab_folio - grab a folio from the page cache
// @mapping: The address space to search
// @index: The page index
//
// Looks up the page cache entry at @mapping & @index. If no folio is found,
// a new folio is created. The folio is locked, marked as accessed, and
// returned.
//
// Return: A found or created folio. ERR_PTR(-ENOMEM) if no folio is found
// and failed to create a folio.
//
// find_get_page - find and get a page reference
// @mapping: the address_space to search
// @offset: the page index
//
// Looks up the page cache slot at @mapping & @offset.  If there is a
// page cache page, it is returned with an increased refcount.
//
// Otherwise, %NULL is returned.
//
extern "C" {
    pub fn pagecache_get_page(_arg: mapping, _arg: offset, _arg: 0, _arg: 0) -> return;
}
extern "C" {
    pub fn pagecache_get_page(_arg: mapping, _arg: offset, _arg: fgp_flags, _arg: 0) -> return;
}
//
// find_lock_page - locate, pin and lock a pagecache page
// @mapping: the address_space to search
// @index: the page index
//
// Looks up the page cache entry at @mapping & @index.  If there is a
// page cache page, it is returned locked and with an increased
// refcount.
//
// Context: May sleep.
// Return: A struct page or %NULL if there is no page in the cache for this
// index.
//
extern "C" {
    pub fn pagecache_get_page(_arg: mapping, _arg: index, _arg: FGP_LOCK, _arg: 0) -> return;
}
//
// find_or_create_page - locate or add a pagecache page
// @mapping: the page's address_space
// @index: the page's index into the mapping
// @gfp_mask: page allocation mode
//
// Looks up the page cache slot at @mapping & @offset.  If there is a
// page cache page, it is returned locked and with an increased
// refcount.
//
// If the page is not present, a new page is allocated using @gfp_mask
// and added to the page cache and the VM's LRU list.  The page is
// returned locked and with an increased refcount.
//
// On memory exhaustion, %NULL is returned.
//
// find_or_create_page() may sleep, even if @gfp_flags specifies an
// atomic allocation!
//
// grab_cache_page_nowait - returns locked page at given index in given cache
// @mapping: target address_space
// @index: the page index
//
// Returns locked page at given index in given cache, creating it if
// needed, but do not wait if the page is locked or to reclaim memory.
// This is intended for speculative data generators, where the data can
// be regenerated if the page couldn't be grabbed.  This routine should
// be safe to call while holding the lock for another page.
//
// Clear __GFP_FS when allocating the page to avoid recursion into the fs
// and deadlock against the caller's locked page.
//
// folio_next_index - Get the index of the next folio.
// @folio: The current folio.
//
// Return: The index of the folio which follows this folio in the file.
//
// folio_next_pos - Get the file position of the next folio.
// @folio: The current folio.
//
// Return: The position of the folio which follows this folio in the file.
//
// folio_file_page - The page for a particular index.
// @folio: The folio which contains this index.
// @index: The index we want to look up.
//
// Sometimes after looking up a folio in the page cache, we need to
// obtain the specific page for an index (eg a page fault).
//
// Return: The page containing the file data for this index.
//
extern "C" {
    pub fn folio_page(_arg: folio, 1): index & (folio_nr_pages(folio) -) -> return;
}
//
// folio_contains - Does this folio contain this index?
// @folio: The folio.
// @index: The page index within the file.
//
// Context: The caller should have the folio locked and ensure
// e.g., shmem did not move this folio to the swap cache.
// Return: true or false.
//
extern "C" {
    pub fn read_cache_page(_arg: mapping, _arg: index, _arg: NULL, _arg: file) -> return;
}
extern "C" {
    pub fn read_cache_folio(_arg: mapping, _arg: index, _arg: NULL, _arg: file) -> return;
}
//
// page_pgoff - Calculate the logical page offset of this page.
// @folio: The folio containing this page.
// @page: The page which we need the offset of.
//
// For file pages, this is the offset from the beginning of the file
// in units of PAGE_SIZE.  For anonymous pages, this is the offset from
// the beginning of the anon_vma in units of PAGE_SIZE.  This will
// return nonsense for KSM pages.
//
// Context: Caller must have a reference on the folio or otherwise
// prevent it from being split or freed.
//
// Return: The offset in units of PAGE_SIZE.
//
// folio_pos - Returns the byte position of this folio in its file.
// @folio: The folio.
//
// Return byte-offset into filesystem object for page.
//
// Get the offset in PAGE_SIZE (even for hugetlb folios).
//
// linear_page_delta() - Determine the relative page offset of @address within
// @vma.
// @vma: The VMA in which @address resides.
// @address: The address whose relative page offset is required.
//
// The result is identical for both file-backed and anonymous mappings and
// simply determines how many pages @address lies from @vma->vm_start.
//
// Returns: The number of pages @address is offset by within @vma.
//
// linear_page_index() - Determine the absolute page offset of @address within
// @vma.
// @vma: The VMA in which @address resides.
// @address: The address whose absolute page offset is required.
//
// See the comment for vma_start_pgoff() for a description of what the page
// offset signifies.
//
// Returns: The absolute page offset of @address within @vma.
//
extern "C" {
    pub fn linear_page_delta(_arg: vma, vma_start_pgoff(vma: address) +) -> return;
}
extern "C" {
    pub fn linear_page_delta(_arg: vma, vma_start_anon_pgoff(vma: address) +) -> return;
}
//
// linear_anon_page_index() - Determine the absolute anonymous page offset of
// @address within @vma.
// @vma: An anonymous or MAP_PRIVATE file-backed VMA in which @address resides.
// @address: The address whose absolute page offset is required.
//
// This returns the anonymous page offset of @address, which is the page offset
// the address possessed at the time the VMA was first faulted.
//
// For anonymous mappings, this returns the same value as linear_page_index().
//
// For MAP_PRIVATE file-backed mappings, this returns the anonymous page offset
// of @address, which is the page offset the address possessed at the time the
// VMA was first faulted.
//
// It is not valid to call this function for shared file-backed mappings.
//
// Returns: The absolute anonymous page offset of @address within @vma.
//
// Account for MAP_PRIVATE-/dev/zero which is only semi-anonymous.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wait_page_key {
    pub folio: *mut folio,
    pub bit_nr: c_int,
    pub page_match: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wait_page_queue {
    pub folio: *mut folio,
    pub bit_nr: c_int,
    pub wait: wait_queue_entry_t,
}

extern "C" {
    pub fn __folio_lock(folio: *mut folio);
}
extern "C" {
    pub fn __folio_lock_killable(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn __folio_lock_or_retry(folio: *mut folio, vmf: *mut vm_fault) -> vm_fault_t;
}
extern "C" {
    pub fn unlock_page(page: *mut page);
}
extern "C" {
    pub fn folio_unlock(folio: *mut folio);
}
//
// folio_trylock() - Attempt to lock a folio.
// @folio: The folio to attempt to lock.
//
// Sometimes it is undesirable to wait for a folio to be unlocked (eg
// when the locks are being taken in the wrong order, or if making
// progress through a batch of folios is more important than processing
// them in order).  Usually folio_lock() is the correct function to call.
//
// Context: Any context.
// Return: Whether the lock was successfully acquired.
//
extern "C" {
    pub fn likely(_arg: !test_and_set_bit_lock(PG_locked, _arg: folio_flags(folio, _arg: 0))) -> return;
}
//
// Return true if the page was successfully locked
//
extern "C" {
    pub fn folio_trylock(_arg: page_folio(page)) -> return;
}
//
// folio_lock() - Lock this folio.
// @folio: The folio to lock.
//
// The folio lock protects against many things, probably more than it
// should.  It is primarily held while a folio is being brought uptodate,
// either from its backing file or from swap.  It is also held while a
// folio is being truncated from its address_space, so holding the lock
// is sufficient to keep folio->mapping stable.
//
// The folio lock is also held while write() is modifying the page to
// provide POSIX atomicity guarantees (as long as the write does not
// cross a page boundary).  Other modifications to the data in the folio
// do not hold the folio lock and can race with writes, eg DMA and stores
// to mapped pages.
//
// Context: May sleep.  If you need to acquire the locks of two or
// more folios, they must be in order of ascending index, if they are
// in the same address_space.  If they are in different address_spaces,
// acquire the lock of the folio which belongs to the address_space which
// has the lowest address in memory first.
//
// lock_page() - Lock the folio containing this page.
// @page: The page to lock.
//
// See folio_lock() for a description of what the lock protects.
// This is a legacy function and new code should probably use folio_lock()
// instead.
//
// Context: May sleep.  Pages in the same folio share a lock, so do not
// attempt to lock two pages which share a folio.
//
// folio_lock_killable() - Lock this folio, interruptible by a fatal signal.
// @folio: The folio to lock.
//
// Attempts to lock the folio, like folio_lock(), except that the sleep
// to acquire the lock is interruptible by a fatal signal.
//
// Context: May sleep; see folio_lock().
// Return: 0 if the lock was acquired; -EINTR if a fatal signal was received.
//
extern "C" {
    pub fn __folio_lock_killable(_arg: folio) -> return;
}
//
// folio_lock_or_retry - Lock the folio, unless this would block and the
// caller indicated that it can handle a retry.
//
// Return value and mmap_lock implications depend on flags; see
// __folio_lock_or_retry().
//
extern "C" {
    pub fn __folio_lock_or_retry(_arg: folio, _arg: vmf) -> return;
}
//
// This is exported only for folio_wait_locked/folio_wait_writeback, etc.,
// and should not be used directly.
//
extern "C" {
    pub fn folio_wait_bit(folio: *mut folio, bit_nr: c_int);
}
extern "C" {
    pub fn folio_wait_bit_killable(folio: *mut folio, bit_nr: c_int) -> c_int;
}
//
// Wait for a folio to be unlocked.
//
// This must be called with the caller "holding" the folio,
// ie with increased folio reference count so that the folio won't
// go away during the wait.
//
extern "C" {
    pub fn folio_wait_bit_killable(_arg: folio, _arg: PG_locked) -> return;
}
extern "C" {
    pub fn folio_end_read(folio: *mut folio, success: bool);
}
extern "C" {
    pub fn wait_on_page_writeback(page: *mut page);
}
extern "C" {
    pub fn folio_wait_writeback(folio: *mut folio);
}
extern "C" {
    pub fn folio_wait_writeback_killable(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn end_page_writeback(page: *mut page);
}
extern "C" {
    pub fn folio_end_writeback(folio: *mut folio);
}
extern "C" {
    pub fn folio_end_writeback_no_dropbehind(folio: *mut folio);
}
extern "C" {
    pub fn folio_end_dropbehind(folio: *mut folio);
}
extern "C" {
    pub fn folio_wait_stable(folio: *mut folio);
}
extern "C" {
    pub fn __folio_mark_dirty(folio: *mut folio, : *mut address_space, warn: c_int);
}
extern "C" {
    pub fn folio_account_cleaned(folio: *mut folio, wb: *mut bdi_writeback);
}
extern "C" {
    pub fn __folio_cancel_dirty(folio: *mut folio);
}
// Avoid atomic ops, locking, etc. when not actually needed.
extern "C" {
    pub fn folio_clear_dirty_for_io(folio: *mut folio) -> bool;
}
extern "C" {
    pub fn clear_page_dirty_for_io(page: *mut page) -> bool;
}
extern "C" {
    pub fn folio_invalidate(folio: *mut folio, offset: usize, length: usize);
}
extern "C" {
    pub fn noop_dirty_folio(mapping: *mut address_space, folio: *mut folio) -> bool;
}

extern "C" {
    pub fn folio_end_private_2(folio: *mut folio);
}
extern "C" {
    pub fn folio_wait_private_2(folio: *mut folio);
}
extern "C" {
    pub fn folio_wait_private_2_killable(folio: *mut folio) -> c_int;
}
//
// Fault in userspace address range.
//
extern "C" {
    pub fn fault_in_writeable(uaddr: *mut char __user, size: usize) -> usize;
}
extern "C" {
    pub fn fault_in_subpage_writeable(uaddr: *mut char __user, size: usize) -> usize;
}
extern "C" {
    pub fn fault_in_safe_writeable(uaddr: *const char __user, size: usize) -> usize;
}
extern "C" {
    pub fn fault_in_readable(uaddr: *const char __user, size: usize) -> usize;
}
extern "C" {
    pub fn filemap_remove_folio(folio: *mut folio);
}
extern "C" {
    pub fn __filemap_remove_folio(folio: *mut folio, shadow: *mut c_void);
}
extern "C" {
    pub fn replace_page_cache_folio(old: *mut folio, new: *mut folio);
}
extern "C" {
    pub fn filemap_release_folio(folio: *mut folio, gfp: gfp_t) -> bool;
}
// Must be non-static for BPF error injection
//
// filemap_range_needs_writeback - check if range potentially needs writeback
// @mapping:           address space within which to check
// @start_byte:        offset in bytes where the range starts
// @end_byte:          offset in bytes where the range ends (inclusive)
//
// Find at least one page in the range supplied, usually used to check if
// direct writing in this range will trigger a writeback. Used by O_DIRECT
// read/write with IOCB_NOWAIT, to see if the caller needs to do
// filemap_write_and_wait_range() before proceeding.
//
// Return: %true if the caller should do filemap_write_and_wait_range() before
// doing O_DIRECT to a page in this range, %false otherwise.
//
extern "C" {
    pub fn filemap_range_has_writeback(_arg: mapping, _arg: start_byte, _arg: end_byte) -> return;
}
//
// struct readahead_control - Describes a readahead request.
//
// A readahead request is for consecutive pages.  Filesystems which
// implement the ->readahead method should call readahead_folio() or
// __readahead_batch() in a loop and attempt to start reads into each
// folio in the request.
//
// Most of the fields in this struct are private and should be accessed
// by the functions below.
//
// @file: The file, used primarily by network filesystems for authentication.
// May be NULL if invoked internally by the filesystem.
// @mapping: Readahead this filesystem object.
// @ra: File readahead state.  May be NULL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct readahead_control {
    pub file: *mut file,
    pub mapping: *mut address_space,
    pub ra: *mut file_ra_state,
// private: use the readahead_* accessors instead
    pub _index: pgoff_t,
    pub _nr_pages: c_uint,
    pub _batch_count: c_uint,
    pub dropbehind: bool,
    pub _workingset: bool,
    pub _pflags: c_ulong,
}

extern "C" {
    pub fn page_cache_sync_ra(: *mut readahead_control, req_count: c_ulong);
}
//
// page_cache_sync_readahead - generic file readahead
// @mapping: address_space which holds the pagecache and I/O vectors
// @ra: file_ra_state which holds the readahead state
// @file: Used by the filesystem for authentication.
// @index: Index of first page to be read.
// @req_count: Total number of pages being read by the caller.
//
// page_cache_sync_readahead() should be called when a cache miss happened:
// it will submit the read.  The readahead logic may decide to piggyback more
// pages onto the read request if access patterns suggest it will improve
// performance.
//
// page_cache_async_readahead - file readahead for marked pages
// @mapping: address_space which holds the pagecache and I/O vectors
// @ra: file_ra_state which holds the readahead state
// @file: Used by the filesystem for authentication.
// @folio: The folio which triggered the readahead call.
// @req_count: Total number of pages being read by the caller.
//
// page_cache_async_readahead() should be called when a page is used which
// is marked as PageReadahead; this is a marker to suggest that the application
// has used up enough of the readahead window that we should start pulling in
// more pages.
//
// readahead_folio - Get the next folio to read.
// @ractl: The current readahead request.
//
// Context: The folio is locked.  The caller should unlock the folio once
// all I/O to that folio has completed.
// Return: A pointer to the next folio, or %NULL if we are done.
//
// readahead_pos - The byte offset into the file of this readahead request.
// @rac: The readahead request.
//
// readahead_length - The number of bytes in this readahead request.
// @rac: The readahead request.
//
// readahead_index - The index of the first page in this readahead request.
// @rac: The readahead request.
//
// readahead_count - The number of pages in this readahead request.
// @rac: The readahead request.
//
// readahead_batch_length - The number of bytes in the current batch.
// @rac: The readahead request.
//
// folio_mkwrite_check_truncate - check if folio was truncated
// @folio: the folio to check
// @inode: the inode to check the folio against
//
// Return: the number of bytes in the folio up to EOF,
// or -EFAULT if the folio was truncated.
//
// folio is wholly inside EOF
extern "C" {
    pub fn folio_size(_arg: folio) -> return;
}
// folio is wholly past EOF
// folio is partially inside EOF
//
// i_blocks_per_folio - How many blocks fit in this folio.
// @inode: The inode which contains the blocks.
// @folio: The folio.
//
// If the block size is larger than the size of this folio, return zero.
//
// Context: The caller should hold a refcount on the folio to prevent it
// from being split.
// Return: The number of filesystem blocks covered by this folio.
//
