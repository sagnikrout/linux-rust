//! Automatically rewritten from C to Rust
//! Source: lib/dim/rdma_dim.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2019, Mellanox Technologies inc.  All rights reserved.
//

#[no_mangle]
unsafe extern "C" fn rdma_dim_step(dim: *mut dim) -> c_int {
    static int rdma_dim_step(struct dim *dim)
    {
    if (dim.tune_state == DIM_GOING_RIGHT) {
    if (dim.profile_ix == (RDMA_DIM_PARAMS_NUM_PROFILES - 1))
    return DIM_ON_EDGE;
    dim.profile_ix++;
    dim.steps_right++;
    }
    if (dim.tune_state == DIM_GOING_LEFT) {
    if (dim.profile_ix == 0)
    return DIM_ON_EDGE;
    dim.profile_ix--;
    dim.steps_left++;
    }
    return DIM_STEPPED;
    }
    static int rdma_dim_stats_compare(struct dim_stats *curr,
    struct dim_stats *prev)
    {
// first stat
    if (!prev.cpms)
    return DIM_STATS_SAME;
    if (IS_SIGNIFICANT_DIFF(curr.cpms, prev.cpms))
    return (curr.cpms > prev.cpms) ? DIM_STATS_BETTER :
    DIM_STATS_WORSE;
    if (IS_SIGNIFICANT_DIFF(curr.cpe_ratio, prev.cpe_ratio))
    return (curr.cpe_ratio > prev.cpe_ratio) ? DIM_STATS_BETTER :
    DIM_STATS_WORSE;
    return DIM_STATS_SAME;
    }
#[no_mangle]
unsafe extern "C" fn rdma_dim_decision(curr_stats: *mut dim_stats, dim: *mut dim) -> bool {
    static bool rdma_dim_decision(struct dim_stats *curr_stats, struct dim *dim)
    {
    let mut prev_ix: c_int = dim.profile_ix;
    let mut state: u8 = dim.tune_state;
    int stats_res;
    int step_res;
    if (state != DIM_PARKING_ON_TOP && state != DIM_PARKING_TIRED) {
    stats_res = rdma_dim_stats_compare(curr_stats,
    &dim.prev_stats);
    switch (stats_res) {
    case DIM_STATS_SAME:
    if (curr_stats.cpe_ratio <= 50 * prev_ix)
    dim.profile_ix = 0;
    break;
    case DIM_STATS_WORSE:
    dim_turn(dim);
    fallthrough;
    case DIM_STATS_BETTER:
    step_res = rdma_dim_step(dim);
    if (step_res == DIM_ON_EDGE)
    dim_turn(dim);
    break;
    }
    }
    dim.prev_stats = *curr_stats;
    return dim.profile_ix != prev_ix;
    }
#[no_mangle]
pub unsafe extern "C" fn rdma_dim(dim: *mut dim, completions: u64) {
    void rdma_dim(struct dim *dim, u64 completions)
    {
    struct dim_sample *curr_sample = &dim.measuring_sample;
    struct dim_stats curr_stats;
    u32 nevents;
    dim_update_sample_with_comps(curr_sample.event_ctr + 1, 0, 0,
    curr_sample.comp_ctr + completions,
    &dim.measuring_sample);
    switch (dim.state) {
    case DIM_MEASURE_IN_PROGRESS:
    nevents = curr_sample.event_ctr - dim.start_sample.event_ctr;
    if (nevents < DIM_NEVENTS)
    break;
    if (!dim_calc_stats(&dim.start_sample, curr_sample, &curr_stats))
    break;
    if (rdma_dim_decision(&curr_stats, dim)) {
    dim.state = DIM_APPLY_NEW_PROFILE;
    schedule_work(&dim.work);
    break;
    }
    fallthrough;
    case DIM_START_MEASURE:
    dim.state = DIM_MEASURE_IN_PROGRESS;
    dim_update_sample_with_comps(curr_sample.event_ctr, 0, 0,
    curr_sample.comp_ctr,
    &dim.start_sample);
    break;
    case DIM_APPLY_NEW_PROFILE:
    break;
    }
    }
    EXPORT_SYMBOL(rdma_dim);
