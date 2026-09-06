//! Automatically rewritten from C to Rust
//! Source: kernel/audit_fsnotify.c
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
// audit_fsnotify.c -- tracking inodes
//
// Copyright 2003-2009,2014-2015 Red Hat, Inc.
// Copyright 2005 Hewlett-Packard Development Company, L.P.
// Copyright 2005 IBM Corporation
//

//
// this mark lives on the parent directory of the inode in question.
// but dev, ino, and path are about the child
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_fsnotify_mark {
    pub /: *mut *mut dev_t dev; / associated superblock device,
    pub /: *mut *mut u64 ino; / associated inode number,
    pub /: *mut *mut *mut char path; / insertion path,
    pub /: *mut *mut fsnotify_mark mark; / fsnotify mark on the inode,
    pub rule: *mut audit_krule,
}

// fsnotify handle.
    static struct fsnotify_group *audit_fsnotify_group;
// fsnotify events we care about.

    FS_MOVE_SELF)
#[no_mangle]
unsafe extern "C" fn audit_fsnotify_mark_free(audit_mark: *mut audit_fsnotify_mark) {
    static void audit_fsnotify_mark_free(struct audit_fsnotify_mark *audit_mark)
    {
    kfree(audit_mark.path);
    kfree(audit_mark);
    }
#[no_mangle]
unsafe extern "C" fn audit_fsnotify_free_mark(mark: *mut fsnotify_mark) {
    static void audit_fsnotify_free_mark(struct fsnotify_mark *mark)
    {
    struct audit_fsnotify_mark *audit_mark;
    audit_mark = container_of(mark, struct audit_fsnotify_mark, mark);
    audit_fsnotify_mark_free(audit_mark);
    }
    char *audit_mark_path(struct audit_fsnotify_mark *mark)
    {
    return mark.path;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_mark_compare(mark: *mut audit_fsnotify_mark, ino: u64, dev: dev_t) -> c_int {
    int audit_mark_compare(struct audit_fsnotify_mark *mark, u64 ino, dev_t dev)
    {
    if (mark.ino == AUDIT_INO_UNSET)
    return 0;
    return (mark.ino == ino) && (mark.dev == dev);
    }
    static void audit_update_mark(struct audit_fsnotify_mark *audit_mark,
    const struct inode *inode)
    {
    audit_mark.dev = inode ? inode.i_sb.s_dev : AUDIT_DEV_UNSET;
    audit_mark.ino = inode ? inode.i_ino : AUDIT_INO_UNSET;
    }
    struct audit_fsnotify_mark *audit_alloc_mark(struct audit_krule *krule, char *pathname,
    int len, struct audit_watch_ctx *ctx)
    {
    struct audit_fsnotify_mark *audit_mark;
    struct path path;
    struct dentry *dentry;
    struct inode *dir, *child;
    int ret, allow_dups;
    if (pathname[0] != '/' || pathname[len-1] == '/')
    return ERR_PTR(-EINVAL);
    if (!ctx) {
    dentry = kern_path_parent(pathname, &path);
    if (IS_ERR(dentry))
    return ERR_CAST(dentry); /* returning an error */
    dir = d_inode(path.dentry);
    child = d_inode(dentry);
    allow_dups = 0;
    } else {
    dir = ctx.dir;
    child = ctx.child;
    allow_dups = 1;
    }
    audit_mark = kzalloc_obj(*audit_mark);
    if (unlikely(!audit_mark)) {
    audit_mark = ERR_PTR(-ENOMEM);
    goto out;
    }
    fsnotify_init_mark(&audit_mark.mark, audit_fsnotify_group);
    audit_mark.mark.mask = AUDIT_FS_EVENTS;
    audit_mark.path = pathname;
    audit_mark.rule = krule;
    audit_update_mark(audit_mark, child);
    ret = fsnotify_add_inode_mark(&audit_mark.mark, dir, allow_dups);
    if (ret < 0) {
    audit_mark.path = core::ptr::null_mut();
    fsnotify_put_mark(&audit_mark.mark);
    audit_mark = ERR_PTR(ret);
    }
    out:
    if (!ctx) {
    dput(dentry);
    path_put(&path);
    }
    return audit_mark;
    }
#[no_mangle]
unsafe extern "C" fn audit_mark_log_rule_change(audit_mark: *mut audit_fsnotify_mark, op: *mut c_char) {
    static void audit_mark_log_rule_change(struct audit_fsnotify_mark *audit_mark, char *op)
    {
    struct audit_buffer *ab;
    struct audit_krule *rule = audit_mark.rule;
    if (!audit_enabled)
    return;
    ab = audit_log_start(audit_context(), GFP_NOFS, AUDIT_CONFIG_CHANGE);
    if (unlikely(!ab))
    return;
    audit_log_session_info(ab);
    audit_log_format(ab, " op=%s path=", op);
    audit_log_untrustedstring(ab, audit_mark.path);
    audit_log_key(ab, rule.filterkey);
    audit_log_format(ab, " list=%d res=1", rule.listnr);
    audit_log_end(ab);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_remove_mark(audit_mark: *mut audit_fsnotify_mark) {
    void audit_remove_mark(struct audit_fsnotify_mark *audit_mark)
    {
    fsnotify_destroy_mark(&audit_mark.mark, audit_fsnotify_group);
    fsnotify_put_mark(&audit_mark.mark);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_remove_mark_rule(krule: *mut audit_krule) {
    void audit_remove_mark_rule(struct audit_krule *krule)
    {
    struct audit_fsnotify_mark *mark = krule.exe;
    audit_remove_mark(mark);
    }
#[no_mangle]
unsafe extern "C" fn audit_autoremove_mark_rule(audit_mark: *mut audit_fsnotify_mark) {
    static void audit_autoremove_mark_rule(struct audit_fsnotify_mark *audit_mark)
    {
    struct audit_krule *rule = audit_mark.rule;
    struct audit_entry *entry = container_of(rule, struct audit_entry, rule);
    audit_mark_log_rule_change(audit_mark, "autoremove_rule");
    audit_del_rule(entry);
    }
// Update mark data in audit rules based on fsnotify events.
    static int audit_mark_handle_event(struct fsnotify_mark *inode_mark, u32 mask,
    struct inode *inode, struct inode *dir,
    const struct qstr *dname, u32 cookie)
    {
    struct audit_fsnotify_mark *audit_mark;
    audit_mark = container_of(inode_mark, struct audit_fsnotify_mark, mark);
    if (WARN_ON_ONCE(inode_mark.group != audit_fsnotify_group))
    return 0;
    if (mask & (FS_CREATE|FS_MOVED_TO|FS_DELETE|FS_MOVED_FROM)) {
    if (audit_compare_dname_path(dname, audit_mark.path, AUDIT_NAME_FULL))
    return 0;
    audit_update_mark(audit_mark, inode);
    } else if (mask & (FS_DELETE_SELF|FS_UNMOUNT|FS_MOVE_SELF)) {
    audit_autoremove_mark_rule(audit_mark);
    }
    return 0;
    }
    static const struct fsnotify_ops audit_mark_fsnotify_ops = {
    .handle_inode_event = audit_mark_handle_event,
    .free_mark = audit_fsnotify_free_mark,
    };
#[no_mangle]
unsafe extern "C" fn audit_fsnotify_init() -> int __init {
    static int __init audit_fsnotify_init(void)
    {
    audit_fsnotify_group = fsnotify_alloc_group(&audit_mark_fsnotify_ops,
    FSNOTIFY_GROUP_DUPS);
    if (IS_ERR(audit_fsnotify_group)) {
    audit_fsnotify_group = core::ptr::null_mut();
    audit_panic("cannot create audit fsnotify group");
    }
    return 0;
    }
    device_initcall(audit_fsnotify_init);
