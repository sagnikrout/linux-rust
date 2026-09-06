//! Automatically rewritten from C to Rust
//! Source: net/rds/tcp_stats.c
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


//
// Copyright (c) 2006 Oracle.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

    DEFINE_PER_CPU(struct rds_tcp_statistics, rds_tcp_stats)
    ____cacheline_aligned;
    static const char * const rds_tcp_stat_names[] = {
    "tcp_data_ready_calls",
    "tcp_write_space_calls",
    "tcp_sndbuf_full",
    "tcp_connect_raced",
    "tcp_listen_closed_stale",
    };
    unsigned int rds_tcp_stats_info_copy(struct rds_info_iterator *iter,
    unsigned int avail)
    {
    let mut stats: rds_tcp_statistics = {0, };
    uint64_t *src;
    uint64_t *sum;
    size_t i;
    int cpu;
    if (avail < ARRAY_SIZE(rds_tcp_stat_names))
    goto out;
    for_each_online_cpu(cpu) {
    src = (uint64_t *)&(per_cpu(rds_tcp_stats, cpu));
    sum = (uint64_t *)&stats;
    for (i = 0; i < sizeof(stats) / sizeof(uint64_t); i++)
// (sum++) += *(src++);
    }
    rds_stats_info_copy(iter, (uint64_t *)&stats, rds_tcp_stat_names,
    ARRAY_SIZE(rds_tcp_stat_names));
    out:
    return ARRAY_SIZE(rds_tcp_stat_names);
    }
