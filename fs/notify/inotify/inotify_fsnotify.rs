//! Automatically rewritten from C to Rust
//! Source: fs/notify/inotify/inotify_fsnotify.c
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
// fs/inotify_user.c - inotify support for userspace
//
// Authors:
// John McCutchan	<ttb@tentacle.dhs.org>
// Robert Love	<rml@novell.com>
//
// Copyright (C) 2005 John McCutchan
// Copyright 2006 Hewlett-Packard Development Company, L.P.
//
// Copyright (C) 2009 Eric Paris <Red Hat Inc>
// inotify was largely rewritten to make use of the fsnotify infrastructure
//

//
// Check if 2 events contain the same information.
//
    static bool event_compare(struct fsnotify_event *old_fsn,
    struct fsnotify_event *new_fsn)
    {
    struct inotify_event_info *old, *new;
    old = INOTIFY_E(old_fsn);
    new = INOTIFY_E(new_fsn);
    if (old.mask & FS_IN_IGNORED)
    return false;
    if ((old.mask == new.mask) &&
    (old.wd == new.wd) &&
    (old.name_len == new.name_len) &&
    (!old.name_len || !strcmp(old.name, new.name)))
    return true;
    return false;
    }
    static int inotify_merge(struct fsnotify_group *group,
    struct fsnotify_event *event)
    {
    struct list_head *list = &group.notification_list;
    struct fsnotify_event *last_event;
    last_event = list_entry(list.prev, struct fsnotify_event, list);
    return event_compare(last_event, event);
    }
    int inotify_handle_inode_event(struct fsnotify_mark *inode_mark, u32 mask,
    struct inode *inode, struct inode *dir,
    const struct qstr *name, u32 cookie)
    {
    struct inotify_inode_mark *i_mark;
    struct inotify_event_info *event;
    struct fsnotify_event *fsn_event;
    struct fsnotify_group *group = inode_mark.group;
    int ret;
    let mut len: c_int = 0, wd;
    let mut alloc_len: c_int = sizeof(struct inotify_event_info);
    struct mem_cgroup *old_memcg;
    if (name) {
    len = name.len;
    alloc_len += len + 1;
    }
    pr_debug("%s: group=%p mark=%p mask=%x\n", __func__, group, inode_mark,
    mask);
    i_mark = container_of(inode_mark, struct inotify_inode_mark,
    fsn_mark);
//
// We can be racing with mark being detached. Don't report event with
// invalid wd.
//
    wd = READ_ONCE(i_mark.wd);
    if (wd == -1)
    return 0;
//
// Whoever is interested in the event, pays for the allocation. Do not
// trigger OOM killer in the target monitoring memcg as it may have
// security repercussion.
//
    old_memcg = set_active_memcg(group.memcg);
    event = kmalloc(alloc_len, GFP_KERNEL_ACCOUNT | __GFP_RETRY_MAYFAIL);
    set_active_memcg(old_memcg);
    if (unlikely(!event)) {
//
// Treat lost event due to ENOMEM the same way as queue
// overflow to let userspace know event was lost.
//
    fsnotify_queue_overflow(group);
    return -ENOMEM;
    }
//
// We now report FS_ISDIR flag with MOVE_SELF and DELETE_SELF events
// for fanotify. inotify never reported IN_ISDIR with those events.
// It looks like an oversight, but to avoid the risk of breaking
// existing inotify programs, mask the flag out from those events.
//
    if (mask & (IN_MOVE_SELF | IN_DELETE_SELF))
    mask &= ~IN_ISDIR;
    fsn_event = &event.fse;
    fsnotify_init_event(fsn_event);
    event.mask = mask;
    event.wd = wd;
    event.sync_cookie = cookie;
    event.name_len = len;
    if (len)
    strscpy(event.name, name.name, event.name_len + 1);
    ret = fsnotify_add_event(group, fsn_event, inotify_merge);
    if (ret) {
// Our event wasn't used in the end. Free it.
    fsnotify_destroy_event(group, fsn_event);
    }
    if (inode_mark.flags & FSNOTIFY_MARK_FLAG_IN_ONESHOT)
    fsnotify_destroy_mark(inode_mark, group);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn inotify_freeing_mark(fsn_mark: *mut fsnotify_mark, group: *mut fsnotify_group) {
    static void inotify_freeing_mark(struct fsnotify_mark *fsn_mark, struct fsnotify_group *group)
    {
    inotify_ignored_and_remove_idr(fsn_mark, group);
    }
//
// This is NEVER supposed to be called.  Inotify marks should either have been
// removed from the idr when the watch was removed or in the
// fsnotify_destroy_mark_by_group() call when the inotify instance was being
// torn down.  This is only called if the idr is about to be freed but there
// are still marks in it.
//
#[no_mangle]
unsafe extern "C" fn idr_callback(id: c_int, p: *mut c_void, data: *mut c_void) -> c_int {
    static int idr_callback(int id, void *p, void *data)
    {
    struct fsnotify_mark *fsn_mark;
    struct inotify_inode_mark *i_mark;
    let mut warned: static bool = false;
    if (warned)
    return 0;
    warned = true;
    fsn_mark = p;
    i_mark = container_of(fsn_mark, struct inotify_inode_mark, fsn_mark);
    WARN(1, "inotify closing but id=%d for fsn_mark=%p in group=%p still in "
    "idr.  Probably leaking memory\n", id, p, data);
//
// I'm taking the liberty of assuming that the mark in question is a
// valid address and I'm dereferencing it.  This might help to figure
// out why we got here and the panic is no worse than the original
// BUG() that was here.
//
    if (fsn_mark)
    printk(KERN_WARNING "fsn_mark.group=%p wd=%d\n",
    fsn_mark.group, i_mark.wd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn inotify_free_group_priv(group: *mut fsnotify_group) {
    static void inotify_free_group_priv(struct fsnotify_group *group)
    {
// ideally the idr is empty and we won't hit the BUG in the callback
    idr_for_each(&group.inotify_data.idr, idr_callback, group);
    idr_destroy(&group.inotify_data.idr);
    if (group.inotify_data.ucounts)
    dec_inotify_instances(group.inotify_data.ucounts);
    }
    static void inotify_free_event(struct fsnotify_group *group,
    struct fsnotify_event *fsn_event)
    {
    kfree(INOTIFY_E(fsn_event));
    }
// ding dong the mark is dead
#[no_mangle]
unsafe extern "C" fn inotify_free_mark(fsn_mark: *mut fsnotify_mark) {
    static void inotify_free_mark(struct fsnotify_mark *fsn_mark)
    {
    struct inotify_inode_mark *i_mark;
    i_mark = container_of(fsn_mark, struct inotify_inode_mark, fsn_mark);
    kmem_cache_free(inotify_inode_mark_cachep, i_mark);
    }
    const struct fsnotify_ops inotify_fsnotify_ops = {
    .handle_inode_event = inotify_handle_inode_event,
    .free_group_priv = inotify_free_group_priv,
    .free_event = inotify_free_event,
    .freeing_mark = inotify_freeing_mark,
    .free_mark = inotify_free_mark,
    };
