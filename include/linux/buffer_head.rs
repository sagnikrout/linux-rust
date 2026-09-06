//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/buffer_head.h
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
// include/linux/buffer_head.h
//
// Everything to do with buffer_heads.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bh_state_bits {
    BH_Uptodate,	/* Contains valid data */
    BH_Dirty,	/* Is dirty */
    BH_Lock,	/* Is locked */
    BH_Req,		/* Has been submitted for I/O */

    BH_Mapped,	/* Has a disk mapping */
    BH_New,		/* Disk mapping was newly created by get_block */
    BH_Async_Read,	/* Is under end_buffer_async_read I/O */
    BH_Async_Write,	/* Is under end_buffer_async_write I/O */
    BH_Delay,	/* Buffer is not yet allocated on disk */
    BH_Boundary,	/* Block is followed by a discontiguity */
    BH_Write_EIO,	/* I/O error on write */
    BH_Unwritten,	/* Buffer is allocated on disk but not written */
    BH_Quiet,	/* Buffer Error Prinks to be quiet */
    BH_Meta,	/* Buffer contains metadata */
    BH_Prio,	/* Buffer should be submitted with REQ_PRIO */
    BH_Defer_Completion, /* Defer AIO completion to workqueue */
    BH_Migrate,     /* Buffer is being migrated (norefs) */

    BH_PrivateStart,/* not a state bit, but the first bit available
// for private allocation by other entities
//
}

//
// Historically, a buffer_head was used to map a single block
// within a page, and of course as the unit of I/O through the
// filesystem and block layers.  Nowadays the basic I/O unit
// is the bio, and buffer_heads are used for extracting block
// mappings (via a get_block_t call), for tracking state within
// a folio (via a folio_mapping) and for wrapping bio submission
// for backward compatibility reasons (e.g. bh_submit).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct buffer_head {
    pub /: *mut *mut unsigned long b_state; / buffer state bitmap (see above),
    pub /: *mut *mut *mut buffer_head b_this_page;/ circular list of page's buffers,
    pub /: *mut *mut *mut page b_page; / the page this bh is mapped to,
    pub /: *mut *mut *mut folio b_folio; / the folio this bh is mapped to,
}

// this buffer is associated with
// serialise IO completion of other
// buffers in the page
//
// macro tricks to expand the set_buffer_foo(), clear_buffer_foo()
// and buffer_foo() functions.
// To avoid reset buffer flags that are already set, because that causes
// a costly cache line transition, check the flag first.
//

//
// test_set_buffer_foo() and test_clear_buffer_foo()
//

//
// Emit the buffer bitops functions.   Note that there are also functions
// of the form "mark_buffer_foo()".  These are higher-level functions which
// do something in addition to setting a b_state bit.
//
// If somebody else already set this uptodate, they will
// have done the memory barrier, and a reader will thus
// see *some* valid buffer state.
//
// Any other serialization (with IO errors or whatever that
// might clear the bit) has to come from other state (eg BH_Lock).
//
// make it consistent with folio_mark_uptodate
// pairs with smp_load_acquire in buffer_uptodate
//
// make it consistent with folio_test_uptodate
// pairs with smp_mb__before_atomic in set_buffer_uptodate
//
extern "C" {
    pub fn test_bit_acquire(_arg: BH_Uptodate, _arg: &bh->b_state) -> return;
}
// If we *know* page->private refers to buffer_heads

//
// Declarations
//
extern "C" {
    pub fn mark_buffer_dirty(bh: *mut buffer_head);
}
extern "C" {
    pub fn mark_buffer_write_io_error(bh: *mut buffer_head);
}
extern "C" {
    pub fn touch_buffer(bh: *mut buffer_head);
}
extern "C" {
    pub fn end_buffer_read_sync(bh: *mut buffer_head, uptodate: c_int);
}
extern "C" {
    pub fn bio_endio_bh(bio: *mut bio, bhp: *mut buffer_head) -> bool;
}
// Completion routines suitable for passing to bh_submit()
extern "C" {
    pub fn bh_end_read(bio: *mut bio);
}
extern "C" {
    pub fn bh_end_write(bio: *mut bio);
}
extern "C" {
    pub fn bh_end_async_write(bio: *mut bio);
}
// Things to do with metadata buffers list
extern "C" {
    pub fn mmb_mark_buffer_dirty(bh: *mut buffer_head, mmb: *mut mapping_metadata_bhs);
}
extern "C" {
    pub fn __wait_on_buffer(: *mut buffer_head);
}
extern "C" {
    pub fn __brelse(: *mut buffer_head);
}
extern "C" {
    pub fn __bforget(: *mut buffer_head);
}
extern "C" {
    pub fn __breadahead(: *mut block_device, block: sector_t, size: c_uint);
}
extern "C" {
    pub fn free_buffer_head(bh: *mut *mut buffer_head);
}
extern "C" {
    pub fn unlock_buffer(bh: *mut buffer_head);
}
extern "C" {
    pub fn __lock_buffer(bh: *mut buffer_head);
}
extern "C" {
    pub fn sync_dirty_buffer(bh: *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn __sync_dirty_buffer(bh: *mut buffer_head, op_flags: blk_opf_t) -> c_int;
}
extern "C" {
    pub fn write_dirty_buffer(bh: *mut buffer_head, op_flags: blk_opf_t);
}
extern "C" {
    pub fn bh_submit(: *mut buffer_head, _arg: blk_opf_t, _arg: bio_end_io_t);
}
extern "C" {
    pub fn bh_uptodate_or_lock(bh: *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn __bh_read(bh: *mut buffer_head, op_flags: blk_opf_t, wait: bool) -> c_int;
}
//
// Generic address_space_operations implementations for buffer_head-backed
// address_spaces.
//
extern "C" {
    pub fn block_invalidate_folio(folio: *mut folio, offset: usize, length: usize);
}
extern "C" {
    pub fn block_read_full_folio(: *mut folio, : *mut get_block_t) -> c_int;
}
extern "C" {
    pub fn block_is_partially_uptodate(: *mut folio, from: usize, count: usize) -> bool;
}
extern "C" {
    pub fn block_write_end(pos: loff_t, len: unsigned, copied: unsigned, : *mut folio) -> c_int;
}
extern "C" {
    pub fn folio_zero_new_buffers(folio: *mut folio, from: usize, to: usize);
}
extern "C" {
    pub fn generic_cont_expand_simple(inode: *mut inode, size: loff_t) -> c_int;
}
extern "C" {
    pub fn block_commit_write(folio: *mut folio, from: usize, to: usize);
}
extern "C" {
    pub fn generic_block_bmap(: *mut address_space, _arg: sector_t, : *mut get_block_t) -> sector_t;
}
extern "C" {
    pub fn block_truncate_page(: *mut address_space, _arg: loff_t, : *mut get_block_t) -> c_int;
}

//
// inline definitions
//
// brelse - Release a buffer.
// @bh: The buffer to release.
//
// Decrement a buffer_head's reference count.  If @bh is NULL, this
// function is a no-op.
//
// If all buffers on a folio have zero reference count, are clean
// and unlocked, and if the folio is unlocked and not under writeback
// then try_to_free_buffers() may strip the buffers from the folio in
// preparation for freeing it (sometimes, rarely, buffers are removed
// from a folio but it ends up not being freed, and buffers may later
// be reattached).
//
// Context: Any context.
//
// bforget - Discard any dirty data in a buffer.
// @bh: The buffer to forget.
//
// Call this function instead of brelse() if the data written to a buffer
// no longer needs to be written back.  It will clear the buffer's dirty
// flag so writeback of this buffer will be skipped.
//
// Context: Any context.
//
extern "C" {
    pub fn __bread_gfp(_arg: sb->s_bdev, _arg: block, _arg: sb->s_blocksize, _arg: __GFP_MOVABLE) -> return;
}
extern "C" {
    pub fn __bread_gfp(_arg: sb->s_bdev, _arg: block, _arg: sb->s_blocksize, _arg: 0) -> return;
}
extern "C" {
    pub fn bdev_getblk(_arg: bdev, _arg: block, _arg: size, _arg: gfp) -> return;
}
extern "C" {
    pub fn bdev_getblk(_arg: bdev, _arg: block, _arg: size, _arg: gfp) -> return;
}
extern "C" {
    pub fn __getblk(_arg: sb->s_bdev, _arg: block, _arg: sb->s_blocksize) -> return;
}
extern "C" {
    pub fn bdev_getblk(_arg: sb->s_bdev, _arg: block, _arg: sb->s_blocksize, _arg: gfp) -> return;
}
extern "C" {
    pub fn __find_get_block(_arg: sb->s_bdev, _arg: block, _arg: sb->s_blocksize) -> return;
}
extern "C" {
    pub fn __find_get_block_nonatomic(_arg: sb->s_bdev, _arg: block, _arg: sb->s_blocksize) -> return;
}
extern "C" {
    pub fn likely(_arg: !test_and_set_bit_lock(BH_Lock, _arg: &bh->b_state)) -> return;
}
// Returns 1 if buffer uptodated, 0 on success, and -EIO on error.
extern "C" {
    pub fn __bh_read(_arg: bh, _arg: op_flags, _arg: true) -> return;
}
//
// __bread() - Read a block.
// @bdev: The block device to read from.
// @block: Block number in units of block size.
// @size: The block size of this device in bytes.
//
// Read a specified block, and return the buffer head that refers
// to it.  The memory is allocated from the movable area so that it can
// be migrated.  The returned buffer head has its refcount increased.
// The caller should call brelse() when it has finished with the buffer.
//
// Context: May sleep waiting for I/O.
// Return: NULL if the block was unreadable.
//
extern "C" {
    pub fn __bread_gfp(_arg: bdev, _arg: block, _arg: size, _arg: __GFP_MOVABLE) -> return;
}
//
// get_nth_bh - Get a reference on the n'th buffer after this one.
// @bh: The buffer to start counting from.
// @count: How many buffers to skip.
//
// This is primarily useful for finding the nth buffer in a folio; in
// that case you pass the head buffer and the byte offset in the folio
// divided by the block size.  It can be used for other purposes, but
// it will wrap at the end of the folio rather than returning NULL or
// proceeding to the next folio for you.
//
// Return: The requested buffer with an elevated refcount.
//
extern "C" {
    pub fn block_dirty_folio(mapping: *mut address_space, folio: *mut folio) -> bool;
}

extern "C" {
    pub fn buffer_init();
}
extern "C" {
    pub fn try_to_free_buffers(folio: *mut folio) -> bool;
}
extern "C" {
    pub fn mmb_init(mmb: *mut mapping_metadata_bhs, mapping: *mut address_space);
}
extern "C" {
    pub fn mmb_has_buffers(mmb: *mut mapping_metadata_bhs) -> bool;
}
extern "C" {
    pub fn mmb_invalidate(mmb: *mut mapping_metadata_bhs);
}
extern "C" {
    pub fn mmb_sync(mmb: *mut mapping_metadata_bhs) -> c_int;
}
extern "C" {
    pub fn invalidate_bh_lrus();
}
extern "C" {
    pub fn invalidate_bh_lrus_cpu();
}
extern "C" {
    pub fn has_bh_in_lru(cpu: c_int, dummy: *mut c_void) -> bool;
}

pub const buffer_heads_over_limit: c_int = 0;

