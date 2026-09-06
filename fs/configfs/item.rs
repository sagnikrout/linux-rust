//! Automatically rewritten from C to Rust
//! Source: fs/configfs/item.c
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
// item.c - library routines for handling generic config items
//
// Based on kobject:
// kobject is Copyright (c) 2002-2003 Patrick Mochel
//
// configfs Copyright (C) 2005 Oracle.  All rights reserved.
//
// Please see the file Documentation/filesystems/configfs.rst for
// critical information about using the config_item interface.
//

    static inline struct config_item *to_item(struct list_head *entry)
    {
    return container_of(entry, struct config_item, ci_entry);
    }
// Evil kernel
    static void config_item_release(struct kref *kref);
//
// config_item_init - initialize item.
// @item:	item in question.
//
#[no_mangle]
unsafe extern "C" fn config_item_init(item: *mut config_item) {
    static void config_item_init(struct config_item *item)
    {
    kref_init(&item.ci_kref);
    INIT_LIST_HEAD(&item.ci_entry);
    }
//
// config_item_set_name - Set the name of an item
// @item:	item.
// @fmt:  The vsnprintf()'s format string.
//
// If strlen(name) >= CONFIGFS_ITEM_NAME_LEN, then use a
// dynamically allocated string that @item->ci_name points to.
// Otherwise, use the static @item->ci_namebuf array.
//
#[no_mangle]
pub unsafe extern "C" fn config_item_set_name(item: *mut config_item, fmt: *const c_char, ...) -> c_int {
    int config_item_set_name(struct config_item *item, const char *fmt, ...)
    {
    let mut limit: c_int = CONFIGFS_ITEM_NAME_LEN;
    int need;
    va_list args;
    char *name;
//
// First, try the static array
//
    va_start(args, fmt);
    need = vsnprintf(item.ci_namebuf, limit, fmt, args);
    va_end(args);
    if (need < limit)
    name = item.ci_namebuf;
    else {
    va_start(args, fmt);
    name = kvasprintf(GFP_KERNEL, fmt, args);
    va_end(args);
    if (!name)
    return -ENOMEM;
    }
// Free the old name, if necessary.
    if (item.ci_name && item.ci_name != item.ci_namebuf)
    kfree(item.ci_name);
// Now, set the new name
    item.ci_name = name;
    return 0;
    }
    EXPORT_SYMBOL(config_item_set_name);
    void config_item_init_type_name(struct config_item *item,
    const char *name,
    const struct config_item_type *type)
    {
    config_item_set_name(item, "%s", name);
    item.ci_type = type;
    config_item_init(item);
    }
    EXPORT_SYMBOL(config_item_init_type_name);
    void config_group_init_type_name(struct config_group *group, const char *name,
    const struct config_item_type *type)
    {
    config_item_set_name(&group.cg_item, "%s", name);
    group.cg_item.ci_type = type;
    config_group_init(group);
    }
    EXPORT_SYMBOL(config_group_init_type_name);
    struct config_item *config_item_get(struct config_item *item)
    {
    if (item)
    kref_get(&item.ci_kref);
    return item;
    }
    EXPORT_SYMBOL(config_item_get);
    struct config_item *config_item_get_unless_zero(struct config_item *item)
    {
    if (item && kref_get_unless_zero(&item.ci_kref))
    return item;
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(config_item_get_unless_zero);
#[no_mangle]
unsafe extern "C" fn config_item_cleanup(item: *mut config_item) {
    static void config_item_cleanup(struct config_item *item)
    {
    const struct config_item_type *t = item.ci_type;
    struct config_group *s = item.ci_group;
    struct config_item *parent = item.ci_parent;
    pr_debug("config_item %s: cleaning up\n", config_item_name(item));
    if (item.ci_name != item.ci_namebuf)
    kfree(item.ci_name);
    item.ci_name = core::ptr::null_mut();
    if (t && t.ct_item_ops && t.ct_item_ops.release)
    t.ct_item_ops.release(item);
    if (s)
    config_group_put(s);
    if (parent)
    config_item_put(parent);
    }
#[no_mangle]
unsafe extern "C" fn config_item_release(kref: *mut kref) {
    static void config_item_release(struct kref *kref)
    {
    config_item_cleanup(container_of(kref, struct config_item, ci_kref));
    }
//
// config_item_put - decrement refcount for item.
// @item:	item.
//
// Decrement the refcount, and if 0, call config_item_cleanup().
//
#[no_mangle]
pub unsafe extern "C" fn config_item_put(item: *mut config_item) {
    void config_item_put(struct config_item *item)
    {
    if (item)
    kref_put(&item.ci_kref, config_item_release);
    }
    EXPORT_SYMBOL(config_item_put);
//
// config_group_init - initialize a group for use
// @group:	config_group
//
#[no_mangle]
pub unsafe extern "C" fn config_group_init(group: *mut config_group) {
    void config_group_init(struct config_group *group)
    {
    config_item_init(&group.cg_item);
    INIT_LIST_HEAD(&group.cg_children);
    INIT_LIST_HEAD(&group.default_groups);
    }
    EXPORT_SYMBOL(config_group_init);
//
// config_group_find_item - search for item in group.
// @group:	group we're looking in.
// @name:	item's name.
//
// Iterate over @group->cg_list, looking for a matching config_item.
// If matching item is found take a reference and return the item.
// Caller must have locked group via @group->cg_subsys->su_mtx.
//
    struct config_item *config_group_find_item(struct config_group *group,
    const char *name)
    {
    struct list_head *entry;
    struct config_item *ret = core::ptr::null_mut();
    list_for_each(entry, &group.cg_children) {
    struct config_item *item = to_item(entry);
    if (config_item_name(item) &&
    !strcmp(config_item_name(item), name)) {
    ret = config_item_get(item);
    break;
    }
    }
    return ret;
    }
    EXPORT_SYMBOL(config_group_find_item);
