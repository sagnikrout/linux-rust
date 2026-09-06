//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/ulp/rtrs/rtrs-srv-stats.c
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
// RDMA Transport Layer
//
// Copyright (c) 2014 - 2018 ProfitBricks GmbH. All rights reserved.
// Copyright (c) 2018 - 2019 1&1 IONOS Cloud GmbH. All rights reserved.
// Copyright (c) 2019 - 2020 1&1 IONOS SE. All rights reserved.
//

#[no_mangle]
pub unsafe extern "C" fn rtrs_srv_reset_rdma_stats(stats: *mut rtrs_srv_stats, enable: bool) -> c_int {
    int rtrs_srv_reset_rdma_stats(struct rtrs_srv_stats *stats, bool enable)
    {
    if (enable) {
    int cpu;
    struct rtrs_srv_stats_rdma_stats *r;
    for_each_possible_cpu(cpu) {
    r = per_cpu_ptr(stats.rdma_stats, cpu);
    memset(r, 0, sizeof(*r));
    }
    return 0;
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn rtrs_srv_stats_rdma_to_str(stats: *mut rtrs_srv_stats, page: *mut c_char) -> isize {
    ssize_t rtrs_srv_stats_rdma_to_str(struct rtrs_srv_stats *stats, char *page)
    {
    int cpu;
    struct rtrs_srv_stats_rdma_stats sum;
    struct rtrs_srv_stats_rdma_stats *r;
    memset(&sum, 0, sizeof(sum));
    for_each_possible_cpu(cpu) {
    r = per_cpu_ptr(stats.rdma_stats, cpu);
    sum.dir[READ].cnt	  += r.dir[READ].cnt;
    sum.dir[READ].size_total  += r.dir[READ].size_total;
    sum.dir[WRITE].cnt	  += r.dir[WRITE].cnt;
    sum.dir[WRITE].size_total += r.dir[WRITE].size_total;
    }
    return sysfs_emit(page, "%llu %llu %llu %llu\n",
    sum.dir[READ].cnt, sum.dir[READ].size_total,
    sum.dir[WRITE].cnt, sum.dir[WRITE].size_total);
    }
