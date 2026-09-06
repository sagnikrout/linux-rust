//! Automatically rewritten from C to Rust
//! Source: fs/notify/dnotify/dnotify.c
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
// Directory notifications for Linux.
//
// Copyright (C) 2000,2001,2002 Stephen Rothwell
//
// Copyright (C) 2009 Eric Paris <Red Hat Inc>
// dnotify was largly rewritten to use the new fsnotify infrastructure
//

    let mut __read_mostly: static int dir_notify_enable = 1;

    static const struct ctl_table dnotify_sysctls[] = {
    {
    .procname	= "dir-notify-enable",
    .data		= &dir_notify_enable,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },
    };
#[no_mangle]
unsafe extern "C" fn dnotify_sysctl_init() -> void __init {
    static void __init dnotify_sysctl_init(void)
    {
    register_sysctl_init("fs", dnotify_sysctls);
    }

    static struct kmem_cache *dnotify_struct_cache __ro_after_init;
    static struct kmem_cache *dnotify_mark_cache __ro_after_init;
    static struct fsnotify_group *dnotify_group __ro_after_init;
//
// dnotify will attach one of these to each inode (i_fsnotify_marks) which
// is being watched by dnotify.  If multiple userspace applications are watching
// the same directory with dnotify their information is chained in dn
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dnotify_mark {
    pub fsn_mark: fsnotify_mark,
    pub dn: *mut dnotify_struct,
}

//
// When a process starts or stops watching an inode the set of events which
// dnotify cares about for that inode may change.  This function runs the
// list of everything receiving dnotify events about this directory and calculates
// the set of all those events.  After it updates what dnotify is interested in
// it calls the fsnotify function so it can update the set of all events relevant
// to this inode.
//
#[no_mangle]
unsafe extern "C" fn dnotify_recalc_inode_mask(fsn_mark: *mut fsnotify_mark) {
    static void dnotify_recalc_inode_mask(struct fsnotify_mark *fsn_mark)
    {
    let mut new_mask: __u32 = 0;
    struct dnotify_struct *dn;
    struct dnotify_mark *dn_mark  = container_of(fsn_mark,
    struct dnotify_mark,
    fsn_mark);
    assert_spin_locked(&fsn_mark.lock);
    for (dn = dn_mark.dn; dn != core::ptr::null_mut(); dn = dn.dn_next)
    new_mask |= (dn.dn_mask & ~FS_DN_MULTISHOT);
    if (fsn_mark.mask == new_mask)
    return;
    fsn_mark.mask = new_mask;
    fsnotify_recalc_mask(fsn_mark.connector);
    }
//
// Mains fsnotify call where events are delivered to dnotify.
// Find the dnotify mark on the relevant inode, run the list of dnotify structs
// on that mark and determine which of them has expressed interest in receiving
// events of this type.  When found send the correct process and signal and
// destroy the dnotify struct if it was not registered to receive multiple
// events.
//
    static int dnotify_handle_event(struct fsnotify_mark *inode_mark, u32 mask,
    struct inode *inode, struct inode *dir,
    const struct qstr *name, u32 cookie)
    {
    struct dnotify_mark *dn_mark;
    struct dnotify_struct *dn;
    struct dnotify_struct **prev;
    struct fown_struct *fown;
    let mut test_mask: __u32 = mask & ~FS_EVENT_ON_CHILD;
// not a dir, dnotify doesn't care
    if (!dir && !(mask & FS_ISDIR))
    return 0;
    dn_mark = container_of(inode_mark, struct dnotify_mark, fsn_mark);
    spin_lock(&inode_mark.lock);
    prev = &dn_mark.dn;
    while ((dn = *prev) != core::ptr::null_mut()) {
    if ((dn.dn_mask & test_mask) == 0) {
    prev = &dn.dn_next;
    continue;
    }
    fown = file_f_owner(dn.dn_filp);
    send_sigio(fown, dn.dn_fd, POLL_MSG);
    if (dn.dn_mask & FS_DN_MULTISHOT)
    prev = &dn.dn_next;
    else {
// prev = dn->dn_next;
    kmem_cache_free(dnotify_struct_cache, dn);
    dnotify_recalc_inode_mask(inode_mark);
    }
    }
    spin_unlock(&inode_mark.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dnotify_free_mark(fsn_mark: *mut fsnotify_mark) {
    static void dnotify_free_mark(struct fsnotify_mark *fsn_mark)
    {
    struct dnotify_mark *dn_mark = container_of(fsn_mark,
    struct dnotify_mark,
    fsn_mark);
    BUG_ON(dn_mark.dn);
    kmem_cache_free(dnotify_mark_cache, dn_mark);
    }
    static const struct fsnotify_ops dnotify_fsnotify_ops = {
    .handle_inode_event = dnotify_handle_event,
    .free_mark = dnotify_free_mark,
    };
//
// Called every time a file is closed.  Looks first for a dnotify mark on the
// inode.  If one is found run all of the ->dn structures attached to that
// mark for one relevant to this process closing the file and remove that
// dnotify_struct.  If that was the last dnotify_struct also remove the
// fsnotify_mark.
//
#[no_mangle]
pub unsafe extern "C" fn dnotify_flush(filp: *mut file, id: fl_owner_t) {
    void dnotify_flush(struct file *filp, fl_owner_t id)
    {
    struct fsnotify_mark *fsn_mark;
    struct dnotify_mark *dn_mark;
    struct dnotify_struct *dn;
    struct dnotify_struct **prev;
    struct inode *inode;
    let mut free: bool = false;
    inode = file_inode(filp);
    if (!S_ISDIR(inode.i_mode))
    return;
    fsn_mark = fsnotify_find_inode_mark(inode, dnotify_group);
    if (!fsn_mark)
    return;
    dn_mark = container_of(fsn_mark, struct dnotify_mark, fsn_mark);
    fsnotify_group_lock(dnotify_group);
    spin_lock(&fsn_mark.lock);
    prev = &dn_mark.dn;
    while ((dn = *prev) != core::ptr::null_mut()) {
    if ((dn.dn_owner == id) && (dn.dn_filp == filp)) {
// prev = dn->dn_next;
    kmem_cache_free(dnotify_struct_cache, dn);
    dnotify_recalc_inode_mask(fsn_mark);
    break;
    }
    prev = &dn.dn_next;
    }
    spin_unlock(&fsn_mark.lock);
// nothing else could have found us thanks to the dnotify_groups
    mark_mutex */
    if (dn_mark.dn == core::ptr::null_mut()) {
    fsnotify_detach_mark(fsn_mark);
    free = true;
    }
    fsnotify_group_unlock(dnotify_group);
    if (free)
    fsnotify_free_mark(fsn_mark);
    fsnotify_put_mark(fsn_mark);
    }
// this conversion is done only at watch creation
#[no_mangle]
unsafe extern "C" fn convert_arg(arg: c_uint) -> __u32 {
    static __u32 convert_arg(unsigned int arg)
    {
    let mut new_mask: __u32 = FS_EVENT_ON_CHILD;
    if (arg & DN_MULTISHOT)
    new_mask |= FS_DN_MULTISHOT;
    if (arg & DN_DELETE)
    new_mask |= (FS_DELETE | FS_MOVED_FROM);
    if (arg & DN_MODIFY)
    new_mask |= FS_MODIFY;
    if (arg & DN_ACCESS)
    new_mask |= FS_ACCESS;
    if (arg & DN_ATTRIB)
    new_mask |= FS_ATTRIB;
    if (arg & DN_RENAME)
    new_mask |= FS_RENAME;
    if (arg & DN_CREATE)
    new_mask |= (FS_CREATE | FS_MOVED_TO);
    return new_mask;
    }
//
// If multiple processes watch the same inode with dnotify there is only one
// dnotify mark in inode->i_fsnotify_marks but we chain a dnotify_struct
// onto that mark.  This function either attaches the new dnotify_struct onto
// that list, or it |= the mask onto an existing dnofiy_struct.
//
    static int attach_dn(struct dnotify_struct *dn, struct dnotify_mark *dn_mark,
    fl_owner_t id, int fd, struct file *filp, __u32 mask)
    {
    struct dnotify_struct *odn;
    odn = dn_mark.dn;
    while (odn != core::ptr::null_mut()) {
// adding more events to existing dnofiy_struct?
    if ((odn.dn_owner == id) && (odn.dn_filp == filp)) {
    odn.dn_fd = fd;
    odn.dn_mask |= mask;
    return -EEXIST;
    }
    odn = odn.dn_next;
    }
    dn.dn_mask = mask;
    dn.dn_fd = fd;
    dn.dn_filp = filp;
    dn.dn_owner = id;
    dn.dn_next = dn_mark.dn;
    dn_mark.dn = dn;
    return 0;
    }
//
// When a process calls fcntl to attach a dnotify watch to a directory it ends
// up here.  Allocate both a mark for fsnotify to add and a dnotify_struct to be
// attached to the fsnotify_mark.
//
#[no_mangle]
pub unsafe extern "C" fn fcntl_dirnotify(fd: c_int, filp: *mut file, arg: c_uint) -> c_int {
    int fcntl_dirnotify(int fd, struct file *filp, unsigned int arg)
    {
    struct dnotify_mark *new_dn_mark, *dn_mark;
    struct fsnotify_mark *new_fsn_mark, *fsn_mark;
    struct dnotify_struct *dn;
    struct inode *inode;
    let mut id: fl_owner_t = current.files;
    struct file *f = core::ptr::null_mut();
    let mut destroy: c_int = 0, error = 0;
    __u32 mask;
// we use these to tell if we need to kfree
    new_fsn_mark = core::ptr::null_mut();
    dn = core::ptr::null_mut();
    if (!dir_notify_enable) {
    error = -EINVAL;
    goto out_err;
    }
// a 0 mask means we are explicitly removing the watch
    if ((arg & ~DN_MULTISHOT) == 0) {
    dnotify_flush(filp, id);
    error = 0;
    goto out_err;
    }
// dnotify only works on directories
    inode = file_inode(filp);
    if (!S_ISDIR(inode.i_mode)) {
    error = -ENOTDIR;
    goto out_err;
    }
//
// convert the userspace DN_* "arg" to the internal FS_
// defined in fsnotify
//
    mask = convert_arg(arg);
    error = security_path_notify(&filp.f_path, mask,
    FSNOTIFY_OBJ_TYPE_INODE);
    if (error)
    goto out_err;
// expect most fcntl to add new rather than augment old
    dn = kmem_cache_alloc(dnotify_struct_cache, GFP_KERNEL);
    if (!dn) {
    error = -ENOMEM;
    goto out_err;
    }
    error = file_f_owner_allocate(filp);
    if (error)
    goto out_err;
// new fsnotify mark, we expect most fcntl calls to add a new mark
    new_dn_mark = kmem_cache_alloc(dnotify_mark_cache, GFP_KERNEL);
    if (!new_dn_mark) {
    error = -ENOMEM;
    goto out_err;
    }
// set up the new_fsn_mark and new_dn_mark
    new_fsn_mark = &new_dn_mark.fsn_mark;
    fsnotify_init_mark(new_fsn_mark, dnotify_group);
    new_fsn_mark.mask = mask;
    new_dn_mark.dn = core::ptr::null_mut();
// this is needed to prevent the fcntl/close race described below
    fsnotify_group_lock(dnotify_group);
// add the new_fsn_mark or find an old one.
    fsn_mark = fsnotify_find_inode_mark(inode, dnotify_group);
    if (fsn_mark) {
    dn_mark = container_of(fsn_mark, struct dnotify_mark, fsn_mark);
    spin_lock(&fsn_mark.lock);
    } else {
    error = fsnotify_add_inode_mark_locked(new_fsn_mark, inode, 0);
    if (error) {
    fsnotify_group_unlock(dnotify_group);
    goto out_err;
    }
    spin_lock(&new_fsn_mark.lock);
    fsn_mark = new_fsn_mark;
    dn_mark = new_dn_mark;
// we used new_fsn_mark, so don't free it
    new_fsn_mark = core::ptr::null_mut();
    }
    f = fget_raw(fd);
// if (f != filp) means that we lost a race and another task/thread
// actually closed the fd we are still playing with before we grabbed
// the dnotify_groups mark_mutex and fsn_mark->lock.  Since closing the
// fd is the only time we clean up the marks we need to get our mark
// off the list.
    if (f != filp) {
// if we added ourselves, shoot ourselves, it's possible that
// the flush actually did shoot this fsn_mark.  That's fine too
// since multiple calls to destroy_mark is perfectly safe, if
// we found a dn_mark already attached to the inode, just sod
// off silently as the flush at close time dealt with it.
//
    if (dn_mark == new_dn_mark)
    destroy = 1;
    error = 0;
    goto out;
    }
    __f_setown(filp, task_pid(current), PIDTYPE_TGID, 0);
    error = attach_dn(dn, dn_mark, id, fd, filp, mask);
// !error means that we attached the dn to the dn_mark, so don't free it
    if (!error)
    dn = core::ptr::null_mut();
// -EEXIST means that we didn't add this new dn and used an old one.
// that isn't an error (and the unused dn should be freed)
#[no_mangle]
pub unsafe extern "C" fn if(-EEXIST: error ==) -> else {
    else if (error == -EEXIST)
    error = 0;
    dnotify_recalc_inode_mask(fsn_mark);
    out:
    spin_unlock(&fsn_mark.lock);
    if (destroy)
    fsnotify_detach_mark(fsn_mark);
    fsnotify_group_unlock(dnotify_group);
    if (destroy)
    fsnotify_free_mark(fsn_mark);
    fsnotify_put_mark(fsn_mark);
    out_err:
    if (new_fsn_mark)
    fsnotify_put_mark(new_fsn_mark);
    if (dn)
    kmem_cache_free(dnotify_struct_cache, dn);
    if (f)
    fput(f);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn dnotify_init() -> int __init {
    static int __init dnotify_init(void)
    {
    dnotify_struct_cache = KMEM_CACHE(dnotify_struct,
    SLAB_PANIC|SLAB_ACCOUNT);
    dnotify_mark_cache = KMEM_CACHE(dnotify_mark, SLAB_PANIC|SLAB_ACCOUNT);
    dnotify_group = fsnotify_alloc_group(&dnotify_fsnotify_ops, 0);
    if (IS_ERR(dnotify_group))
    panic("unable to allocate fsnotify group for dnotify\n");
    dnotify_sysctl_init();
    return 0;
    }
    module_init(dnotify_init)
