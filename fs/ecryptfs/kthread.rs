//! Automatically rewritten from C to Rust
//! Source: fs/ecryptfs/kthread.c
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
// eCryptfs: Linux filesystem encryption layer
//
// Copyright (C) 2008 International Business Machines Corp.
// Author(s): Michael A. Halcrow <mahalcro@us.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_open_req {
    pub lower_file: *mut file,
    pub path: path,
    pub done: completion,
    pub kthread_ctl_list: list_head,
}

    static struct ecryptfs_kthread_ctl {
pub const ECRYPTFS_KTHREAD_ZOMBIE: c_uint = 0x00000001;
    u32 flags;
    struct mutex mux;
    struct list_head req_list;
    wait_queue_head_t wait;
    } ecryptfs_kthread_ctl;
    static struct task_struct *ecryptfs_kthread;
//
// ecryptfs_threadfn
// @ignored: ignored
//
// The eCryptfs kernel thread that has the responsibility of getting
// the lower file with RW permissions.
//
// Returns zero on success; non-zero otherwise
//
#[no_mangle]
unsafe extern "C" fn ecryptfs_threadfn(ignored: *mut c_void) -> c_int {
    static int ecryptfs_threadfn(void *ignored)
    {
    set_freezable();
    while (1)  {
    struct ecryptfs_open_req *req;
    wait_event_freezable(
    ecryptfs_kthread_ctl.wait,
    (!list_empty(&ecryptfs_kthread_ctl.req_list)
    || kthread_should_stop()));
    mutex_lock(&ecryptfs_kthread_ctl.mux);
    if (ecryptfs_kthread_ctl.flags & ECRYPTFS_KTHREAD_ZOMBIE) {
    mutex_unlock(&ecryptfs_kthread_ctl.mux);
    goto out;
    }
    while (!list_empty(&ecryptfs_kthread_ctl.req_list)) {
    req = list_first_entry(&ecryptfs_kthread_ctl.req_list,
    struct ecryptfs_open_req,
    kthread_ctl_list);
    list_del(&req.kthread_ctl_list);
// req->lower_file = dentry_open(&req->path,
    (O_RDWR | O_LARGEFILE), current_cred());
    complete(&req.done);
    }
    mutex_unlock(&ecryptfs_kthread_ctl.mux);
    }
    out:
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ecryptfs_init_kthread() -> int __init {
    int __init ecryptfs_init_kthread(void)
    {
    let mut rc: c_int = 0;
    mutex_init(&ecryptfs_kthread_ctl.mux);
    init_waitqueue_head(&ecryptfs_kthread_ctl.wait);
    INIT_LIST_HEAD(&ecryptfs_kthread_ctl.req_list);
    ecryptfs_kthread = kthread_run(&ecryptfs_threadfn, core::ptr::null_mut(),
    "ecryptfs-kthread");
    if (IS_ERR(ecryptfs_kthread)) {
    rc = PTR_ERR(ecryptfs_kthread);
    printk(KERN_ERR "%s: Failed to create kernel thread; rc = [%d]"
    "\n", __func__, rc);
    }
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn ecryptfs_destroy_kthread() {
    void ecryptfs_destroy_kthread(void)
    {
    struct ecryptfs_open_req *req, *tmp;
    mutex_lock(&ecryptfs_kthread_ctl.mux);
    ecryptfs_kthread_ctl.flags |= ECRYPTFS_KTHREAD_ZOMBIE;
    list_for_each_entry_safe(req, tmp, &ecryptfs_kthread_ctl.req_list,
    kthread_ctl_list) {
    list_del(&req.kthread_ctl_list);
// req->lower_file = ERR_PTR(-EIO);
    complete(&req.done);
    }
    mutex_unlock(&ecryptfs_kthread_ctl.mux);
    kthread_stop(ecryptfs_kthread);
    wake_up(&ecryptfs_kthread_ctl.wait);
    }
//
// ecryptfs_privileged_open
// @lower_file: Result of dentry_open by root on lower dentry
// @lower_dentry: Lower dentry for file to open
// @lower_mnt: Lower vfsmount for file to open
// @cred: credential to use for this call
//
// This function gets a r/w file opened against the lower dentry.
//
// Returns zero on success; non-zero otherwise
//
    int ecryptfs_privileged_open(struct file **lower_file,
    struct dentry *lower_dentry,
    struct vfsmount *lower_mnt,
    const struct cred *cred)
    {
    struct ecryptfs_open_req req;
    let mut flags: c_int = O_LARGEFILE;
    let mut rc: c_int = 0;
    init_completion(&req.done);
    req.lower_file = lower_file;
    req.path.dentry = lower_dentry;
    req.path.mnt = lower_mnt;
// Corresponding dput() and mntput() are done when the
// lower file is fput() when all eCryptfs files for the inode are
// released.
    flags |= IS_RDONLY(d_inode(lower_dentry)) ? O_RDONLY : O_RDWR;
    (*lower_file) = dentry_open(&req.path, flags, cred);
    if (!IS_ERR(*lower_file))
    goto out;
    if ((flags & O_ACCMODE) == O_RDONLY) {
    rc = PTR_ERR((*lower_file));
    goto out;
    }
    mutex_lock(&ecryptfs_kthread_ctl.mux);
    if (ecryptfs_kthread_ctl.flags & ECRYPTFS_KTHREAD_ZOMBIE) {
    rc = -EIO;
    mutex_unlock(&ecryptfs_kthread_ctl.mux);
    printk(KERN_ERR "%s: We are in the middle of shutting down; "
    "aborting privileged request to open lower file\n",
    __func__);
    goto out;
    }
    list_add_tail(&req.kthread_ctl_list, &ecryptfs_kthread_ctl.req_list);
    mutex_unlock(&ecryptfs_kthread_ctl.mux);
    wake_up(&ecryptfs_kthread_ctl.wait);
    wait_for_completion(&req.done);
    if (IS_ERR(*lower_file))
    rc = PTR_ERR(*lower_file);
    out:
    return rc;
    }
