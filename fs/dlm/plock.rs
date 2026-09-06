//! Automatically rewritten from C to Rust
//! Source: fs/dlm/plock.c
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
// Copyright (C) 2005-2008 Red Hat, Inc.  All rights reserved.
//

    static DEFINE_SPINLOCK(ops_lock);
    static LIST_HEAD(send_list);
    static LIST_HEAD(recv_list);
    static DECLARE_WAIT_QUEUE_HEAD(send_wq);
    static DECLARE_WAIT_QUEUE_HEAD(recv_wq);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plock_async_data {
    pub fl: *mut c_void,
    pub file: *mut c_void,
    pub flc: file_lock,
    pub result): *mut *mut *mut int (callback)(struct file_lock fl, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plock_op {
    pub list: list_head,
    pub done: c_int,
    pub info: dlm_plock_info,
// if set indicates async handling
    pub data: *mut plock_async_data,
}

#[no_mangle]
pub unsafe extern "C" fn set_version(info: *mut dlm_plock_info) {
    static inline void set_version(struct dlm_plock_info *info)
    {
    info.version[0] = DLM_PLOCK_VERSION_MAJOR;
    info.version[1] = DLM_PLOCK_VERSION_MINOR;
    info.version[2] = DLM_PLOCK_VERSION_PATCH;
    }
    static struct plock_op *plock_lookup_waiter(const struct dlm_plock_info *info)
    {
    struct plock_op *op = core::ptr::null_mut(), *iter;
    list_for_each_entry(iter, &recv_list, list) {
    if (iter.info.fsid == info.fsid &&
    iter.info.number == info.number &&
    iter.info.owner == info.owner &&
    iter.info.pid == info.pid &&
    iter.info.start == info.start &&
    iter.info.end == info.end &&
    iter.info.ex == info.ex &&
    iter.info.wait) {
    op = iter;
    break;
    }
    }
    return op;
    }
#[no_mangle]
unsafe extern "C" fn check_version(info: *mut dlm_plock_info) -> c_int {
    static int check_version(struct dlm_plock_info *info)
    {
    if ((DLM_PLOCK_VERSION_MAJOR != info.version[0]) ||
    (DLM_PLOCK_VERSION_MINOR < info.version[1])) {
    log_print("plock device version mismatch: "
    "kernel (%u.%u.%u), user (%u.%u.%u)",
    DLM_PLOCK_VERSION_MAJOR,
    DLM_PLOCK_VERSION_MINOR,
    DLM_PLOCK_VERSION_PATCH,
    info.version[0],
    info.version[1],
    info.version[2]);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dlm_release_plock_op(op: *mut plock_op) {
    static void dlm_release_plock_op(struct plock_op *op)
    {
    kfree(op.data);
    kfree(op);
    }
#[no_mangle]
unsafe extern "C" fn send_op(op: *mut plock_op) {
    static void send_op(struct plock_op *op)
    {
    set_version(&op.info);
    spin_lock(&ops_lock);
    list_add_tail(&op.list, &send_list);
    spin_unlock(&ops_lock);
    wake_up(&send_wq);
    }
#[no_mangle]
unsafe extern "C" fn do_lock_cancel(orig_info: *const dlm_plock_info) -> c_int {
    static int do_lock_cancel(const struct dlm_plock_info *orig_info)
    {
    struct plock_op *op;
    int rv;
    op = kzalloc_obj(*op, GFP_NOFS);
    if (!op)
    return -ENOMEM;
    op.info = *orig_info;
    op.info.optype = DLM_PLOCK_OP_CANCEL;
    op.info.wait = 0;
    send_op(op);
    wait_event(recv_wq, (op.done != 0));
    rv = op.info.rv;
    dlm_release_plock_op(op);
    return rv;
    }
    int dlm_posix_lock(dlm_lockspace_t *lockspace, u64 number, struct file *file,
    int cmd, struct file_lock *fl)
    {
    struct plock_async_data *op_data;
    struct dlm_ls *ls;
    struct plock_op *op;
    int rv;
    ls = dlm_find_lockspace_local(lockspace);
    if (!ls)
    return -EINVAL;
    op = kzalloc_obj(*op, GFP_NOFS);
    if (!op) {
    rv = -ENOMEM;
    goto out;
    }
    op.info.optype		= DLM_PLOCK_OP_LOCK;
    op.info.pid		= fl.c.flc_pid;
    op.info.ex		= lock_is_write(fl);
    op.info.wait		= !!(fl.c.flc_flags & FL_SLEEP);
    op.info.fsid		= ls.ls_global_id;
    op.info.number		= number;
    op.info.start		= fl.fl_start;
    op.info.end		= fl.fl_end;
    op.info.owner = (__u64)(long) fl.c.flc_owner;
// async handling
    if (fl.fl_lmops && fl.fl_lmops.lm_grant) {
    op_data = kzalloc_obj(*op_data, GFP_NOFS);
    if (!op_data) {
    dlm_release_plock_op(op);
    rv = -ENOMEM;
    goto out;
    }
    op_data.callback = fl.fl_lmops.lm_grant;
    locks_init_lock(&op_data.flc);
    locks_copy_lock(&op_data.flc, fl);
    op_data.fl		= fl;
    op_data.file	= file;
    op.data = op_data;
    send_op(op);
    rv = FILE_LOCK_DEFERRED;
    goto out;
    }
    send_op(op);
    if (op.info.wait) {
    rv = wait_event_interruptible(recv_wq, (op.done != 0));
    if (rv == -ERESTARTSYS) {
    spin_lock(&ops_lock);
// recheck under ops_lock if we got a done != 0,
// if so this interrupt case should be ignored
//
    if (op.done != 0) {
    spin_unlock(&ops_lock);
    goto do_lock_wait;
    }
    spin_unlock(&ops_lock);
    rv = do_lock_cancel(&op.info);
    switch (rv) {
    case 0:
// waiter was deleted in user space, answer will never come
// remove original request. The original request must be
// on recv_list because the answer of do_lock_cancel()
// synchronized it.
//
    spin_lock(&ops_lock);
    list_del(&op.list);
    spin_unlock(&ops_lock);
    rv = -EINTR;
    break;
    case -ENOENT:
// cancellation wasn't successful but op should be done
    fallthrough;
    default:
// internal error doing cancel we need to wait
    goto wait;
    }
    log_debug(ls, "%s: wait interrupted %x %llx pid %d",
    __func__, ls.ls_global_id,
    (unsigned long long)number, op.info.pid);
    dlm_release_plock_op(op);
    goto out;
    }
    } else {
    wait:
    wait_event(recv_wq, (op.done != 0));
    }
    do_lock_wait:
    WARN_ON(!list_empty(&op.list));
    rv = op.info.rv;
    if (!rv) {
    if (locks_lock_file_wait(file, fl) < 0)
    log_error(ls, "dlm_posix_lock: vfs lock error %llx",
    (unsigned long long)number);
    }
    dlm_release_plock_op(op);
    out:
    dlm_put_lockspace(ls);
    return rv;
    }
    EXPORT_SYMBOL_GPL(dlm_posix_lock);
// Returns failure iff a successful lock operation should be canceled
#[no_mangle]
unsafe extern "C" fn dlm_plock_callback(op: *mut plock_op) -> c_int {
    static int dlm_plock_callback(struct plock_op *op)
    {
    struct plock_async_data *op_data = op.data;
    struct file *file;
    struct file_lock *fl;
    struct file_lock *flc;
    int (*notify)(struct file_lock *fl, int result) = core::ptr::null_mut();
    let mut rv: c_int = 0;
    WARN_ON(!list_empty(&op.list));
// check if the following 2 are still valid or make a copy
    file = op_data.file;
    flc = &op_data.flc;
    fl = op_data.fl;
    notify = op_data.callback;
    if (op.info.rv) {
    notify(fl, op.info.rv);
    goto out;
    }
// got fs lock; bookkeep locally as well:
    flc.c.flc_flags &= ~FL_SLEEP;
    if (posix_lock_file(file, flc, core::ptr::null_mut())) {
//
// This can only happen in the case of kmalloc() failure.
// The filesystem's own lock is the authoritative lock,
// so a failure to get the lock locally is not a disaster.
// As long as the fs cannot reliably cancel locks (especially
// in a low-memory situation), we're better off ignoring
// this failure than trying to recover.
//
    log_print("dlm_plock_callback: vfs lock error %llx file %p fl %p",
    (unsigned long long)op.info.number, file, fl);
    }
    rv = notify(fl, 0);
    if (rv) {
// XXX: We need to cancel the fs lock here:
    log_print("%s: lock granted after lock request failed; dangling lock!",
    __func__);
    goto out;
    }
    out:
    dlm_release_plock_op(op);
    return rv;
    }
    int dlm_posix_unlock(dlm_lockspace_t *lockspace, u64 number, struct file *file,
    struct file_lock *fl)
    {
    struct dlm_ls *ls;
    struct plock_op *op;
    int rv;
    let mut saved_flags: c_uchar = fl.c.flc_flags;
    ls = dlm_find_lockspace_local(lockspace);
    if (!ls)
    return -EINVAL;
    op = kzalloc_obj(*op, GFP_NOFS);
    if (!op) {
    rv = -ENOMEM;
    goto out;
    }
// cause the vfs unlock to return ENOENT if lock is not found
    fl.c.flc_flags |= FL_EXISTS;
    rv = locks_lock_file_wait(file, fl);
    if (rv == -ENOENT) {
    rv = 0;
    goto out_free;
    }
    if (rv < 0) {
    log_error(ls, "dlm_posix_unlock: vfs unlock error %d %llx",
    rv, (unsigned long long)number);
    }
    op.info.optype		= DLM_PLOCK_OP_UNLOCK;
    op.info.pid		= fl.c.flc_pid;
    op.info.fsid		= ls.ls_global_id;
    op.info.number		= number;
    op.info.start		= fl.fl_start;
    op.info.end		= fl.fl_end;
    op.info.owner = (__u64)(long) fl.c.flc_owner;
    if (fl.c.flc_flags & FL_CLOSE) {
    op.info.flags |= DLM_PLOCK_FL_CLOSE;
    send_op(op);
    rv = 0;
    goto out;
    }
    send_op(op);
    wait_event(recv_wq, (op.done != 0));
    WARN_ON(!list_empty(&op.list));
    rv = op.info.rv;
    if (rv == -ENOENT)
    rv = 0;
    out_free:
    dlm_release_plock_op(op);
    out:
    dlm_put_lockspace(ls);
    fl.c.flc_flags = saved_flags;
    return rv;
    }
    EXPORT_SYMBOL_GPL(dlm_posix_unlock);
//
// NOTE: This implementation can only handle async lock requests as nfs
// do it. It cannot handle cancellation of a pending lock request sitting
// in wait_event(), but for now only nfs is the only user local kernel
// user.
//
    int dlm_posix_cancel(dlm_lockspace_t *lockspace, u64 number, struct file *file,
    struct file_lock *fl)
    {
    struct dlm_plock_info info;
    struct plock_op *op;
    struct dlm_ls *ls;
    int rv;
// this only works for async request for now and nfs is the only
// kernel user right now.
//
    if (WARN_ON_ONCE(!fl.fl_lmops || !fl.fl_lmops.lm_grant))
    return -EOPNOTSUPP;
    ls = dlm_find_lockspace_local(lockspace);
    if (!ls)
    return -EINVAL;
    memset(&info, 0, sizeof(info));
    info.pid = fl.c.flc_pid;
    info.ex = lock_is_write(fl);
    info.fsid = ls.ls_global_id;
    dlm_put_lockspace(ls);
    info.number = number;
    info.start = fl.fl_start;
    info.end = fl.fl_end;
    info.owner = (__u64)(long) fl.c.flc_owner;
    rv = do_lock_cancel(&info);
    switch (rv) {
    case 0:
    spin_lock(&ops_lock);
// lock request to cancel must be on recv_list because
// do_lock_cancel() synchronizes it.
//
    op = plock_lookup_waiter(&info);
    if (WARN_ON_ONCE(!op)) {
    spin_unlock(&ops_lock);
    rv = -ENOLCK;
    break;
    }
    list_del(&op.list);
    spin_unlock(&ops_lock);
    WARN_ON(op.info.optype != DLM_PLOCK_OP_LOCK);
    op.data.callback(op.data.fl, -EINTR);
    dlm_release_plock_op(op);
    rv = -EINTR;
    break;
    case -ENOENT:
// if cancel wasn't successful we probably were to late
// or it was a non-blocking lock request, so just unlock it.
//
    rv = dlm_posix_unlock(lockspace, number, file, fl);
    break;
    default:
    break;
    }
    return rv;
    }
    EXPORT_SYMBOL_GPL(dlm_posix_cancel);
    int dlm_posix_get(dlm_lockspace_t *lockspace, u64 number, struct file *file,
    struct file_lock *fl)
    {
    struct dlm_ls *ls;
    struct plock_op *op;
    int rv;
    ls = dlm_find_lockspace_local(lockspace);
    if (!ls)
    return -EINVAL;
    op = kzalloc_obj(*op, GFP_NOFS);
    if (!op) {
    rv = -ENOMEM;
    goto out;
    }
    op.info.optype		= DLM_PLOCK_OP_GET;
    op.info.pid		= fl.c.flc_pid;
    op.info.ex		= lock_is_write(fl);
    op.info.fsid		= ls.ls_global_id;
    op.info.number		= number;
    op.info.start		= fl.fl_start;
    op.info.end		= fl.fl_end;
    op.info.owner = (__u64)(long) fl.c.flc_owner;
    send_op(op);
    wait_event(recv_wq, (op.done != 0));
    WARN_ON(!list_empty(&op.list));
// info.rv from userspace is 1 for conflict, 0 for no-conflict,
    -ENOENT if there are no locks on the file */
    rv = op.info.rv;
    fl.c.flc_type = F_UNLCK;
    if (rv == -ENOENT)
    rv = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: rv >) -> else {
    locks_init_lock(fl);
    fl.c.flc_type = (op.info.ex) ? F_WRLCK : F_RDLCK;
    fl.c.flc_flags = FL_POSIX;
    fl.c.flc_pid = op.info.pid;
    if (op.info.nodeid != dlm_our_nodeid())
    fl.c.flc_pid = -fl.c.flc_pid;
    fl.fl_start = op.info.start;
    fl.fl_end = op.info.end;
    rv = 0;
    }
    dlm_release_plock_op(op);
    out:
    dlm_put_lockspace(ls);
    return rv;
    }
    EXPORT_SYMBOL_GPL(dlm_posix_get);
// a read copies out one plock request from the send list
    static ssize_t dev_read(struct file *file, char __user *u, size_t count,
    loff_t *ppos)
    {
    struct dlm_plock_info info;
    struct plock_op *op = core::ptr::null_mut();
    if (count < sizeof(info))
    return -EINVAL;
    spin_lock(&ops_lock);
    if (!list_empty(&send_list)) {
    op = list_first_entry(&send_list, struct plock_op, list);
    if (op.info.flags & DLM_PLOCK_FL_CLOSE)
    list_del(&op.list);
    else
    list_move_tail(&op.list, &recv_list);
    memcpy(&info, &op.info, sizeof(info));
    }
    spin_unlock(&ops_lock);
    if (!op)
    return -EAGAIN;
    trace_dlm_plock_read(&info);
// there is no need to get a reply from userspace for unlocks
    that were generated by the vfs cleaning up for a close
    (the process did not make an unlock call). */
    if (op.info.flags & DLM_PLOCK_FL_CLOSE)
    dlm_release_plock_op(op);
    if (copy_to_user(u, &info, sizeof(info)))
    return -EFAULT;
    return sizeof(info);
    }
// a write copies in one plock result that should match a plock_op
    on the recv list */
    static ssize_t dev_write(struct file *file, const char __user *u, size_t count,
    loff_t *ppos)
    {
    struct plock_op *op = core::ptr::null_mut(), *iter;
    struct dlm_plock_info info;
    let mut do_callback: c_int = 0;
    if (count != sizeof(info))
    return -EINVAL;
    if (copy_from_user(&info, u, sizeof(info)))
    return -EFAULT;
    trace_dlm_plock_write(&info);
    if (check_version(&info))
    return -EINVAL;
//
// The results for waiting ops (SETLKW) can be returned in any
// order, so match all fields to find the op.  The results for
// non-waiting ops are returned in the order that they were sent
// to userspace, so match the result with the first non-waiting op.
//
    spin_lock(&ops_lock);
    if (info.wait) {
    op = plock_lookup_waiter(&info);
    } else {
    list_for_each_entry(iter, &recv_list, list) {
    if (!iter.info.wait &&
    iter.info.fsid == info.fsid) {
    op = iter;
    break;
    }
    }
    }
    if (op) {
// Sanity check that op and info match.
    if (info.wait)
    WARN_ON(op.info.optype != DLM_PLOCK_OP_LOCK);
    else
    WARN_ON(op.info.number != info.number ||
    op.info.owner != info.owner ||
    op.info.optype != info.optype);
    list_del_init(&op.list);
    memcpy(&op.info, &info, sizeof(info));
    if (op.data)
    do_callback = 1;
    else
    op.done = 1;
    }
    spin_unlock(&ops_lock);
    if (op) {
    if (do_callback)
    dlm_plock_callback(op);
    else
    wake_up(&recv_wq);
    } else
    pr_debug("%s: no op %x %llx", __func__,
    info.fsid, (unsigned long long)info.number);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn dev_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t dev_poll(struct file *file, poll_table *wait)
    {
    let mut mask: __poll_t = 0;
    poll_wait(file, &send_wq, wait);
    spin_lock(&ops_lock);
    if (!list_empty(&send_list))
    mask = EPOLLIN | EPOLLRDNORM;
    spin_unlock(&ops_lock);
    return mask;
    }
    static const struct file_operations dev_fops = {
    .read    = dev_read,
    .write   = dev_write,
    .poll    = dev_poll,
    .owner   = THIS_MODULE,
    .llseek  = noop_llseek,
    };
    static struct miscdevice plock_dev_misc = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = DLM_PLOCK_MISC_NAME,
    .fops = &dev_fops
    };
#[no_mangle]
pub unsafe extern "C" fn dlm_plock_init() -> c_int {
    int dlm_plock_init(void)
    {
    int rv;
    rv = misc_register(&plock_dev_misc);
    if (rv)
    log_print("dlm_plock_init: misc_register failed %d", rv);
    return rv;
    }
#[no_mangle]
pub unsafe extern "C" fn dlm_plock_exit() {
    void dlm_plock_exit(void)
    {
    misc_deregister(&plock_dev_misc);
    WARN_ON(!list_empty(&send_list));
    WARN_ON(!list_empty(&recv_list));
    }
