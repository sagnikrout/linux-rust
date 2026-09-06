//! Automatically rewritten from C to Rust
//! Source: fs/xfs/scrub/fscounters_repair.c
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
// Copyright (c) 2018-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//

//
// FS Summary Counters
// ===================
//
// We correct errors in the filesystem summary counters by setting them to the
// values computed during the obligatory scrub phase.  However, we must be
// careful not to allow any other thread to change the counters while we're
// computing and setting new values.  To achieve this, we freeze the
// filesystem for the whole operation if the REPAIR flag is set.  The checking
// function is stricter when we've frozen the fs.
//
// Reset the superblock counters.  Caller is responsible for freezing the
// filesystem during the calculation and reset phases.
//
    int
    xrep_fscounters(
    struct xfs_scrub	*sc)
    {
    struct xfs_mount	*mp = sc.mp;
    struct xchk_fscounters	*fsc = sc.buf;
//
// Reinitialize the in-core counters from what we computed.  We froze
// the filesystem, so there shouldn't be anyone else trying to modify
// these counters.
//
    if (!fsc.frozen) {
    ASSERT(fsc.frozen);
    return -EFSCORRUPTED;
    }
    trace_xrep_reset_counters(mp, fsc);
    percpu_counter_set(&mp.m_icount, fsc.icount);
    percpu_counter_set(&mp.m_ifree, fsc.ifree);
    xfs_set_freecounter(mp, XC_FREE_BLOCKS, fsc.fdblocks);
//
// Online repair is only supported on v5 file systems, which require
// lazy sb counters and thus no update of sb_fdblocks here.  But
// sb_frextents only uses a lazy counter with rtgroups, and thus needs
// to be updated directly here otherwise.  And for that we need to keep
// track of the delalloc reservations separately, as they are are
// subtracted from m_frextents, but not included in sb_frextents.
//
    if (!xfs_has_zoned(mp)) {
    xfs_set_freecounter(mp, XC_FREE_RTEXTENTS,
    fsc.frextents - fsc.frextents_delayed);
    if (!xfs_has_rtgroups(mp))
    mp.m_sb.sb_frextents = fsc.frextents;
    }
    return 0;
    }
