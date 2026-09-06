//! Automatically rewritten from C to Rust
//! Source: fs/xfs/scrub/dqiterate.c
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
// Copyright (C) 2023 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//

// Initialize a dquot iteration cursor.
    void
    xchk_dqiter_init(
    struct xchk_dqiter	*cursor,
    struct xfs_scrub	*sc,
    xfs_dqtype_t		dqtype)
    {
    cursor.sc = sc;
    cursor.bmap.br_startoff = NULLFILEOFF;
    cursor.dqtype = dqtype & XFS_DQTYPE_REC_MASK;
    cursor.quota_ip = xfs_quota_inode(sc.mp, cursor.dqtype);
    cursor.id = 0;
    }
//
// Ensure that the cached data fork mapping for the dqiter cursor is fresh and
// covers the dquot pointed to by the scan cursor.
//
    STATIC int
    xchk_dquot_iter_revalidate_bmap(
    struct xchk_dqiter	*cursor)
    {
    struct xfs_quotainfo	*qi = cursor.sc.mp.m_quotainfo;
    struct xfs_ifork	*ifp = xfs_ifork_ptr(cursor.quota_ip,
    XFS_DATA_FORK);
    xfs_fileoff_t		fileoff;
    let mut this_id: xfs_dqid_t = cursor.id;
    let mut nmaps: c_int = 1;
    int			error;
    fileoff = this_id / qi.qi_dqperchunk;
//
// If we have a mapping for cursor->id and it's still fresh, there's
// no need to reread the bmbt.
//
    if (cursor.bmap.br_startoff != NULLFILEOFF &&
    cursor.if_seq == ifp.if_seq &&
    cursor.bmap.br_startoff + cursor.bmap.br_blockcount > fileoff)
    return 0;
// Look up the data fork mapping for the dquot id of interest.
    error = xfs_bmapi_read(cursor.quota_ip, fileoff,
    XFS_MAX_FILEOFF - fileoff, &cursor.bmap, &nmaps, 0);
    if (error)
    return error;
    if (!nmaps) {
    ASSERT(nmaps > 0);
    return -EFSCORRUPTED;
    }
    if (cursor.bmap.br_startoff > fileoff) {
    ASSERT(cursor.bmap.br_startoff == fileoff);
    return -EFSCORRUPTED;
    }
    cursor.if_seq = ifp.if_seq;
    trace_xchk_dquot_iter_revalidate_bmap(cursor, cursor.id);
    return 0;
    }
// Advance the dqiter cursor to the next non-sparse region of the quota file.
    STATIC int
    xchk_dquot_iter_advance_bmap(
    struct xchk_dqiter	*cursor,
    uint64_t		*next_ondisk_id)
    {
    struct xfs_quotainfo	*qi = cursor.sc.mp.m_quotainfo;
    struct xfs_ifork	*ifp = xfs_ifork_ptr(cursor.quota_ip,
    XFS_DATA_FORK);
    xfs_fileoff_t		fileoff;
    uint64_t		next_id;
    let mut nmaps: c_int = 1;
    int			error;
// Find the dquot id for the next non-hole mapping.
    do {
    fileoff = cursor.bmap.br_startoff + cursor.bmap.br_blockcount;
    if (fileoff > XFS_DQ_ID_MAX / qi.qi_dqperchunk) {
// The hole goes beyond the max dquot id, we're done
// next_ondisk_id = -1ULL;
    return 0;
    }
    error = xfs_bmapi_read(cursor.quota_ip, fileoff,
    XFS_MAX_FILEOFF - fileoff, &cursor.bmap,
    &nmaps, 0);
    if (error)
    return error;
    if (!nmaps) {
// Must have reached the end of the mappings.
// next_ondisk_id = -1ULL;
    return 0;
    }
    if (cursor.bmap.br_startoff > fileoff) {
    ASSERT(cursor.bmap.br_startoff == fileoff);
    return -EFSCORRUPTED;
    }
    } while (!xfs_bmap_is_real_extent(&cursor.bmap));
    next_id = cursor.bmap.br_startoff * qi.qi_dqperchunk;
    if (next_id > XFS_DQ_ID_MAX) {
// The hole goes beyond the max dquot id, we're done
// next_ondisk_id = -1ULL;
    return 0;
    }
// Propose jumping forward to the dquot in the next allocated block.
// next_ondisk_id = next_id;
    cursor.if_seq = ifp.if_seq;
    trace_xchk_dquot_iter_advance_bmap(cursor, *next_ondisk_id);
    return 0;
    }
//
// Find the id of the next highest incore dquot.  Normally this will correspond
// exactly with the quota file block mappings, but repair might have erased a
// mapping because it was crosslinked; in that case, we need to re-allocate the
// space so that we can reset q_blkno.
//
    STATIC void
    xchk_dquot_iter_advance_incore(
    struct xchk_dqiter	*cursor,
    uint64_t		*next_incore_id)
    {
    struct xfs_quotainfo	*qi = cursor.sc.mp.m_quotainfo;
    struct radix_tree_root	*tree = xfs_dquot_tree(qi, cursor.dqtype);
    struct xfs_dquot	*dq;
    unsigned int		nr_found;
// next_incore_id = -1ULL;
    mutex_lock(&qi.qi_tree_lock);
    nr_found = radix_tree_gang_lookup(tree, (void **)&dq, cursor.id, 1);
    if (nr_found)
// next_incore_id = dq->q_id;
    mutex_unlock(&qi.qi_tree_lock);
    trace_xchk_dquot_iter_advance_incore(cursor, *next_incore_id);
    }
//
// Walk all incore dquots of this filesystem.  Caller must set *@cursorp to
// zero before the first call, and must not hold the quota file ILOCK.
// Returns 1 and a valid *@dqpp; 0 and *@dqpp == NULL when there are no more
// dquots to iterate; or a negative errno.
//
    int
    xchk_dquot_iter(
    struct xchk_dqiter	*cursor,
    struct xfs_dquot	**dqpp)
    {
    struct xfs_mount	*mp = cursor.sc.mp;
    struct xfs_dquot	*dq = core::ptr::null_mut();
    uint64_t		next_ondisk, next_incore = -1ULL;
    unsigned int		lock_mode;
    let mut error: c_int = 0;
    if (cursor.id > XFS_DQ_ID_MAX)
    return 0;
    next_ondisk = cursor.id;
// Revalidate and/or advance the cursor.
    lock_mode = xfs_ilock_data_map_shared(cursor.quota_ip);
    error = xchk_dquot_iter_revalidate_bmap(cursor);
    if (!error && !xfs_bmap_is_real_extent(&cursor.bmap))
    error = xchk_dquot_iter_advance_bmap(cursor, &next_ondisk);
    xfs_iunlock(cursor.quota_ip, lock_mode);
    if (error)
    return error;
    if (next_ondisk > cursor.id)
    xchk_dquot_iter_advance_incore(cursor, &next_incore);
// Pick the next dquot in the sequence and return it.
    cursor.id = min(next_ondisk, next_incore);
    if (cursor.id > XFS_DQ_ID_MAX)
    return 0;
    trace_xchk_dquot_iter(cursor, cursor.id);
    error = xfs_qm_dqget(mp, cursor.id, cursor.dqtype, false, &dq);
    if (error)
    return error;
    cursor.id = (uint64_t)dq.q_id + 1;
// dqpp = dq;
    return 1;
    }
