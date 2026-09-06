//! Automatically rewritten from C to Rust
//! Source: net/mac80211/tests/ttlm.c
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
// KUnit tests for negotiated TTLM (TID-To-Link Mapping) parsing
//
// Copyright (C) 2026 Michael Bommarito <michael.bommarito@gmail.com>
//

    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
//
// Build a negotiated TTLM element in caller-supplied buffer.
//
// @buf:       destination buffer (must be at least elem_size bytes)
// @elem_size: sizeof(ttlm_elem) + 1 (presence byte) + npresent * bm_size
// @presence:  link_map_presence bitmask; each set bit => one map follows
// @bm_size:   bytes per map (1 or 2); 2 => LINK_MAP_SIZE bit clear
// @maps:      array of npresent u16 maps, one per set bit in presence
//
// Control field encodes direction=BOTH; no switch-time, no expected-dur,
// no DEF_LINK_MAP.  LINK_MAP_SIZE bit is set iff bm_size==1.
//
// Returns pointer to the ieee80211_ttlm_elem at buf.
//
    static const struct ieee80211_ttlm_elem *
    build_neg_ttlm_elem(u8 *buf, size_t elem_size,
    u8 presence, u8 bm_size, const u16 *maps)
    {
    struct ieee80211_ttlm_elem *t = (void *)buf;
    u8 control;
    u8 *pos;
    int i, tid;
    memset(buf, 0, elem_size);
    control = IEEE80211_TTLM_DIRECTION_BOTH; /* bits [1:0] = 2 */
    if (bm_size == 1)
    control |= IEEE80211_TTLM_CONTROL_LINK_MAP_SIZE;
    t.control = control;
    pos = (u8 *)t.optional;
// pos++ = presence;
    i = 0;
    for (tid = 0; tid < IEEE80211_TTLM_NUM_TIDS; tid++) {
    if (!(presence & BIT(tid)))
    continue;
    if (bm_size == 1)
// pos = (u8)maps[i];
    else
    put_unaligned_le16(maps[i], pos);
    pos += bm_size;
    i++;
    }
    return t;
    }
//
// sparse_presence_no_oob_read - BIT(0)|BIT(7) presence, bm_size=2
//
// Only TID 0 and TID 7 have maps; TIDs 1-6 are absent.  Element length
// is exactly 6 bytes (1 control + 1 presence + 2 * 2-byte maps).
//
// Pre-fix the parser advanced pos by bm_size AFTER the switch() block
// (i.e. unconditionally for every TID), so when processing TID 7 it
// had already advanced 6 * bm_size = 12 bytes past the presence byte
// for the absent TIDs before reading the TID-7 map - 14 bytes past the
// end of the 2-byte TID-7 map.  Under KASAN that is a slab-out-of-bounds.
//
// After the fix pos is advanced only inside the presence-bit branch so
// the cursor lands exactly at end-of-element after processing TID 7.
//
#[no_mangle]
unsafe extern "C" fn sparse_presence_no_oob_read(test: *mut kunit) {
    static void sparse_presence_no_oob_read(struct kunit *test)
    {
//
// presence = BIT(0)|BIT(7): 2 maps present.
// elem_size = sizeof(ttlm_elem) + 1 (presence) + 2*2 (maps) = 6.
//
    let mut presence: u8 = BIT(0) | BIT(7);
    let mut bm_size: u8 = 2;
    let mut npresent: c_int = 2;
    const size_t elem_size = sizeof(struct ieee80211_ttlm_elem)
    + 1 + npresent * bm_size;
//
// Allocate exact-size buffer so a pre-fix OOB read walks into the
// KASAN red zone immediately after the allocation.
//
    u8 *buf = kunit_kzalloc(test, elem_size, GFP_KERNEL);
    const struct ieee80211_ttlm_elem *ttlm;
    let mut neg_ttlm: ieee80211_neg_ttlm = {};
// Non-zero maps so the parser does not reject with -EINVAL.
    const u16 maps[2] = { 0x0001, 0x0001 };
    let mut direction: u8 = 0;
    int ret;
    KUNIT_ASSERT_NOT_NULL(test, buf);
    ttlm = build_neg_ttlm_elem(buf, elem_size, presence, bm_size, maps);
//
// Pass NULL for sdata: the only sdata dereference in this code path
// is inside mlme_dbg() on error returns, which are guarded by
// MAC80211_MLME_DEBUG == 0 in non-debug builds and by the dead-code
// eliminator in KUnit builds.  The success path does not touch sdata.
//
    ret = ieee80211_parse_neg_ttlm(core::ptr::null_mut(), ttlm, &neg_ttlm, &direction);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, (int)direction, IEEE80211_TTLM_DIRECTION_BOTH);
// TID 0: map present
    KUNIT_EXPECT_EQ(test, (int)neg_ttlm.downlink[0], 0x0001);
    KUNIT_EXPECT_EQ(test, (int)neg_ttlm.uplink[0],   0x0001);
// TID 3: absent => map should be 0
    KUNIT_EXPECT_EQ(test, (int)neg_ttlm.downlink[3], 0);
    KUNIT_EXPECT_EQ(test, (int)neg_ttlm.uplink[3],   0);
// TID 7: map present
    KUNIT_EXPECT_EQ(test, (int)neg_ttlm.downlink[7], 0x0001);
    KUNIT_EXPECT_EQ(test, (int)neg_ttlm.uplink[7],   0x0001);
    }
//
// dense_presence_baseline - presence=0xff (all 8 TIDs), bm_size=2
//
// Every TID has a map; this is the dense layout the parser handled
// correctly even before the fix.  Confirms the cursor-advance fix
// does not regress the already-correct path.
//
#[no_mangle]
unsafe extern "C" fn dense_presence_baseline(test: *mut kunit) {
    static void dense_presence_baseline(struct kunit *test)
    {
    let mut presence: u8 = 0xff;
    let mut bm_size: u8 = 2;
    let mut npresent: c_int = 8;
    const size_t elem_size = sizeof(struct ieee80211_ttlm_elem)
    + 1 + npresent * bm_size;
    u8 *buf = kunit_kzalloc(test, elem_size, GFP_KERNEL);
    const struct ieee80211_ttlm_elem *ttlm;
    let mut neg_ttlm: ieee80211_neg_ttlm = {};
    const u16 maps[8] = {
    0x0003, 0x0003, 0x0003, 0x0003,
    0x0003, 0x0003, 0x0003, 0x0003,
    };
    let mut direction: u8 = 0;
    int ret;
    KUNIT_ASSERT_NOT_NULL(test, buf);
    ttlm = build_neg_ttlm_elem(buf, elem_size, presence, bm_size, maps);
    ret = ieee80211_parse_neg_ttlm(core::ptr::null_mut(), ttlm, &neg_ttlm, &direction);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, (int)direction, IEEE80211_TTLM_DIRECTION_BOTH);
// All TIDs present: every downlink/uplink entry must be 0x0003.
    for (int tid = 0; tid < IEEE80211_TTLM_NUM_TIDS; tid++) {
    KUNIT_EXPECT_EQ(test, (int)neg_ttlm.downlink[tid], 0x0003);
    KUNIT_EXPECT_EQ(test, (int)neg_ttlm.uplink[tid],   0x0003);
    }
    }
    static struct kunit_case mac80211_ttlm_test_cases[] = {
    KUNIT_CASE(sparse_presence_no_oob_read),
    KUNIT_CASE(dense_presence_baseline),
    {}
    };
    static struct kunit_suite mac80211_ttlm = {
    .name = "mac80211-ttlm",
    .test_cases = mac80211_ttlm_test_cases,
    };
    kunit_test_suite(mac80211_ttlm);
