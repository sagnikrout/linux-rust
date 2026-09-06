//! Automatically rewritten from C to Rust
//! Source: fs/zonefs/sysfs.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Simple file system for zoned block devices exposing zones as files.
//
// Copyright (C) 2022 Western Digital Corporation or its affiliates.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zonefs_sysfs_attr {
    pub attr: attribute,
    pub buf): *mut *mut *mut ssize_t (show)(struct zonefs_sb_info sbi, char,
}

    static struct zonefs_sysfs_attr zonefs_sysfs_attr_##name = __ATTR_RO(name)

    static ssize_t zonefs_sysfs_attr_show(struct kobject *kobj,
    struct attribute *attr, char *buf)
    {
    struct zonefs_sb_info *sbi =
    container_of(kobj, struct zonefs_sb_info, s_kobj);
    struct zonefs_sysfs_attr *zonefs_attr =
    container_of(attr, struct zonefs_sysfs_attr, attr);
    if (!zonefs_attr.show)
    return 0;
    return zonefs_attr.show(sbi, buf);
    }
#[no_mangle]
unsafe extern "C" fn max_wro_seq_files_show(sbi: *mut zonefs_sb_info, buf: *mut c_char) -> isize {
    static ssize_t max_wro_seq_files_show(struct zonefs_sb_info *sbi, char *buf)
    {
    return sysfs_emit(buf, "%u\n", sbi.s_max_wro_seq_files);
    }
    ZONEFS_SYSFS_ATTR_RO(max_wro_seq_files);
#[no_mangle]
unsafe extern "C" fn nr_wro_seq_files_show(sbi: *mut zonefs_sb_info, buf: *mut c_char) -> isize {
    static ssize_t nr_wro_seq_files_show(struct zonefs_sb_info *sbi, char *buf)
    {
    return sysfs_emit(buf, "%d\n", atomic_read(&sbi.s_wro_seq_files));
    }
    ZONEFS_SYSFS_ATTR_RO(nr_wro_seq_files);
#[no_mangle]
unsafe extern "C" fn max_active_seq_files_show(sbi: *mut zonefs_sb_info, buf: *mut c_char) -> isize {
    static ssize_t max_active_seq_files_show(struct zonefs_sb_info *sbi, char *buf)
    {
    return sysfs_emit(buf, "%u\n", sbi.s_max_active_seq_files);
    }
    ZONEFS_SYSFS_ATTR_RO(max_active_seq_files);
#[no_mangle]
unsafe extern "C" fn nr_active_seq_files_show(sbi: *mut zonefs_sb_info, buf: *mut c_char) -> isize {
    static ssize_t nr_active_seq_files_show(struct zonefs_sb_info *sbi, char *buf)
    {
    return sysfs_emit(buf, "%d\n", atomic_read(&sbi.s_active_seq_files));
    }
    ZONEFS_SYSFS_ATTR_RO(nr_active_seq_files);
    static struct attribute *zonefs_sysfs_attrs[] = {
    ATTR_LIST(max_wro_seq_files),
    ATTR_LIST(nr_wro_seq_files),
    ATTR_LIST(max_active_seq_files),
    ATTR_LIST(nr_active_seq_files),
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(zonefs_sysfs);
#[no_mangle]
unsafe extern "C" fn zonefs_sysfs_sb_release(kobj: *mut kobject) {
    static void zonefs_sysfs_sb_release(struct kobject *kobj)
    {
    struct zonefs_sb_info *sbi =
    container_of(kobj, struct zonefs_sb_info, s_kobj);
    complete(&sbi.s_kobj_unregister);
    }
    static const struct sysfs_ops zonefs_sysfs_attr_ops = {
    .show	= zonefs_sysfs_attr_show,
    };
    static const struct kobj_type zonefs_sb_ktype = {
    .default_groups = zonefs_sysfs_groups,
    .sysfs_ops	= &zonefs_sysfs_attr_ops,
    .release	= zonefs_sysfs_sb_release,
    };
    static struct kobject *zonefs_sysfs_root;
#[no_mangle]
pub unsafe extern "C" fn zonefs_sysfs_register(sb: *mut super_block) -> c_int {
    int zonefs_sysfs_register(struct super_block *sb)
    {
    struct zonefs_sb_info *sbi = ZONEFS_SB(sb);
    int ret;
    super_set_sysfs_name_id(sb);
    init_completion(&sbi.s_kobj_unregister);
    ret = kobject_init_and_add(&sbi.s_kobj, &zonefs_sb_ktype,
    zonefs_sysfs_root, "%s", sb.s_id);
    if (ret) {
    kobject_put(&sbi.s_kobj);
    wait_for_completion(&sbi.s_kobj_unregister);
    return ret;
    }
    sbi.s_sysfs_registered = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn zonefs_sysfs_unregister(sb: *mut super_block) {
    void zonefs_sysfs_unregister(struct super_block *sb)
    {
    struct zonefs_sb_info *sbi = ZONEFS_SB(sb);
    if (!sbi || !sbi.s_sysfs_registered)
    return;
    kobject_del(&sbi.s_kobj);
    kobject_put(&sbi.s_kobj);
    wait_for_completion(&sbi.s_kobj_unregister);
    }
#[no_mangle]
pub unsafe extern "C" fn zonefs_sysfs_init() -> int __init {
    int __init zonefs_sysfs_init(void)
    {
    zonefs_sysfs_root = kobject_create_and_add("zonefs", fs_kobj);
    if (!zonefs_sysfs_root)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn zonefs_sysfs_exit() {
    void zonefs_sysfs_exit(void)
    {
    kobject_put(zonefs_sysfs_root);
    zonefs_sysfs_root = core::ptr::null_mut();
    }
