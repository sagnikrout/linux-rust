//! Automatically rewritten from C to Rust
//! Source: fs/lockd/clntlock.c
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
// linux/fs/lockd/clntlock.c
//
// Lock handling for the client side NLM implementation
//
// Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
//

//
// Local function prototypes
//
    static int			reclaimer(void *ptr);
//
// The following functions handle blocking and granting from the
// client perspective.
//
    static LIST_HEAD(nlm_blocked);
    static DEFINE_SPINLOCK(nlm_blocked_lock);
//
// nlmclnt_init - Set up per-NFS mount point lockd data structures
// @nlm_init: pointer to arguments structure
//
// Returns pointer to an appropriate nlm_host struct,
// or an ERR_PTR value.
//
    struct nlm_host *nlmclnt_init(const struct nlmclnt_initdata *nlm_init)
    {
    struct nlm_host *host;
    let mut nlm_version: u32 = (nlm_init.nfs_version == 2) ? 1 : 4;
    int status;
    status = lockd_up(nlm_init.net, nlm_init.cred);
    if (status < 0)
    return ERR_PTR(status);
    host = nlmclnt_lookup_host(nlm_init.address, nlm_init.addrlen,
    nlm_init.protocol, nlm_version,
    nlm_init.hostname, nlm_init.noresvport,
    nlm_init.net, nlm_init.cred);
    if (host == core::ptr::null_mut())
    goto out_nohost;
    if (host.h_rpcclnt == core::ptr::null_mut() && nlm_bind_host(host) == core::ptr::null_mut())
    goto out_nobind;
    host.h_nlmclnt_ops = nlm_init.nlmclnt_ops;
    return host;
    out_nobind:
    nlmclnt_release_host(host);
    out_nohost:
    lockd_down(nlm_init.net);
    return ERR_PTR(-ENOLCK);
    }
    EXPORT_SYMBOL_GPL(nlmclnt_init);
//
// nlmclnt_done - Release resources allocated by nlmclnt_init()
// @host: nlm_host structure reserved by nlmclnt_init()
//
#[no_mangle]
pub unsafe extern "C" fn nlmclnt_done(host: *mut nlm_host) {
    void nlmclnt_done(struct nlm_host *host)
    {
    struct net *net = host.net;
    nlmclnt_release_host(host);
    lockd_down(net);
    }
    EXPORT_SYMBOL_GPL(nlmclnt_done);
#[no_mangle]
pub unsafe extern "C" fn nlmclnt_prepare_block(block: *mut nlm_wait, host: *mut nlm_host, fl: *mut file_lock) {
    void nlmclnt_prepare_block(struct nlm_wait *block, struct nlm_host *host, struct file_lock *fl)
    {
    block.b_host = host;
    block.b_lock = fl;
    init_waitqueue_head(&block.b_wait);
    block.b_status = nlm_lck_blocked;
    }
    struct rpc_clnt *nlmclnt_rpc_clnt(struct nlm_host *host)
    {
    return host.h_rpcclnt;
    }
    EXPORT_SYMBOL_GPL(nlmclnt_rpc_clnt);
//
// Queue up a lock for blocking so that the GRANTED request can see it
//
#[no_mangle]
pub unsafe extern "C" fn nlmclnt_queue_block(block: *mut nlm_wait) {
    void nlmclnt_queue_block(struct nlm_wait *block)
    {
    spin_lock(&nlm_blocked_lock);
    list_add(&block.b_list, &nlm_blocked);
    spin_unlock(&nlm_blocked_lock);
    }
//
// Dequeue the block and return its final status
//
#[no_mangle]
pub unsafe extern "C" fn nlmclnt_dequeue_block(block: *mut nlm_wait) -> __be32 {
    __be32 nlmclnt_dequeue_block(struct nlm_wait *block)
    {
    __be32 status;
    spin_lock(&nlm_blocked_lock);
    list_del(&block.b_list);
    status = block.b_status;
    spin_unlock(&nlm_blocked_lock);
    return status;
    }
//
// Block on a lock
//
#[no_mangle]
pub unsafe extern "C" fn nlmclnt_wait(block: *mut nlm_wait, req: *mut nlm_rqst, timeout: c_long) -> c_int {
    int nlmclnt_wait(struct nlm_wait *block, struct nlm_rqst *req, long timeout)
    {
    long ret;
// A borken server might ask us to block even if we didn't
// request it. Just say no!
//
    if (block == core::ptr::null_mut())
    return -EAGAIN;
// Go to sleep waiting for GRANT callback. Some servers seem
// to lose callbacks, however, so we're going to poll from
// time to time just to make sure.
//
// For now, the retry frequency is pretty high; normally
// a 1 minute timeout would do. See the comment before
// nlmclnt_lock for an explanation.
//
    ret = wait_event_interruptible_timeout(block.b_wait,
    block.b_status != nlm_lck_blocked,
    timeout);
    if (ret < 0)
    return -ERESTARTSYS;
// Reset the lock status after a server reboot so we resend
    if (block.b_status == nlm_lck_denied_grace_period)
    block.b_status = nlm_lck_blocked;
    return 0;
    }
//
// The server lockd has called us back to tell us the lock was granted
//
#[no_mangle]
pub unsafe extern "C" fn nlmclnt_grant(addr: *const sockaddr, lock: *const lockd_lock) -> __be32 {
    __be32 nlmclnt_grant(const struct sockaddr *addr, const struct lockd_lock *lock)
    {
    const struct file_lock *fl = &lock.fl;
    const struct nfs_fh *fh = &lock.fh;
    struct nlm_wait	*block;
    let mut res: __be32 = nlm_lck_denied;
//
// Look up blocked request based on arguments.
// Warning: must not use cookie to match it!
//
    spin_lock(&nlm_blocked_lock);
    list_for_each_entry(block, &nlm_blocked, b_list) {
    struct file_lock *fl_blocked = block.b_lock;
    if (fl_blocked.fl_start != fl.fl_start)
    continue;
    if (fl_blocked.fl_end != fl.fl_end)
    continue;
//
// Careful! The NLM server will return the 32-bit "pid" that
// we put on the wire: in this case the lockowner "pid".
//
    if (fl_blocked.fl_u.nfs_fl.owner.pid != lock.svid)
    continue;
    if (!rpc_cmp_addr(nlm_addr(block.b_host), addr))
    continue;
    if (nfs_compare_fh(NFS_FH(file_inode(fl_blocked.c.flc_file)), fh) != 0)
    continue;
// Alright, we found a lock. Set the return status
// and wake up the caller
//
    block.b_status = nlm_granted;
    wake_up(&block.b_wait);
    res = nlm_granted;
    }
    spin_unlock(&nlm_blocked_lock);
    trace_nlmclnt_grant(lock, addr, svc_addr_len(addr), res);
    return res;
    }
//
// The following procedures deal with the recovery of locks after a
// server crash.
//
// Reclaim all locks on server host. We do this by spawning a separate
// reclaimer thread.
//
    void
    nlmclnt_recovery(struct nlm_host *host)
    {
    struct task_struct *task;
    if (!host.h_reclaiming++) {
    nlm_get_host(host);
    task = kthread_run(reclaimer, host, "%s-reclaim", host.h_name);
    if (IS_ERR(task))
    printk(KERN_ERR "lockd: unable to spawn reclaimer "
    "thread. Locks for %s won't be reclaimed! "
    "(%ld)\n", host.h_name, PTR_ERR(task));
    }
    }
    static int
    reclaimer(void *ptr)
    {
    struct nlm_host	  *host = (struct nlm_host *) ptr;
    struct nlm_wait	  *block;
    struct nlm_rqst   *req;
    struct file_lock *fl, *next;
    u32 nsmstate;
    struct net *net = host.net;
    req = kmalloc_obj(*req);
    if (!req)
    return 0;
    allow_signal(SIGKILL);
    down_write(&host.h_rwsem);
    lockd_up(net, core::ptr::null_mut());	/* note: this cannot fail as lockd is already running */
    dprintk("lockd: reclaiming locks for host %s\n", host.h_name);
    restart:
    nsmstate = host.h_nsmstate;
// Force a portmap getport - the peer's lockd will
// most likely end up on a different port.
//
    host.h_nextrebind = jiffies;
    nlm_rebind_host(host);
// First, reclaim all locks that have been granted.
    list_splice_init(&host.h_granted, &host.h_reclaim);
    list_for_each_entry_safe(fl, next, &host.h_reclaim, fl_u.nfs_fl.list) {
    list_del_init(&fl.fl_u.nfs_fl.list);
//
// sending this thread a SIGKILL will result in any unreclaimed
// locks being removed from the h_granted list. This means that
// the kernel will not attempt to reclaim them again if a new
// reclaimer thread is spawned for this host.
//
    if (signalled())
    continue;
    if (nlmclnt_reclaim(host, fl, req) != 0)
    continue;
    list_add_tail(&fl.fl_u.nfs_fl.list, &host.h_granted);
    if (host.h_nsmstate != nsmstate) {
// Argh! The server rebooted again!
    goto restart;
    }
    }
    host.h_reclaiming = 0;
    up_write(&host.h_rwsem);
    dprintk("NLM: done reclaiming locks for host %s\n", host.h_name);
// Now, wake up all processes that sleep on a blocked lock
    spin_lock(&nlm_blocked_lock);
    list_for_each_entry(block, &nlm_blocked, b_list) {
    if (block.b_host == host) {
    block.b_status = nlm_lck_denied_grace_period;
    wake_up(&block.b_wait);
    }
    }
    spin_unlock(&nlm_blocked_lock);
// Release host handle after use
    nlmclnt_release_host(host);
    lockd_down(net);
    kfree(req);
    return 0;
    }
