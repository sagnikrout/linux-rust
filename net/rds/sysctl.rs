//! Automatically rewritten from C to Rust
//! Source: net/rds/sysctl.c
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

    static struct ctl_table_header *rds_sysctl_reg_table;
    let mut rds_sysctl_reconnect_min: static unsigned long = 1;
    let mut rds_sysctl_reconnect_max: static unsigned long = ~0UL;
    unsigned long rds_sysctl_reconnect_min_jiffies;
    let mut rds_sysctl_reconnect_max_jiffies: c_ulong = HZ;
    let mut rds_sysctl_max_unacked_packets: c_uint = 8;
    let mut rds_sysctl_max_unacked_bytes: c_uint = (16 << 20);
    let mut rds_sysctl_ping_enable: c_uint = 1;
    static struct ctl_table rds_sysctl_rds_table[] = {
    {
    .procname       = "reconnect_min_delay_ms",
    .data		= &rds_sysctl_reconnect_min_jiffies,
    .maxlen         = sizeof(unsigned long),
    .mode           = 0644,
    .proc_handler   = proc_doulongvec_ms_jiffies_minmax,
    .extra1		= &rds_sysctl_reconnect_min,
    .extra2		= &rds_sysctl_reconnect_max_jiffies,
    },
    {
    .procname       = "reconnect_max_delay_ms",
    .data		= &rds_sysctl_reconnect_max_jiffies,
    .maxlen         = sizeof(unsigned long),
    .mode           = 0644,
    .proc_handler   = proc_doulongvec_ms_jiffies_minmax,
    .extra1		= &rds_sysctl_reconnect_min_jiffies,
    .extra2		= &rds_sysctl_reconnect_max,
    },
    {
    .procname	= "max_unacked_packets",
    .data		= &rds_sysctl_max_unacked_packets,
    .maxlen         = sizeof(int),
    .mode           = 0644,
    .proc_handler   = proc_dointvec,
    },
    {
    .procname	= "max_unacked_bytes",
    .data		= &rds_sysctl_max_unacked_bytes,
    .maxlen         = sizeof(int),
    .mode           = 0644,
    .proc_handler   = proc_dointvec,
    },
    {
    .procname	= "ping_enable",
    .data		= &rds_sysctl_ping_enable,
    .maxlen         = sizeof(int),
    .mode           = 0644,
    .proc_handler   = proc_dointvec,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn rds_sysctl_exit() {
    void rds_sysctl_exit(void)
    {
    unregister_net_sysctl_table(rds_sysctl_reg_table);
    }
#[no_mangle]
pub unsafe extern "C" fn rds_sysctl_init() -> c_int {
    int rds_sysctl_init(void)
    {
    rds_sysctl_reconnect_min = msecs_to_jiffies(1);
    rds_sysctl_reconnect_min_jiffies = rds_sysctl_reconnect_min;
    rds_sysctl_reg_table =
    register_net_sysctl(&init_net, "net/rds", rds_sysctl_rds_table);
    if (!rds_sysctl_reg_table)
    return -ENOMEM;
    return 0;
    }
