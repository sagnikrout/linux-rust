//! Automatically rewritten from C to Rust
//! Source: net/rds/ib_sysctl.c
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

    static struct ctl_table_header *rds_ib_sysctl_hdr;
    let mut rds_ib_sysctl_max_send_wr: c_ulong = RDS_IB_DEFAULT_SEND_WR;
    let mut rds_ib_sysctl_max_recv_wr: c_ulong = RDS_IB_DEFAULT_RECV_WR;
    let mut rds_ib_sysctl_max_recv_allocation: c_ulong = (128 * 1024 * 1024) / RDS_FRAG_SIZE;
    let mut rds_ib_sysctl_max_wr_min: static unsigned long = 1;
// hardware will fail CQ creation long before this
    let mut rds_ib_sysctl_max_wr_max: static unsigned long = (u32)~0;
    let mut rds_ib_sysctl_max_unsig_wrs: c_ulong = 16;
    let mut rds_ib_sysctl_max_unsig_wr_min: static unsigned long = 1;
    let mut rds_ib_sysctl_max_unsig_wr_max: static unsigned long = 64;
//
// This sysctl does nothing.
//
// Backwards compatibility with RDS 3.0 wire protocol
// disables initial FC credit exchange.
// If it's ever possible to drop 3.0 support,
// setting this to 1 and moving init/refill of send/recv
// rings from ib_cm_connect_complete() back into ib_setup_qp()
// will cause credits to be added before protocol negotiation.
//
    let mut rds_ib_sysctl_flow_control: c_uint = 0;
    static struct ctl_table rds_ib_sysctl_table[] = {
    {
    .procname       = "max_send_wr",
    .data		= &rds_ib_sysctl_max_send_wr,
    .maxlen         = sizeof(unsigned long),
    .mode           = 0644,
    .proc_handler   = proc_doulongvec_minmax,
    .extra1		= &rds_ib_sysctl_max_wr_min,
    .extra2		= &rds_ib_sysctl_max_wr_max,
    },
    {
    .procname       = "max_recv_wr",
    .data		= &rds_ib_sysctl_max_recv_wr,
    .maxlen         = sizeof(unsigned long),
    .mode           = 0644,
    .proc_handler   = proc_doulongvec_minmax,
    .extra1		= &rds_ib_sysctl_max_wr_min,
    .extra2		= &rds_ib_sysctl_max_wr_max,
    },
    {
    .procname       = "max_unsignaled_wr",
    .data		= &rds_ib_sysctl_max_unsig_wrs,
    .maxlen         = sizeof(unsigned long),
    .mode           = 0644,
    .proc_handler   = proc_doulongvec_minmax,
    .extra1		= &rds_ib_sysctl_max_unsig_wr_min,
    .extra2		= &rds_ib_sysctl_max_unsig_wr_max,
    },
    {
    .procname       = "max_recv_allocation",
    .data		= &rds_ib_sysctl_max_recv_allocation,
    .maxlen         = sizeof(unsigned long),
    .mode           = 0644,
    .proc_handler   = proc_doulongvec_minmax,
    },
    {
    .procname	= "flow_control",
    .data		= &rds_ib_sysctl_flow_control,
    .maxlen		= sizeof(rds_ib_sysctl_flow_control),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn rds_ib_sysctl_exit() {
    void rds_ib_sysctl_exit(void)
    {
    if (rds_ib_sysctl_hdr)
    unregister_net_sysctl_table(rds_ib_sysctl_hdr);
    }
#[no_mangle]
pub unsafe extern "C" fn rds_ib_sysctl_init() -> c_int {
    int rds_ib_sysctl_init(void)
    {
    rds_ib_sysctl_hdr = register_net_sysctl(&init_net, "net/rds/ib", rds_ib_sysctl_table);
    if (!rds_ib_sysctl_hdr)
    return -ENOMEM;
    return 0;
    }
