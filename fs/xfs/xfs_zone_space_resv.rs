//! Automatically rewritten from C to Rust
//! Source: fs/xfs/xfs_zone_space_resv.c
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
// Copyright (c) 2023-2025 Christoph Hellwig.
// Copyright (c) 2024-2025, Western Digital Corporation or its affiliates.
//

//
// Note: the zoned allocator does not support a rtextsize > 1, so this code and
// the allocator itself uses file system blocks interchangeable with realtime
// extents without doing the otherwise required conversions.
//
// Per-task space reservation.
//
// Tasks that need to wait for GC to free up space allocate one of these
// on-stack and adds it to the per-mount zi_reclaim_reservations lists.
// The GC thread will then wake the tasks in order when space becomes available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_zone_reservation {
    pub entry: list_head,
    pub task: *mut task_struct,
    pub count_fsb: xfs_filblks_t,
}

//
// Calculate the number of reserved blocks.
//
// XC_FREE_RTEXTENTS counts the user available capacity, to which the file
// system can be filled, while XC_FREE_RTAVAILABLE counts the blocks instantly
// available for writes without waiting for GC.
//
// For XC_FREE_RTAVAILABLE only the smaller reservation required for GC and
// block zeroing is excluded from the user capacity, while XC_FREE_RTEXTENTS
// is further restricted by at least one zone as well as the optional
// persistently reserved blocks.  This allows the allocator to run more
// smoothly by not always triggering GC.
//
    uint64_t
    xfs_zoned_default_resblks(
    struct xfs_mount	*mp,
    enum xfs_free_counter	ctr)
    {
    switch (ctr) {
    case XC_FREE_RTEXTENTS:
    return xfs_rtgs_to_rfsbs(mp, XFS_RESERVED_ZONES) +
    mp.m_sb.sb_rtreserved;
    case XC_FREE_RTAVAILABLE:
    return xfs_rtgs_to_rfsbs(mp, XFS_GC_ZONES);
    default:
    ASSERT(0);
    return 0;
    }
    }
    void
    xfs_zoned_resv_wake_all(
    struct xfs_mount		*mp)
    {
    struct xfs_zone_info		*zi = mp.m_zone_info;
    struct xfs_zone_reservation	*reservation;
    spin_lock(&zi.zi_reservation_lock);
    list_for_each_entry(reservation, &zi.zi_reclaim_reservations, entry)
    wake_up_process(reservation.task);
    spin_unlock(&zi.zi_reservation_lock);
    }
    void
    xfs_zoned_add_available(
    struct xfs_mount		*mp,
    xfs_filblks_t			count_fsb)
    {
    struct xfs_zone_info		*zi = mp.m_zone_info;
    struct xfs_zone_reservation	*reservation;
    if (list_empty_careful(&zi.zi_reclaim_reservations)) {
    xfs_add_freecounter(mp, XC_FREE_RTAVAILABLE, count_fsb);
    return;
    }
    spin_lock(&zi.zi_reservation_lock);
    xfs_add_freecounter(mp, XC_FREE_RTAVAILABLE, count_fsb);
    count_fsb = xfs_sum_freecounter(mp, XC_FREE_RTAVAILABLE);
    list_for_each_entry(reservation, &zi.zi_reclaim_reservations, entry) {
    if (reservation.count_fsb > count_fsb)
    break;
    wake_up_process(reservation.task);
    count_fsb -= reservation.count_fsb;
    }
    spin_unlock(&zi.zi_reservation_lock);
    }
    static int
    xfs_zoned_space_wait_error(
    struct xfs_mount		*mp)
    {
    if (xfs_is_shutdown(mp))
    return -EIO;
    if (fatal_signal_pending(current))
    return -EINTR;
    return 0;
    }
    static int
    xfs_zoned_reserve_available(
    struct xfs_mount		*mp,
    xfs_filblks_t			count_fsb,
    unsigned int			flags)
    {
    struct xfs_zone_info		*zi = mp.m_zone_info;
    struct xfs_zone_reservation	reservation = {
    .task		= current,
    .count_fsb	= count_fsb,
    };
    int				error;
//
// If there are no waiters, try to directly grab the available blocks
// from the percpu counter.
//
// If the caller wants to dip into the reserved pool also bypass the
// wait list.  This relies on the fact that we have a very graciously
// sized reserved pool that always has enough space.  If the reserved
// allocations fail we're in trouble.
//
    if (likely(list_empty_careful(&zi.zi_reclaim_reservations) ||
    (flags & XFS_ZR_RESERVED))) {
    error = xfs_dec_freecounter(mp, XC_FREE_RTAVAILABLE, count_fsb,
    flags & XFS_ZR_RESERVED);
    if (error != -ENOSPC)
    return error;
    }
    if (flags & XFS_ZR_NOWAIT)
    return -EAGAIN;
    spin_lock(&zi.zi_reservation_lock);
    list_add_tail(&reservation.entry, &zi.zi_reclaim_reservations);
    while ((error = xfs_zoned_space_wait_error(mp)) == 0) {
    set_current_state(TASK_KILLABLE);
    error = xfs_dec_freecounter(mp, XC_FREE_RTAVAILABLE, count_fsb,
    flags & XFS_ZR_RESERVED);
    if (error != -ENOSPC)
    break;
//
// Make sure to start GC if it is not running already. As we
// check the rtavailable count when filling up zones, GC is
// normally already running at this point, but in some setups
// with very few zones we may completely run out of non-
// reserved blocks in between filling zones.
//
    if (!xfs_is_zonegc_running(mp))
    wake_up_process(zi.zi_gc_thread);
//
// If there is no reclaimable group left and we aren't still
// processing a pending GC request give up as we're fully out
// of space.
//
    if (!xfs_zoned_have_reclaimable(mp.m_zone_info) &&
    !xfs_is_zonegc_running(mp))
    break;
    spin_unlock(&zi.zi_reservation_lock);
    schedule();
    spin_lock(&zi.zi_reservation_lock);
    }
    list_del(&reservation.entry);
    spin_unlock(&zi.zi_reservation_lock);
    __set_current_state(TASK_RUNNING);
    return error;
    }
//
// Implement greedy space allocation for short writes by trying to grab all
// that is left after locking out other threads from trying to do the same.
//
// This isn't exactly optimal and can hopefully be replaced by a proper
// percpu_counter primitive one day.
//
    static int
    xfs_zoned_reserve_extents_greedy(
    struct xfs_mount		*mp,
    xfs_filblks_t			*count_fsb,
    unsigned int			flags)
    {
    struct xfs_zone_info		*zi = mp.m_zone_info;
    let mut len: i64 = *count_fsb;
    let mut error: c_int = -ENOSPC;
    spin_lock(&zi.zi_reservation_lock);
    len = min(len, xfs_sum_freecounter(mp, XC_FREE_RTEXTENTS));
    if (len > 0) {
// count_fsb = len;
    error = xfs_dec_freecounter(mp, XC_FREE_RTEXTENTS, *count_fsb,
    flags & XFS_ZR_RESERVED);
    }
    spin_unlock(&zi.zi_reservation_lock);
    return error;
    }
    int
    xfs_zoned_space_reserve(
    struct xfs_mount		*mp,
    xfs_filblks_t			count_fsb,
    unsigned int			flags,
    struct xfs_zone_alloc_ctx	*ac)
    {
    int				error;
    ASSERT(ac.reserved_blocks == 0);
    ASSERT(ac.open_zone == core::ptr::null_mut());
    error = xfs_dec_freecounter(mp, XC_FREE_RTEXTENTS, count_fsb,
    flags & XFS_ZR_RESERVED);
    if (error == -ENOSPC && !(flags & XFS_ZR_NOWAIT)) {
    xfs_inodegc_flush(mp);
    error = xfs_dec_freecounter(mp, XC_FREE_RTEXTENTS, count_fsb,
    flags & XFS_ZR_RESERVED);
    }
    if (error == -ENOSPC && (flags & XFS_ZR_GREEDY) && count_fsb > 1)
    error = xfs_zoned_reserve_extents_greedy(mp, &count_fsb, flags);
    if (error)
    return error;
    error = xfs_zoned_reserve_available(mp, count_fsb, flags);
    if (error) {
    xfs_add_freecounter(mp, XC_FREE_RTEXTENTS, count_fsb);
    return error;
    }
    ac.reserved_blocks = count_fsb;
    return 0;
    }
    void
    xfs_zoned_space_unreserve(
    struct xfs_mount		*mp,
    struct xfs_zone_alloc_ctx	*ac)
    {
    if (ac.reserved_blocks > 0) {
    xfs_zoned_add_available(mp, ac.reserved_blocks);
    xfs_add_freecounter(mp, XC_FREE_RTEXTENTS, ac.reserved_blocks);
    }
    if (ac.open_zone)
    xfs_open_zone_put(ac.open_zone);
    }
