//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iomap.h
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
pub const LINUX_IOMAP_H: c_int = 1;

//
// Types of block ranges for iomap mappings:
//

//
// Flags reported by the file system from iomap_begin:
//
// IOMAP_F_NEW indicates that the blocks have been newly allocated and need
// zeroing for areas that no data is copied to.
//
// IOMAP_F_DIRTY indicates the inode has uncommitted metadata needed to access
// written data and requires fdatasync to commit them to persistent storage.
// This needs to take into account metadata changes that *may* be made at IO
// completion, such as file size updates from direct IO.
//
// IOMAP_F_SHARED indicates that the blocks are shared, and will need to be
// unshared as part a write.
//
// IOMAP_F_MERGED indicates that the iomap contains the merge of multiple block
// mappings.
//
// IOMAP_F_BUFFER_HEAD indicates that the file system requires the use of
// buffer heads for this mapping.
//
// IOMAP_F_XATTR indicates that the iomap is for an extended attribute extent
// rather than a file data extent.
//
// IOMAP_F_BOUNDARY indicates that I/O and I/O completions for this iomap must
// never be merged with the mapping before it.
//
// IOMAP_F_ANON_WRITE indicates that (write) I/O does not have a target block
// assigned to it yet and the file system will do that in the bio submission
// handler, splitting the I/O as needed.
//
// IOMAP_F_ATOMIC_BIO indicates that (write) I/O will be issued as an atomic
// bio, i.e. set REQ_ATOMIC.
//
// IOMAP_F_INTEGRITY indicates that the filesystems handles integrity metadata.
//
// IOMAP_F_ZERO_TAIL indicates the remainder of the block after the data
// written should be zeroed.
//

pub const IOMAP_F_BUFFER_HEAD: c_int = 0;

pub const IOMAP_F_INTEGRITY: c_int = 0;

//
// Indicates reads and writes of fsverity metadata.
//
// Fsverity metadata is stored after the regular file data and thus beyond
// i_size.
//

//
// Flag reserved for file system specific usage
//

//
// Flags set by the core iomap code during operations:
//
// IOMAP_F_FOLIO_BATCH indicates that the folio batch mechanism is active
// for this operation, set by iomap_fill_dirty_folios().
//
// IOMAP_F_SIZE_CHANGED indicates to the iomap_end method that the file size
// has changed as the result of this write operation.
//
// IOMAP_F_STALE indicates that the iomap is not valid any longer and the file
// range it covers needs to be remapped by the high level before the operation
// can proceed.
//

//
// Magic value for addr:
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iomap {
    pub /: *mut *mut u64 addr; / disk offset of mapping, bytes,
    pub /: *mut *mut loff_t offset; / file offset of mapping, bytes,
    pub /: *mut *mut u64 length; / length of mapping, bytes,
    pub /: *mut *mut u16 type; / type of mapping,
    pub /: *mut *mut u16 flags; / flags for mapping,
    pub /: *mut *mut *mut block_device bdev; / block device for I/O,
    pub /: *mut *mut *mut dax_device dax_dev; / dax_dev for dax operations,
    pub inline_data: *mut c_void,
    pub /: *mut *mut *mut void private; / filesystem private,
    pub /: *mut *mut u64 validity_cookie; / used with .iomap_valid(),
}

//
// Returns the inline data pointer for logical offset @pos.
//
// When get_folio succeeds, put_folio will always be called to do any
// cleanup work necessary.  put_folio is responsible for unlocking and putting
// @folio.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iomap_write_ops {
    pub len): unsigned,
    pub folio): *mut folio,
//
// Check that the cached iomap still maps correctly to the filesystem's
// internal extent map. FS internal extent maps can change while iomap
// is iterating a cached iomap, so this hook allows iomap to detect that
// the iomap needs to be refreshed during a long running write
// operation.
//
// The filesystem can store internal state (e.g. a sequence number) in
// iomap->validity_cookie when the iomap is first mapped to be able to
// detect changes between mapping time and whenever .iomap_valid() is
// called.
//
// This is called with the folio over the specified file position held
// locked by the iomap code.
//
    pub iomap): *const *const *const bool (iomap_valid)(struct inode inode, struct iomap,
//
// Optional if the filesystem wishes to provide a custom handler for
// reading in the contents of a folio, otherwise iomap will default to
// submitting a bio read request.
//
// The read must be done synchronously.
//
    pub len): *mut *mut folio folio, loff_t pos, size_t,
}

//
// Flags for iomap_begin / iomap_end.  No flag implies a read.
//

pub const IOMAP_DAX: c_int = 0;

//
// Return the existing mapping at pos, or reserve space starting at pos for up
// to length, as long as we can do it as a single mapping.
// The actual length is returned in iomap->length.
//
// Commit and/or unreserve space previously allocated by iomap_iter_begin_fn.
// Written indicates the length of the successful write operation which needs
// to be committed, while the rest needs to be unreserved.
// Written might be zero if no data was written.
//
// Produce the next mapping (finishing the previous one if needed).
// Return 1 to continue iterating, 0 if the range is fully consumed, or a
// negative error on failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iomap_ops {
    pub iomap_begin: iomap_iter_begin_fn,
    pub iomap_end: iomap_iter_end_fn,
    pub iomap_next: iomap_iter_next_fn,
}

//
// struct iomap_iter - Iterate through a range of a file
// @inode: Set at the start of the iteration and should not change.
// @pos: The current file position we are operating on.  It is updated by
// calls to iomap_iter().  Treat as read-only in the body.
// @len: The remaining length of the file segment we're operating on.
// It is updated at the same time as @pos.
// @iter_start_pos: The original start pos for the current iomap. Used for
// incremental iter advance.
// @status: Status of the most recent iteration. Zero on success or a negative
// errno on error.
// @flags: Zero or more of the iomap_begin flags above.
// @iomap: Map describing the I/O iteration
// @srcmap: Source map for COW operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iomap_iter {
    pub inode: *mut inode,
    pub pos: loff_t,
    pub len: u64,
    pub iter_start_pos: loff_t,
    pub status: c_int,
    pub flags: unsigned,
    pub iomap: iomap,
    pub srcmap: iomap,
    pub fbatch: *mut folio_batch,
    pub private: *mut c_void,
}

extern "C" {
    pub fn iomap_iter(iter: *mut iomap_iter, ops: *const iomap_ops) -> c_int;
}
extern "C" {
    pub fn iomap_iter_advance(iter: *mut iomap_iter, count: u64) -> c_int;
}
//
// iomap_length_trim - trimmed length of the current iomap iteration
// @iter: iteration structure
// @pos: File position to trim from.
// @len: Length of the mapping to trim to.
//
// Returns a trimmed length that the operation applies to for the current
// iteration.
//
extern "C" {
    pub fn min(_arg: len, pos: end -) -> return;
}
//
// iomap_length - length of the current iomap iteration
// @iter: iteration structure
//
// Returns the length that the operation applies to for the current iteration.
//
extern "C" {
    pub fn iomap_length_trim(_arg: iter, _arg: iter->pos, _arg: iter->len) -> return;
}
//
// iomap_iter_advance_full - advance by the full length of current map
//
extern "C" {
    pub fn iomap_iter_advance(_arg: iter, _arg: iomap_length(iter)) -> return;
}
//
// iomap_iter_srcmap - return the source map for the current iomap iteration
// @i: iteration structure
//
// Write operations on file systems with reflink support might require a
// source and a destination map.  This function retourns the source map
// for a given operation, which may or may no be identical to the destination
// map in &i->iomap.
//
// iomap_iter_next - finish the previous mapping and produce the next one
// @iter: iteration structure
// @iomap: mapping to finish and then repopulate
// @srcmap: source mapping to finish and then repopulate
// @begin: callback that produces a mapping for the current position
// @end: optional callback that finishes the previous mapping, or NULL
//
// Inline helper that implements the common body of an ->iomap_next()
// callback: it finishes the previous mapping via @end (if present), decides
// via iomap_iter_continue() whether to keep going, and obtains the next
// mapping via @begin.
//
// This helper is marked __always_inline so that when a caller passes
// compile-time-constant @begin and @end callbacks, the compiler can call them
// directly, avoiding the indirect-call overhead.
//
// Returns 1 to continue iterating, 0 once the range is fully consumed, or a
// negative errno on error.
//
// Calculate how far the iter was advanced and the
// original length bytes for end().
//

//
// Return the file offset for the first unchanged block after a short write.
//
// If nothing was written, round @pos down to point at the first block in
// the range, else round up to include the partially written block.
//
extern "C" {
    pub fn round_down(_arg: pos, _arg: i_blocksize(inode)) -> return;
}
extern "C" {
    pub fn round_up(written: pos +, _arg: i_blocksize(inode)) -> return;
}
//
// Check if the range needs to be unshared for a FALLOC_FL_UNSHARE_RANGE
// operation.
//
// Don't bother with blocks that are not shared to start with; or mappings that
// cannot be shared, such as inline data, delalloc reservations, holes or
// unwritten extents.
//
// Note that we use srcmap directly instead of iomap_iter_srcmap as unsharing
// requires providing a separate source map, and the presence of one is a good
// indicator that unsharing is needed, unlike IOMAP_F_SHARED which can be set
// for any data that goes into the COW fork for XFS.
//
extern "C" {
    pub fn iomap_is_partially_uptodate(: *mut folio, from: usize, count: usize) -> bool;
}
extern "C" {
    pub fn iomap_release_folio(folio: *mut folio, gfp_flags: gfp_t) -> bool;
}
extern "C" {
    pub fn iomap_invalidate_folio(folio: *mut folio, offset: usize, len: usize);
}
extern "C" {
    pub fn iomap_dirty_folio(mapping: *mut address_space, folio: *mut folio) -> bool;
}
extern "C" {
    pub fn iomap_folio_mark_uptodate(folio: *mut folio);
}
//
// Flags for iomap_ioend->io_flags.
//
// shared COW extent

// unwritten extent

// don't merge into previous ioend

// is direct I/O

//
// Flags that if set on either ioend prevent the merge of two ioends.
// (IOMAP_IOEND_BOUNDARY also prevents merges, but only one-way)
//

//
// Structure for writeback I/O completions.
//
// File systems can split a bio generated by iomap.  In that case the parent
// ioend it was split from is recorded in ioend->io_parent.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iomap_ioend {
    pub /: *mut *mut list_head io_list; / next ioend in chain,
    pub /: *mut *mut *mut u16 io_flags; / IOMAP_IOEND_,
    pub /: *mut *mut *mut inode io_inode; / file being written to,
    pub /: *mut *mut size_t io_size; / size of the extent,
    pub /: *mut *mut atomic_t io_remaining; / completetion defer count,
    pub /: *mut *mut int io_error; / stashed away status,
    pub /: *mut *mut *mut iomap_ioend io_parent; / parent for completions,
    pub /: *mut *mut loff_t io_offset; / offset in the file,
    pub /: *mut *mut sector_t io_sector; / start sector of ioend,
    pub /: *mut *mut *mut void io_private; / file system private data,
    pub /: *mut *mut *mut fsverity_info io_vi; / fsverity info,
    pub /: *mut *mut bio io_bio; / MUST BE LAST!,
}

extern "C" {
    pub fn container_of(_arg: bio, iomap_ioend: struct, _arg: io_bio) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iomap_writeback_ops {
//
// Performs writeback on the passed in range
//
// Can map arbitrarily large regions, but we need to call into it at
// least once per folio to allow the file systems to synchronize with
// the write path that could be invalidating mappings.
//
// An existing mapping from a previous call to this method can be reused
// by the file system if it is still valid.
//
// If this succeeds, iomap_finish_folio_write() must be called once
// writeback completes for the range, regardless of whether the
// writeback succeeded or failed.
//
// Returns the number of bytes processed or a negative errno.
//
    pub end_pos): u64,
//
// Submit a writeback context previously build up by ->writeback_range.
//
// Returns 0 if the context was successfully submitted, or a negative
// error code if not.  If @error is non-zero a failure occurred, and
// the writeback context should be completed with an error.
//
    pub error): *mut *mut *mut int (writeback_submit)(struct iomap_writepage_ctx wpc, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iomap_writepage_ctx {
    pub iomap: iomap,
    pub inode: *mut inode,
    pub wbc: *mut writeback_control,
    pub ops: *const iomap_writeback_ops,
    pub /: *mut *mut u32 nr_folios; / folios added to the ioend,
    pub /: *mut *mut *mut void wb_ctx; / pending writeback context,
}

extern "C" {
    pub fn iomap_finish_ioends(ioend: *mut iomap_ioend, error: c_int);
}
extern "C" {
    pub fn iomap_sort_ioends(ioend_list: *mut list_head);
}
extern "C" {
    pub fn iomap_ioend_writeback_submit(wpc: *mut iomap_writepage_ctx, error: c_int) -> c_int;
}
extern "C" {
    pub fn iomap_writeback_folio(wpc: *mut iomap_writepage_ctx, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn iomap_writepages(wpc: *mut iomap_writepage_ctx) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iomap_read_folio_ctx {
    pub ops: *const iomap_read_ops,
    pub cur_folio: *mut folio,
    pub rac: *mut readahead_control,
    pub read_ctx: *mut c_void,
    pub read_ctx_file_offset: loff_t,
    pub vi: *mut fsverity_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iomap_read_ops {
//
// Read in a folio range.
//
// If this succeeds, iomap_finish_folio_read() must be called after the
// range is read in, regardless of whether the read succeeded or failed.
//
// Returns 0 on success or a negative error on failure.
//
    pub len): *mut *mut iomap_read_folio_ctx ctx, size_t,
//
// Submit any pending read requests.
//
// This is optional.
//
    pub ctx): *mut iomap_read_folio_ctx,
//
// Optional, allows filesystem to specify own bio_set, so new bio's
// can be allocated from the provided bio_set.
//
    pub bio_set: *mut bio_set,
}

//
// Flags for direct I/O ->end_io:
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iomap_dio_ops {
    pub flags): unsigned,
    pub file_offset): loff_t,
//
// Filesystems wishing to attach private information to a direct io bio
// must provide a ->submit_io method that attaches the additional
// information to the bio and changes the ->bi_end_io callback to a
// custom function.  This function should, at a minimum, perform any
// relevant post-processing of the bio and end with a call to
// iomap_dio_bio_end_io.
//
    pub bio_set: *mut bio_set,
}

//
// Wait for the I/O to complete in iomap_dio_rw even if the kiocb is not
// synchronous.
//

//
// Do not allocate blocks or zero partial blocks, but instead fall back to
// the caller by returning -EAGAIN.  Used to optimize direct I/O writes that
// are not aligned to the file system block size.
//

//
// When a page fault occurs, return a partial synchronous result and allow
// the caller to retry the rest of the operation after dealing with the page
// fault.
//

//
// Ensure each bio is aligned to fs block size.
//
// For filesystems which need to calculate/verify the checksum of each fs
// block. Otherwise they may not be able to handle unaligned bios.
//

//
// Bounce buffer instead of using zero copy access.
//
// This is needed if the device needs stable data to checksum or generate
// parity.  The file system must hook into the I/O submission and offload
// completions to user context for reads when this is set.
//

extern "C" {
    pub fn iomap_dio_complete(dio: *mut iomap_dio) -> isize;
}
extern "C" {
    pub fn iomap_dio_bio_end_io(bio: *mut bio);
}
//
// Fast path for small, block-aligned direct I/Os that map to a single
// contiguous on-disk extent.
//
// @iter must describe a non-empty READ no larger than the inode block size:
// writes, zero-length I/O, and larger requests need the generic iomap direct
// I/O path.
//
// Does not support iomap_dio_ops, dio_flags, done_before or private data.
// The range must also stay within i_size and encrypted inodes must use the
// generic iomap direct I/O path.
//
// -ENOTBLK indicates the generic path must be used by the caller instead.
// Any other errno is a real result and is propagated as-is, in particular
// -EAGAIN for IOCB_NOWAIT must reach the caller.
//
// The caller can only provide an iomap begin handler, and the iterator
// is never advanced.
//
// Simple dio is an optimization for small IO. Filter out large IO
// early as it's the most common case to fail for typical direct IO
// workloads.
//
extern "C" {
    pub fn __iomap_dio_read_simple(_arg: iocb, _arg: iter, _arg: &iomi) -> return;
}

