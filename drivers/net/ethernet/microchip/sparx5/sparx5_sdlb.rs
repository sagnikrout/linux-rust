//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/sparx5/sparx5_sdlb.c
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


// SPDX-License-Identifier: GPL-2.0+
// Microchip Sparx5 Switch driver
//
// Copyright (c) 2023 Microchip Technology Inc. and its subsidiaries.
//

    struct sparx5_sdlb_group sdlb_groups[SPX5_SDLB_GROUP_CNT] = {
    { SPX5_SDLB_GROUP_RATE_MAX,    8192 / 1, 64 }, /*  25 G */
    { 15000000000ULL,              8192 / 1, 64 }, /*  15 G */
    { 10000000000ULL,              8192 / 1, 64 }, /*  10 G */
    {  5000000000ULL,              8192 / 1, 64 }, /*   5 G */
    {  2500000000ULL,              8192 / 1, 64 }, /* 2.5 G */
    {  1000000000ULL,              8192 / 2, 64 }, /*   1 G */
    {   500000000ULL,              8192 / 2, 64 }, /* 500 M */
    {   100000000ULL,              8192 / 4, 64 }, /* 100 M */
    {    50000000ULL,              8192 / 4, 64 }, /*  50 M */
    {     5000000ULL,              8192 / 8, 64 }  /*   5 M */
    };
    struct sparx5_sdlb_group *sparx5_get_sdlb_group(int idx)
    {
    return &sdlb_groups[idx];
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_sdlb_clk_hz_get(sparx5: *mut sparx5) -> u64 {
    u64 sparx5_sdlb_clk_hz_get(struct sparx5 *sparx5)
    {
    u64 clk_hz;
    clk_hz = (10 * 1000 * 1000) /
    (sparx5_clk_period(sparx5.coreclock) / 100);
    return clk_hz *= 1000;
    }
    static int sparx5_sdlb_pup_interval_get(struct sparx5 *sparx5, u32 max_token,
    u64 max_rate)
    {
    u64 clk_hz;
    clk_hz = sparx5_sdlb_clk_hz_get(sparx5);
    return div64_u64((8 * clk_hz * max_token), max_rate);
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_sdlb_pup_token_get(sparx5: *mut sparx5, pup_interval: u32, rate: u64) -> c_int {
    int sparx5_sdlb_pup_token_get(struct sparx5 *sparx5, u32 pup_interval, u64 rate)
    {
    u64 clk_hz;
    if (!rate)
    return SPX5_SDLB_PUP_TOKEN_DISABLE;
    clk_hz = sparx5_sdlb_clk_hz_get(sparx5);
    return DIV64_U64_ROUND_UP((rate * pup_interval), (clk_hz * 8));
    }
#[no_mangle]
unsafe extern "C" fn sparx5_sdlb_group_disable(sparx5: *mut sparx5, group: u32) {
    static void sparx5_sdlb_group_disable(struct sparx5 *sparx5, u32 group)
    {
    spx5_rmw(ANA_AC_SDLB_PUP_CTRL_PUP_ENA_SET(0),
    ANA_AC_SDLB_PUP_CTRL_PUP_ENA, sparx5,
    ANA_AC_SDLB_PUP_CTRL(group));
    }
#[no_mangle]
unsafe extern "C" fn sparx5_sdlb_group_enable(sparx5: *mut sparx5, group: u32) {
    static void sparx5_sdlb_group_enable(struct sparx5 *sparx5, u32 group)
    {
    spx5_rmw(ANA_AC_SDLB_PUP_CTRL_PUP_ENA_SET(1),
    ANA_AC_SDLB_PUP_CTRL_PUP_ENA, sparx5,
    ANA_AC_SDLB_PUP_CTRL(group));
    }
#[no_mangle]
unsafe extern "C" fn sparx5_sdlb_group_get_first(sparx5: *mut sparx5, group: u32) -> u32 {
    static u32 sparx5_sdlb_group_get_first(struct sparx5 *sparx5, u32 group)
    {
    u32 val;
    val = spx5_rd(sparx5, ANA_AC_SDLB_XLB_START(group));
    return ANA_AC_SDLB_XLB_START_LBSET_START_GET(val);
    }
    static u32 sparx5_sdlb_group_get_next(struct sparx5 *sparx5, u32 group,
    u32 lb)
    {
    u32 val;
    val = spx5_rd(sparx5, ANA_AC_SDLB_XLB_NEXT(lb));
    return ANA_AC_SDLB_XLB_NEXT_LBSET_NEXT_GET(val);
    }
    static bool sparx5_sdlb_group_is_first(struct sparx5 *sparx5, u32 group,
    u32 lb)
    {
    let mut lb: return = = sparx5_sdlb_group_get_first(sparx5, group);
    }
    static bool sparx5_sdlb_group_is_last(struct sparx5 *sparx5, u32 group,
    u32 lb)
    {
    let mut lb: return = = sparx5_sdlb_group_get_next(sparx5, group, lb);
    }
#[no_mangle]
unsafe extern "C" fn sparx5_sdlb_group_is_empty(sparx5: *mut sparx5, group: u32) -> bool {
    static bool sparx5_sdlb_group_is_empty(struct sparx5 *sparx5, u32 group)
    {
    u32 val;
    val = spx5_rd(sparx5, ANA_AC_SDLB_PUP_CTRL(group));
    return ANA_AC_SDLB_PUP_CTRL_PUP_ENA_GET(val) == 0;
    }
#[no_mangle]
unsafe extern "C" fn sparx5_sdlb_group_get_last(sparx5: *mut sparx5, group: u32) -> u32 {
    static u32 sparx5_sdlb_group_get_last(struct sparx5 *sparx5, u32 group)
    {
    u32 itr, next;
    itr = sparx5_sdlb_group_get_first(sparx5, group);
    for (;;) {
    next = sparx5_sdlb_group_get_next(sparx5, group, itr);
    if (itr == next)
    return itr;
    itr = next;
    }
    }
#[no_mangle]
unsafe extern "C" fn sparx5_sdlb_group_is_singular(sparx5: *mut sparx5, group: u32) -> bool {
    static bool sparx5_sdlb_group_is_singular(struct sparx5 *sparx5, u32 group)
    {
    if (sparx5_sdlb_group_is_empty(sparx5, group))
    return false;
    return sparx5_sdlb_group_get_first(sparx5, group) ==
    sparx5_sdlb_group_get_last(sparx5, group);
    }
    static int sparx5_sdlb_group_get_adjacent(struct sparx5 *sparx5, u32 group,
    u32 idx, u32 *prev, u32 *next,
    u32 *first)
    {
    u32 itr;
// first = sparx5_sdlb_group_get_first(sparx5, group);
// prev = *first;
// next = *first;
    itr = *first;
    for (;;) {
// next = sparx5_sdlb_group_get_next(sparx5, group, itr);
    if (itr == idx)
    return 0; /* Found it */
    if (itr == *next)
    return -EINVAL; /* Was not found */
// prev = itr;
    itr = *next;
    }
    }
#[no_mangle]
unsafe extern "C" fn sparx5_sdlb_group_get_count(sparx5: *mut sparx5, group: u32) -> c_int {
    static int sparx5_sdlb_group_get_count(struct sparx5 *sparx5, u32 group)
    {
    u32 itr, next;
    let mut count: c_int = 0;
    itr = sparx5_sdlb_group_get_first(sparx5, group);
    for (;;) {
    next = sparx5_sdlb_group_get_next(sparx5, group, itr);
    if (itr == next)
    return count;
    itr = next;
    count++;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_sdlb_group_get_by_rate(sparx5: *mut sparx5, rate: u32, burst: u32) -> c_int {
    int sparx5_sdlb_group_get_by_rate(struct sparx5 *sparx5, u32 rate, u32 burst)
    {
    const struct sparx5_ops *ops = sparx5.data.ops;
    const struct sparx5_sdlb_group *group;
    u64 rate_bps;
    int i, count;
    rate_bps = rate * 1000;
    for (i = sparx5.data.consts.n_lb_groups - 1; i >= 0; i--) {
    group = ops.get_sdlb_group(i);
    count = sparx5_sdlb_group_get_count(sparx5, i);
// Check that this group is not full.
// According to LB group configuration rules: the number of XLBs
// in a group must not exceed PUP_INTERVAL/4 - 1.
//
    if (count > ((group.pup_interval / 4) - 1))
    continue;
    if (rate_bps < group.max_rate)
    return i;
    }
    return -ENOSPC;
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_sdlb_group_get_by_index(sparx5: *mut sparx5, idx: u32, group: *mut u32) -> c_int {
    int sparx5_sdlb_group_get_by_index(struct sparx5 *sparx5, u32 idx, u32 *group)
    {
    u32 itr, next;
    int i;
    for (i = 0; i < sparx5.data.consts.n_lb_groups; i++) {
    if (sparx5_sdlb_group_is_empty(sparx5, i))
    continue;
    itr = sparx5_sdlb_group_get_first(sparx5, i);
    for (;;) {
    next = sparx5_sdlb_group_get_next(sparx5, i, itr);
    if (itr == idx) {
// group = i;
    return 0; /* Found it */
    }
    if (itr == next)
    break; /* Was not found */
    itr = next;
    }
    }
    return -EINVAL;
    }
    static int sparx5_sdlb_group_link(struct sparx5 *sparx5, u32 group, u32 idx,
    u32 first, u32 next, bool empty)
    {
// Stop leaking
    sparx5_sdlb_group_disable(sparx5, group);
    if (empty)
    return 0;
// Link insertion lb to next lb
    spx5_wr(ANA_AC_SDLB_XLB_NEXT_LBSET_NEXT_SET(next) |
    ANA_AC_SDLB_XLB_NEXT_LBGRP_SET(group),
    sparx5, ANA_AC_SDLB_XLB_NEXT(idx));
// Set the first lb
    spx5_wr(ANA_AC_SDLB_XLB_START_LBSET_START_SET(first), sparx5,
    ANA_AC_SDLB_XLB_START(group));
// Start leaking
    sparx5_sdlb_group_enable(sparx5, group);
    return 0;
    };
#[no_mangle]
pub unsafe extern "C" fn sparx5_sdlb_group_add(sparx5: *mut sparx5, group: u32, idx: u32) -> c_int {
    int sparx5_sdlb_group_add(struct sparx5 *sparx5, u32 group, u32 idx)
    {
    u32 first, next;
// We always add to head of the list
    first = idx;
    if (sparx5_sdlb_group_is_empty(sparx5, group))
    next = idx;
    else
    next = sparx5_sdlb_group_get_first(sparx5, group);
    return sparx5_sdlb_group_link(sparx5, group, idx, first, next, false);
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_sdlb_group_del(sparx5: *mut sparx5, group: u32, idx: u32) -> c_int {
    int sparx5_sdlb_group_del(struct sparx5 *sparx5, u32 group, u32 idx)
    {
    u32 first, next, prev;
    let mut empty: bool = false;
    if (sparx5_sdlb_group_get_adjacent(sparx5, group, idx, &prev, &next,
    &first) < 0) {
    pr_err("%s:%d Could not find idx: %d in group: %d", __func__,
    __LINE__, idx, group);
    return -EINVAL;
    }
    if (sparx5_sdlb_group_is_singular(sparx5, group)) {
    empty = true;
    } else if (sparx5_sdlb_group_is_last(sparx5, group, idx)) {
// idx is removed, prev is now last
    idx = prev;
    next = prev;
    } else if (sparx5_sdlb_group_is_first(sparx5, group, idx)) {
// idx is removed and points to itself, first is next
    first = next;
    next = idx;
    } else {
// Next is not touched
    idx = prev;
    }
    return sparx5_sdlb_group_link(sparx5, group, idx, first, next, empty);
    }
    void sparx5_sdlb_group_init(struct sparx5 *sparx5, u64 max_rate, u32 min_burst,
    u32 frame_size, u32 idx)
    {
    const struct sparx5_ops *ops = sparx5.data.ops;
    u32 thres_shift, mask = 0x01, power = 0;
    struct sparx5_sdlb_group *group;
    u64 max_token;
    group = ops.get_sdlb_group(idx);
// Number of positions to right-shift LB's threshold value.
    while ((min_burst & mask) == 0) {
    power++;
    mask <<= 1;
    }
    thres_shift = SPX5_SDLB_2CYCLES_TYPE2_THRES_OFFSET - power;
    max_token = (min_burst > SPX5_SDLB_PUP_TOKEN_MAX) ?
    SPX5_SDLB_PUP_TOKEN_MAX :
    min_burst;
    group.pup_interval =
    sparx5_sdlb_pup_interval_get(sparx5, max_token, max_rate);
    group.frame_size = frame_size;
    spx5_wr(ANA_AC_SDLB_PUP_INTERVAL_PUP_INTERVAL_SET(group.pup_interval),
    sparx5, ANA_AC_SDLB_PUP_INTERVAL(idx));
    spx5_wr(ANA_AC_SDLB_FRM_RATE_TOKENS_FRM_RATE_TOKENS_SET(frame_size),
    sparx5, ANA_AC_SDLB_FRM_RATE_TOKENS(idx));
    spx5_wr(ANA_AC_SDLB_LBGRP_MISC_THRES_SHIFT_SET(thres_shift), sparx5,
    ANA_AC_SDLB_LBGRP_MISC(idx));
    }
