//! Automatically rewritten from C to Rust
//! Source: fs/sysfs/mount.c
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
// fs/sysfs/symlink.c - operations for initializing and mounting sysfs
//
// Copyright (c) 2001-3 Patrick Mochel
// Copyright (c) 2007 SUSE Linux Products GmbH
// Copyright (c) 2007 Tejun Heo <teheo@suse.de>
//
// Please see Documentation/filesystems/sysfs.rst for more information.
//

    static struct kernfs_root *sysfs_root;
    struct kernfs_node *sysfs_root_kn;
#[no_mangle]
unsafe extern "C" fn sysfs_fs_context_free(fc: *mut fs_context) {
    static void sysfs_fs_context_free(struct fs_context *fc)
    {
    struct kernfs_fs_context *kfc = fc.fs_private;
    if (kfc.ns_tag)
    kobj_ns_drop(KOBJ_NS_TYPE_NET, kfc.ns_tag);
    kernfs_free_fs_context(fc);
    kfree(kfc);
    }
    static const struct fs_context_operations sysfs_fs_context_ops = {
    .free		= sysfs_fs_context_free,
    .get_tree	= kernfs_get_tree,
    };
#[no_mangle]
unsafe extern "C" fn sysfs_init_fs_context(fc: *mut fs_context) -> c_int {
    static int sysfs_init_fs_context(struct fs_context *fc)
    {
    struct kernfs_fs_context *kfc;
    struct ns_common *ns;
    if (!(fc.sb_flags & SB_KERNMOUNT)) {
    if (!kobj_ns_current_may_mount(KOBJ_NS_TYPE_NET))
    return -EPERM;
    }
    kfc = kzalloc_obj(struct kernfs_fs_context);
    if (!kfc)
    return -ENOMEM;
    kfc.ns_tag = ns = kobj_ns_grab_current(KOBJ_NS_TYPE_NET);
    kfc.root = sysfs_root;
    kfc.magic = SYSFS_MAGIC;
    fc.fs_private = kfc;
    fc.ops = &sysfs_fs_context_ops;
    if (ns) {
    struct net *netns = to_net_ns(ns);
    put_user_ns(fc.user_ns);
    fc.user_ns = get_user_ns(netns.user_ns);
    }
    fc.global = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sysfs_kill_sb(sb: *mut super_block) {
    static void sysfs_kill_sb(struct super_block *sb)
    {
    struct ns_common *ns = (struct ns_common *)kernfs_super_ns(sb);
    kernfs_kill_sb(sb);
    kobj_ns_drop(KOBJ_NS_TYPE_NET, ns);
    }
    static struct file_system_type sysfs_fs_type = {
    .name			= "sysfs",
    .init_fs_context	= sysfs_init_fs_context,
    .kill_sb		= sysfs_kill_sb,
    .fs_flags		= FS_USERNS_MOUNT | FS_USERNS_MOUNT_RESTRICTED,
    };
#[no_mangle]
pub unsafe extern "C" fn sysfs_init() -> int __init {
    int __init sysfs_init(void)
    {
    int err;
    sysfs_root = kernfs_create_root(core::ptr::null_mut(), KERNFS_ROOT_EXTRA_OPEN_PERM_CHECK,
    core::ptr::null_mut());
    if (IS_ERR(sysfs_root))
    return PTR_ERR(sysfs_root);
    sysfs_root_kn = kernfs_root_to_node(sysfs_root);
    err = register_filesystem(&sysfs_fs_type);
    if (err) {
    kernfs_destroy_root(sysfs_root);
    return err;
    }
    return 0;
    }
