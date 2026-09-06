//! Automatically rewritten from C to Rust
//! Source: fs/ubifs/sysfs.c
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
// This file is part of UBIFS.
//
// Copyright (C) 2021 Cisco Systems
//
// Author: Stefan Schaeckeler
//

    enum attr_id_t {
    attr_errors_magic,
    attr_errors_node,
    attr_errors_crc,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_attr {
    pub attr: attribute,
    pub attr_id: enum attr_id_t,
}

    static struct ubifs_attr ubifs_attr_##_name = {				\
    .attr = {.name = __stringify(_name), .mode = _mode },		\
    .attr_id = attr_##_id,						\
    }

    UBIFS_ATTR_FUNC(errors_magic, 0444);
    UBIFS_ATTR_FUNC(errors_crc, 0444);
    UBIFS_ATTR_FUNC(errors_node, 0444);

    static struct attribute *ubifs_attrs[] = {
    ATTR_LIST(errors_magic),
    ATTR_LIST(errors_node),
    ATTR_LIST(errors_crc),
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(ubifs);
    static ssize_t ubifs_attr_show(struct kobject *kobj,
    struct attribute *attr, char *buf)
    {
    struct ubifs_info *sbi = container_of(kobj, struct ubifs_info,
    kobj);
    struct ubifs_attr *a = container_of(attr, struct ubifs_attr, attr);
    switch (a.attr_id) {
    case attr_errors_magic:
    return sysfs_emit(buf, "%u\n", sbi.stats.magic_errors);
    case attr_errors_node:
    return sysfs_emit(buf, "%u\n", sbi.stats.node_errors);
    case attr_errors_crc:
    return sysfs_emit(buf, "%u\n", sbi.stats.crc_errors);
    }
    return 0;
    };
#[no_mangle]
unsafe extern "C" fn ubifs_sb_release(kobj: *mut kobject) {
    static void ubifs_sb_release(struct kobject *kobj)
    {
    struct ubifs_info *c = container_of(kobj, struct ubifs_info, kobj);
    complete(&c.kobj_unregister);
    }
    static const struct sysfs_ops ubifs_attr_ops = {
    .show	= ubifs_attr_show,
    };
    static const struct kobj_type ubifs_sb_ktype = {
    .default_groups	= ubifs_groups,
    .sysfs_ops	= &ubifs_attr_ops,
    .release	= ubifs_sb_release,
    };
    static const struct kobj_type ubifs_ktype = {
    .sysfs_ops	= &ubifs_attr_ops,
    };
    static struct kset ubifs_kset = {
    .kobj	= {.ktype = &ubifs_ktype},
    };
#[no_mangle]
pub unsafe extern "C" fn ubifs_sysfs_register(c: *mut ubifs_info) -> c_int {
    int ubifs_sysfs_register(struct ubifs_info *c)
    {
    int ret, n;
    char dfs_dir_name[UBIFS_DFS_DIR_LEN];
    c.stats = kzalloc_obj(struct ubifs_stats_info);
    if (!c.stats) {
    ret = -ENOMEM;
    goto out_last;
    }
    n = snprintf(dfs_dir_name, UBIFS_DFS_DIR_LEN, UBIFS_DFS_DIR_NAME,
    c.vi.ubi_num, c.vi.vol_id);
    if (n >= UBIFS_DFS_DIR_LEN) {
// The array size is too small
    ret = -EINVAL;
    goto out_free;
    }
    c.kobj.kset = &ubifs_kset;
    init_completion(&c.kobj_unregister);
    ret = kobject_init_and_add(&c.kobj, &ubifs_sb_ktype, core::ptr::null_mut(),
    "%s", dfs_dir_name);
    if (ret)
    goto out_put;
    return 0;
    out_put:
    kobject_put(&c.kobj);
    wait_for_completion(&c.kobj_unregister);
    out_free:
    kfree(c.stats);
    out_last:
    ubifs_err(c, "cannot create sysfs entry for ubifs%d_%d, error %d\n",
    c.vi.ubi_num, c.vi.vol_id, ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ubifs_sysfs_unregister(c: *mut ubifs_info) {
    void ubifs_sysfs_unregister(struct ubifs_info *c)
    {
    kobject_del(&c.kobj);
    kobject_put(&c.kobj);
    wait_for_completion(&c.kobj_unregister);
    kfree(c.stats);
    }
#[no_mangle]
pub unsafe extern "C" fn ubifs_sysfs_init() -> int __init {
    int __init ubifs_sysfs_init(void)
    {
    int ret;
    kobject_set_name(&ubifs_kset.kobj, "ubifs");
    ubifs_kset.kobj.parent = fs_kobj;
    ret = kset_register(&ubifs_kset);
    if (ret)
    kset_put(&ubifs_kset);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ubifs_sysfs_exit() {
    void ubifs_sysfs_exit(void)
    {
    kset_unregister(&ubifs_kset);
    }
