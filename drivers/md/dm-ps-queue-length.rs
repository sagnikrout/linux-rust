//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-ps-queue-length.c
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
// Copyright (C) 2004-2005 IBM Corp.  All Rights Reserved.
// Copyright (C) 2006-2009 NEC Corporation.
//
// dm-queue-length.c
//
// Module Author: Stefan Bader, IBM
// Modified by: Kiyoshi Ueda, NEC
//
// This file is released under the GPL.
//
// queue-length path selector - choose a path with the least number of
// in-flight I/Os.
//

pub const QL_MIN_IO: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct selector {
    pub valid_paths: list_head,
    pub failed_paths: list_head,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct path_info {
    pub list: list_head,
    pub path: *mut dm_path,
    pub repeat_count: c_uint,
    pub /: *mut *mut atomic_t qlen; / the number of in-flight I/Os,
}

    static struct selector *alloc_selector(void)
    {
    struct selector *s = kmalloc_obj(*s);
    if (s) {
    INIT_LIST_HEAD(&s.valid_paths);
    INIT_LIST_HEAD(&s.failed_paths);
    spin_lock_init(&s.lock);
    }
    return s;
    }
#[no_mangle]
unsafe extern "C" fn ql_create(ps: *mut path_selector, argc: c_uint, argv: *mut c_char) -> c_int {
    static int ql_create(struct path_selector *ps, unsigned int argc, char **argv)
    {
    struct selector *s = alloc_selector();
    if (!s)
    return -ENOMEM;
    ps.context = s;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ql_free_paths(paths: *mut list_head) {
    static void ql_free_paths(struct list_head *paths)
    {
    struct path_info *pi, *next;
    list_for_each_entry_safe(pi, next, paths, list) {
    list_del(&pi.list);
    kfree(pi);
    }
    }
#[no_mangle]
unsafe extern "C" fn ql_destroy(ps: *mut path_selector) {
    static void ql_destroy(struct path_selector *ps)
    {
    struct selector *s = ps.context;
    ql_free_paths(&s.valid_paths);
    ql_free_paths(&s.failed_paths);
    kfree(s);
    ps.context = core::ptr::null_mut();
    }
    static int ql_status(struct path_selector *ps, struct dm_path *path,
    status_type_t type, char *result, unsigned int maxlen)
    {
    let mut sz: c_uint = 0;
    struct path_info *pi;
// When called with NULL path, return selector status/args.
    if (!path)
    DMEMIT("0 ");
    else {
    pi = path.pscontext;
    switch (type) {
    case STATUSTYPE_INFO:
    DMEMIT("%d ", atomic_read(&pi.qlen));
    break;
    case STATUSTYPE_TABLE:
    DMEMIT("%u ", pi.repeat_count);
    break;
    case STATUSTYPE_IMA:
// result = '\0';
    break;
    }
    }
    return sz;
    }
    static int ql_add_path(struct path_selector *ps, struct dm_path *path,
    int argc, char **argv, char **error)
    {
    struct selector *s = ps.context;
    struct path_info *pi;
    let mut repeat_count: c_uint = QL_MIN_IO;
    char dummy;
    unsigned long flags;
//
// Arguments: [<repeat_count>]
// <repeat_count>: The number of I/Os before switching path.
// If not given, default (QL_MIN_IO) is used.
//
    if (argc > 1) {
// error = "queue-length ps: incorrect number of arguments";
    return -EINVAL;
    }
    if ((argc == 1) && (sscanf(argv[0], "%u%c", &repeat_count, &dummy) != 1)) {
// error = "queue-length ps: invalid repeat count";
    return -EINVAL;
    }
    if (repeat_count > 1) {
    DMWARN_LIMIT("repeat_count > 1 is deprecated, using 1 instead");
    repeat_count = 1;
    }
// Allocate the path information structure
    pi = kmalloc_obj(*pi);
    if (!pi) {
// error = "queue-length ps: Error allocating path information";
    return -ENOMEM;
    }
    pi.path = path;
    pi.repeat_count = repeat_count;
    atomic_set(&pi.qlen, 0);
    path.pscontext = pi;
    spin_lock_irqsave(&s.lock, flags);
    list_add_tail(&pi.list, &s.valid_paths);
    spin_unlock_irqrestore(&s.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ql_fail_path(ps: *mut path_selector, path: *mut dm_path) {
    static void ql_fail_path(struct path_selector *ps, struct dm_path *path)
    {
    struct selector *s = ps.context;
    struct path_info *pi = path.pscontext;
    unsigned long flags;
    spin_lock_irqsave(&s.lock, flags);
    list_move(&pi.list, &s.failed_paths);
    spin_unlock_irqrestore(&s.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ql_reinstate_path(ps: *mut path_selector, path: *mut dm_path) -> c_int {
    static int ql_reinstate_path(struct path_selector *ps, struct dm_path *path)
    {
    struct selector *s = ps.context;
    struct path_info *pi = path.pscontext;
    unsigned long flags;
    spin_lock_irqsave(&s.lock, flags);
    list_move_tail(&pi.list, &s.valid_paths);
    spin_unlock_irqrestore(&s.lock, flags);
    return 0;
    }
//
// Select a path having the minimum number of in-flight I/Os
//
    static struct dm_path *ql_select_path(struct path_selector *ps, size_t nr_bytes)
    {
    struct selector *s = ps.context;
    struct path_info *pi = core::ptr::null_mut(), *best = core::ptr::null_mut();
    struct dm_path *ret = core::ptr::null_mut();
    unsigned long flags;
    spin_lock_irqsave(&s.lock, flags);
    if (list_empty(&s.valid_paths))
    goto out;
    list_for_each_entry(pi, &s.valid_paths, list) {
    if (!best ||
    (atomic_read(&pi.qlen) < atomic_read(&best.qlen)))
    best = pi;
    if (!atomic_read(&best.qlen))
    break;
    }
    if (!best)
    goto out;
// Move most recently used to least preferred to evenly balance.
    list_move_tail(&best.list, &s.valid_paths);
    ret = best.path;
    out:
    spin_unlock_irqrestore(&s.lock, flags);
    return ret;
    }
    static int ql_start_io(struct path_selector *ps, struct dm_path *path,
    size_t nr_bytes)
    {
    struct path_info *pi = path.pscontext;
    atomic_inc(&pi.qlen);
    return 0;
    }
    static int ql_end_io(struct path_selector *ps, struct dm_path *path,
    size_t nr_bytes, u64 start_time)
    {
    struct path_info *pi = path.pscontext;
    atomic_dec(&pi.qlen);
    return 0;
    }
    static struct path_selector_type ql_ps = {
    .name		= "queue-length",
    .module		= THIS_MODULE,
    .table_args	= 1,
    .info_args	= 1,
    .create		= ql_create,
    .destroy	= ql_destroy,
    .status		= ql_status,
    .add_path	= ql_add_path,
    .fail_path	= ql_fail_path,
    .reinstate_path	= ql_reinstate_path,
    .select_path	= ql_select_path,
    .start_io	= ql_start_io,
    .end_io		= ql_end_io,
    };
#[no_mangle]
unsafe extern "C" fn dm_ql_init() -> int __init {
    static int __init dm_ql_init(void)
    {
    let mut r: c_int = dm_register_path_selector(&ql_ps);
    if (r < 0) {
    DMERR("register failed %d", r);
    return r;
    }
    DMINFO("version " QL_VERSION " loaded");
    return r;
    }
#[no_mangle]
unsafe extern "C" fn dm_ql_exit() -> void __exit {
    static void __exit dm_ql_exit(void)
    {
    dm_unregister_path_selector(&ql_ps);
    }
    module_init(dm_ql_init);
    module_exit(dm_ql_exit);
    MODULE_AUTHOR("Stefan Bader <Stefan.Bader at de.ibm.com>");
    MODULE_DESCRIPTION(
    "(C) Copyright IBM Corp. 2004,2005   All Rights Reserved.\n"
    DM_NAME " path selector to balance the number of in-flight I/Os"
    );
    MODULE_LICENSE("GPL");
