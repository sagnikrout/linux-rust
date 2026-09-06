//! Automatically rewritten from C to Rust
//! Source: net/sunrpc/xprtrdma/svc_rdma.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (c) 2015-2018 Oracle.  All rights reserved.
// Copyright (c) 2005-2006 Network Appliance, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the BSD-type
// license below:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//
// Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials provided
// with the distribution.
//
// Neither the name of the Network Appliance, Inc. nor the names of
// its contributors may be used to endorse or promote products
// derived from this software without specific prior written
// permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Author: Tom Tucker <tom@opengridcomputing.com>
//

// RPC/RDMA parameters
    unsigned int svcrdma_ord = 16;	/* historical default */
    let mut min_ord: static unsigned int = 1;
    let mut max_ord: static unsigned int = 255;
    let mut svcrdma_max_requests: c_uint = RPCRDMA_MAX_REQUESTS;
    let mut svcrdma_max_bc_requests: c_uint = RPCRDMA_MAX_BC_REQUESTS;
    let mut min_max_requests: static unsigned int = 4;
    let mut max_max_requests: static unsigned int = 16384;
    let mut svcrdma_max_req_size: c_uint = RPCRDMA_DEF_INLINE_THRESH;
    let mut min_max_inline: static unsigned int = RPCRDMA_DEF_INLINE_THRESH;
    let mut max_max_inline: static unsigned int = RPCRDMA_MAX_INLINE_THRESH;
    static unsigned int svcrdma_stat_unused;
    static unsigned int zero;
    struct percpu_counter svcrdma_stat_read;
    struct percpu_counter svcrdma_stat_recv;
    struct percpu_counter svcrdma_stat_sq_starve;
    struct percpu_counter svcrdma_stat_write;
    enum {
    SVCRDMA_COUNTER_BUFSIZ	= sizeof(unsigned long long),
    };
    static int svcrdma_counter_handler(const struct ctl_table *table, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    struct percpu_counter *stat = (struct percpu_counter *)table.data;
    char tmp[SVCRDMA_COUNTER_BUFSIZ + 1];
    int len;
    if (write) {
    percpu_counter_set(stat, 0);
    return 0;
    }
    len = snprintf(tmp, SVCRDMA_COUNTER_BUFSIZ, "%lld\n",
    percpu_counter_sum_positive(stat));
    if (len >= SVCRDMA_COUNTER_BUFSIZ)
    return -EFAULT;
    len = strlen(tmp);
    if (*ppos > len) {
// lenp = 0;
    return 0;
    }
    len -= *ppos;
    if (len > *lenp)
    len = *lenp;
    if (len)
    memcpy(buffer, tmp, len);
// lenp = len;
// ppos += len;
    return 0;
    }
    static struct ctl_table_header *svcrdma_table_header;
    static struct ctl_table svcrdma_parm_table[] = {
    {
    .procname	= "max_requests",
    .data		= &svcrdma_max_requests,
    .maxlen		= sizeof(unsigned int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &min_max_requests,
    .extra2		= &max_max_requests
    },
    {
    .procname	= "max_req_size",
    .data		= &svcrdma_max_req_size,
    .maxlen		= sizeof(unsigned int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &min_max_inline,
    .extra2		= &max_max_inline
    },
    {
    .procname	= "max_outbound_read_requests",
    .data		= &svcrdma_ord,
    .maxlen		= sizeof(unsigned int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &min_ord,
    .extra2		= &max_ord,
    },
    {
    .procname	= "rdma_stat_read",
    .data		= &svcrdma_stat_read,
    .maxlen		= SVCRDMA_COUNTER_BUFSIZ,
    .mode		= 0644,
    .proc_handler	= svcrdma_counter_handler,
    },
    {
    .procname	= "rdma_stat_recv",
    .data		= &svcrdma_stat_recv,
    .maxlen		= SVCRDMA_COUNTER_BUFSIZ,
    .mode		= 0644,
    .proc_handler	= svcrdma_counter_handler,
    },
    {
    .procname	= "rdma_stat_write",
    .data		= &svcrdma_stat_write,
    .maxlen		= SVCRDMA_COUNTER_BUFSIZ,
    .mode		= 0644,
    .proc_handler	= svcrdma_counter_handler,
    },
    {
    .procname	= "rdma_stat_sq_starve",
    .data		= &svcrdma_stat_sq_starve,
    .maxlen		= SVCRDMA_COUNTER_BUFSIZ,
    .mode		= 0644,
    .proc_handler	= svcrdma_counter_handler,
    },
    {
    .procname	= "rdma_stat_rq_starve",
    .data		= &svcrdma_stat_unused,
    .maxlen		= sizeof(unsigned int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &zero,
    .extra2		= &zero,
    },
    {
    .procname	= "rdma_stat_rq_poll",
    .data		= &svcrdma_stat_unused,
    .maxlen		= sizeof(unsigned int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &zero,
    .extra2		= &zero,
    },
    {
    .procname	= "rdma_stat_rq_prod",
    .data		= &svcrdma_stat_unused,
    .maxlen		= sizeof(unsigned int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &zero,
    .extra2		= &zero,
    },
    {
    .procname	= "rdma_stat_sq_poll",
    .data		= &svcrdma_stat_unused,
    .maxlen		= sizeof(unsigned int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &zero,
    .extra2		= &zero,
    },
    {
    .procname	= "rdma_stat_sq_prod",
    .data		= &svcrdma_stat_unused,
    .maxlen		= sizeof(unsigned int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &zero,
    .extra2		= &zero,
    },
    };
#[no_mangle]
unsafe extern "C" fn svc_rdma_proc_cleanup() {
    static void svc_rdma_proc_cleanup(void)
    {
    if (!svcrdma_table_header)
    return;
    unregister_sysctl_table(svcrdma_table_header);
    svcrdma_table_header = core::ptr::null_mut();
    percpu_counter_destroy(&svcrdma_stat_write);
    percpu_counter_destroy(&svcrdma_stat_sq_starve);
    percpu_counter_destroy(&svcrdma_stat_recv);
    percpu_counter_destroy(&svcrdma_stat_read);
    }
#[no_mangle]
unsafe extern "C" fn svc_rdma_proc_init() -> c_int {
    static int svc_rdma_proc_init(void)
    {
    int rc;
    if (svcrdma_table_header)
    return 0;
    rc = percpu_counter_init(&svcrdma_stat_read, 0, GFP_KERNEL);
    if (rc)
    goto err;
    rc = percpu_counter_init(&svcrdma_stat_recv, 0, GFP_KERNEL);
    if (rc)
    goto err_read;
    rc = percpu_counter_init(&svcrdma_stat_sq_starve, 0, GFP_KERNEL);
    if (rc)
    goto err_recv;
    rc = percpu_counter_init(&svcrdma_stat_write, 0, GFP_KERNEL);
    if (rc)
    goto err_sq;
    svcrdma_table_header = register_sysctl("sunrpc/svc_rdma",
    svcrdma_parm_table);
    if (!svcrdma_table_header)
    goto err_write;
    return 0;
    err_write:
    rc = -ENOMEM;
    percpu_counter_destroy(&svcrdma_stat_write);
    err_sq:
    percpu_counter_destroy(&svcrdma_stat_sq_starve);
    err_recv:
    percpu_counter_destroy(&svcrdma_stat_recv);
    err_read:
    percpu_counter_destroy(&svcrdma_stat_read);
    err:
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn svc_rdma_cleanup() {
    void svc_rdma_cleanup(void)
    {
    svc_unreg_xprt_class(&svc_rdma_class);
    svc_rdma_proc_cleanup();
    dprintk("SVCRDMA Module Removed, deregister RPC RDMA transport\n");
    }
#[no_mangle]
pub unsafe extern "C" fn svc_rdma_init() -> c_int {
    int svc_rdma_init(void)
    {
    int rc;
    rc = svc_rdma_proc_init();
    if (rc)
    return rc;
    svc_reg_xprt_class(&svc_rdma_class);
    dprintk("SVCRDMA Module Init, register RPC RDMA transport\n");
    dprintk("\tsvcrdma_ord      : %d\n", svcrdma_ord);
    dprintk("\tmax_requests     : %u\n", svcrdma_max_requests);
    dprintk("\tmax_bc_requests  : %u\n", svcrdma_max_bc_requests);
    dprintk("\tmax_inline       : %d\n", svcrdma_max_req_size);
    return 0;
    }
