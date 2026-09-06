//! Automatically rewritten from C to Rust
//! Source: net/sunrpc/xprtrdma/module.c
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
// Copyright (c) 2015, 2017 Oracle.  All rights reserved.
//
// rpcrdma.ko module initialization
//

// Macro flag: #define CREATE_TRACE_POINTS

    MODULE_AUTHOR("Open Grid Computing and Network Appliance, Inc.");
    MODULE_DESCRIPTION("RPC/RDMA Transport");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_ALIAS("svcrdma");
    MODULE_ALIAS("xprtrdma");
    MODULE_ALIAS("rpcrdma6");
#[no_mangle]
unsafe extern "C" fn rpc_rdma_cleanup() -> void __exit {
    static void __exit rpc_rdma_cleanup(void)
    {
    xprt_rdma_cleanup();
    svc_rdma_cleanup();
    rpcrdma_ib_client_unregister();
    }
#[no_mangle]
unsafe extern "C" fn rpc_rdma_init() -> int __init {
    static int __init rpc_rdma_init(void)
    {
    int rc;
    rc = rpcrdma_ib_client_register();
    if (rc)
    goto out_rc;
    rc = svc_rdma_init();
    if (rc)
    goto out_ib_client;
    rc = xprt_rdma_init();
    if (rc)
    goto out_svc_rdma;
    return 0;
    out_svc_rdma:
    svc_rdma_cleanup();
    out_ib_client:
    rpcrdma_ib_client_unregister();
    out_rc:
    return rc;
    }
    module_init(rpc_rdma_init);
    module_exit(rpc_rdma_cleanup);
