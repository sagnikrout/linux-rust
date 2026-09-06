//! Automatically rewritten from C to Rust
//! Source: mm/mmzone.c
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
// linux/mm/mmzone.c
//
// management codes for pgdats, zones and page flags
//

    struct pglist_data *first_online_pgdat(void)
    {
    return NODE_DATA(first_online_node);
    }
    struct pglist_data *next_online_pgdat(struct pglist_data *pgdat)
    {
    let mut nid: c_int = next_online_node(pgdat.node_id);
    if (nid == MAX_NUMNODES)
    return core::ptr::null_mut();
    return NODE_DATA(nid);
    }
//
// next_zone - helper magic for for_each_zone()
//
    struct zone *next_zone(struct zone *zone)
    {
    pg_data_t *pgdat = zone.zone_pgdat;
    if (zone < pgdat.node_zones + MAX_NR_ZONES - 1)
    zone++;
    else {
    pgdat = next_online_pgdat(pgdat);
    if (pgdat)
    zone = pgdat.node_zones;
    else
    zone = core::ptr::null_mut();
    }
    return zone;
    }
    static inline int zref_in_nodemask(struct zoneref *zref,
    const nodemask_t *nodes)
    {

    return node_isset(zonelist_node_idx(zref), *nodes);

    return 1;

    }
// Returns the next zone at or below highest_zoneidx in a zonelist
    struct zoneref *__next_zones_zonelist(struct zoneref *z,
    enum zone_type highest_zoneidx,
    const nodemask_t *nodes)
    {
//
// Find the next suitable zone to use for the allocation.
// Only filter based on nodemask if it's set
//
    if (unlikely(nodes == core::ptr::null_mut()))
    while (zonelist_zone_idx(z) > highest_zoneidx)
    z++;
    else
    while (zonelist_zone_idx(z) > highest_zoneidx ||
    (zonelist_zone(z) && !zref_in_nodemask(z, nodes)))
    z++;
    return z;
    }
#[no_mangle]
pub unsafe extern "C" fn lruvec_init(lruvec: *mut lruvec) {
    void lruvec_init(struct lruvec *lruvec)
    {
    enum lru_list lru;
    memset(lruvec, 0, sizeof(struct lruvec));
    spin_lock_init(&lruvec.lru_lock);
    spin_lock_init(&lruvec.cost_lock);
    zswap_lruvec_state_init(lruvec);
    for_each_lru(lru)
    INIT_LIST_HEAD(&lruvec.lists[lru]);
//
// The "Unevictable LRU" is imaginary: though its size is maintained,
// it is never scanned, and unevictable pages are not threaded on it
// (so that their lru fields can be reused to hold mlock_count).
// Poison its list head, so that any operations on it would crash.
//
    list_del(&lruvec.lists[LRU_UNEVICTABLE]);
    lru_gen_init_lruvec(lruvec);
    }

#[no_mangle]
pub unsafe extern "C" fn folio_xchg_last_cpupid(folio: *mut folio, cpupid: c_int) -> c_int {
    int folio_xchg_last_cpupid(struct folio *folio, int cpupid)
    {
    unsigned long old_flags, flags;
    int last_cpupid;
    old_flags = READ_ONCE(folio.flags.f);
    do {
    flags = old_flags;
    last_cpupid = (flags >> LAST_CPUPID_PGSHIFT) & LAST_CPUPID_MASK;
    flags &= ~(LAST_CPUPID_MASK << LAST_CPUPID_PGSHIFT);
    flags |= (cpupid & LAST_CPUPID_MASK) << LAST_CPUPID_PGSHIFT;
    } while (unlikely(!try_cmpxchg(&folio.flags.f, &old_flags, flags)));
    return last_cpupid;
    }
