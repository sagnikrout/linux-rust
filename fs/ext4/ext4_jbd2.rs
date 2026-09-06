//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ext4/ext4_jbd2.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// ext4_jbd2.h
//
// Written by Stephen C. Tweedie <sct@redhat.com>, 1999
//
// Copyright 1998--1999 Red Hat corp --- All Rights Reserved
//
// Ext4-specific journaling extensions.
//

// Define the number of blocks we need to account to a transaction to
// modify one block of data.
//
// We may have to touch one inode, one bitmap buffer, up to three
// indirection blocks, the group and superblock summaries, and the data
// block to complete the transaction.
//
// For extents-enabled fs we may have to allocate and modify up to
// 5 levels of tree, data block (for each of these we need bitmap + group
// summaries), root which is stored in the inode, sb
//

// Extended attribute operations touch at most two data buffers,
// two bitmap buffers, and two group summaries, in addition to the inode
// and the superblock, which are already accounted for.

// Define the minimum size for a transaction which modifies data.  This
// needs to take into account the fact that we may end up modifying two
// quota files too (one for the group, one for the user quota).  The
// superblock only gets updated once, of course, so don't bother
// counting that again for the quota updates.

//
// Define the number of metadata blocks we need to account to modify data.
//
// This include super block, inode block, quota blocks and xattr blocks
//

// Define an arbitrary limit for the amount of data we will anticipate
// writing to any given transaction.  For unbounded transactions such as
// write(2) and truncate(2) we can write more than this, but we always
// start off at the maximum transaction size and grow the transaction
// optimistically as we go.

// We break up a large truncate or write transaction once the handle's
// buffer credits gets this low, we need either to extend the
// transaction or to start a new one.  Reserve enough space here for
// inode, bitmap, superblock, group and indirection updates for at least
// one block, plus two quota updates.  Quota allocations are not
// needed.

//
// Number of credits needed if we need to insert an entry into a
// directory.  For each new index block, we need 4 blocks (old index
// block, new index block, bitmap block, bg summary).  For normal
// htree directories there are 2 levels; if the largedir feature
// enabled it's 3 levels.
//

// Amount of blocks needed for quota update - we know that the structure was
// allocated so we need to update only data block

// Amount of blocks needed for quota insert/delete - we do some block writes
// but inode, sb and group updates are done only once

pub const EXT4_QUOTA_TRANS_BLOCKS(sb): c_int = 0;
pub const EXT4_QUOTA_INIT_BLOCKS(sb): c_int = 0;
pub const EXT4_QUOTA_DEL_BLOCKS(sb): c_int = 0;

//
// Ext4 handle operation types -- for logging purposes
//
pub const EXT4_HT_MISC: c_int = 0;
pub const EXT4_HT_INODE: c_int = 1;
pub const EXT4_HT_WRITE_PAGE: c_int = 2;
pub const EXT4_HT_MAP_BLOCKS: c_int = 3;
pub const EXT4_HT_DIR: c_int = 4;
pub const EXT4_HT_TRUNCATE: c_int = 5;
pub const EXT4_HT_QUOTA: c_int = 6;
pub const EXT4_HT_RESIZE: c_int = 7;
pub const EXT4_HT_MIGRATE: c_int = 8;
pub const EXT4_HT_MOVE_EXTENTS: c_int = 9;
pub const EXT4_HT_XATTR: c_int = 10;
pub const EXT4_HT_EXT_CONVERT: c_int = 11;
pub const EXT4_HT_MAX: c_int = 12;
//
// On success, We end up with an outstanding reference count against
// iloc->bh.  This _must_ be cleaned up later.
//

//
// Wrapper functions with which ext4 calls into JBD.
//

extern "C" {
    pub fn __ext4_journal_stop(where: *const c_char, line: c_uint, handle: *mut handle_t) -> c_int;
}
// Note:  Do not use this for NULL handles.  This is only to determine if
// a properly allocated handle is using a journal or not.
extern "C" {
    pub fn is_handle_aborted(_arg: handle) -> return;
}
// Freeing each metadata block can result in freeing one cluster
extern "C" {
    pub fn ext4_free_metadata_revoke_credits(_arg: sb, _arg: 8) -> return;
}

extern "C" {
    pub fn journal_current_handle() -> return;
}
extern "C" {
    pub fn jbd2_journal_extend(_arg: handle, _arg: nblocks, _arg: revoke) -> return;
}
extern "C" {
    pub fn jbd2__journal_restart(_arg: handle, _arg: nblocks, _arg: revoke, _arg: GFP_NOFS) -> return;
}
//
// Ensure @handle has at least @check_creds credits available. If not,
// transaction will be extended or restarted to contain at least @extend_cred
// credits. Before restarting transaction @fn is executed to allow for cleanup
// before the transaction is restarted.
//
// The return value is < 0 in case of error, 0 in case the handle has enough
// credits or transaction extension succeeded, 1 in case transaction had to be
// restarted.
//

//
// Ensure given handle has at least requested amount of credits available,
// possibly restarting transaction if needed. We also make sure the transaction
// has space for at least ext4_trans_default_revoke_credits(sb) revoke records
// as freeing one or two blocks is very common pattern and requesting this is
// very cheap.
//
extern "C" {
    pub fn jbd2_journal_blocks_per_folio(_arg: inode) -> return;
}
extern "C" {
    pub fn jbd2_journal_force_commit(_arg: journal) -> return;
}
// super.c
extern "C" {
    pub fn ext4_force_commit(sb: *mut super_block) -> c_int;
}
//
// Ext4 inode journal modes
//
pub const EXT4_INODE_JOURNAL_DATA_MODE: c_uint = 0x01 /* journal data mode */;
pub const EXT4_INODE_ORDERED_DATA_MODE: c_uint = 0x02 /* ordered data mode */;
pub const EXT4_INODE_WRITEBACK_DATA_MODE: c_uint = 0x04 /* writeback data mode */;
extern "C" {
    pub fn ext4_inode_journal_mode(inode: *mut inode) -> c_int;
}
//
// Data blocks in one extent are contiguous, just account for partial
// clusters at extent boundaries
//
// This function controls whether or not we should try to go down the
// dioread_nolock code paths, which makes it safe to avoid taking
// i_rwsem for direct I/O reads.  This only works for extent-based
// files, and it doesn't work if data journaling is enabled, since the
// dioread_nolock code uses b_private to pass information back to the
// I/O completion handler, and this conflicts with the jbd's use of
// b_private.
//
// temporary fix to prevent generic/422 test failures
//
// Pass journal explicitly as it may not be cached in the sbi->s_journal in some
// cases
//
// At this point only two things can be operating on the journal.
// JBD2 thread performing transaction commit and s_sb_upd_work
// issuing sb update through the journal. Once we set
// EXT4_JOURNAL_DESTROY, new ext4_handle_error() calls will not
// queue s_sb_upd_work and ext4_force_commit() makes sure any
// ext4_handle_error() calls from the running transaction commit are
// finished. Hence no new s_sb_upd_work can be queued after we
// flush it here.
//
