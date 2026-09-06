//! Automatically rewritten from C to Rust
//! Source: fs/netfs/main.c
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
// Miscellaneous bits for the netfs support library.
//
// Copyright (C) 2022 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

// Macro flag: #define CREATE_TRACE_POINTS

    MODULE_DESCRIPTION("Network fs support");
    MODULE_AUTHOR("Red Hat, Inc.");
    MODULE_LICENSE("GPL");
    EXPORT_TRACEPOINT_SYMBOL(netfs_sreq);
    unsigned netfs_debug;
    module_param_named(debug, netfs_debug, uint, S_IWUSR | S_IRUGO);
    MODULE_PARM_DESC(netfs_debug, "Netfs support debugging mask");
    static struct kmem_cache *netfs_request_slab;
    static struct kmem_cache *netfs_subrequest_slab;
    mempool_t netfs_request_pool;
    mempool_t netfs_subrequest_pool;
    mempool_t netfs_folioq_pool;

    LIST_HEAD(netfs_io_requests);
    DEFINE_SPINLOCK(netfs_proc_lock);
    static const char *netfs_origins[nr__netfs_io_origin] = {
    [NETFS_READAHEAD]		= "RA",
    [NETFS_READPAGE]		= "RP",
    [NETFS_READ_GAPS]		= "RG",
    [NETFS_READ_SINGLE]		= "R1",
    [NETFS_READ_FOR_WRITE]		= "RW",
    [NETFS_UNBUFFERED_READ]		= "UR",
    [NETFS_DIO_READ]		= "DR",
    [NETFS_WRITEBACK]		= "WB",
    [NETFS_WRITEBACK_SINGLE]	= "W1",
    [NETFS_WRITETHROUGH]		= "WT",
    [NETFS_UNBUFFERED_WRITE]	= "UW",
    [NETFS_DIO_WRITE]		= "DW",
    [NETFS_PGPRIV2_COPY_TO_CACHE]	= "2C",
    };
//
// Generate a list of I/O requests in /proc/fs/netfs/requests
//
#[no_mangle]
unsafe extern "C" fn netfs_requests_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int netfs_requests_seq_show(struct seq_file *m, void *v)
    {
    struct netfs_io_request *rreq;
    if (v == &netfs_io_requests) {
    seq_puts(m,
    "REQUEST  OR REF FLAG ERR  OPS COVERAGE\n"
    "======== == === ==== ==== === =========\n"
    );
    return 0;
    }
    rreq = list_entry(v, struct netfs_io_request, proc_link);
    seq_printf(m,
    "%08x %s %3d %4lx %4ld %3d @%04llx %llx/%llx",
    rreq.debug_id,
    netfs_origins[rreq.origin],
    refcount_read(&rreq.ref),
    rreq.flags,
    rreq.error,
    0,
    rreq.start, rreq.submitted, rreq.len);
    seq_putc(m, '\n');
    return 0;
    }
    static void *netfs_requests_seq_start(struct seq_file *m, loff_t *_pos)
    __acquires(rcu)
    {
    rcu_read_lock();
    return seq_list_start_head(&netfs_io_requests, *_pos);
    }
    static void *netfs_requests_seq_next(struct seq_file *m, void *v, loff_t *_pos)
    {
    return seq_list_next(v, &netfs_io_requests, _pos);
    }
#[no_mangle]
unsafe extern "C" fn netfs_requests_seq_stop(m: *mut seq_file, v: *mut c_void) {
    static void netfs_requests_seq_stop(struct seq_file *m, void *v)
    __releases(rcu)
    {
    rcu_read_unlock();
    }
    static const struct seq_operations netfs_requests_seq_ops = {
    .start  = netfs_requests_seq_start,
    .next   = netfs_requests_seq_next,
    .stop   = netfs_requests_seq_stop,
    .show   = netfs_requests_seq_show,
    };

#[no_mangle]
unsafe extern "C" fn netfs_init() -> int __init {
    static int __init netfs_init(void)
    {
    let mut ret: c_int = -ENOMEM;
    if (mempool_init_kmalloc_pool(&netfs_folioq_pool, 100, sizeof(struct folio_queue)) < 0)
    goto error_folioq_pool;
    netfs_request_slab = kmem_cache_create("netfs_request",
    sizeof(struct netfs_io_request), 0,
    SLAB_HWCACHE_ALIGN | SLAB_ACCOUNT,
    core::ptr::null_mut());
    if (!netfs_request_slab)
    goto error_req;
    if (mempool_init_slab_pool(&netfs_request_pool, 100, netfs_request_slab) < 0)
    goto error_reqpool;
    netfs_subrequest_slab = kmem_cache_create("netfs_subrequest",
    sizeof(struct netfs_io_subrequest) + 16, 0,
    SLAB_HWCACHE_ALIGN | SLAB_ACCOUNT,
    core::ptr::null_mut());
    if (!netfs_subrequest_slab)
    goto error_subreq;
    if (mempool_init_slab_pool(&netfs_subrequest_pool, 100, netfs_subrequest_slab) < 0)
    goto error_subreqpool;

    if (!proc_mkdir("fs/netfs", core::ptr::null_mut()))
    goto error_proc;
    if (!proc_create_seq("fs/netfs/requests", S_IFREG | 0444, core::ptr::null_mut(),
    &netfs_requests_seq_ops))
    goto error_procfile;

    if (!proc_create_single("fs/netfs/stats", S_IFREG | 0444, core::ptr::null_mut(),
    netfs_stats_show))
    goto error_procfile;

    ret = fscache_init();
    if (ret < 0)
    goto error_fscache;
    return 0;
    error_fscache:

    error_procfile:
    remove_proc_subtree("fs/netfs", core::ptr::null_mut());
    error_proc:

    mempool_exit(&netfs_subrequest_pool);
    error_subreqpool:
    kmem_cache_destroy(netfs_subrequest_slab);
    error_subreq:
    mempool_exit(&netfs_request_pool);
    error_reqpool:
    kmem_cache_destroy(netfs_request_slab);
    error_req:
    mempool_exit(&netfs_folioq_pool);
    error_folioq_pool:
    return ret;
    }
    fs_initcall(netfs_init);
#[no_mangle]
unsafe extern "C" fn netfs_exit() -> void __exit {
    static void __exit netfs_exit(void)
    {
    fscache_exit();
    remove_proc_subtree("fs/netfs", core::ptr::null_mut());
    mempool_exit(&netfs_subrequest_pool);
    kmem_cache_destroy(netfs_subrequest_slab);
    mempool_exit(&netfs_request_pool);
    kmem_cache_destroy(netfs_request_slab);
    mempool_exit(&netfs_folioq_pool);
    }
    module_exit(netfs_exit);
