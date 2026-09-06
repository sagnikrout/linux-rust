//! Automatically rewritten from C to Rust
//! Source: net/sunrpc/backchannel_rqst.c
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
    (c) 2007 Network Appliance, Inc.  All Rights Reserved.
    (c) 2009 NetApp.  All Rights Reserved.
//

#[no_mangle]
pub unsafe extern "C" fn xprt_bc_max_slots(xprt: *mut rpc_xprt) -> c_uint {
    unsigned int xprt_bc_max_slots(struct rpc_xprt *xprt)
    {
    return BC_MAX_SLOTS;
    }
//
// Close the backchannel producer side, drain any requests still
// queued on sv_cb_list, then destroy the callback service.
//
#[no_mangle]
pub unsafe extern "C" fn xprt_svc_destroy_nullify_bc(xprt: *mut rpc_xprt, serv: *mut svc_serv) {
    void xprt_svc_destroy_nullify_bc(struct rpc_xprt *xprt, struct svc_serv **serv)
    {
    struct svc_serv *bc_serv = *serv;
    struct rpc_rqst *req;
    xprt_svc_shutdown_bc(xprt);
    while ((req = lwq_dequeue(&bc_serv.sv_cb_list, struct rpc_rqst,
    rq_bc_list)) != core::ptr::null_mut()) {
    atomic_dec(&req.rq_xprt.bc_slot_count);
    xprt_free_bc_request(req);
    }
    svc_destroy(serv);
    }
    EXPORT_SYMBOL_GPL(xprt_svc_destroy_nullify_bc);
//
// Clear the backchannel server pointer in the transport.  The NULL
// store is serialized under bc_pa_lock against readers of
// xprt->bc_serv in xprt_complete_bc_request() and
// rpcrdma_bc_receive_call().  Clearing it before the callback service
// is stopped prevents a producer from enqueueing onto a service that
// is being torn down.
//
#[no_mangle]
pub unsafe extern "C" fn xprt_svc_shutdown_bc(xprt: *mut rpc_xprt) {
    void xprt_svc_shutdown_bc(struct rpc_xprt *xprt)
    {
    spin_lock(&xprt.bc_pa_lock);
    xprt.bc_serv = core::ptr::null_mut();
    spin_unlock(&xprt.bc_pa_lock);
    }
    EXPORT_SYMBOL_GPL(xprt_svc_shutdown_bc);
//
// Helper routines that track the number of preallocation elements
// on the transport.
//
#[no_mangle]
pub unsafe extern "C" fn xprt_need_to_requeue(xprt: *mut rpc_xprt) -> c_int {
    static inline int xprt_need_to_requeue(struct rpc_xprt *xprt)
    {
    return xprt.bc_alloc_count < xprt.bc_alloc_max;
    }
//
// Free the preallocated rpc_rqst structure and the memory
// buffers hanging off of it.
//
#[no_mangle]
unsafe extern "C" fn xprt_free_allocation(req: *mut rpc_rqst) {
    static void xprt_free_allocation(struct rpc_rqst *req)
    {
    struct xdr_buf *xbufp;
    dprintk("RPC:        free allocations for req= %p\n", req);
    WARN_ON_ONCE(test_bit(RPC_BC_PA_IN_USE, &req.rq_bc_pa_state));
    xbufp = &req.rq_rcv_buf;
    free_page((unsigned long)xbufp.head[0].iov_base);
    xbufp = &req.rq_snd_buf;
    free_page((unsigned long)xbufp.head[0].iov_base);
    kfree(req);
    }
#[no_mangle]
unsafe extern "C" fn xprt_bc_reinit_xdr_buf(buf: *mut xdr_buf) {
    static void xprt_bc_reinit_xdr_buf(struct xdr_buf *buf)
    {
    buf.head[0].iov_len = PAGE_SIZE;
    buf.tail[0].iov_len = 0;
    buf.pages = core::ptr::null_mut();
    buf.page_len = 0;
    buf.flags = 0;
    buf.len = 0;
    buf.buflen = PAGE_SIZE;
    }
#[no_mangle]
unsafe extern "C" fn xprt_alloc_xdr_buf(buf: *mut xdr_buf, gfp_flags: gfp_t) -> c_int {
    static int xprt_alloc_xdr_buf(struct xdr_buf *buf, gfp_t gfp_flags)
    {
    struct page *page;
// Preallocate one XDR receive buffer
    page = alloc_page(gfp_flags);
    if (page == core::ptr::null_mut())
    return -ENOMEM;
    xdr_buf_init(buf, page_address(page), PAGE_SIZE);
    return 0;
    }
    static struct rpc_rqst *xprt_alloc_bc_req(struct rpc_xprt *xprt)
    {
    let mut gfp_flags: gfp_t = GFP_KERNEL | __GFP_NORETRY | __GFP_NOWARN;
    struct rpc_rqst *req;
// Pre-allocate one backchannel rpc_rqst
    req = kzalloc_obj(*req, gfp_flags);
    if (req == core::ptr::null_mut())
    return core::ptr::null_mut();
    req.rq_xprt = xprt;
// Preallocate one XDR receive buffer
    if (xprt_alloc_xdr_buf(&req.rq_rcv_buf, gfp_flags) < 0) {
    printk(KERN_ERR "Failed to create bc receive xbuf\n");
    goto out_free;
    }
    req.rq_rcv_buf.len = PAGE_SIZE;
// Preallocate one XDR send buffer
    if (xprt_alloc_xdr_buf(&req.rq_snd_buf, gfp_flags) < 0) {
    printk(KERN_ERR "Failed to create bc snd xbuf\n");
    goto out_free;
    }
    return req;
    out_free:
    xprt_free_allocation(req);
    return core::ptr::null_mut();
    }
//
// Preallocate up to min_reqs structures and related buffers for use
// by the backchannel.  This function can be called multiple times
// when creating new sessions that use the same rpc_xprt.  The
// preallocated buffers are added to the pool of resources used by
// the rpc_xprt.  Any one of these resources may be used by an
// incoming callback request.  It's up to the higher levels in the
// stack to enforce that the maximum number of session slots is not
// being exceeded.
//
// Some callback arguments can be large.  For example, a pNFS server
// using multiple deviceids.  The list can be unbound, but the client
// has the ability to tell the server the maximum size of the callback
// requests.  Each deviceID is 16 bytes, so allocate one page
// for the arguments to have enough room to receive a number of these
// deviceIDs.  The NFS client indicates to the pNFS server that its
// callback requests can be up to 4096 bytes in size.
//
#[no_mangle]
pub unsafe extern "C" fn xprt_setup_backchannel(xprt: *mut rpc_xprt, min_reqs: c_uint) -> c_int {
    int xprt_setup_backchannel(struct rpc_xprt *xprt, unsigned int min_reqs)
    {
    if (!xprt.ops.bc_setup)
    return 0;
    return xprt.ops.bc_setup(xprt, min_reqs);
    }
    EXPORT_SYMBOL_GPL(xprt_setup_backchannel);
#[no_mangle]
pub unsafe extern "C" fn xprt_setup_bc(xprt: *mut rpc_xprt, min_reqs: c_uint) -> c_int {
    int xprt_setup_bc(struct rpc_xprt *xprt, unsigned int min_reqs)
    {
    struct rpc_rqst *req;
    LIST_HEAD(tmp_list);
    int i;
    dprintk("RPC:       setup backchannel transport\n");
    if (min_reqs > BC_MAX_SLOTS)
    min_reqs = BC_MAX_SLOTS;
//
// We use a temporary list to keep track of the preallocated
// buffers.  Once we're done building the list we splice it
// into the backchannel preallocation list off of the rpc_xprt
// struct.  This helps minimize the amount of time the list
// lock is held on the rpc_xprt struct.  It also makes cleanup
// easier in case of memory allocation errors.
//
    for (i = 0; i < min_reqs; i++) {
// Pre-allocate one backchannel rpc_rqst
    req = xprt_alloc_bc_req(xprt);
    if (req == core::ptr::null_mut()) {
    printk(KERN_ERR "Failed to create bc rpc_rqst\n");
    goto out_free;
    }
// Add the allocated buffer to the tmp list
    dprintk("RPC:       adding req= %p\n", req);
    list_add(&req.rq_bc_pa_list, &tmp_list);
    }
//
// Add the temporary list to the backchannel preallocation list
//
    spin_lock(&xprt.bc_pa_lock);
    list_splice(&tmp_list, &xprt.bc_pa_list);
    xprt.bc_alloc_count += min_reqs;
    xprt.bc_alloc_max += min_reqs;
    atomic_add(min_reqs, &xprt.bc_slot_count);
    spin_unlock(&xprt.bc_pa_lock);
    dprintk("RPC:       setup backchannel transport done\n");
    return 0;
    out_free:
//
// Memory allocation failed, free the temporary list
//
    while (!list_empty(&tmp_list)) {
    req = list_first_entry(&tmp_list,
    struct rpc_rqst,
    rq_bc_pa_list);
    list_del(&req.rq_bc_pa_list);
    xprt_free_allocation(req);
    }
    dprintk("RPC:       setup backchannel transport failed\n");
    return -ENOMEM;
    }
//
// xprt_destroy_backchannel - Destroys the backchannel preallocated structures.
// @xprt:	the transport holding the preallocated strucures
// @max_reqs:	the maximum number of preallocated structures to destroy
//
// Since these structures may have been allocated by multiple calls
// to xprt_setup_backchannel, we only destroy up to the maximum number
// of reqs specified by the caller.
//
#[no_mangle]
pub unsafe extern "C" fn xprt_destroy_backchannel(xprt: *mut rpc_xprt, max_reqs: c_uint) {
    void xprt_destroy_backchannel(struct rpc_xprt *xprt, unsigned int max_reqs)
    {
    if (xprt.ops.bc_destroy)
    xprt.ops.bc_destroy(xprt, max_reqs);
    }
    EXPORT_SYMBOL_GPL(xprt_destroy_backchannel);
#[no_mangle]
pub unsafe extern "C" fn xprt_destroy_bc(xprt: *mut rpc_xprt, max_reqs: c_uint) {
    void xprt_destroy_bc(struct rpc_xprt *xprt, unsigned int max_reqs)
    {
    struct rpc_rqst *req = core::ptr::null_mut(), *tmp = core::ptr::null_mut();
    dprintk("RPC:        destroy backchannel transport\n");
    if (max_reqs == 0)
    goto out;
    spin_lock_bh(&xprt.bc_pa_lock);
    xprt.bc_alloc_max -= min(max_reqs, xprt.bc_alloc_max);
    list_for_each_entry_safe(req, tmp, &xprt.bc_pa_list, rq_bc_pa_list) {
    dprintk("RPC:        req=%p\n", req);
    list_del(&req.rq_bc_pa_list);
    xprt_free_allocation(req);
    xprt.bc_alloc_count--;
    atomic_dec(&xprt.bc_slot_count);
    if (--max_reqs == 0)
    break;
    }
    spin_unlock_bh(&xprt.bc_pa_lock);
    out:
    dprintk("RPC:        backchannel list empty= %s\n",
    list_empty(&xprt.bc_pa_list) ? "true" : "false");
    }
    static struct rpc_rqst *xprt_get_bc_request(struct rpc_xprt *xprt, __be32 xid,
    struct rpc_rqst *new)
    {
    struct rpc_rqst *req = core::ptr::null_mut();
    dprintk("RPC:       allocate a backchannel request\n");
    if (list_empty(&xprt.bc_pa_list)) {
    if (!new)
    goto not_found;
    if (atomic_read(&xprt.bc_slot_count) >= BC_MAX_SLOTS)
    goto not_found;
    list_add_tail(&new.rq_bc_pa_list, &xprt.bc_pa_list);
    xprt.bc_alloc_count++;
    atomic_inc(&xprt.bc_slot_count);
    }
    req = list_first_entry(&xprt.bc_pa_list, struct rpc_rqst,
    rq_bc_pa_list);
    req.rq_reply_bytes_recvd = 0;
    memcpy(&req.rq_private_buf, &req.rq_rcv_buf,
    sizeof(req.rq_private_buf));
    req.rq_xid = xid;
    req.rq_connect_cookie = xprt.connect_cookie;
    dprintk("RPC:       backchannel req=%p\n", req);
    not_found:
    return req;
    }
//
// Return the preallocated rpc_rqst structure and XDR buffers
// associated with this rpc_task.
//
#[no_mangle]
pub unsafe extern "C" fn xprt_free_bc_request(req: *mut rpc_rqst) {
    void xprt_free_bc_request(struct rpc_rqst *req)
    {
    struct rpc_xprt *xprt = req.rq_xprt;
    xprt.ops.bc_free_rqst(req);
    }
#[no_mangle]
pub unsafe extern "C" fn xprt_free_bc_rqst(req: *mut rpc_rqst) {
    void xprt_free_bc_rqst(struct rpc_rqst *req)
    {
    struct rpc_xprt *xprt = req.rq_xprt;
    dprintk("RPC:       free backchannel req=%p\n", req);
    req.rq_connect_cookie = xprt.connect_cookie - 1;
    smp_mb__before_atomic();
    clear_bit(RPC_BC_PA_IN_USE, &req.rq_bc_pa_state);
    smp_mb__after_atomic();
//
// Return it to the list of preallocations so that it
// may be reused by a new callback request.
//
    spin_lock_bh(&xprt.bc_pa_lock);
    if (xprt_need_to_requeue(xprt)) {
    xprt_bc_reinit_xdr_buf(&req.rq_snd_buf);
    xprt_bc_reinit_xdr_buf(&req.rq_rcv_buf);
    req.rq_rcv_buf.len = PAGE_SIZE;
    list_add_tail(&req.rq_bc_pa_list, &xprt.bc_pa_list);
    xprt.bc_alloc_count++;
    atomic_inc(&xprt.bc_slot_count);
    req = core::ptr::null_mut();
    }
    spin_unlock_bh(&xprt.bc_pa_lock);
    if (req != core::ptr::null_mut()) {
//
// The last remaining session was destroyed while this
// entry was in use.  Free the entry and don't attempt
// to add back to the list because there is no need to
// have anymore preallocated entries.
//
    dprintk("RPC:       Last session removed req=%p\n", req);
    xprt_free_allocation(req);
    }
    xprt_put(xprt);
    }
//
// One or more rpc_rqst structure have been preallocated during the
// backchannel setup.  Buffer space for the send and private XDR buffers
// has been preallocated as well.  Use xprt_alloc_bc_request to allocate
// to this request.  Use xprt_free_bc_request to return it.
//
// We know that we're called in soft interrupt context, grab the spin_lock
// since there is no need to grab the bottom half spin_lock.
//
// Return an available rpc_rqst, otherwise NULL if non are available.
//
    struct rpc_rqst *xprt_lookup_bc_request(struct rpc_xprt *xprt, __be32 xid)
    {
    struct rpc_rqst *req, *new = core::ptr::null_mut();
    do {
    spin_lock(&xprt.bc_pa_lock);
    list_for_each_entry(req, &xprt.bc_pa_list, rq_bc_pa_list) {
    if (req.rq_connect_cookie != xprt.connect_cookie)
    continue;
    if (req.rq_xid == xid)
    goto found;
    }
    req = xprt_get_bc_request(xprt, xid, new);
    found:
    spin_unlock(&xprt.bc_pa_lock);
    if (new) {
    if (req != new)
    xprt_free_allocation(new);
    break;
    } else if (req)
    break;
    new = xprt_alloc_bc_req(xprt);
    } while (new);
    return req;
    }
//
// Add callback request to callback list.  Wake a thread
// on the first pool (usually the only pool) to handle it.
//
#[no_mangle]
pub unsafe extern "C" fn xprt_complete_bc_request(req: *mut rpc_rqst, copied: u32) {
    void xprt_complete_bc_request(struct rpc_rqst *req, uint32_t copied)
    {
    struct rpc_xprt *xprt = req.rq_xprt;
    spin_lock(&xprt.bc_pa_lock);
    list_del(&req.rq_bc_pa_list);
    xprt.bc_alloc_count--;
    spin_unlock(&xprt.bc_pa_lock);
    req.rq_private_buf.len = copied;
    set_bit(RPC_BC_PA_IN_USE, &req.rq_bc_pa_state);
    dprintk("RPC:       add callback request to list\n");
    xprt_enqueue_bc_request(req);
    }
#[no_mangle]
pub unsafe extern "C" fn xprt_enqueue_bc_request(req: *mut rpc_rqst) {
    void xprt_enqueue_bc_request(struct rpc_rqst *req)
    {
    struct rpc_xprt *xprt = req.rq_xprt;
    struct svc_serv *bc_serv;
    xprt_get(xprt);
    spin_lock(&xprt.bc_pa_lock);
    bc_serv = xprt.bc_serv;
    if (bc_serv) {
    lwq_enqueue(&req.rq_bc_list, &bc_serv.sv_cb_list);
    svc_pool_wake_idle_thread(&bc_serv.sv_pools[0]);
    spin_unlock(&xprt.bc_pa_lock);
    return;
    }
    spin_unlock(&xprt.bc_pa_lock);
    atomic_dec(&xprt.bc_slot_count);
    xprt_free_bc_request(req);
    }
    EXPORT_SYMBOL_GPL(xprt_enqueue_bc_request);
