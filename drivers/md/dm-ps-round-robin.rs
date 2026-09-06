//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-ps-round-robin.c
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
// Copyright (C) 2003 Sistina Software.
// Copyright (C) 2004-2005 Red Hat, Inc. All rights reserved.
//
// Module Author: Heinz Mauelshagen
//
// This file is released under the GPL.
//
// Round-robin path selector.
//

pub const RR_MIN_IO: c_int = 1;

//
// ---------------------------------------------------------------
// Path-handling code, paths are held in lists
// ---------------------------------------------------------------
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct path_info {
    pub list: list_head,
    pub path: *mut dm_path,
    pub repeat_count: c_uint,
}

#[no_mangle]
unsafe extern "C" fn free_paths(paths: *mut list_head) {
    static void free_paths(struct list_head *paths)
    {
    struct path_info *pi, *next;
    list_for_each_entry_safe(pi, next, paths, list) {
    list_del(&pi.list);
    kfree(pi);
    }
    }
//
// ---------------------------------------------------------------
// Round-robin selector
// ---------------------------------------------------------------
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct selector {
    pub valid_paths: list_head,
    pub invalid_paths: list_head,
    pub lock: spinlock_t,
}

    static struct selector *alloc_selector(void)
    {
    struct selector *s = kmalloc_obj(*s);
    if (s) {
    INIT_LIST_HEAD(&s.valid_paths);
    INIT_LIST_HEAD(&s.invalid_paths);
    spin_lock_init(&s.lock);
    }
    return s;
    }
#[no_mangle]
unsafe extern "C" fn rr_create(ps: *mut path_selector, argc: c_uint, argv: *mut c_char) -> c_int {
    static int rr_create(struct path_selector *ps, unsigned int argc, char **argv)
    {
    struct selector *s;
    s = alloc_selector();
    if (!s)
    return -ENOMEM;
    ps.context = s;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rr_destroy(ps: *mut path_selector) {
    static void rr_destroy(struct path_selector *ps)
    {
    struct selector *s = ps.context;
    free_paths(&s.valid_paths);
    free_paths(&s.invalid_paths);
    kfree(s);
    ps.context = core::ptr::null_mut();
    }
    static int rr_status(struct path_selector *ps, struct dm_path *path,
    status_type_t type, char *result, unsigned int maxlen)
    {
    struct path_info *pi;
    let mut sz: c_int = 0;
    if (!path)
    DMEMIT("0 ");
    else {
    switch (type) {
    case STATUSTYPE_INFO:
    break;
    case STATUSTYPE_TABLE:
    pi = path.pscontext;
    DMEMIT("%u ", pi.repeat_count);
    break;
    case STATUSTYPE_IMA:
// result = '\0';
    break;
    }
    }
    return sz;
    }
//
// Called during initialisation to register each path with an
// optional repeat_count.
//
    static int rr_add_path(struct path_selector *ps, struct dm_path *path,
    int argc, char **argv, char **error)
    {
    struct selector *s = ps.context;
    struct path_info *pi;
    let mut repeat_count: c_uint = RR_MIN_IO;
    char dummy;
    unsigned long flags;
    if (argc > 1) {
// error = "round-robin ps: incorrect number of arguments";
    return -EINVAL;
    }
// First path argument is number of I/Os before switching path
    if ((argc == 1) && (sscanf(argv[0], "%u%c", &repeat_count, &dummy) != 1)) {
// error = "round-robin ps: invalid repeat count";
    return -EINVAL;
    }
    if (repeat_count > 1) {
    DMWARN_LIMIT("repeat_count > 1 is deprecated, using 1 instead");
    repeat_count = 1;
    }
// allocate the path
    pi = kmalloc_obj(*pi);
    if (!pi) {
// error = "round-robin ps: Error allocating path context";
    return -ENOMEM;
    }
    pi.path = path;
    pi.repeat_count = repeat_count;
    path.pscontext = pi;
    spin_lock_irqsave(&s.lock, flags);
    list_add_tail(&pi.list, &s.valid_paths);
    spin_unlock_irqrestore(&s.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rr_fail_path(ps: *mut path_selector, p: *mut dm_path) {
    static void rr_fail_path(struct path_selector *ps, struct dm_path *p)
    {
    unsigned long flags;
    struct selector *s = ps.context;
    struct path_info *pi = p.pscontext;
    spin_lock_irqsave(&s.lock, flags);
    list_move(&pi.list, &s.invalid_paths);
    spin_unlock_irqrestore(&s.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn rr_reinstate_path(ps: *mut path_selector, p: *mut dm_path) -> c_int {
    static int rr_reinstate_path(struct path_selector *ps, struct dm_path *p)
    {
    unsigned long flags;
    struct selector *s = ps.context;
    struct path_info *pi = p.pscontext;
    spin_lock_irqsave(&s.lock, flags);
    list_move(&pi.list, &s.valid_paths);
    spin_unlock_irqrestore(&s.lock, flags);
    return 0;
    }
    static struct dm_path *rr_select_path(struct path_selector *ps, size_t nr_bytes)
    {
    unsigned long flags;
    struct selector *s = ps.context;
    struct path_info *pi = core::ptr::null_mut();
    spin_lock_irqsave(&s.lock, flags);
    if (!list_empty(&s.valid_paths)) {
    pi = list_entry(s.valid_paths.next, struct path_info, list);
    list_move_tail(&pi.list, &s.valid_paths);
    }
    spin_unlock_irqrestore(&s.lock, flags);
    return pi ? pi.path : core::ptr::null_mut();
    }
    static struct path_selector_type rr_ps = {
    .name = "round-robin",
    .module = THIS_MODULE,
    .table_args = 1,
    .info_args = 0,
    .create = rr_create,
    .destroy = rr_destroy,
    .status = rr_status,
    .add_path = rr_add_path,
    .fail_path = rr_fail_path,
    .reinstate_path = rr_reinstate_path,
    .select_path = rr_select_path,
    };
#[no_mangle]
unsafe extern "C" fn dm_rr_init() -> int __init {
    static int __init dm_rr_init(void)
    {
    let mut r: c_int = dm_register_path_selector(&rr_ps);
    if (r < 0) {
    DMERR("register failed %d", r);
    return r;
    }
    DMINFO("version " RR_VERSION " loaded");
    return r;
    }
#[no_mangle]
unsafe extern "C" fn dm_rr_exit() -> void __exit {
    static void __exit dm_rr_exit(void)
    {
    dm_unregister_path_selector(&rr_ps);
    }
    module_init(dm_rr_init);
    module_exit(dm_rr_exit);
    MODULE_DESCRIPTION(DM_NAME " round-robin multipath path selector");
    MODULE_AUTHOR("Sistina Software <dm-devel@lists.linux.dev>");
    MODULE_LICENSE("GPL");
