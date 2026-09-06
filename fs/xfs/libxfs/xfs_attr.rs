//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_attr.h
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
// Copyright (c) 2000,2002-2003,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// Large attribute lists are structured around Btrees where all the data
// elements are in the leaf nodes.  Attribute names are hashed into an int,
// then that int is used as the index into the Btree.  Since the hashval
// of an attribute name may not be unique, we may have duplicate keys.
// The internal links in the Btree are logical block offsets into the file.
//
// Small attribute lists use a different format and are packed as tightly
// as possible so as to fit into the literal area of the inode.
//
// The maximum size (into the kernel or returned from the kernel) of an
// attribute value or the buffer used for an attr_list() call.  Larger
// sizes will result in an ERANGE return code.
//

//
// Kernel-internal version of the attrlist cursor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attrlist_cursor_kern {
    pub /: *mut *mut __u32 hashval; / hash value of next entry to add,
    pub /: *mut *mut __u32 blkno; / block containing entry (suggestion),
    pub /: *mut *mut __u32 offset; / offset in list of equal-hashvals,
    pub /: *mut *mut __u16 pad1; / padding to match user-level,
    pub /: *mut *mut __u8 pad2; / padding to match user-level,
    pub /: *mut *mut __u8 initted; / T/F: cursor has been initialized,
}

// ========================================================================
// Structure used to pass context around among the routines.
// ========================================================================
// void; state communicated via *context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attr_list_context {
    pub tp: *mut xfs_trans,
    pub /: *mut *mut *mut xfs_inode dp; / inode,
    pub /: *mut *mut xfs_attrlist_cursor_kern cursor; / position in list,
// output buffer
    pub __counted_by_ptr(bufsize): *mut *mut void buffer,
//
// Abort attribute list iteration if non-zero.  Can be used to pass
// error values to the xfs_attr_list caller.
//
    pub seen_enough: c_int,
    pub allow_incomplete: bool,
    pub /: *mut *mut ssize_t count; / num used entries,
    pub /: *mut *mut int dupcnt; / count dup hashvals seen,
    pub /: *mut *mut int bufsize; / total buffer size,
    pub /: *mut *mut int firstu; / first used byte in buffer,
    pub /: *mut *mut unsigned int attr_filter; / XFS_ATTR_{ROOT,SECURE},
    pub /: *mut *mut int resynch; / T/F: resynch with cursor,
    pub /: *mut *mut put_listent_func_t put_listent; / list output fmt function,
    pub /: *mut *mut int index; / index into output buffer,
}

//
// ========================================================================
// Structure used to pass context around among the delayed routines.
// ========================================================================
//
// Below is a state machine diagram for attr remove operations. The  XFS_DAS_
// states indicate places where the function would return -EAGAIN, and then
// immediately resume from after being called by the calling function. States
// marked as a "subroutine state" indicate that they belong to a subroutine, and
// so the calling function needs to pass them back to that subroutine to allow
// it to finish where it left off. But they otherwise do not have a role in the
// calling function other than just passing through.
//
// xfs_attr_remove_iter()
// │
// v
// have attr to remove? ──n──> done
// │
// y
// │
// v
// are we short form? ──y──> xfs_attr_shortform_remove ──> done
// │
// n
// │
// V
// are we leaf form? ──y──> xfs_attr_leaf_removename ──> done
// │
// n
// │
// V
// ┌── need to setup state?
// │          │
// n          y
// │          │
// │          v
// │ find attr and get state
// │ attr has remote blks? ──n─┐
// │          │                v
// │          │         find and invalidate
// │          y         the remote blocks.
// │          │         mark attr incomplete
// │          ├────────────────┘
// └──────────┤
// │
// v
// Have remote blks to remove? ───y─────┐
// │        ^          remove the blks
// │        │                │
// │        │                v
// │  XFS_DAS_RMTBLK <─n── done?
// │  re-enter with          │
// │  one less blk to        y
// │      remove             │
// │                         V
// │                  refill the state
// n                         │
// │                         v
// │                   XFS_DAS_RM_NAME
// │                         │
// ├─────────────────────────┘
// │
// v
// remove leaf and
// update hash with
// xfs_attr_node_remove_cleanup
// │
// v
// need to
// shrink tree? ─n─┐
// │         │
// y         │
// │         │
// v         │
// join leaf     │
// │         │
// v         │
// XFS_DAS_RM_SHRINK │
// │         │
// v         │
// do the shrink    │
// │         │
// v         │
// free state <──┘
// │
// v
// done
//
// Below is a state machine diagram for attr set operations.
//
// It seems the challenge with understanding this system comes from trying to
// absorb the state machine all at once, when really one should only be looking
// at it with in the context of a single function. Once a state sensitive
// function is called, the idea is that it "takes ownership" of the
// state machine. It isn't concerned with the states that may have belonged to
// it's calling parent. Only the states relevant to itself or any other
// subroutines there in. Once a calling function hands off the state machine to
// a subroutine, it needs to respect the simple rule that it doesn't "own" the
// state machine anymore, and it's the responsibility of that calling function
// to propagate the -EAGAIN back up the call stack. Upon reentry, it is
// committed to re-calling that subroutine until it returns something other than
// -EAGAIN. Once that subroutine signals completion (by returning anything other
// than -EAGAIN), the calling function can resume using the state machine.
//
// xfs_attr_set_iter()
// │
// v
// ┌─y─ has an attr fork?
// │          |
// │          n
// │          |
// │          V
// │       add a fork
// │          │
// └──────────┤
// │
// V
// ┌─── is shortform?
// │          │
// │          y
// │          │
// │          V
// │   xfs_attr_set_fmt
// │          |
// │          V
// │ xfs_attr_try_sf_addname
// │          │
// │          V
// │      had enough ──y──> done
// │        space?
// n          │
// │          n
// │          │
// │          V
// │   transform to leaf
// │          │
// │          V
// │   hold the leaf buffer
// │          │
// │          V
// │     return -EAGAIN
// │      Re-enter in
// │       leaf form
// │
// └─> release leaf buffer
// if needed
// │
// V
// ┌───n── fork has
// │      only 1 blk?
// │          │
// │          y
// │          │
// │          v
// │ xfs_attr_leaf_try_add()
// │          │
// │          v
// │      had enough ──────────────y─────────────┐
// │        space?                               │
// │          │                                  │
// │          n                                  │
// │          │                                  │
// │          v                                  │
// │    return -EAGAIN                           │
// │      re-enter in                            │
// │        node form                            │
// │          │                                  │
// └──────────┤                                  │
// │                                  │
// V                                  │
// xfs_attr_node_addname_find_attr                 │
// determines if this                       │
// is create or rename                       │
// find space to store attr                    │
// │                                  │
// v                                  │
// xfs_attr_node_addname                       │
// │                                  │
// v                                  │
// fits in a node leaf? ────n─────┐              │
// │     ^             v              │
// │     │       single leaf node?    │
// │     │         │            │     │
// y     │         y            n     │
// │     │         │            │     │
// v     │         v            v     │
// update  │    grow the leaf  split if │
// hashvals └── return -EAGAIN   needed  │
// │         retry leaf add     │     │
// │           on reentry       │     │
// ├────────────────────────────┘     │
// │                                  │
// v                                  │
// need to alloc                           │
// ┌─y── or flip flag?                           │
// │          │                                  │
// │          n                                  │
// │          │                                  │
// │          v                                  │
// │         done                                │
// │                                             │
// │         XFS_DAS_FOUND_LBLK <────────────────┘
// │                  │
// │                  V
// │        xfs_attr_leaf_addname()
// │                  │
// │                  v
// │      ┌──first time through?
// │      │          │
// │      │          y
// │      │          │
// │      n          v
// │      │    if we have rmt blks
// │      │    find space for them
// │      │          │
// │      └──────────┤
// │                 │
// │                 v
// │            still have
// │      ┌─n─ blks to alloc? <──┐
// │      │          │           │
// │      │          y           │
// │      │          │           │
// │      │          v           │
// │      │     alloc one blk    │
// │      │     return -EAGAIN ──┘
// │      │    re-enter with one
// │      │    less blk to alloc
// │      │
// │      └───> set the rmt
// │               value
// │                 │
// │                 v
// │               was this
// │              a rename? ──n─┐
// │                 │          │
// │                 y          │
// │                 │          │
// │                 v          │
// │           flip incomplete  │
// │               flag         │
// │                 │          │
// │                 v          │
// │         XFS_DAS_FLIP_LFLAG │
// │                 │          │
// │                 v          │
// │          need to remove    │
// │              old bks? ──n──┤
// │                 │          │
// │                 y          │
// │                 │          │
// │                 V          │
// │               remove       │
// │        ┌───> old blks      │
// │        │        │          │
// │ XFS_DAS_RM_LBLK │          │
// │        ^        │          │
// │        │        v          │
// │        └──y── more to      │
// │              remove?       │
// │                 │          │
// │                 n          │
// │                 │          │
// │                 v          │
// │          XFS_DAS_RD_LEAF   │
// │                 │          │
// │                 v          │
// │            remove leaf     │
// │                 │          │
// │                 v          │
// │            shrink to sf    │
// │             if needed      │
// │                 │          │
// │                 v          │
// │                done <──────┘
// │
// └──────> XFS_DAS_FOUND_NBLK
// │
// v
// ┌─────n──  need to
// │        alloc blks?
// │             │
// │             y
// │             │
// │             v
// │        find space
// │             │
// │             v
// │  ┌─>XFS_DAS_ALLOC_NODE
// │  │          │
// │  │          v
// │  │      alloc blk
// │  │          │
// │  │          v
// │  └──y── need to alloc
// │         more blocks?
// │             │
// │             n
// │             │
// │             v
// │      set the rmt value
// │             │
// │             v
// │          was this
// └────────> a rename? ──n─┐
// │          │
// y          │
// │          │
// v          │
// flip incomplete  │
// flag         │
// │          │
// v          │
// XFS_DAS_FLIP_NFLAG │
// │          │
// v          │
// need to        │
// remove blks? ─n──┤
// │          │
// y          │
// │          │
// v          │
// remove       │
// ┌────────> old blks     │
// │            │          │
// XFS_DAS_RM_NBLK    │          │
// ^            │          │
// │            v          │
// └──────y── more to      │
// remove       │
// │          │
// n          │
// │          │
// v          │
// XFS_DAS_CLR_FLAG  │
// │          │
// v          │
// clear flags     │
// │          │
// ├──────────┘
// │
// v
// done
//
// Enum values for xfs_attr_intent.xattri_da_state
//
// These values are used by delayed attribute operations to keep track  of where
// they were before they returned -EAGAIN.  A return code of -EAGAIN signals the
// calling function to roll the transaction, and then call the subroutine to
// finish the operation.  The enum is then used by the subroutine to jump back
// to where it was and resume executing where it left off.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_delattr_state {
    XFS_DAS_UNINIT		= 0,	/* No state has been set yet */

//
// Initial sequence states. The replace setup code relies on the
// ADD and REMOVE states for a specific format to be sequential so
// that we can transform the initial operation to be performed
// according to the xfs_has_larp() state easily.
//
    XFS_DAS_SF_ADD,			/* Initial sf add state */
    XFS_DAS_SF_REMOVE,		/* Initial sf replace/remove state */

    XFS_DAS_LEAF_ADD,		/* Initial leaf add state */
    XFS_DAS_LEAF_REMOVE,		/* Initial leaf replace/remove state */

    XFS_DAS_NODE_ADD,		/* Initial node add state */
    XFS_DAS_NODE_REMOVE,		/* Initial node replace/remove state */

// Leaf state set/replace/remove sequence
    XFS_DAS_LEAF_SET_RMT,		/* set a remote xattr from a leaf */
    XFS_DAS_LEAF_ALLOC_RMT,		/* We are allocating remote blocks */
    XFS_DAS_LEAF_REPLACE,		/* Perform replace ops on a leaf */
    XFS_DAS_LEAF_REMOVE_OLD,	/* Start removing old attr from leaf */
    XFS_DAS_LEAF_REMOVE_RMT,	/* A rename is removing remote blocks */
    XFS_DAS_LEAF_REMOVE_ATTR,	/* Remove the old attr from a leaf */

// Node state sequence, must match leaf state above
    XFS_DAS_NODE_SET_RMT,		/* set a remote xattr from a node */
    XFS_DAS_NODE_ALLOC_RMT,		/* We are allocating remote blocks */
    XFS_DAS_NODE_REPLACE,		/* Perform replace ops on a node */
    XFS_DAS_NODE_REMOVE_OLD,	/* Start removing old attr from node */
    XFS_DAS_NODE_REMOVE_RMT,	/* A rename is removing remote blocks */
    XFS_DAS_NODE_REMOVE_ATTR,	/* Remove the old attr from a node */

    XFS_DAS_DONE,			/* finished operation */
}

//
// Context used for keeping track of delayed attribute operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attr_intent {
//
// used to log this item to an intent containing a list of attrs to
// commit later
//
    pub xattri_list: list_head,
// Used in xfs_attr_node_removename to roll through removing blocks
    pub xattri_da_state: *mut xfs_da_state,
    pub xattri_da_args: *mut xfs_da_args,
//
// Shared buffer containing the attr name, new name, and value so that
// the logging code can share large memory buffers between log items.
//
    pub xattri_nameval: *mut xfs_attri_log_nameval,
// Used to keep track of current state of delayed operation
    pub xattri_dela_state: xfs_delattr_state,
//
// Attr operation being performed - XFS_ATTRI_OP_FLAGS_
//
    pub xattri_op_flags: c_uint,
// Used in xfs_attr_rmtval_set_blk to roll through allocating blocks
    pub xattri_lblkno: xfs_dablk_t,
    pub xattri_blkcnt: c_int,
    pub xattri_map: xfs_bmbt_irec,
}

// ========================================================================
// Function prototypes for the kernel.
// ========================================================================
//
// Overall external interface routines.
//
extern "C" {
    pub fn xfs_attr_inactive(dp: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_attr_list_ilocked(: *mut xfs_attr_list_context) -> c_int;
}
extern "C" {
    pub fn xfs_attr_list(: *mut xfs_attr_list_context) -> c_int;
}
extern "C" {
    pub fn xfs_inode_hasattr(ip: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_attr_is_leaf(ip: *mut xfs_inode) -> bool;
}
extern "C" {
    pub fn xfs_attr_get_ilocked(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_attr_get(args: *mut xfs_da_args) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_attr_update {
    XFS_ATTRUPDATE_REMOVE,	/* remove attr */
    XFS_ATTRUPDATE_UPSERT,	/* set value, replace any existing attr */
    XFS_ATTRUPDATE_CREATE,	/* set value, fail if attr already exists */
    XFS_ATTRUPDATE_REPLACE,	/* set value, fail if attr does not exist */
}

extern "C" {
    pub fn xfs_attr_set(args: *mut xfs_da_args, op: xfs_attr_update, rsvd: bool) -> c_int;
}
extern "C" {
    pub fn xfs_attr_set_iter(attr: *mut xfs_attr_intent) -> c_int;
}
extern "C" {
    pub fn xfs_attr_remove_iter(attr: *mut xfs_attr_intent) -> c_int;
}
extern "C" {
    pub fn xfs_attr_check_namespace(attr_flags: c_uint) -> bool;
}
extern "C" {
    pub fn xfs_attr_calc_size(args: *mut xfs_da_args, local: *mut c_int) -> c_int;
}
extern "C" {
    pub fn xfs_attr_set_resv(args: *const xfs_da_args) -> xfs_trans_res;
}
//
// Check to see if the attr should be upgraded from non-existent or shortform to
// single-leaf-block attribute list.
//
// When called from the completion of a attr remove to determine the
// next state, the attribute fork may be null. This can occur only occur
// on a pure remove, but we grab the next state before we check if a
// replace operation is being performed. If we are called from any other
// context, i_af is guaranteed to exist. Hence if the attr fork is
// null, we were called from a pure remove operation and so we are done.
//
// If we are logging the attributes, then we have to start with removal of the
// old attribute so that there is always consistent state that we can recover
// from if the system goes down part way through. We always log the new attr
// value, so even when we remove the attr first we still have the information in
// the log to finish the replace operation atomically.
//
extern "C" {
    pub fn xfs_attr_init_remove_state(_arg: args) -> return;
}
extern "C" {
    pub fn xfs_attr_init_add_state(_arg: args) -> return;
}
extern "C" {
    pub fn xfs_attr_hashname(name: *const u8, namelen: c_int) -> xfs_dahash_t;
}
// Set the hash value for any extended attribute from any namespace.
extern "C" {
    pub fn xfs_attr_intent_init_cache() -> int __init;
}
extern "C" {
    pub fn xfs_attr_intent_destroy_cache();
}
extern "C" {
    pub fn xfs_attr_sf_totsize(dp: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_attr_add_fork(ip: *mut xfs_inode, size: c_int, rsvd: c_int) -> c_int;
}
extern "C" {
    pub fn xfs_attr_setname(args: *mut xfs_da_args, rmt_blks: c_int) -> c_int;
}
extern "C" {
    pub fn xfs_attr_removename(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_attr_replacename(args: *mut xfs_da_args, rmt_blks: c_int) -> c_int;
}
