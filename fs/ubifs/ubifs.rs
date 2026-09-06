//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ubifs/ubifs.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// This file is part of UBIFS.
//
// Copyright (C) 2006-2008 Nokia Corporation
//
// Authors: Artem Bityutskiy (Битюцкий Артём)
// Adrian Hunter
//

// Version of this UBIFS implementation
pub const UBIFS_VERSION: c_int = 1;
// UBIFS file system VFS magic number
pub const UBIFS_SUPER_MAGIC: c_uint = 0x24051905;
// Number of UBIFS blocks per VFS page

// "File system end of life" sequence number watermark
pub const SQNUM_WARN_WATERMARK: c_uint = 0xFFFFFFFF00000000ULL;
pub const SQNUM_WATERMARK: c_uint = 0xFFFFFFFFFF000000ULL;
//
// Minimum amount of LEBs reserved for the index. At present the index needs at
// least 2 LEBs: one for the index head and one for in-the-gaps method (which
// currently does not cater for the index head and so excludes it from
// consideration).
//
pub const MIN_INDEX_LEBS: c_int = 2;
// Minimum amount of data UBIFS writes to the flash

//
// Currently we do not support inode number overlapping and re-using, so this
// watermark defines dangerous inode number level. This should be fixed later,
// although it is difficult to exceed current limit. Another option is to use
// 64-bit inode numbers, but this means more overhead.
//
pub const INUM_WARN_WATERMARK: c_uint = 0xFFF00000;
pub const INUM_WATERMARK: c_uint = 0xFFFFFF00;
// Maximum number of entries in each LPT (LEB category) heap
pub const LPT_HEAP_SZ: c_int = 256;
//
// Background thread name pattern. The numbers are UBI device and volume
// numbers.
//

// Maximum possible inode number (only 32-bit inodes are supported now)
pub const MAX_INUM: c_uint = 0xFFFFFFFF;
// Number of non-data journal heads
pub const NONDATA_JHEADS_CNT: c_int = 2;
// Shorter names for journal head numbers for internal usage

// 'No change' value for 'ubifs_change_lp()'
pub const LPROPS_NC: c_uint = 0x80000001;
//
// There is no notion of truncation key because truncation nodes do not exist
// in TNC. However, when replaying, it is handy to introduce fake "truncation"
// keys for truncation nodes because the code becomes simpler. So we define
// %UBIFS_TRUN_KEY type.
//
// But otherwise, out of the journal reply scope, the truncation keys are
// invalid.
//

//
// How much a directory entry/extended attribute entry adds to the parent/host
// inode.
//

// How much an extended attribute adds to the host inode

//
// Znodes which were not touched for 'OLD_ZNODE_AGE' seconds are considered
// "old", and znode which were touched last 'YOUNG_ZNODE_AGE' seconds ago are
// considered "young". This is used by shrinker when selecting znode to trim
// off.
//
pub const OLD_ZNODE_AGE: c_int = 20;
pub const YOUNG_ZNODE_AGE: c_int = 5;

pub const UBIFS_CIPHER_BLOCK_SIZE: c_int = 0;

//
// How much memory is needed for a buffer where we compress a data node.
//

// Maximum expected tree height for use by bottom_up_buf
pub const BOTTOM_UP_HEIGHT: c_int = 64;
// Maximum number of data nodes to bulk-read
pub const UBIFS_MAX_BULK_READ: c_int = 32;

pub const UBIFS_HASH_ARR_SZ: c_int = 0;
pub const UBIFS_HMAC_ARR_SZ: c_int = 0;

//
// Lockdep classes for UBIFS inode @ui_mutex.
//
// Znode flags (actually, bit numbers which store the flags).
//
// DIRTY_ZNODE: znode is dirty
// COW_ZNODE: znode is being committed and a new instance of this znode has to
// be created before changing this znode
// OBSOLETE_ZNODE: znode is obsolete, which means it was deleted, but it is
// still in the commit list and the ongoing commit operation
// will commit it, and delete this znode after it is done
//
// Commit states.
//
// COMMIT_RESTING: commit is not wanted
// COMMIT_BACKGROUND: background commit has been requested
// COMMIT_REQUIRED: commit is required
// COMMIT_RUNNING_BACKGROUND: background commit is running
// COMMIT_RUNNING_REQUIRED: commit is running and it is required
// COMMIT_BROKEN: commit failed
//
// 'ubifs_scan_a_node()' return values.
//
// SCANNED_GARBAGE:  scanned garbage
// SCANNED_EMPTY_SPACE: scanned empty space
// SCANNED_A_NODE: scanned a valid node
// SCANNED_A_CORRUPT_NODE: scanned a corrupted node
// SCANNED_A_BAD_PAD_NODE: scanned a padding node with invalid pad length
//
// Greater than zero means: 'scanned that number of padding bytes'
//
// LPT cnode flag bits.
//
// DIRTY_CNODE: cnode is dirty
// OBSOLETE_CNODE: cnode is being committed and has been copied (or deleted),
// so it can (and must) be freed when the commit is finished
// COW_CNODE: cnode is being committed and must be copied before writing
//
// Dirty flag bits (lpt_drty_flgs) for LPT special nodes.
//
// LTAB_DIRTY: ltab node is dirty
// LSAVE_DIRTY: lsave node is dirty
//
// Return codes used by the garbage collector.
// @LEB_FREED: the logical eraseblock was freed and is ready to use
// @LEB_FREED_IDX: indexing LEB was freed and can be used only after the commit
// @LEB_RETAINED: the logical eraseblock was freed and retained for GC purposes
//
// Action taken upon a failed ubifs_assert().
// @ASSACT_REPORT: just report the failed assertion
// @ASSACT_RO: switch to read-only mode
// @ASSACT_PANIC: call BUG() and possible panic the kernel
//
// struct ubifs_old_idx - index node obsoleted since last commit start.
// @rb: rb-tree node
// @lnum: LEB number of obsoleted index node
// @offs: offset of obsoleted index node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_old_idx {
    pub rb: rb_node,
    pub lnum: c_int,
    pub offs: c_int,
}

// The below union makes it easier to deal with keys
#[repr(C)]
#[derive(Copy, Clone)]
pub union ubifs_key {
    pub u8: [u8; UBIFS_SK_LEN],
    pub u32: [u32; UBIFS_SK_LEN/4],
    pub u64: [u64; UBIFS_SK_LEN/8],
    pub j32: [__le32; UBIFS_SK_LEN/4],
}

//
// struct ubifs_scan_node - UBIFS scanned node information.
// @list: list of scanned nodes
// @key: key of node scanned (if it has one)
// @sqnum: sequence number
// @type: type of node scanned
// @offs: offset with LEB of node scanned
// @len: length of node scanned
// @node: raw node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_scan_node {
    pub list: list_head,
    pub key: ubifs_key,
    pub sqnum: c_ulonglong,
    pub type: c_int,
    pub offs: c_int,
    pub len: c_int,
    pub node: *mut c_void,
}

//
// struct ubifs_scan_leb - UBIFS scanned LEB information.
// @lnum: logical eraseblock number
// @nodes_cnt: number of nodes scanned
// @nodes: list of struct ubifs_scan_node
// @endpt: end point (and therefore the start of empty space)
// @buf: buffer containing entire LEB scanned
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_scan_leb {
    pub lnum: c_int,
    pub nodes_cnt: c_int,
    pub nodes: list_head,
    pub endpt: c_int,
    pub buf: *mut c_void,
}

//
// struct ubifs_gced_idx_leb - garbage-collected indexing LEB.
// @list: list
// @lnum: LEB number
// @unmap: OK to unmap this LEB
//
// This data structure is used to temporary store garbage-collected indexing
// LEBs - they are not released immediately, but only after the next commit.
// This is needed to guarantee recoverability.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_gced_idx_leb {
    pub list: list_head,
    pub lnum: c_int,
    pub unmap: c_int,
}

//
// struct ubifs_inode - UBIFS in-memory inode description.
// @vfs_inode: VFS inode description object
// @creat_sqnum: sequence number at time of creation
// @del_cmtno: commit number corresponding to the time the inode was deleted,
// protected by @c->commit_sem;
// @xattr_size: summarized size of all extended attributes in bytes
// @xattr_cnt: count of extended attributes this inode has
// @xattr_names: sum of lengths of all extended attribute names belonging to
// this inode
// @dirty: non-zero if the inode is dirty
// @xattr: non-zero if this is an extended attribute inode
// @bulk_read: non-zero if bulk-read should be used
// @ui_mutex: serializes inode write-back with the rest of VFS operations,
// serializes "clean <-> dirty" state changes, serializes bulk-read,
// protects @dirty, @bulk_read, @ui_size, and @xattr_size
// @xattr_sem: serilizes write operations (remove|set|create) on xattr
// @ui_lock: protects @synced_i_size
// @synced_i_size: synchronized size of inode, i.e. the value of inode size
// currently stored on the flash; used only for regular file
// inodes
// @ui_size: inode size used by UBIFS when writing to flash
// @flags: inode flags (@UBIFS_COMPR_FL, etc)
// @compr_type: default compression type used for this inode
// @last_page_read: page number of last page read (for bulk read)
// @read_in_a_row: number of consecutive pages read in a row (for bulk read)
// @data_len: length of the data attached to the inode
// @data: inode's data
// @i_crypt_info: inode's fscrypt information
//
// @ui_mutex exists for two main reasons. At first it prevents inodes from
// being written back while UBIFS changing them, being in the middle of an VFS
// operation. This way UBIFS makes sure the inode fields are consistent. For
// example, in 'ubifs_rename()' we change 4 inodes simultaneously, and
// write-back must not write any of them before we have finished.
//
// The second reason is budgeting - UBIFS has to budget all operations. If an
// operation is going to mark an inode dirty, it has to allocate budget for
// this. It cannot just mark it dirty because there is no guarantee there will
// be enough flash space to write the inode back later. This means UBIFS has
// to have full control over inode "clean <-> dirty" transitions (and pages
// actually). But unfortunately, VFS marks inodes dirty in many places, and it
// does not ask the file-system if it is allowed to do so (there is a notifier,
// but it is not enough), i.e., there is no mechanism to synchronize with this.
// So UBIFS has its own inode dirty flag and its own mutex to serialize
// "clean <-> dirty" transitions.
//
// The @synced_i_size field is used to make sure we never write pages which are
// beyond last synchronized inode size. See 'ubifs_writepage()' for more
// information.
//
// The @ui_size is a "shadow" variable for @inode->i_size and UBIFS uses
// @ui_size instead of @inode->i_size. The reason for this is that UBIFS cannot
// make sure @inode->i_size is always changed under @ui_mutex, because it
// cannot call 'truncate_setsize()' with @ui_mutex locked, because it would
// deadlock with 'ubifs_writepage()' (see file.c). All the other inode fields
// are changed under @ui_mutex, so they do not need "shadow" fields. Note, one
// could consider to rework locking and base it on "shadow" fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_inode {
    pub vfs_inode: inode,
    pub creat_sqnum: c_ulonglong,
    pub del_cmtno: c_ulonglong,
    pub xattr_size: c_uint,
    pub xattr_cnt: c_uint,
    pub xattr_names: c_uint,
    pub dirty:1: c_uint,
    pub xattr:1: c_uint,
    pub bulk_read:1: c_uint,
    pub compr_type:2: c_uint,
    pub ui_mutex: mutex,
    pub xattr_sem: rw_semaphore,
    pub ui_lock: spinlock_t,
    pub synced_i_size: loff_t,
    pub ui_size: loff_t,
    pub flags: c_int,
    pub last_page_read: pgoff_t,
    pub read_in_a_row: pgoff_t,
    pub data_len: c_int,
    pub data: *mut c_void,

    pub i_crypt_info: *mut fscrypt_inode_info,

}

//
// struct ubifs_unclean_leb - records a LEB recovered under read-only mode.
// @list: list
// @lnum: LEB number of recovered LEB
// @endpt: offset where recovery ended
//
// This structure records a LEB identified during recovery that needs to be
// cleaned but was not because UBIFS was mounted read-only. The information
// is used to clean the LEB when remounting to read-write mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_unclean_leb {
    pub list: list_head,
    pub lnum: c_int,
    pub endpt: c_int,
}

//
// LEB properties flags.
//
// LPROPS_UNCAT: not categorized
// LPROPS_DIRTY: dirty > free, dirty >= @c->dead_wm, not index
// LPROPS_DIRTY_IDX: dirty + free > @c->min_idx_node_sze and index
// LPROPS_FREE: free > 0, dirty < @c->dead_wm, not empty, not index
// LPROPS_HEAP_CNT: number of heaps used for storing categorized LEBs
// LPROPS_EMPTY: LEB is empty, not taken
// LPROPS_FREEABLE: free + dirty == leb_size, not index, not taken
// LPROPS_FRDI_IDX: free + dirty == leb_size and index, may be taken
// LPROPS_CAT_MASK: mask for the LEB categories above
// LPROPS_TAKEN: LEB was taken (this flag is not saved on the media)
// LPROPS_INDEX: LEB contains indexing nodes (this flag also exists on flash)
//
// struct ubifs_lprops - logical eraseblock properties.
// @free: amount of free space in bytes
// @dirty: amount of dirty space in bytes
// @flags: LEB properties flags (see above)
// @lnum: LEB number
// @list: list of same-category lprops (for LPROPS_EMPTY and LPROPS_FREEABLE)
// @hpos: heap position in heap of same-category lprops (other categories)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_lprops {
    pub free: c_int,
    pub dirty: c_int,
    pub flags: c_int,
    pub lnum: c_int,
    pub list: list_head,
    pub hpos: c_int,
}

//
// struct ubifs_lpt_lprops - LPT logical eraseblock properties.
// @free: amount of free space in bytes
// @dirty: amount of dirty space in bytes
// @tgc: trivial GC flag (1 => unmap after commit end)
// @cmt: commit flag (1 => reserved for commit)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_lpt_lprops {
    pub free: c_int,
    pub dirty: c_int,
    pub tgc:1: unsigned,
    pub cmt:1: unsigned,
}

//
// struct ubifs_lp_stats - statistics of eraseblocks in the main area.
// @empty_lebs: number of empty LEBs
// @taken_empty_lebs: number of taken LEBs
// @idx_lebs: number of indexing LEBs
// @total_free: total free space in bytes (includes all LEBs)
// @total_dirty: total dirty space in bytes (includes all LEBs)
// @total_used: total used space in bytes (does not include index LEBs)
// @total_dead: total dead space in bytes (does not include index LEBs)
// @total_dark: total dark space in bytes (does not include index LEBs)
//
// The @taken_empty_lebs field counts the LEBs that are in the transient state
// of having been "taken" for use but not yet written to. @taken_empty_lebs is
// needed to account correctly for @gc_lnum, otherwise @empty_lebs could be
// used by itself (in which case 'unused_lebs' would be a better name). In the
// case of @gc_lnum, it is "taken" at mount time or whenever a LEB is retained
// by GC, but unlike other empty LEBs that are "taken", it may not be written
// straight away (i.e. before the next commit start or unmount), so either
// @gc_lnum must be specially accounted for, or the current approach followed
// i.e. count it under @taken_empty_lebs.
//
// @empty_lebs includes @taken_empty_lebs.
//
// @total_used, @total_dead and @total_dark fields do not account indexing
// LEBs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_lp_stats {
    pub empty_lebs: c_int,
    pub taken_empty_lebs: c_int,
    pub idx_lebs: c_int,
    pub total_free: c_longlong,
    pub total_dirty: c_longlong,
    pub total_used: c_longlong,
    pub total_dead: c_longlong,
    pub total_dark: c_longlong,
}

//
// struct ubifs_cnode - LEB Properties Tree common node.
// @parent: parent nnode
// @cnext: next cnode to commit
// @flags: flags (%DIRTY_LPT_NODE or %OBSOLETE_LPT_NODE)
// @iip: index in parent
// @level: level in the tree (zero for pnodes, greater than zero for nnodes)
// @num: node number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_cnode {
    pub parent: *mut ubifs_nnode,
    pub cnext: *mut ubifs_cnode,
    pub flags: c_ulong,
    pub iip: c_int,
    pub level: c_int,
    pub num: c_int,
}

//
// struct ubifs_pnode - LEB Properties Tree leaf node.
// @parent: parent nnode
// @cnext: next cnode to commit
// @flags: flags (%DIRTY_LPT_NODE or %OBSOLETE_LPT_NODE)
// @iip: index in parent
// @level: level in the tree (always zero for pnodes)
// @num: node number
// @lprops: LEB properties array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_pnode {
    pub parent: *mut ubifs_nnode,
    pub cnext: *mut ubifs_cnode,
    pub flags: c_ulong,
    pub iip: c_int,
    pub level: c_int,
    pub num: c_int,
    pub lprops: [ubifs_lprops; UBIFS_LPT_FANOUT],
}

//
// struct ubifs_nbranch - LEB Properties Tree internal node branch.
// @lnum: LEB number of child
// @offs: offset of child
// @nnode: nnode child
// @pnode: pnode child
// @cnode: cnode child
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_nbranch {
    pub lnum: c_int,
    pub offs: c_int,
    pub nnode: *mut ubifs_nnode,
    pub pnode: *mut ubifs_pnode,
    pub cnode: *mut ubifs_cnode,
}

//
// struct ubifs_nnode - LEB Properties Tree internal node.
// @parent: parent nnode
// @cnext: next cnode to commit
// @flags: flags (%DIRTY_LPT_NODE or %OBSOLETE_LPT_NODE)
// @iip: index in parent
// @level: level in the tree (always greater than zero for nnodes)
// @num: node number
// @nbranch: branches to child nodes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_nnode {
    pub parent: *mut ubifs_nnode,
    pub cnext: *mut ubifs_cnode,
    pub flags: c_ulong,
    pub iip: c_int,
    pub level: c_int,
    pub num: c_int,
    pub nbranch: [ubifs_nbranch; UBIFS_LPT_FANOUT],
}

//
// struct ubifs_lpt_heap - heap of categorized lprops.
// @arr: heap array
// @cnt: number in heap
// @max_cnt: maximum number allowed in heap
//
// There are %LPROPS_HEAP_CNT heaps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_lpt_heap {
    pub arr: *mut ubifs_lprops,
    pub cnt: c_int,
    pub max_cnt: c_int,
}

//
// Return codes for LPT scan callback function.
//
// LPT_SCAN_CONTINUE: continue scanning
// LPT_SCAN_ADD: add the LEB properties scanned to the tree in memory
// LPT_SCAN_STOP: stop scanning
//
// Callback used by the 'ubifs_lpt_scan_nolock()' function
//
// struct ubifs_wbuf - UBIFS write-buffer.
// @c: UBIFS file-system description object
// @buf: write-buffer (of min. flash I/O unit size)
// @lnum: logical eraseblock number the write-buffer points to
// @offs: write-buffer offset in this logical eraseblock
// @avail: number of bytes available in the write-buffer
// @used:  number of used bytes in the write-buffer
// @size: write-buffer size (in [@c->min_io_size, @c->max_write_size] range)
// @jhead: journal head the mutex belongs to (note, needed only to shut lockdep
// up by 'mutex_lock_nested()).
// @sync_callback: write-buffer synchronization callback
// @io_mutex: serializes write-buffer I/O
// @lock: serializes @buf, @lnum, @offs, @avail, @used, @next_ino and @inodes
// fields
// @timer: write-buffer timer
// @no_timer: non-zero if this write-buffer does not have a timer
// @need_sync: non-zero if the timer expired and the wbuf needs sync'ing
// @next_ino: points to the next position of the following inode number
// @inodes: stores the inode numbers of the nodes which are in wbuf
//
// The write-buffer synchronization callback is called when the write-buffer is
// synchronized in order to notify how much space was wasted due to
// write-buffer padding and how much free space is left in the LEB.
//
// Note: the fields @buf, @lnum, @offs, @avail and @used can be read under
// spin-lock or mutex because they are written under both mutex and spin-lock.
// @buf is appended to under mutex but overwritten under both mutex and
// spin-lock. Thus the data between @buf and @buf + @used can be read under
// spinlock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_wbuf {
    pub c: *mut ubifs_info,
    pub buf: *mut c_void,
    pub lnum: c_int,
    pub offs: c_int,
    pub avail: c_int,
    pub used: c_int,
    pub size: c_int,
    pub jhead: c_int,
    pub pad): *mut *mut *mut int (sync_callback)(struct ubifs_info c, int lnum, int free, int,
    pub io_mutex: mutex,
    pub lock: spinlock_t,
    pub timer: hrtimer,
    pub no_timer:1: c_uint,
    pub need_sync:1: c_uint,
    pub next_ino: c_int,
    pub inodes: *mut ino_t,
}

//
// struct ubifs_bud - bud logical eraseblock.
// @lnum: logical eraseblock number
// @start: where the (uncommitted) bud data starts
// @jhead: journal head number this bud belongs to
// @list: link in the list buds belonging to the same journal head
// @rb: link in the tree of all buds
// @log_hash: the log hash from the commit start node up to this bud
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_bud {
    pub lnum: c_int,
    pub start: c_int,
    pub jhead: c_int,
    pub list: list_head,
    pub rb: rb_node,
    pub log_hash: *mut shash_desc,
}

//
// struct ubifs_jhead - journal head.
// @wbuf: head's write-buffer
// @buds_list: list of bud LEBs belonging to this journal head
// @grouped: non-zero if UBIFS groups nodes when writing to this journal head
// @log_hash: the log hash from the commit start node up to this journal head
//
// Note, the @buds list is protected by the @c->buds_lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_jhead {
    pub wbuf: ubifs_wbuf,
    pub buds_list: list_head,
    pub grouped:1: c_uint,
    pub log_hash: *mut shash_desc,
}

//
// struct ubifs_zbranch - key/coordinate/length branch stored in znodes.
// @key: key
// @znode: znode address in memory
// @leaf: leaf node
// @lnum: LEB number of the target node (indexing node or data node)
// @offs: target node offset within @lnum
// @len: target node length
// @hash: the hash of the target node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_zbranch {
    pub key: ubifs_key,
    pub znode: *mut ubifs_znode,
    pub leaf: *mut c_void,
}

//
// struct ubifs_znode - in-memory representation of an indexing node.
// @parent: parent znode or NULL if it is the root
// @cnext: next znode to commit
// @cparent: parent node for this commit
// @ciip: index in cparent's zbranch array
// @flags: znode flags (%DIRTY_ZNODE, %COW_ZNODE or %OBSOLETE_ZNODE)
// @time: last access time (seconds)
// @level: level of the entry in the TNC tree
// @child_cnt: count of child znodes
// @iip: index in parent's zbranch array
// @alt: lower bound of key range has altered i.e. child inserted at slot 0
// @lnum: LEB number of the corresponding indexing node
// @offs: offset of the corresponding indexing node
// @len: length  of the corresponding indexing node
// @zbranch: array of znode branches (@c->fanout elements)
//
// Note! The @lnum, @offs, and @len fields are not really needed - we have them
// only for internal consistency check. They could be removed to save some RAM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_znode {
    pub parent: *mut ubifs_znode,
    pub cnext: *mut ubifs_znode,
    pub cparent: *mut ubifs_znode,
    pub ciip: c_int,
    pub flags: c_ulong,
    pub time: time64_t,
    pub level: c_int,
    pub child_cnt: c_int,
    pub iip: c_int,
    pub alt: c_int,
    pub lnum: c_int,
    pub offs: c_int,
    pub len: c_int,
    pub zbranch: [ubifs_zbranch; ],
}

//
// struct bu_info - bulk-read information.
// @key: first data node key
// @zbranch: zbranches of data nodes to bulk read
// @buf: buffer to read into
// @buf_len: buffer length
// @gc_seq: GC sequence number to detect races with GC
// @cnt: number of data nodes for bulk read
// @blk_cnt: number of data blocks including holes
// @eof: end of file reached
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bu_info {
    pub key: ubifs_key,
    pub zbranch: [ubifs_zbranch; UBIFS_MAX_BULK_READ],
    pub buf: *mut c_void,
    pub buf_len: c_int,
    pub gc_seq: c_int,
    pub cnt: c_int,
    pub blk_cnt: c_int,
    pub eof: c_int,
}

//
// struct ubifs_node_range - node length range description data structure.
// @len: fixed node length
// @min_len: minimum possible node length
// @max_len: maximum possible node length
//
// If @max_len is %0, the node has fixed length @len.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_node_range {
    pub len: c_int,
    pub min_len: c_int,
}

//
// struct ubifs_compressor - UBIFS compressor description structure.
// @compr_type: compressor type (%UBIFS_COMPR_LZO, etc)
// @cc: cryptoapi compressor handle
// @name: compressor name
// @capi_name: cryptoapi compressor name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_compressor {
    pub compr_type: c_int,
    pub cc: *mut crypto_acomp,
    pub name: *const c_char,
    pub capi_name: *const c_char,
}

//
// struct ubifs_budget_req - budget requirements of an operation.
//
// @fast: non-zero if the budgeting should try to acquire budget quickly and
// should not try to call write-back
// @recalculate: non-zero if @idx_growth, @data_growth, and @dd_growth fields
// have to be re-calculated
// @new_page: non-zero if the operation adds a new page
// @dirtied_page: non-zero if the operation makes a page dirty
// @new_dent: non-zero if the operation adds a new directory entry
// @mod_dent: non-zero if the operation removes or modifies an existing
// directory entry
// @new_ino: non-zero if the operation adds a new inode
// @new_ino_d: how much data newly created inode contains
// @dirtied_ino: how many inodes the operation makes dirty
// @dirtied_ino_d: how much data dirtied inode contains
// @idx_growth: how much the index will supposedly grow
// @data_growth: how much new data the operation will supposedly add
// @dd_growth: how much data that makes other data dirty the operation will
// supposedly add
//
// @idx_growth, @data_growth and @dd_growth are not used in budget request. The
// budgeting subsystem caches index and data growth values there to avoid
// re-calculating them when the budget is released. However, if @idx_growth is
// %-1, it is calculated by the release function using other fields.
//
// An inode may contain 4KiB of data at max., thus the widths of @new_ino_d
// is 13 bits, and @dirtied_ino_d - 15, because up to 4 inodes may be made
// dirty by the re-name operation.
//
// Note, UBIFS aligns node lengths to 8-bytes boundary, so the requester has to
// make sure the amount of inode data which contribute to @new_ino_d and
// @dirtied_ino_d fields are aligned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_budget_req {
    pub fast:1: c_uint,
    pub recalculate:1: c_uint,

    pub new_page:1: c_uint,
    pub dirtied_page:1: c_uint,
    pub new_dent:1: c_uint,
    pub mod_dent:1: c_uint,
    pub new_ino:1: c_uint,
    pub new_ino_d:13: c_uint,
    pub dirtied_ino:4: c_uint,
    pub dirtied_ino_d:15: c_uint,

// Not bit-fields to check for overflows
    pub new_page: c_uint,
    pub dirtied_page: c_uint,
    pub new_dent: c_uint,
    pub mod_dent: c_uint,
    pub new_ino: c_uint,
    pub new_ino_d: c_uint,
    pub dirtied_ino: c_uint,
    pub dirtied_ino_d: c_uint,

    pub idx_growth: c_int,
    pub data_growth: c_int,
    pub dd_growth: c_int,
}

//
// struct ubifs_orphan - stores the inode number of an orphan.
// @rb: rb-tree node of rb-tree of orphans sorted by inode number
// @list: list head of list of orphans in order added
// @new_list: list head of list of orphans added since the last commit
// @cnext: next orphan to commit
// @dnext: next orphan to delete
// @inum: inode number
// @new: %1 => added since the last commit, otherwise %0
// @cmt: %1 => commit pending, otherwise %0
// @del: %1 => delete pending, otherwise %0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_orphan {
    pub rb: rb_node,
    pub list: list_head,
    pub new_list: list_head,
    pub cnext: *mut ubifs_orphan,
    pub dnext: *mut ubifs_orphan,
    pub inum: ino_t,
    pub new:1: unsigned,
    pub cmt:1: unsigned,
    pub del:1: unsigned,
}

//
// struct ubifs_mount_opts - UBIFS-specific mount options information.
// @unmount_mode: selected unmount mode (%0 default, %1 normal, %2 fast)
// @bulk_read: enable/disable bulk-reads (%0 default, %1 disable, %2 enable)
// @chk_data_crc: enable/disable CRC data checking when reading data nodes
// (%0 default, %1 disable, %2 enable)
// @override_compr: override default compressor (%0 - do not override and use
// superblock compressor, %1 - override and use compressor
// specified in @compr_type)
// @compr_type: compressor type to override the superblock compressor with
// (%UBIFS_COMPR_NONE, etc)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_mount_opts {
    pub unmount_mode:2: c_uint,
    pub bulk_read:2: c_uint,
    pub chk_data_crc:2: c_uint,
    pub override_compr:1: c_uint,
    pub compr_type:2: c_uint,
}

//
// struct ubifs_budg_info - UBIFS budgeting information.
// @idx_growth: amount of bytes budgeted for index growth
// @data_growth: amount of bytes budgeted for cached data
// @dd_growth: amount of bytes budgeted for cached data that will make
// other data dirty
// @uncommitted_idx: amount of bytes were budgeted for growth of the index, but
// which still have to be taken into account because the index
// has not been committed so far
// @old_idx_sz: size of index on flash
// @min_idx_lebs: minimum number of LEBs required for the index
// @nospace: non-zero if the file-system does not have flash space (used as
// optimization)
// @nospace_rp: the same as @nospace, but additionally means that even reserved
// pool is full
// @page_budget: budget for a page (constant, never changed after mount)
// @inode_budget: budget for an inode (constant, never changed after mount)
// @dent_budget: budget for a directory entry (constant, never changed after
// mount)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_budg_info {
    pub idx_growth: c_longlong,
    pub data_growth: c_longlong,
    pub dd_growth: c_longlong,
    pub uncommitted_idx: c_longlong,
    pub old_idx_sz: c_ulonglong,
    pub min_idx_lebs: c_int,
    pub nospace:1: c_uint,
    pub nospace_rp:1: c_uint,
    pub page_budget: c_int,
    pub inode_budget: c_int,
    pub dent_budget: c_int,
}

//
// struct ubifs_stats_info - per-FS statistics information.
// @magic_errors: number of bad magic numbers (will be reset with a new mount).
// @node_errors: number of bad nodes (will be reset with a new mount).
// @crc_errors: number of bad crcs (will be reset with a new mount).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_stats_info {
    pub magic_errors: c_uint,
    pub node_errors: c_uint,
    pub crc_errors: c_uint,
}

//
// struct ubifs_info - UBIFS file-system description data structure
// (per-superblock).
// @vfs_sb: VFS @struct super_block object
// @sup_node: The super block node as read from the device
//
// @highest_inum: highest used inode number
// @max_sqnum: current global sequence number
// @cmt_no: commit number of the last successfully completed commit, protected
// by @commit_sem
// @cnt_lock: protects @highest_inum and @max_sqnum counters
// @fmt_version: UBIFS on-flash format version
// @ro_compat_version: R/O compatibility version
// @uuid: UUID from super block
//
// @lhead_lnum: log head logical eraseblock number
// @lhead_offs: log head offset
// @ltail_lnum: log tail logical eraseblock number (offset is always 0)
// @log_mutex: protects the log, @lhead_lnum, @lhead_offs, @ltail_lnum, and
// @bud_bytes
// @min_log_bytes: minimum required number of bytes in the log
// @cmt_bud_bytes: used during commit to temporarily amount of bytes in
// committed buds
//
// @buds: tree of all buds indexed by bud LEB number
// @bud_bytes: how many bytes of flash is used by buds
// @buds_lock: protects the @buds tree, @bud_bytes, and per-journal head bud
// lists
// @jhead_cnt: count of journal heads
// @jheads: journal heads (head zero is base head)
// @max_bud_bytes: maximum number of bytes allowed in buds
// @bg_bud_bytes: number of bud bytes when background commit is initiated
// @old_buds: buds to be released after commit ends
// @max_bud_cnt: maximum number of buds
// @need_wait_space: Non %0 means space reservation tasks need to wait in queue
// @reserve_space_wq: wait queue to sleep on if @need_wait_space is not %0
//
// @commit_sem: synchronizes committer with other processes
// @cmt_state: commit state
// @cs_lock: commit state lock
// @cmt_wq: wait queue to sleep on if the log is full and a commit is running
//
// @big_lpt: flag that LPT is too big to write whole during commit
// @space_fixup: flag indicating that free space in LEBs needs to be cleaned up
// @double_hash: flag indicating that we can do lookups by hash
// @encrypted: flag indicating that this file system contains encrypted files
// @no_chk_data_crc: do not check CRCs when reading data nodes (except during
// recovery)
// @bulk_read: enable bulk-reads
// @default_compr: default compression algorithm (%UBIFS_COMPR_LZO, etc)
// @rw_incompat: the media is not R/W compatible
// @assert_action: action to take when a ubifs_assert() fails
// @authenticated: flag indigating the FS is mounted in authenticated mode
// @superblock_need_write: superblock node needs to be written
//
// @tnc_mutex: protects the Tree Node Cache (TNC), @zroot, @cnext, @enext, and
// @calc_idx_sz
// @zroot: zbranch which points to the root index node and znode
// @cnext: next znode to commit
// @enext: next znode to commit to empty space
// @gap_lebs: array of LEBs used by the in-gaps commit method
// @cbuf: commit buffer
// @ileb_buf: buffer for commit in-the-gaps method
// @ileb_len: length of data in ileb_buf
// @ihead_lnum: LEB number of index head
// @ihead_offs: offset of index head
// @ilebs: pre-allocated index LEBs
// @ileb_cnt: number of pre-allocated index LEBs
// @ileb_nxt: next pre-allocated index LEBs
// @old_idx: tree of index nodes obsoleted since the last commit start
// @bottom_up_buf: a buffer which is used by 'dirty_cow_bottom_up()' in tnc.c
//
// @mst_node: master node
// @mst_offs: offset of valid master node
//
// @max_bu_buf_len: maximum bulk-read buffer length
// @bu_mutex: protects the pre-allocated bulk-read buffer and @c->bu
// @bu: pre-allocated bulk-read information
//
// @write_reserve_mutex: protects @write_reserve_buf
// @write_reserve_buf: on the write path we allocate memory, which might
// sometimes be unavailable, in which case we use this
// write reserve buffer
//
// @log_lebs: number of logical eraseblocks in the log
// @log_bytes: log size in bytes
// @log_last: last LEB of the log
// @lpt_lebs: number of LEBs used for lprops table
// @lpt_first: first LEB of the lprops table area
// @lpt_last: last LEB of the lprops table area
// @orph_lebs: number of LEBs used for the orphan area
// @orph_first: first LEB of the orphan area
// @orph_last: last LEB of the orphan area
// @main_lebs: count of LEBs in the main area
// @main_first: first LEB of the main area
// @main_bytes: main area size in bytes
//
// @key_hash_type: type of the key hash
// @key_hash: direntry key hash function
// @key_fmt: key format
// @key_len: key length
// @hash_len: The length of the index node hashes
// @fanout: fanout of the index tree (number of links per indexing node)
//
// @min_io_size: minimal input/output unit size
// @min_io_shift: number of bits in @min_io_size minus one
// @max_write_size: maximum amount of bytes the underlying flash can write at a
// time (MTD write buffer size)
// @max_write_shift: number of bits in @max_write_size minus one
// @leb_size: logical eraseblock size in bytes
// @leb_start: starting offset of logical eraseblocks within physical
// eraseblocks
// @half_leb_size: half LEB size
// @idx_leb_size: how many bytes of an LEB are effectively available when it is
// used to store indexing nodes (@leb_size - @max_idx_node_sz)
// @leb_cnt: count of logical eraseblocks
// @max_leb_cnt: maximum count of logical eraseblocks
// @ro_media: the underlying UBI volume is read-only
// @ro_mount: the file-system was mounted as read-only
// @ro_error: UBIFS switched to R/O mode because an error happened
//
// @dirty_pg_cnt: number of dirty pages (not used)
// @dirty_zn_cnt: number of dirty znodes
// @clean_zn_cnt: number of clean znodes
//
// @space_lock: protects @bi and @lst
// @lst: lprops statistics
// @bi: budgeting information
// @calc_idx_sz: temporary variable which is used to calculate new index size
// (contains accurate new index size at end of TNC commit start)
//
// @ref_node_alsz: size of the LEB reference node aligned to the min. flash
// I/O unit
// @mst_node_alsz: master node aligned size
// @min_idx_node_sz: minimum indexing node aligned on 8-bytes boundary
// @max_idx_node_sz: maximum indexing node aligned on 8-bytes boundary
// @max_inode_sz: maximum possible inode size in bytes
// @max_znode_sz: size of znode in bytes
//
// @leb_overhead: how many bytes are wasted in an LEB when it is filled with
// data nodes of maximum size - used in free space reporting
// @dead_wm: LEB dead space watermark
// @dark_wm: LEB dark space watermark
// @block_cnt: count of 4KiB blocks on the FS
//
// @ranges: UBIFS node length ranges
// @ubi: UBI volume descriptor
// @di: UBI device information
// @vi: UBI volume information
//
// @orph_tree: rb-tree of orphan inode numbers
// @orph_list: list of orphan inode numbers in order added
// @orph_new: list of orphan inode numbers added since last commit
// @orph_cnext: next orphan to commit
// @orph_dnext: next orphan to delete
// @orphan_lock: lock for orph_tree and orph_new
// @orph_buf: buffer for orphan nodes
// @new_orphans: number of orphans since last commit
// @cmt_orphans: number of orphans being committed
// @tot_orphans: number of orphans in the rb_tree
// @max_orphans: maximum number of orphans allowed
// @ohead_lnum: orphan head LEB number
// @ohead_offs: orphan head offset
// @no_orphs: non-zero if there are no orphans
//
// @bgt: UBIFS background thread
// @bgt_name: background thread name
// @need_bgt: if background thread should run
// @need_wbuf_sync: if write-buffers have to be synchronized
//
// @gc_lnum: LEB number used for garbage collection
// @sbuf: a buffer of LEB size used by GC and replay for scanning
// @idx_gc: list of index LEBs that have been garbage collected
// @idx_gc_cnt: number of elements on the idx_gc list
// @gc_seq: incremented for every non-index LEB garbage collected
// @gced_lnum: last non-index LEB that was garbage collected
//
// @infos_list: links all 'ubifs_info' objects
// @umount_mutex: serializes shrinker and un-mount
// @shrinker_run_no: shrinker run number
//
// @space_bits: number of bits needed to record free or dirty space
// @lpt_lnum_bits: number of bits needed to record a LEB number in the LPT
// @lpt_offs_bits: number of bits needed to record an offset in the LPT
// @lpt_spc_bits: number of bits needed to space in the LPT
// @pcnt_bits: number of bits needed to record pnode or nnode number
// @lnum_bits: number of bits needed to record LEB number
// @nnode_sz: size of on-flash nnode
// @pnode_sz: size of on-flash pnode
// @ltab_sz: size of on-flash LPT lprops table
// @lsave_sz: size of on-flash LPT save table
// @pnode_cnt: number of pnodes
// @nnode_cnt: number of nnodes
// @lpt_hght: height of the LPT
// @pnodes_have: number of pnodes in memory
//
// @lp_mutex: protects lprops table and all the other lprops-related fields
// @lpt_lnum: LEB number of the root nnode of the LPT
// @lpt_offs: offset of the root nnode of the LPT
// @nhead_lnum: LEB number of LPT head
// @nhead_offs: offset of LPT head
// @lpt_drty_flgs: dirty flags for LPT special nodes e.g. ltab
// @dirty_nn_cnt: number of dirty nnodes
// @dirty_pn_cnt: number of dirty pnodes
// @check_lpt_free: flag that indicates LPT GC may be needed
// @lpt_sz: LPT size
// @lpt_nod_buf: buffer for an on-flash nnode or pnode
// @lpt_buf: buffer of LEB size used by LPT
// @nroot: address in memory of the root nnode of the LPT
// @lpt_cnext: next LPT node to commit
// @lpt_heap: array of heaps of categorized lprops
// @dirty_idx: a (reverse sorted) copy of the LPROPS_DIRTY_IDX heap as at
// previous commit start
// @uncat_list: list of un-categorized LEBs
// @empty_list: list of empty LEBs
// @freeable_list: list of freeable non-index LEBs (free + dirty == @leb_size)
// @frdi_idx_list: list of freeable index LEBs (free + dirty == @leb_size)
// @freeable_cnt: number of freeable LEBs in @freeable_list
// @in_a_category_cnt: count of lprops which are in a certain category, which
// basically meants that they were loaded from the flash
//
// @ltab_lnum: LEB number of LPT's own lprops table
// @ltab_offs: offset of LPT's own lprops table
// @ltab: LPT's own lprops table
// @ltab_cmt: LPT's own lprops table (commit copy)
// @lsave_cnt: number of LEB numbers in LPT's save table
// @lsave_lnum: LEB number of LPT's save table
// @lsave_offs: offset of LPT's save table
// @lsave: LPT's save table
// @lscan_lnum: LEB number of last LPT scan
//
// @rp_size: size of the reserved pool in bytes
// @report_rp_size: size of the reserved pool reported to user-space
// @rp_uid: reserved pool user ID
// @rp_gid: reserved pool group ID
//
// @hash_tfm: the hash transformation used for hashing nodes
// @hmac_tfm: the HMAC transformation for this filesystem
// @hmac_desc_len: length of the HMAC used for authentication
// @auth_key_name: the authentication key name
// @auth_hash_name: the name of the hash algorithm used for authentication
// @auth_hash_algo: the authentication hash used for this fs
// @log_hash: the log hash from the commit start node up to the latest reference
// node.
//
// @empty: %1 if the UBI device is empty
// @need_recovery: %1 if the file-system needs recovery
// @replaying: %1 during journal replay
// @mounting: %1 while mounting
// @probing: %1 while attempting to mount if SB_SILENT mount flag is set
// @remounting_rw: %1 while re-mounting from R/O mode to R/W mode
// @replay_list: temporary list used during journal replay
// @replay_buds: list of buds to replay
// @cs_sqnum: sequence number of first node in the log (commit start node)
// @unclean_leb_list: LEBs to recover when re-mounting R/O mounted FS to R/W
// mode
// @rcvrd_mst_node: recovered master node to write when re-mounting R/O mounted
// FS to R/W mode
// @size_tree: inode size information for recovery
// @mount_opts: UBIFS-specific mount options
//
// @dbg: debugging-related information
// @stats: statistics exported over sysfs
//
// @kobj: kobject for /sys/fs/ubifs
// @kobj_unregister: completion to unregister sysfs kobject
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_info {
    pub vfs_sb: *mut super_block,
    pub sup_node: *mut ubifs_sb_node,
    pub highest_inum: ino_t,
    pub max_sqnum: c_ulonglong,
    pub cmt_no: c_ulonglong,
    pub cnt_lock: spinlock_t,
    pub fmt_version: c_int,
    pub ro_compat_version: c_int,
    pub uuid: [c_uchar; 16],
    pub lhead_lnum: c_int,
    pub lhead_offs: c_int,
    pub ltail_lnum: c_int,
    pub log_mutex: mutex,
    pub min_log_bytes: c_int,
    pub cmt_bud_bytes: c_longlong,
    pub buds: rb_root,
    pub bud_bytes: c_longlong,
    pub buds_lock: spinlock_t,
    pub jhead_cnt: c_int,
    pub jheads: *mut ubifs_jhead,
    pub max_bud_bytes: c_longlong,
    pub bg_bud_bytes: c_longlong,
    pub old_buds: list_head,
    pub max_bud_cnt: c_int,
    pub need_wait_space: core::sync::atomic::AtomicI32,
    pub reserve_space_wq: wait_queue_head_t,
    pub commit_sem: rw_semaphore,
    pub cmt_state: c_int,
    pub cs_lock: spinlock_t,
    pub cmt_wq: wait_queue_head_t,
    pub kobj: kobject,
    pub kobj_unregister: completion,
    pub big_lpt:1: c_uint,
    pub space_fixup:1: c_uint,
    pub double_hash:1: c_uint,
    pub encrypted:1: c_uint,
    pub no_chk_data_crc:1: c_uint,
    pub bulk_read:1: c_uint,
    pub default_compr:2: c_uint,
    pub rw_incompat:1: c_uint,
    pub assert_action:2: c_uint,
    pub authenticated:1: c_uint,
    pub superblock_need_write:1: c_uint,
    pub tnc_mutex: mutex,
    pub zroot: ubifs_zbranch,
    pub cnext: *mut ubifs_znode,
    pub enext: *mut ubifs_znode,
    pub gap_lebs: *mut c_int,
    pub cbuf: *mut c_void,
    pub ileb_buf: *mut c_void,
    pub ileb_len: c_int,
    pub ihead_lnum: c_int,
    pub ihead_offs: c_int,
    pub ilebs: *mut c_int,
    pub ileb_cnt: c_int,
    pub ileb_nxt: c_int,
    pub old_idx: rb_root,
    pub bottom_up_buf: *mut c_int,
    pub mst_node: *mut ubifs_mst_node,
    pub mst_offs: c_int,
    pub max_bu_buf_len: c_int,
    pub bu_mutex: mutex,
    pub bu: bu_info,
    pub write_reserve_mutex: mutex,
    pub write_reserve_buf: *mut c_void,
    pub log_lebs: c_int,
    pub log_bytes: c_longlong,
    pub log_last: c_int,
    pub lpt_lebs: c_int,
    pub lpt_first: c_int,
    pub lpt_last: c_int,
    pub orph_lebs: c_int,
    pub orph_first: c_int,
    pub orph_last: c_int,
    pub main_lebs: c_int,
    pub main_first: c_int,
    pub main_bytes: c_longlong,
    pub key_hash_type: u8,
    pub len): *const *const *const uint32_t (key_hash)(char str, int,
    pub key_fmt: c_int,
    pub key_len: c_int,
    pub hash_len: c_int,
    pub fanout: c_int,
    pub min_io_size: c_int,
    pub min_io_shift: c_int,
    pub max_write_size: c_int,
    pub max_write_shift: c_int,
    pub leb_size: c_int,
    pub leb_start: c_int,
    pub half_leb_size: c_int,
    pub idx_leb_size: c_int,
    pub leb_cnt: c_int,
    pub max_leb_cnt: c_int,
    pub ro_media:1: c_uint,
    pub ro_mount:1: c_uint,
    pub ro_error:1: c_uint,
    pub dirty_pg_cnt: atomic_long_t,
    pub dirty_zn_cnt: atomic_long_t,
    pub clean_zn_cnt: atomic_long_t,
    pub space_lock: spinlock_t,
    pub lst: ubifs_lp_stats,
    pub bi: ubifs_budg_info,
    pub calc_idx_sz: c_ulonglong,
    pub ref_node_alsz: c_int,
    pub mst_node_alsz: c_int,
    pub min_idx_node_sz: c_int,
    pub max_idx_node_sz: c_int,
    pub max_inode_sz: c_longlong,
    pub max_znode_sz: c_int,
    pub leb_overhead: c_int,
    pub dead_wm: c_int,
    pub dark_wm: c_int,
    pub block_cnt: c_int,
    pub ranges: [ubifs_node_range; UBIFS_NODE_TYPES_CNT],
    pub ubi: *mut ubi_volume_desc,
    pub di: ubi_device_info,
    pub vi: ubi_volume_info,
    pub orph_tree: rb_root,
    pub orph_list: list_head,
    pub orph_new: list_head,
    pub orph_cnext: *mut ubifs_orphan,
    pub orph_dnext: *mut ubifs_orphan,
    pub orphan_lock: spinlock_t,
    pub orph_buf: *mut c_void,
    pub new_orphans: c_int,
    pub cmt_orphans: c_int,
    pub tot_orphans: c_int,
    pub max_orphans: c_int,
    pub ohead_lnum: c_int,
    pub ohead_offs: c_int,
    pub no_orphs: c_int,
    pub bgt: *mut task_struct,
    pub 9]: char bgt_name[sizeof(BGT_NAME_PATTERN) +,
    pub need_bgt: c_int,
    pub need_wbuf_sync: c_int,
    pub gc_lnum: c_int,
    pub sbuf: *mut c_void,
    pub idx_gc: list_head,
    pub idx_gc_cnt: c_int,
    pub gc_seq: c_int,
    pub gced_lnum: c_int,
    pub infos_list: list_head,
    pub umount_mutex: mutex,
    pub shrinker_run_no: c_uint,
    pub space_bits: c_int,
    pub lpt_lnum_bits: c_int,
    pub lpt_offs_bits: c_int,
    pub lpt_spc_bits: c_int,
    pub pcnt_bits: c_int,
    pub lnum_bits: c_int,
    pub nnode_sz: c_int,
    pub pnode_sz: c_int,
    pub ltab_sz: c_int,
    pub lsave_sz: c_int,
    pub pnode_cnt: c_int,
    pub nnode_cnt: c_int,
    pub lpt_hght: c_int,
    pub pnodes_have: c_int,
    pub lp_mutex: mutex,
    pub lpt_lnum: c_int,
    pub lpt_offs: c_int,
    pub nhead_lnum: c_int,
    pub nhead_offs: c_int,
    pub lpt_drty_flgs: c_int,
    pub dirty_nn_cnt: c_int,
    pub dirty_pn_cnt: c_int,
    pub check_lpt_free: c_int,
    pub lpt_sz: c_longlong,
    pub lpt_nod_buf: *mut c_void,
    pub lpt_buf: *mut c_void,
    pub nroot: *mut ubifs_nnode,
    pub lpt_cnext: *mut ubifs_cnode,
    pub lpt_heap: [ubifs_lpt_heap; LPROPS_HEAP_CNT],
    pub dirty_idx: ubifs_lpt_heap,
    pub uncat_list: list_head,
    pub empty_list: list_head,
    pub freeable_list: list_head,
    pub frdi_idx_list: list_head,
    pub freeable_cnt: c_int,
    pub in_a_category_cnt: c_int,
    pub ltab_lnum: c_int,
    pub ltab_offs: c_int,
    pub ltab: *mut ubifs_lpt_lprops,
    pub ltab_cmt: *mut ubifs_lpt_lprops,
    pub lsave_cnt: c_int,
    pub lsave_lnum: c_int,
    pub lsave_offs: c_int,
    pub lsave: *mut c_int,
    pub lscan_lnum: c_int,
    pub rp_size: c_longlong,
    pub report_rp_size: c_longlong,
    pub rp_uid: kuid_t,
    pub rp_gid: kgid_t,
    pub hash_tfm: *mut crypto_shash,
    pub hmac_tfm: *mut crypto_shash,
    pub hmac_desc_len: c_int,
    pub auth_key_name: *mut c_char,
    pub auth_hash_name: *mut c_char,
    pub auth_hash_algo: hash_algo,
    pub log_hash: *mut shash_desc,
// The below fields are used only during mounting and re-mounting
    pub empty:1: c_uint,
    pub need_recovery:1: c_uint,
    pub replaying:1: c_uint,
    pub mounting:1: c_uint,
    pub remounting_rw:1: c_uint,
    pub probing:1: c_uint,
    pub replay_list: list_head,
    pub replay_buds: list_head,
    pub cs_sqnum: c_ulonglong,
    pub unclean_leb_list: list_head,
    pub rcvrd_mst_node: *mut ubifs_mst_node,
    pub size_tree: rb_root,
    pub mount_opts: ubifs_mount_opts,
    pub dbg: *mut ubifs_debug_info,
    pub stats: *mut ubifs_stats_info,
}

// auth.c
extern "C" {
    pub fn crypto_shash_init(_arg: desc) -> return;
}
extern "C" {
    pub fn __ubifs_node_calc_hash(_arg: c, _arg: buf, _arg: hash) -> return;
}
//
// ubifs_check_hash - compare two hashes
// @c: UBIFS file-system description object
// @expected: first hash
// @got: second hash
//
// Compare two hashes @expected and @got.
//
// Returns: 0 when they are equal, a negative error code otherwise.
//
extern "C" {
    pub fn crypto_memneq(_arg: expected, _arg: got, _arg: c->hash_len) -> return;
}
//
// ubifs_check_hmac - compare two HMACs
// @c: UBIFS file-system description object
// @expected: first HMAC
// @got: second HMAC
//
// Compare two hashes @expected and @got.
//
// Returns: 0 when they are equal, a negative error code otherwise.
//
extern "C" {
    pub fn crypto_memneq(_arg: expected, _arg: got, _arg: c->hmac_desc_len) -> return;
}

extern "C" {
    pub fn __ubifs_node_check_hash(_arg: c, _arg: buf, _arg: expected) -> return;
}
extern "C" {
    pub fn ubifs_init_authentication(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn __ubifs_exit_authentication(c: *mut ubifs_info);
}
//
// ubifs_branch_hash - returns a pointer to the hash of a branch
// @c: UBIFS file-system description object
// @br: branch to get the hash from
//
// Returns: a pointer to the hash of a branch. Since the key already is a
// dynamically sized object we cannot use a struct member here.
//
// ubifs_copy_hash - copy a hash
// @c: UBIFS file-system description object
// @from: source hash
// @to: destination hash
//
// With authentication this copies a hash, otherwise does nothing.
//
extern "C" {
    pub fn __ubifs_node_insert_hmac(_arg: c, _arg: buf, _arg: len, _arg: ofs_hmac) -> return;
}
extern "C" {
    pub fn __ubifs_node_verify_hmac(_arg: c, _arg: buf, _arg: len, _arg: ofs_hmac) -> return;
}
//
// ubifs_auth_node_sz - returns the size of an authentication node
// @c: UBIFS file-system description object
//
// Returns: the size of an authentication node which can
// be 0 for unauthenticated filesystems or the real size of an auth node
// authentication is enabled.
//
extern "C" {
    pub fn ubifs_hmac_zero(c: *mut ubifs_info, hmac: *const u8) -> bool;
}
extern "C" {
    pub fn ubifs_hmac_wkm(c: *mut ubifs_info, hmac: *mut u8) -> c_int;
}
extern "C" {
    pub fn __ubifs_shash_copy_state(_arg: c, _arg: src, _arg: target) -> return;
}
// io.c
extern "C" {
    pub fn ubifs_ro_mode(c: *mut ubifs_info, err: c_int);
}
extern "C" {
    pub fn ubifs_leb_change(c: *mut ubifs_info, lnum: c_int, buf: *const c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn ubifs_leb_unmap(c: *mut ubifs_info, lnum: c_int) -> c_int;
}
extern "C" {
    pub fn ubifs_leb_map(c: *mut ubifs_info, lnum: c_int) -> c_int;
}
extern "C" {
    pub fn ubifs_is_mapped(c: *const ubifs_info, lnum: c_int) -> c_int;
}
extern "C" {
    pub fn ubifs_wbuf_write_nolock(wbuf: *mut ubifs_wbuf, buf: *mut c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn ubifs_wbuf_seek_nolock(wbuf: *mut ubifs_wbuf, lnum: c_int, offs: c_int) -> c_int;
}
extern "C" {
    pub fn ubifs_wbuf_init(c: *mut ubifs_info, wbuf: *mut ubifs_wbuf) -> c_int;
}
extern "C" {
    pub fn ubifs_init_node(c: *mut ubifs_info, buf: *mut c_void, len: c_int, pad: c_int);
}
extern "C" {
    pub fn ubifs_crc_node(buf: *mut c_void, len: c_int);
}
extern "C" {
    pub fn ubifs_prepare_node(c: *mut ubifs_info, buf: *mut c_void, len: c_int, pad: c_int);
}
extern "C" {
    pub fn ubifs_prep_grp_node(c: *mut ubifs_info, node: *mut c_void, len: c_int, last: c_int);
}
extern "C" {
    pub fn ubifs_io_init(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_pad(c: *const ubifs_info, buf: *mut c_void, pad: c_int);
}
extern "C" {
    pub fn ubifs_wbuf_sync_nolock(wbuf: *mut ubifs_wbuf) -> c_int;
}
extern "C" {
    pub fn ubifs_bg_wbufs_sync(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_wbuf_add_ino_nolock(wbuf: *mut ubifs_wbuf, inum: ino_t);
}
extern "C" {
    pub fn ubifs_sync_wbufs_by_inode(c: *mut ubifs_info, inode: *mut inode) -> c_int;
}
// scan.c
extern "C" {
    pub fn ubifs_scan_destroy(sleb: *mut ubifs_scan_leb);
}
// log.c
extern "C" {
    pub fn ubifs_add_bud(c: *mut ubifs_info, bud: *mut ubifs_bud);
}
extern "C" {
    pub fn ubifs_create_buds_lists(c: *mut ubifs_info);
}
extern "C" {
    pub fn ubifs_add_bud_to_log(c: *mut ubifs_info, jhead: c_int, lnum: c_int, offs: c_int) -> c_int;
}
extern "C" {
    pub fn ubifs_log_start_commit(c: *mut ubifs_info, ltail_lnum: *mut c_int) -> c_int;
}
extern "C" {
    pub fn ubifs_log_end_commit(c: *mut ubifs_info, new_ltail_lnum: c_int) -> c_int;
}
extern "C" {
    pub fn ubifs_log_post_commit(c: *mut ubifs_info, old_ltail_lnum: c_int) -> c_int;
}
extern "C" {
    pub fn ubifs_consolidate_log(c: *mut ubifs_info) -> c_int;
}
// journal.c
extern "C" {
    pub fn ubifs_jnl_write_inode(c: *mut ubifs_info, inode: *const inode) -> c_int;
}
extern "C" {
    pub fn ubifs_jnl_delete_inode(c: *mut ubifs_info, inode: *const inode) -> c_int;
}
// budget.c
extern "C" {
    pub fn ubifs_budget_space(c: *mut ubifs_info, req: *mut ubifs_budget_req) -> c_int;
}
extern "C" {
    pub fn ubifs_release_budget(c: *mut ubifs_info, req: *mut ubifs_budget_req);
}
extern "C" {
    pub fn ubifs_get_free_space(c: *mut ubifs_info) -> c_longlong;
}
extern "C" {
    pub fn ubifs_get_free_space_nolock(c: *mut ubifs_info) -> c_longlong;
}
extern "C" {
    pub fn ubifs_calc_min_idx_lebs(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_convert_page_budget(c: *mut ubifs_info);
}
extern "C" {
    pub fn ubifs_reported_space(c: *const ubifs_info, free: c_longlong) -> c_longlong;
}
extern "C" {
    pub fn ubifs_calc_available(c: *const ubifs_info, min_idx_lebs: c_int) -> c_longlong;
}
// find.c
extern "C" {
    pub fn ubifs_find_free_leb_for_idx(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_find_dirty_idx_leb(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_save_dirty_idx_lnums(c: *mut ubifs_info) -> c_int;
}
// tnc.c
extern "C" {
    pub fn ubifs_tnc_remove(c: *mut ubifs_info, key: *const ubifs_key) -> c_int;
}
extern "C" {
    pub fn ubifs_tnc_remove_ino(c: *mut ubifs_info, inum: ino_t) -> c_int;
}
extern "C" {
    pub fn ubifs_tnc_close(c: *mut ubifs_info);
}
// Shared by tnc.c for tnc_commit.c
extern "C" {
    pub fn destroy_old_idx(c: *mut ubifs_info);
}
extern "C" {
    pub fn insert_old_idx_znode(c: *mut ubifs_info, znode: *mut ubifs_znode) -> c_int;
}
extern "C" {
    pub fn ubifs_tnc_get_bu_keys(c: *mut ubifs_info, bu: *mut bu_info) -> c_int;
}
extern "C" {
    pub fn ubifs_tnc_bulk_read(c: *mut ubifs_info, bu: *mut bu_info) -> c_int;
}
// tnc_misc.c
extern "C" {
    pub fn ubifs_destroy_tnc_tree(c: *mut ubifs_info);
}
// tnc_commit.c
extern "C" {
    pub fn ubifs_tnc_start_commit(c: *mut ubifs_info, zroot: *mut ubifs_zbranch) -> c_int;
}
extern "C" {
    pub fn ubifs_tnc_end_commit(c: *mut ubifs_info) -> c_int;
}
// shrinker.c
// commit.c
extern "C" {
    pub fn ubifs_bg_thread(info: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ubifs_commit_required(c: *mut ubifs_info);
}
extern "C" {
    pub fn ubifs_request_bg_commit(c: *mut ubifs_info);
}
extern "C" {
    pub fn ubifs_run_commit(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_recovery_commit(c: *mut ubifs_info);
}
extern "C" {
    pub fn ubifs_gc_should_commit(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_wait_for_commit(c: *mut ubifs_info);
}
// master.c
extern "C" {
    pub fn ubifs_compare_master_node(c: *mut ubifs_info, m1: *mut c_void, m2: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ubifs_read_master(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_write_master(c: *mut ubifs_info) -> c_int;
}
// sb.c
extern "C" {
    pub fn ubifs_read_superblock(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_write_sb_node(c: *mut ubifs_info, sup: *mut ubifs_sb_node) -> c_int;
}
extern "C" {
    pub fn ubifs_fixup_free_space(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_enable_encryption(c: *mut ubifs_info) -> c_int;
}
// replay.c
extern "C" {
    pub fn ubifs_replay_journal(c: *mut ubifs_info) -> c_int;
}
// gc.c
extern "C" {
    pub fn ubifs_garbage_collect(c: *mut ubifs_info, anyway: c_int) -> c_int;
}
extern "C" {
    pub fn ubifs_gc_start_commit(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_gc_end_commit(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_destroy_idx_gc(c: *mut ubifs_info);
}
extern "C" {
    pub fn ubifs_get_idx_gc_leb(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_garbage_collect_leb(c: *mut ubifs_info, lp: *mut ubifs_lprops) -> c_int;
}
// orphan.c
extern "C" {
    pub fn ubifs_add_orphan(c: *mut ubifs_info, inum: ino_t) -> c_int;
}
extern "C" {
    pub fn ubifs_delete_orphan(c: *mut ubifs_info, inum: ino_t);
}
extern "C" {
    pub fn ubifs_orphan_start_commit(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_orphan_end_commit(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_mount_orphans(c: *mut ubifs_info, unclean: c_int, read_only: c_int) -> c_int;
}
extern "C" {
    pub fn ubifs_clear_orphans(c: *mut ubifs_info) -> c_int;
}
// lpt.c
extern "C" {
    pub fn ubifs_calc_lpt_geom(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_lpt_init(c: *mut ubifs_info, rd: c_int, wr: c_int) -> c_int;
}
// Shared by lpt.c for lpt_commit.c
extern "C" {
    pub fn ubifs_pack_lsave(c: *mut ubifs_info, buf: *mut c_void, lsave: *mut c_int);
}
extern "C" {
    pub fn ubifs_read_nnode(c: *mut ubifs_info, parent: *mut ubifs_nnode, iip: c_int) -> c_int;
}
extern "C" {
    pub fn ubifs_add_lpt_dirt(c: *mut ubifs_info, lnum: c_int, dirty: c_int);
}
extern "C" {
    pub fn ubifs_add_nnode_dirt(c: *mut ubifs_info, nnode: *mut ubifs_nnode);
}
extern "C" {
    pub fn ubifs_unpack_bits(c: *const ubifs_info, addr: *mut u8, pos: *mut c_int, nrbits: c_int) -> u32;
}
// Needed only in debugging code in lpt_commit.c
extern "C" {
    pub fn ubifs_lpt_calc_hash(c: *mut ubifs_info, hash: *mut u8) -> c_int;
}
// lpt_commit.c
extern "C" {
    pub fn ubifs_lpt_start_commit(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_lpt_end_commit(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_lpt_post_commit(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_lpt_free(c: *mut ubifs_info, wr_only: c_int);
}
// lprops.c
extern "C" {
    pub fn ubifs_get_lp_stats(c: *mut ubifs_info, lst: *mut ubifs_lp_stats);
}
extern "C" {
    pub fn ubifs_ensure_cat(c: *mut ubifs_info, lprops: *mut ubifs_lprops);
}
extern "C" {
    pub fn ubifs_read_one_lp(c: *mut ubifs_info, lnum: c_int, lp: *mut ubifs_lprops) -> c_int;
}
extern "C" {
    pub fn ubifs_calc_dark(c: *const ubifs_info, spc: c_int) -> c_int;
}
// file.c
extern "C" {
    pub fn ubifs_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int;
}
// dir.c
extern "C" {
    pub fn ubifs_check_dir_empty(dir: *mut inode) -> c_int;
}
// xattr.c

extern "C" {
    pub fn ubifs_listxattr(dentry: *mut dentry, buffer: *mut c_char, size: usize) -> isize;
}
extern "C" {
    pub fn ubifs_purge_xattrs(host: *mut inode) -> c_int;
}

// super.c
// recovery.c
extern "C" {
    pub fn ubifs_recover_master_node(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_write_rcvrd_mst_node(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_recover_inl_heads(c: *mut ubifs_info, sbuf: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ubifs_clean_lebs(c: *mut ubifs_info, sbuf: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ubifs_rcvry_gc_commit(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_recover_size(c: *mut ubifs_info, in_place: bool) -> c_int;
}
extern "C" {
    pub fn ubifs_destroy_size_tree(c: *mut ubifs_info);
}
// ioctl.c
extern "C" {
    pub fn ubifs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn ubifs_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn ubifs_set_inode_flags(inode: *mut inode);
}

extern "C" {
    pub fn ubifs_compat_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}

// compressor.c
extern "C" {
    pub fn ubifs_compressors_init() -> int __init;
}
extern "C" {
    pub fn ubifs_compressors_exit();
}
// sysfs.c
extern "C" {
    pub fn ubifs_sysfs_init() -> c_int;
}
extern "C" {
    pub fn ubifs_sysfs_exit();
}
extern "C" {
    pub fn ubifs_sysfs_register(c: *mut ubifs_info) -> c_int;
}
extern "C" {
    pub fn ubifs_sysfs_unregister(c: *mut ubifs_info);
}

// crypto.c

// Normal UBIFS messages
extern "C" {
    pub fn ubifs_msg(c: *const ubifs_info, fmt: *const c_char, ...);
}
extern "C" {
    pub fn ubifs_err(c: *const ubifs_info, fmt: *const c_char, ...);
}
extern "C" {
    pub fn ubifs_warn(c: *const ubifs_info, fmt: *const c_char, ...);
}
//
// A conditional variant of 'ubifs_err()' which doesn't output anything
// if probing (ie. SB_SILENT set).
//

