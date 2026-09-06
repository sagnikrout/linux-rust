//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/journal.h
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
// journal.h
//
// Defines journalling api and structures.
//
// Copyright (C) 2003, 2005 Oracle.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_journal_state {
    OCFS2_JOURNAL_FREE = 0,
    OCFS2_JOURNAL_LOADED,
    OCFS2_JOURNAL_IN_SHUTDOWN,
}

//
// The recovery_list is a simple linked list of node numbers to recover.
// It is protected by the recovery_lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_recovery_map {
    pub rm_used: c_uint,
    pub rm_entries: [c_uint; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_journal {
    pub /: *mut *mut ocfs2_journal_state j_state; / Journals current state,
    pub /: *mut *mut *mut journal_t j_journal; / The kernels journal type,
    pub to: *mut *mut *mut inode j_inode; / Kernel inode pointing,
// this journal
    pub super: *mut *mut *mut ocfs2_super j_osb; / pointer to the,
// block for the node
// we're currently
// running on -- not
// necessarily the super
// block from the node
// which we usually run
// from (recovery,
// etc)
    pub /: *mut *mut *mut buffer_head j_bh; / Journal disk inode block,
    pub transactions: *mut *mut atomic_t j_num_trans; / Number of,
// currently in the system.
    pub j_lock: spinlock_t,
    pub j_trans_id: c_ulong,
    pub j_trans_barrier: rw_semaphore,
    pub j_checkpointed: wait_queue_head_t,
// both fields protected by j_lock
    pub j_la_cleanups: list_head,
    pub j_recovery_work: work_struct,
}

// wrap j_trans_id so we never have it equal to zero.
// Used to figure out whether it's safe to drop a metadata lock on an
// cached object. Returns true if all the object's changes have been
// checkpointed to disk. You should be holding the spinlock on the
// metadata lock while calling this to be sure that nobody can take
// the lock and put it on another transaction.
// convenience function to check if an object backed by struct
// ocfs2_caching_info  is still new (has never hit disk) Will do you a
// favor and set created_trans = 0 when you've
// been checkpointed.  returns '1' if the ci is still new.
// Wrapper for inodes so we can check system files
// System files are never "new" as they're written out by
// mkfs. This helps us early during mount, before we have the
// journal open and j_trans_id could be junk.
extern "C" {
    pub fn ocfs2_ci_is_new(_arg: INODE_CACHE(inode)) -> return;
}
// Exported only for the journal struct init code in super.c. Do not call.
extern "C" {
    pub fn ocfs2_orphan_scan_init(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_orphan_scan_start(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_orphan_scan_stop(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_complete_recovery(work: *mut work_struct);
}
extern "C" {
    pub fn ocfs2_wait_for_recovery(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_recovery_init(osb: *mut ocfs2_super) -> c_int;
}
extern "C" {
    pub fn ocfs2_recovery_exit(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_recovery_disable_quota(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_compute_replay_slots(osb: *mut ocfs2_super) -> c_int;
}
extern "C" {
    pub fn ocfs2_free_replay_slots(osb: *mut ocfs2_super);
}
//
// Journal Control:
// Initialize, Load, Shutdown, Wipe a journal.
//
// ocfs2_journal_alloc    - Initialize skeleton for journal structure.
// ocfs2_journal_init     - Initialize journal structures in the OSB.
// ocfs2_journal_load     - Load the given journal off disk. Replay it if
// there's transactions still in there.
// ocfs2_journal_shutdown - Shutdown a journal, this will flush all
// uncommitted, uncheckpointed transactions.
// ocfs2_journal_wipe     - Wipe transactions from a journal. Optionally
// zero out each block.
// ocfs2_recovery_thread  - Perform recovery on a node. osb is our own osb.
// ocfs2_mark_dead_nodes - Start recovery on nodes we won't get a heartbeat
// event on.
// ocfs2_start_checkpoint - Kick the commit thread to do a checkpoint.
//
extern "C" {
    pub fn ocfs2_set_journal_params(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_journal_alloc(osb: *mut ocfs2_super) -> c_int;
}
extern "C" {
    pub fn ocfs2_journal_init(osb: *mut ocfs2_super, dirty: *mut c_int) -> c_int;
}
extern "C" {
    pub fn ocfs2_journal_shutdown(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_check_journals_nolocks(osb: *mut ocfs2_super) -> c_int;
}
extern "C" {
    pub fn ocfs2_mark_dead_nodes(osb: *mut ocfs2_super) -> c_int;
}
extern "C" {
    pub fn ocfs2_complete_mount_recovery(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_complete_quota_recovery(osb: *mut ocfs2_super);
}
// WARNING: This only kicks off a single
// checkpoint. If someone races you and adds more
// metadata to the journal, you won't know, and will
// wind up waiting *a lot* longer than necessary. Right
// now we only use this in clear_inode so that's
// OK.
//
// Transaction Handling:
// Manage the lifetime of a transaction handle.
//
// ocfs2_start_trans      - Begin a transaction. Give it an upper estimate of
// the number of blocks that will be changed during
// this handle.
// ocfs2_commit_trans - Complete a handle. It might return -EIO if
// the journal was aborted. The majority of paths don't
// check the return value as an error there comes too
// late to do anything (and will be picked up in a
// later transaction).
// ocfs2_extend_trans     - Extend a handle by nblocks credits. This may
// commit the handle to disk in the process, but will
// not release any locks taken during the transaction.
// ocfs2_journal_access* - Notify the handle that we want to journal this
// buffer. Will have to call ocfs2_journal_dirty once
// we've actually dirtied it. Type is one of . or .
// Always call the specific flavor of
// ocfs2_journal_access_*() unless you intend to
// manage the checksum by hand.
// ocfs2_journal_dirty    - Mark a journalled buffer as having dirty data.
// ocfs2_jbd2_inode_add_write  - Mark an inode with range so that its data goes
// out before the current handle commits.
//
// You must always start_trans with a number of buffs > 0, but it's
// perfectly legal to go through an entire transaction without having
// dirtied any buffers.
extern "C" {
    pub fn ocfs2_extend_trans(handle: *mut handle_t, nblocks: c_int) -> c_int;
}
//
// Define an arbitrary limit for the amount of data we will anticipate
// writing to any given transaction.  For unbounded transactions such as
// fallocate(2) we can write more than this, but we always
// start off at the maximum transaction size and grow the transaction
// optimistically as we go.
//

//
// Create access is for when we get a newly created buffer and we're
// not gonna read it off disk, but rather fill it ourselves.  Right
// now, we don't do anything special with this (it turns into a write
// request), but this is a good placeholder in case we do...
//
// Write access is for when we read a block off disk and are going to
// modify it. This way the journalling layer knows it may need to make
// a copy of that block (if it's part of another, uncommitted
// transaction) before we do so.
//
pub const OCFS2_JOURNAL_ACCESS_CREATE: c_int = 0;
pub const OCFS2_JOURNAL_ACCESS_WRITE: c_int = 1;
pub const OCFS2_JOURNAL_ACCESS_UNDO: c_int = 2;
// ocfs2_inode
// ocfs2_extent_block
// ocfs2_refcount_block
// ocfs2_group_desc
// ocfs2_xattr_block
// quota blocks
// dirblock
// ocfs2_dx_root_block
// ocfs2_dx_leaf
// Anything that has no ecc
//
// A word about the journal_access/journal_dirty "dance". It is
// entirely legal to journal_access a buffer more than once (as long
// as the access type is the same -- I'm not sure what will happen if
// access type is different but this should never happen anyway) It is
// also legal to journal_dirty a buffer more than once. In fact, you
// can even journal_access a buffer after you've done a
// journal_access/journal_dirty pair. The only thing you cannot do
// however, is journal_dirty a buffer which you haven't yet passed to
// journal_access at least once.
//
// That said, 99% of the time this doesn't matter and this is what the
// path looks like:
//
// <read a bh>
// ocfs2_journal_access(handle, bh,	OCFS2_JOURNAL_ACCESS_WRITE);
// <modify the bh>
// ocfs2_journal_dirty(handle, bh);
//
extern "C" {
    pub fn ocfs2_journal_dirty(handle: *mut handle_t, bh: *mut buffer_head);
}
//
// Credit Macros:
// Convenience macros to calculate number of credits needed.
//
// For convenience sake, I have a set of macros here which calculate
// the *maximum* number of sectors which will be changed for various
// metadata updates.
//
// simple file updates like chmod, etc.
pub const OCFS2_INODE_UPDATE_CREDITS: c_int = 1;
// extended attribute block update
pub const OCFS2_XATTR_BLOCK_UPDATE_CREDITS: c_int = 1;
// Update of a single quota block
pub const OCFS2_QUOTA_BLOCK_UPDATE_CREDITS: c_int = 1;
// global quotafile inode update, data block

//
// The two writes below can accidentally see global info dirty due
// to set_info() quotactl so make them prepared for the writes.
//
// quota data block, global info
// Write to local quota file

// global quota data block, local quota data block, global quota inode,
// global quota info

// group extend. inode update and last group update.

// group add. inode update and the new group update.

// get one bit out of a suballocator: dinode + group descriptor +
// prev. group desc. if we relink.

// dinode + group descriptor update. We don't relink on free yet.

// data block for new dir/symlink, allocation of directory block, dx_root
// update for free list

// 1 block for index, 2 allocs (data, metadata), 1 clusters
// worth of blocks for initial extent.
// parent fe, parent block, new file entry, index leaf, inode alloc fe, inode
// alloc group descriptor + mkdir/symlink blocks + dir blocks + xattr
// blocks + quota update
// local alloc metadata change + main bitmap updates

// used when we don't need an allocation change for a dir extend. One
// for the dinode, one for the new block.

// file update (nlink, etc) + directory mtime/ctime + dir entry block + quota
// update on dir + index leaf + dx root update for free list +
// previous dirblock update in the free list
// inode + dir inode (if we unlink a dir), + dir entry block + orphan
// dir inode link + dir inode index leaf + dir index root
// The quota update from ocfs2_link_credits is unused here...
// dinode + orphan dir dinode + inode alloc dinode + orphan dir entry +
// inode alloc group descriptor + orphan dir index root +
// orphan dir index leaf

// dinode + orphan dir dinode + extent tree leaf block + orphan dir entry +
// orphan dir index root + orphan dir index leaf

// dinode update, old dir dinode update, new dir dinode update, old
// dir dir entry, new dir dir entry, dir entry update for renaming
// directory + target unlink + 3 x dir index leaves
// global bitmap dinode, group desc., relinked group,
// suballocator dinode, group desc., relinked group,
// dinode, xattr block

// inode update, removal of dx root block from allocator

// inode update, new refcount block and its allocation credits.

// inode and the refcount block update.

//
// inode and the refcount block update.
// It doesn't include the credits for sub alloc change.
// So if we need to free the bit, OCFS2_SUBALLOC_FREE needs to be added.
//

// 2 metadata alloc, 2 new blocks and root refcount block

//
// Please note that the caller must make sure that root_el is the root
// of extent tree. So for an inode, it should be &fe->id2.i_list. Otherwise
// the result may be wrong.
//
// bitmap dinode, group desc. + relinked group.
// we might need to shift tree depth so lets assume an
// absolute worst case of complete fragmentation.  Even with
// that, we only need one update for the dinode, and then
// however many metadata chunks needed * a remaining suballoc
// alloc.
// this does not include *new* metadata blocks, which are
// accounted for in sysfile_bitmap_blocks. root_el +
// prev. last_eb_blk + blocks along edge of tree.
// calc_symlink_credits passes because we just need 1
// credit for the dinode there.
// links can be longer than one block so we may update many
// within our single allocated extent.
// parent inode update + new block group header + bitmap inode update
//
// Allocating a discontiguous block group requires the credits from
// ocfs2_calc_group_alloc_credits() as well as enough credits to fill
// the group descriptor's extent list.  The caller already has started
// the transaction with ocfs2_calc_group_alloc_credits().  They extend
// it with these credits.
//
extern "C" {
    pub fn ocfs2_extent_recs_per_gd(_arg: sb) -> return;
}
