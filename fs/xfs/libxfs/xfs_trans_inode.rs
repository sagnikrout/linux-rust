//! Automatically rewritten from C to Rust
//! Source: fs/xfs/libxfs/xfs_trans_inode.c
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
// Copyright (c) 2000,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//

//
// Add a locked inode to the transaction.
//
// The inode must be locked, and it cannot be associated with any transaction.
// If lock_flags is non-zero the inode will be unlocked on transaction commit.
//
    void
    xfs_trans_ijoin(
    struct xfs_trans	*tp,
    struct xfs_inode	*ip,
    uint			lock_flags)
    {
    struct xfs_inode_log_item *iip;
    xfs_assert_ilocked(ip, XFS_ILOCK_EXCL);
    if (ip.i_itemp == core::ptr::null_mut())
    xfs_inode_item_init(ip, ip.i_mount);
    iip = ip.i_itemp;
    ASSERT(iip.ili_lock_flags == 0);
    iip.ili_lock_flags = lock_flags;
    ASSERT(!xfs_iflags_test(ip, XFS_ISTALE));
// Reset the per-tx dirty context and add the item to the tx.
    iip.ili_dirty_flags = 0;
    xfs_trans_add_item(tp, &iip.ili_item);
    }
//
// Transactional inode timestamp update. Requires the inode to be locked and
// joined to the transaction supplied. Relies on the transaction subsystem to
// track dirty state and update/writeback the inode accordingly.
//
    void
    xfs_trans_ichgtime(
    struct xfs_trans	*tp,
    struct xfs_inode	*ip,
    int			flags)
    {
    struct inode		*inode = VFS_I(ip);
    struct timespec64	tv;
    ASSERT(tp);
    xfs_assert_ilocked(ip, XFS_ILOCK_EXCL);
// If the mtime changes, then ctime must also change
    ASSERT(flags & XFS_ICHGTIME_CHG);
    tv = inode_set_ctime_current(inode);
    if (flags & XFS_ICHGTIME_MOD)
    inode_set_mtime_to_ts(inode, tv);
    if (flags & XFS_ICHGTIME_ACCESS)
    inode_set_atime_to_ts(inode, tv);
    if (flags & XFS_ICHGTIME_CREATE)
    ip.i_crtime = tv;
    }
//
// This is called to mark the fields indicated in fieldmask as needing to be
// logged when the transaction is committed.  The inode must already be
// associated with the given transaction. All we do here is record where the
// inode was dirtied and mark the transaction and inode log item dirty;
// everything else is done in the ->precommit log item operation after the
// changes in the transaction have been completed.
//
    void
    xfs_trans_log_inode(
    struct xfs_trans	*tp,
    struct xfs_inode	*ip,
    uint			flags)
    {
    struct xfs_inode_log_item *iip = ip.i_itemp;
    struct inode		*inode = VFS_I(ip);
    ASSERT(iip);
    xfs_assert_ilocked(ip, XFS_ILOCK_EXCL);
    ASSERT(!xfs_iflags_test(ip, XFS_ISTALE));
    tp.t_flags |= XFS_TRANS_DIRTY;
//
// First time we log the inode in a transaction, bump the inode change
// counter if it is configured for this to occur. While we have the
// inode locked exclusively for metadata modification, we can usually
// avoid setting XFS_ILOG_CORE if no one has queried the value since
// the last time it was incremented. If we have XFS_ILOG_CORE already
// set however, then go ahead and bump the i_version counter
// unconditionally.
//
    if (!test_and_set_bit(XFS_LI_DIRTY, &iip.ili_item.li_flags)) {
    if (IS_I_VERSION(inode) &&
    inode_maybe_inc_iversion(inode, flags & XFS_ILOG_CORE))
    flags |= XFS_ILOG_IVERSION;
    }
    iip.ili_dirty_flags |= flags;
    }
    int
    xfs_trans_roll_inode(
    struct xfs_trans	**tpp,
    struct xfs_inode	*ip)
    {
    int			error;
    xfs_trans_log_inode(*tpp, ip, XFS_ILOG_CORE);
    error = xfs_trans_roll(tpp);
    if (!error)
    xfs_trans_ijoin(*tpp, ip, 0);
    return error;
    }
