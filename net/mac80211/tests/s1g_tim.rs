//! Automatically rewritten from C to Rust
//! Source: net/mac80211/tests/s1g_tim.c
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
// KUnit tests for S1G TIM PVB decoding. This test suite covers
// IEEE80211-2024 Annex L figures 8, 9, 10, 12, 13, 14. ADE mode
// is not covered as it is an optional encoding format and is not
// currently supported by mac80211.
//
// Copyright (C) 2025 Morse Micro
//

pub const MAX_AID: c_int = 128;

    ((((blk_off) & 0x1f) << 3) | ((inverse) ? BIT(2) : 0) | \
    ((enc_mode) & 0x3))
#[no_mangle]
unsafe extern "C" fn byte_to_bitstr(v: u8, out: *mut c_char) {
    static void byte_to_bitstr(u8 v, char *out)
    {
    for (int b = 7; b >= 0; b--)
// out++ = (v & BIT(b)) ? '1' : '0';
// out = '\0';
    }
    static void dump_tim_bits(struct kunit *test,
    const struct ieee80211_tim_ie *tim, u8 tim_len)
    {
    const u8 *ptr = tim.virtual_map;
    const u8 *end = (const u8 *)tim + tim_len;
    let mut oct: c_uint = 1;
    let mut blk: c_uint = 0;
    char bits[9];
    while (ptr < end) {
    let mut ctrl: u8 = *ptr++;
    let mut mode: u8 = ctrl & 0x03;
    let mut inverse: bool = ctrl & BIT(2);
    let mut blk_off: u8 = ctrl >> 3;
    kunit_info(
    test, "Block %u (ENC=%s, blk_off=%u, inverse=%u)", blk,
    (mode == IEEE80211_S1G_TIM_ENC_MODE_BLOCK)  ? "BLOCK" :
    (mode == IEEE80211_S1G_TIM_ENC_MODE_SINGLE) ? "SINGLE" :
    "OLB",
    blk_off, inverse);
    byte_to_bitstr(ctrl, bits);
    kunit_info(test, "  octet %2u (ctrl)    : %s (0x%02x)", oct,
    bits, ctrl);
    ++oct;
    switch (mode) {
    case IEEE80211_S1G_TIM_ENC_MODE_BLOCK: {
    let mut blkmap: u8 = *ptr++;
    byte_to_bitstr(blkmap, bits);
    kunit_info(test, "  octet %2u (blk-map) : %s (0x%02x)",
    oct, bits, blkmap);
    ++oct;
    for (u8 sb = 0; sb < 8; sb++) {
    if (!(blkmap & BIT(sb)))
    continue;
    let mut sub: u8 = *ptr++;
    byte_to_bitstr(sub, bits);
    kunit_info(
    test,
    "  octet %2u (SB %2u)   : %s (0x%02x)",
    oct, sb, bits, sub);
    ++oct;
    }
    break;
    }
    case IEEE80211_S1G_TIM_ENC_MODE_SINGLE: {
    let mut single: u8 = *ptr++;
    byte_to_bitstr(single, bits);
    kunit_info(test, "  octet %2u (single)  : %s (0x%02x)",
    oct, bits, single);
    ++oct;
    break;
    }
    case IEEE80211_S1G_TIM_ENC_MODE_OLB: {
    let mut len: u8 = *ptr++;
    byte_to_bitstr(len, bits);
    kunit_info(test, "  octet %2u (len=%2u)  : %s (0x%02x)",
    oct, len, bits, len);
    ++oct;
    for (u8 i = 0; i < len && ptr < end; i++) {
    let mut sub: u8 = *ptr++;
    byte_to_bitstr(sub, bits);
    kunit_info(
    test,
    "  octet %2u (SB %2u)   : %s (0x%02x)",
    oct, i, bits, sub);
    ++oct;
    }
    break;
    }
    default:
    kunit_info(test, "  ** unknown encoding 0x%x **", mode);
    return;
    }
    blk++;
    }
    }
#[no_mangle]
unsafe extern "C" fn tim_push(p: *mut u8, v: u8) {
    static void tim_push(u8 **p, u8 v)
    {
// (*p)++ = v;
    }
#[no_mangle]
unsafe extern "C" fn tim_begin(tim: *mut ieee80211_tim_ie, p: *mut u8) {
    static void tim_begin(struct ieee80211_tim_ie *tim, u8 **p)
    {
    tim.dtim_count = 0;
    tim.dtim_period = 1;
    tim.bitmap_ctrl = 0;
// p = tim->virtual_map;
    }
#[no_mangle]
unsafe extern "C" fn tim_end(tim: *mut ieee80211_tim_ie, tail: *mut u8) -> u8 {
    static u8 tim_end(struct ieee80211_tim_ie *tim, u8 *tail)
    {
    return tail - (u8 *)tim;
    }
    static void pvb_add_block_bitmap(u8 **p, u8 blk_off, bool inverse, u8 blk_bmap,
    const u8 *subblocks)
    {
    let mut enc: u8 = IEEE80211_S1G_TIM_ENC_MODE_BLOCK;
    let mut n: u8 = hweight8(blk_bmap);
    tim_push(p, BC(enc, inverse, blk_off));
    tim_push(p, blk_bmap);
    for (u8 i = 0; i < n; i++)
    tim_push(p, subblocks[i]);
    }
#[no_mangle]
unsafe extern "C" fn pvb_add_single_aid(p: *mut u8, blk_off: u8, inverse: bool, single6: u8) {
    static void pvb_add_single_aid(u8 **p, u8 blk_off, bool inverse, u8 single6)
    {
    let mut enc: u8 = IEEE80211_S1G_TIM_ENC_MODE_SINGLE;
    tim_push(p, BC(enc, inverse, blk_off));
    tim_push(p, single6 & GENMASK(5, 0));
    }
    static void pvb_add_olb(u8 **p, u8 blk_off, bool inverse, const u8 *subblocks,
    u8 len)
    {
    let mut enc: u8 = IEEE80211_S1G_TIM_ENC_MODE_OLB;
    tim_push(p, BC(enc, inverse, blk_off));
    tim_push(p, len);
    for (u8 i = 0; i < len; i++)
    tim_push(p, subblocks[i]);
    }
    static void check_all_aids(struct kunit *test,
    const struct ieee80211_tim_ie *tim, u8 tim_len,
    const unsigned long *expected)
    {
    for (u16 aid = 1; aid <= MAX_AID; aid++) {
    let mut want: bool = test_bit(aid, expected);
    let mut got: bool = ieee80211_s1g_check_tim(tim, tim_len, aid);
    KUNIT_ASSERT_EQ_MSG(test, got, want,
    "AID %u mismatch (got=%d want=%d)", aid,
    got, want);
    }
    }
#[no_mangle]
unsafe extern "C" fn fill_bitmap(bm: *mut c_ulong, list: *const u16, n: usize) {
    static void fill_bitmap(unsigned long *bm, const u16 *list, size_t n)
    {
    size_t i;
    bitmap_zero(bm, MAX_AID + 1);
    for (i = 0; i < n; i++)
    __set_bit(list[i], bm);
    }
    static void fill_bitmap_inverse(unsigned long *bm, u16 max_aid,
    const u16 *except, size_t n_except)
    {
    bitmap_zero(bm, MAX_AID + 1);
    for (u16 aid = 1; aid <= max_aid; aid++)
    __set_bit(aid, bm);
    for (size_t i = 0; i < n_except; i++)
    if (except[i] <= max_aid)
    __clear_bit(except[i], bm);
    }
#[no_mangle]
unsafe extern "C" fn s1g_tim_block_test(test: *mut kunit) {
    static void s1g_tim_block_test(struct kunit *test)
    {
    u8 buf[256] = {};
    struct ieee80211_tim_ie *tim = (void *)buf;
    u8 *p, tim_len;
    static const u8 subblocks[] = {
    0x42, /* SB m=0: AIDs 1,6 */
    0xA0, /* SB m=2: AIDs 21,23 */
    };
    u8 blk_bmap = 0x05; /* bits 0 and 2 set */
    let mut inverse: bool = false;
    static const u16 set_list[] = { 1, 6, 21, 23 };
    DECLARE_BITMAP(exp, MAX_AID + 1);
    tim_begin(tim, &p);
    pvb_add_block_bitmap(&p, 0, inverse, blk_bmap, subblocks);
    tim_len = tim_end(tim, p);
    fill_bitmap(exp, set_list, ARRAY_SIZE(set_list));
    dump_tim_bits(test, tim, tim_len);
    check_all_aids(test, tim, tim_len, exp);
    }
#[no_mangle]
unsafe extern "C" fn s1g_tim_single_test(test: *mut kunit) {
    static void s1g_tim_single_test(struct kunit *test)
    {
    u8 buf[256] = {};
    struct ieee80211_tim_ie *tim = (void *)buf;
    u8 *p, tim_len;
    let mut inverse: bool = false;
    let mut blk_off: u8 = 0;
    u8 single6 = 0x1f; /* 31 */
    static const u16 set_list[] = { 31 };
    DECLARE_BITMAP(exp, MAX_AID + 1);
    tim_begin(tim, &p);
    pvb_add_single_aid(&p, blk_off, inverse, single6);
    tim_len = tim_end(tim, p);
    fill_bitmap(exp, set_list, ARRAY_SIZE(set_list));
    dump_tim_bits(test, tim, tim_len);
    check_all_aids(test, tim, tim_len, exp);
    }
#[no_mangle]
unsafe extern "C" fn s1g_tim_olb_test(test: *mut kunit) {
    static void s1g_tim_olb_test(struct kunit *test)
    {
    u8 buf[256] = {};
    struct ieee80211_tim_ie *tim = (void *)buf;
    u8 *p, tim_len;
    let mut inverse: bool = false;
    let mut blk_off: u8 = 0;
    static const u16 set_list[] = { 1,  6,	13, 15, 17, 22, 29, 31, 33,
    38, 45, 47, 49, 54, 61, 63, 65, 70 };
    static const u8 subblocks[] = { 0x42, 0xA0, 0x42, 0xA0, 0x42,
    0xA0, 0x42, 0xA0, 0x42 };
    let mut len: u8 = ARRAY_SIZE(subblocks);
    DECLARE_BITMAP(exp, MAX_AID + 1);
    tim_begin(tim, &p);
    pvb_add_olb(&p, blk_off, inverse, subblocks, len);
    tim_len = tim_end(tim, p);
    fill_bitmap(exp, set_list, ARRAY_SIZE(set_list));
    dump_tim_bits(test, tim, tim_len);
    check_all_aids(test, tim, tim_len, exp);
    }
#[no_mangle]
unsafe extern "C" fn s1g_tim_inverse_block_test(test: *mut kunit) {
    static void s1g_tim_inverse_block_test(struct kunit *test)
    {
    u8 buf[256] = {};
    struct ieee80211_tim_ie *tim = (void *)buf;
    u8 *p, tim_len;
// Same sub-block content as Figure L-8, but inverse = true
    static const u8 subblocks[] = {
    0x42, /* SB m=0: AIDs 1,6 */
    0xA0, /* SB m=2: AIDs 21,23 */
    };
    let mut blk_bmap: u8 = 0x05;
    let mut inverse: bool = true;
// All AIDs except 1,6,21,23 are set
    static const u16 except[] = { 1, 6, 21, 23 };
    DECLARE_BITMAP(exp, MAX_AID + 1);
    tim_begin(tim, &p);
    pvb_add_block_bitmap(&p, 0, inverse, blk_bmap, subblocks);
    tim_len = tim_end(tim, p);
    fill_bitmap_inverse(exp, 63, except, ARRAY_SIZE(except));
    dump_tim_bits(test, tim, tim_len);
    check_all_aids(test, tim, tim_len, exp);
    }
#[no_mangle]
unsafe extern "C" fn s1g_tim_inverse_single_test(test: *mut kunit) {
    static void s1g_tim_inverse_single_test(struct kunit *test)
    {
    u8 buf[256] = {};
    struct ieee80211_tim_ie *tim = (void *)buf;
    u8 *p, tim_len;
    let mut inverse: bool = true;
    let mut blk_off: u8 = 0;
    u8 single6 = 0x1f; /* 31 */
// All AIDs except 31 are set
    static const u16 except[] = { 31 };
    DECLARE_BITMAP(exp, MAX_AID + 1);
    tim_begin(tim, &p);
    pvb_add_single_aid(&p, blk_off, inverse, single6);
    tim_len = tim_end(tim, p);
    fill_bitmap_inverse(exp, 63, except, ARRAY_SIZE(except));
    dump_tim_bits(test, tim, tim_len);
    check_all_aids(test, tim, tim_len, exp);
    }
#[no_mangle]
unsafe extern "C" fn s1g_tim_inverse_olb_test(test: *mut kunit) {
    static void s1g_tim_inverse_olb_test(struct kunit *test)
    {
    u8 buf[256] = {};
    struct ieee80211_tim_ie *tim = (void *)buf;
    u8 *p, tim_len;
    let mut inverse: bool = true;
    let mut blk_off: u8 = 0, len;
// All AIDs except the list below are set
    static const u16 except[] = { 1,  6,  13, 15, 17, 22, 29, 31, 33,
    38, 45, 47, 49, 54, 61, 63, 65, 70 };
    static const u8 subblocks[] = { 0x42, 0xA0, 0x42, 0xA0, 0x42,
    0xA0, 0x42, 0xA0, 0x42 };
    len = ARRAY_SIZE(subblocks);
    DECLARE_BITMAP(exp, MAX_AID + 1);
    tim_begin(tim, &p);
    pvb_add_olb(&p, blk_off, inverse, subblocks, len);
    tim_len = tim_end(tim, p);
    fill_bitmap_inverse(exp, 127, except, ARRAY_SIZE(except));
    dump_tim_bits(test, tim, tim_len);
    check_all_aids(test, tim, tim_len, exp);
    }
    static struct kunit_case s1g_tim_test_cases[] = {
    KUNIT_CASE(s1g_tim_block_test),
    KUNIT_CASE(s1g_tim_single_test),
    KUNIT_CASE(s1g_tim_olb_test),
    KUNIT_CASE(s1g_tim_inverse_block_test),
    KUNIT_CASE(s1g_tim_inverse_single_test),
    KUNIT_CASE(s1g_tim_inverse_olb_test),
    {}
    };
    static struct kunit_suite s1g_tim = {
    .name = "mac80211-s1g-tim",
    .test_cases = s1g_tim_test_cases,
    };
    kunit_test_suite(s1g_tim);
