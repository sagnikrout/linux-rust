//! Automatically rewritten from C to Rust
//! Source: fs/xfs/scrub/rgsuper.c
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
// Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//

// Set us up with a transaction and an empty context.
    int
    xchk_setup_rgsuperblock(
    struct xfs_scrub	*sc)
    {
    if (xchk_need_intent_drain(sc))
    xchk_fsgates_enable(sc, XCHK_FSGATES_DRAIN);
    return xchk_trans_alloc(sc, 0);
    }
// Cross-reference with the other rt metadata.
    STATIC void
    xchk_rgsuperblock_xref(
    struct xfs_scrub	*sc)
    {
    if (sc.sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT)
    return;
    xchk_xref_is_used_rt_space(sc, xfs_rgbno_to_rtb(sc.sr.rtg, 0), 1);
    xchk_xref_is_only_rt_owned_by(sc, 0, 1, &XFS_RMAP_OINFO_FS);
    }
    int
    xchk_rgsuperblock(
    struct xfs_scrub	*sc)
    {
    let mut rgno: xfs_rgnumber_t = sc.sm.sm_agno;
    unsigned int		flags;
    int			error;
//
// Only rtgroup 0 has a superblock.  We may someday want to use higher
// rgno for other functions, similar to what we do with the primary
// super scrub function.
//
    if (rgno != 0)
    return -ENOENT;
//
// Grab an active reference to the rtgroup structure.  If we can't get
// it, we're racing with something that's tearing down the group, so
// signal that the group no longer exists.  Take the rtbitmap in shared
// mode so that the group can't change while we're doing things.
//
    error = xchk_rtgroup_init_existing(sc, rgno, &sc.sr);
    if (!xchk_xref_process_error(sc, 0, 0, &error))
    return error;
    if (xfs_has_rtrmapbt(sc.mp))
    flags = XFS_RTGLOCK_BITMAP | XFS_RTGLOCK_RMAP;
    else
    flags = XFS_RTGLOCK_BITMAP_SHARED;
    error = xchk_rtgroup_lock(sc, &sc.sr, flags);
    if (error)
    return error;
//
// Since we already validated the rt superblock at mount time, we don't
// need to check its contents again.  All we need is to cross-reference.
//
    xchk_rgsuperblock_xref(sc);
    return 0;
    }

    int
    xrep_rgsuperblock(
    struct xfs_scrub	*sc)
    {
    struct xfs_buf		*sb_bp;
    ASSERT(rtg_rgno(sc.sr.rtg) == 0);
    sb_bp = xfs_trans_getsb(sc.tp);
    xfs_log_sb(sc.tp);
    xfs_log_rtsb(sc.tp, sb_bp);
    return 0;
    }
