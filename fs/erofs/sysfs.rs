//! Automatically rewritten from C to Rust
//! Source: fs/erofs/sysfs.c
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
// Copyright (C), 2008-2021, OPPO Mobile Comm Corp., Ltd.
// https://www.oppo.com
//

    enum {
    attr_feature,
    attr_drop_caches,
    attr_pointer_ui,
    attr_pointer_bool,
    attr_accel,
    };
    enum {
    struct_erofs_sb_info,
    struct_erofs_mount_opts,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_attr {
    pub attr: attribute,
    pub attr_id: c_short,
    pub offset: int struct_type,,
}

    static struct erofs_attr erofs_attr_##_name = {				\
    .attr = {.name = __stringify(_name), .mode = _mode },		\
    .attr_id = attr_##_id,						\
    }

    static struct erofs_attr erofs_attr_##_name = {			\
    .attr = {.name = __stringify(_name), .mode = _mode },	\
    .attr_id = attr_##_id,					\
    .struct_type = struct_##_struct,			\
    .offset = offsetof(struct _struct, _name),\
    }

    EROFS_ATTR_OFFSET(_name, 0644, _id, _struct)

    EROFS_ATTR_OFFSET(_name, 0444, _id, _struct)

    EROFS_ATTR_RW(_name, pointer_ui, _struct)

    EROFS_ATTR_RW(_name, pointer_bool, _struct)

    EROFS_ATTR_RW_UI(sync_decompress, erofs_sb_info);
    EROFS_ATTR_FUNC(drop_caches, 0200);

    EROFS_ATTR_FUNC(accel, 0644);

    EROFS_ATTR_RW_UI(dir_ra_bytes, erofs_sb_info);
    static struct attribute *erofs_sb_attrs[] = {

    ATTR_LIST(sync_decompress),
    ATTR_LIST(drop_caches),

    ATTR_LIST(dir_ra_bytes),
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(erofs_sb);
    static struct attribute *erofs_attrs[] = {

    ATTR_LIST(accel),

    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(erofs);
// Features this copy of erofs supports
    EROFS_ATTR_FEATURE(compr_cfgs);
    EROFS_ATTR_FEATURE(big_pcluster);
    EROFS_ATTR_FEATURE(chunked_file);
    EROFS_ATTR_FEATURE(device_table);
    EROFS_ATTR_FEATURE(compr_head2);
    EROFS_ATTR_FEATURE(sb_chksum);
    EROFS_ATTR_FEATURE(ztailpacking);
    EROFS_ATTR_FEATURE(fragments);
    EROFS_ATTR_FEATURE(dedupe);
    EROFS_ATTR_FEATURE(48bit);
    EROFS_ATTR_FEATURE(metabox);
    static struct attribute *erofs_feat_attrs[] = {
    ATTR_LIST(compr_cfgs),
    ATTR_LIST(big_pcluster),
    ATTR_LIST(chunked_file),
    ATTR_LIST(device_table),
    ATTR_LIST(compr_head2),
    ATTR_LIST(sb_chksum),
    ATTR_LIST(ztailpacking),
    ATTR_LIST(fragments),
    ATTR_LIST(dedupe),
    ATTR_LIST(48bit),
    ATTR_LIST(metabox),
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(erofs_feat);
    static unsigned char *__struct_ptr(struct erofs_sb_info *sbi,
    int struct_type, int offset)
    {
    if (struct_type == struct_erofs_sb_info)
    return (unsigned char *)sbi + offset;
    if (struct_type == struct_erofs_mount_opts)
    return (unsigned char *)&sbi.opt + offset;
    return core::ptr::null_mut();
    }
    static ssize_t erofs_attr_show(struct kobject *kobj,
    struct attribute *attr, char *buf)
    {
    struct erofs_sb_info *sbi = container_of(kobj, struct erofs_sb_info,
    s_kobj);
    struct erofs_attr *a = container_of(attr, struct erofs_attr, attr);
    unsigned char *ptr = __struct_ptr(sbi, a.struct_type, a.offset);
    switch (a.attr_id) {
    case attr_feature:
    return sysfs_emit(buf, "supported\n");
    case attr_pointer_ui:
    if (!ptr)
    return 0;
    return sysfs_emit(buf, "%u\n", *(unsigned int *)ptr);
    case attr_pointer_bool:
    if (!ptr)
    return 0;
    return sysfs_emit(buf, "%d\n", *(bool *)ptr);
    case attr_accel:
    return z_erofs_crypto_show_engines(buf, PAGE_SIZE, '\n');
    }
    return 0;
    }
    static ssize_t erofs_attr_store(struct kobject *kobj, struct attribute *attr,
    const char *buf, size_t len)
    {
    struct erofs_sb_info *sbi = container_of(kobj, struct erofs_sb_info,
    s_kobj);
    struct erofs_attr *a = container_of(attr, struct erofs_attr, attr);
    unsigned char *ptr = __struct_ptr(sbi, a.struct_type, a.offset);
    unsigned long t;
    int ret;
    switch (a.attr_id) {
    case attr_pointer_ui:
    if (!ptr)
    return 0;
    ret = kstrtoul(skip_spaces(buf), 0, &t);
    if (ret)
    return ret;
    if (t != (unsigned int)t)
    return -ERANGE;
    if (IS_ENABLED(CONFIG_EROFS_FS_ZIP) &&
    !strcmp(a.attr.name, "sync_decompress") &&
    (t > EROFS_SYNC_DECOMPRESS_FORCE_OFF))
    return -EINVAL;
// (unsigned int *)ptr = t;
    return len;
    case attr_pointer_bool:
    if (!ptr)
    return 0;
    ret = kstrtoul(skip_spaces(buf), 0, &t);
    if (ret)
    return ret;
    if (t != 0 && t != 1)
    return -EINVAL;
// (bool *)ptr = !!t;
    return len;

    case attr_drop_caches:
    ret = kstrtoul(skip_spaces(buf), 0, &t);
    if (ret)
    return ret;
    if (t < 1 || t > 3)
    return -EINVAL;
    if (t & 2)
    z_erofs_shrink_scan(sbi, ~0UL);
    if (t & 1)
    invalidate_mapping_pages(MNGD_MAPPING(sbi), 0, -1);
    return len;

    case attr_accel:
    buf = skip_spaces(buf);
    z_erofs_crypto_disable_all_engines();
    while (*buf) {
    t = strcspn(buf, "\n");
    ret = z_erofs_crypto_enable_engine(buf, t);
    if (ret < 0)
    return ret;
    buf += buf[t] != '\0' ? t + 1 : t;
    }
    return len;

    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn erofs_sb_release(kobj: *mut kobject) {
    static void erofs_sb_release(struct kobject *kobj)
    {
    struct erofs_sb_info *sbi = container_of(kobj, struct erofs_sb_info,
    s_kobj);
    complete(&sbi.s_kobj_unregister);
    }
    static const struct sysfs_ops erofs_attr_ops = {
    .show	= erofs_attr_show,
    .store	= erofs_attr_store,
    };
    static const struct kobj_type erofs_sb_ktype = {
    .default_groups = erofs_sb_groups,
    .sysfs_ops	= &erofs_attr_ops,
    .release	= erofs_sb_release,
    };
    static const struct kobj_type erofs_ktype = {
    .default_groups = erofs_groups,
    .sysfs_ops	= &erofs_attr_ops,
    };
    static struct kset erofs_root = {
    .kobj	= {.ktype = &erofs_ktype},
    };
    static const struct kobj_type erofs_feat_ktype = {
    .default_groups = erofs_feat_groups,
    .sysfs_ops	= &erofs_attr_ops,
    };
    static struct kobject erofs_feat = {
    .kset	= &erofs_root,
    };
#[no_mangle]
pub unsafe extern "C" fn erofs_register_sysfs(sb: *mut super_block) -> c_int {
    int erofs_register_sysfs(struct super_block *sb)
    {
    struct erofs_sb_info *sbi = EROFS_SB(sb);
    int err;
    sbi.s_kobj.kset = &erofs_root;
    init_completion(&sbi.s_kobj_unregister);
    err = kobject_init_and_add(&sbi.s_kobj, &erofs_sb_ktype, core::ptr::null_mut(), "%s",
    sb.s_sysfs_name);
    if (err) {
    kobject_put(&sbi.s_kobj);
    wait_for_completion(&sbi.s_kobj_unregister);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn erofs_unregister_sysfs(sb: *mut super_block) {
    void erofs_unregister_sysfs(struct super_block *sb)
    {
    struct erofs_sb_info *sbi = EROFS_SB(sb);
    if (sbi.s_kobj.state_in_sysfs) {
    kobject_del(&sbi.s_kobj);
    kobject_put(&sbi.s_kobj);
    wait_for_completion(&sbi.s_kobj_unregister);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn erofs_exit_sysfs() {
    void erofs_exit_sysfs(void)
    {
    kobject_put(&erofs_feat);
    kset_unregister(&erofs_root);
    }
#[no_mangle]
pub unsafe extern "C" fn erofs_init_sysfs() -> int __init {
    int __init erofs_init_sysfs(void)
    {
    int ret;
    kobject_set_name(&erofs_root.kobj, "erofs");
    erofs_root.kobj.parent = fs_kobj;
    ret = kset_register(&erofs_root);
    if (!ret) {
    ret = kobject_init_and_add(&erofs_feat, &erofs_feat_ktype,
    core::ptr::null_mut(), "features");
    if (!ret)
    return 0;
    erofs_exit_sysfs();
    }
    return ret;
    }
