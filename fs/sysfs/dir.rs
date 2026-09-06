//! Automatically rewritten from C to Rust
//! Source: fs/sysfs/dir.c
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
// fs/sysfs/dir.c - sysfs core and dir operation implementation
//
// Copyright (c) 2001-3 Patrick Mochel
// Copyright (c) 2007 SUSE Linux Products GmbH
// Copyright (c) 2007 Tejun Heo <teheo@suse.de>
//
// Please see Documentation/filesystems/sysfs.rst for more information.
//

    DEFINE_SPINLOCK(sysfs_symlink_target_lock);
#[no_mangle]
pub unsafe extern "C" fn sysfs_warn_dup(parent: *mut kernfs_node, name: *const c_char) {
    void sysfs_warn_dup(struct kernfs_node *parent, const char *name)
    {
    char *buf;
    buf = kzalloc(PATH_MAX, GFP_KERNEL);
    if (buf)
    kernfs_path(parent, buf, PATH_MAX);
    pr_warn("cannot create duplicate filename '%s/%s'\n", buf, name);
    dump_stack();
    kfree(buf);
    }
//
// sysfs_create_dir_ns - create a directory for an object with a namespace tag
// @kobj: object we're creating directory for
// @ns: the namespace tag to use
//
#[no_mangle]
pub unsafe extern "C" fn sysfs_create_dir_ns(kobj: *mut kobject, ns: *const ns_common) -> c_int {
    int sysfs_create_dir_ns(struct kobject *kobj, const struct ns_common *ns)
    {
    struct kernfs_node *parent, *kn;
    kuid_t uid;
    kgid_t gid;
    if (WARN_ON(!kobj))
    return -EINVAL;
    if (kobj.parent)
    parent = kobj.parent.sd;
    else
    parent = sysfs_root_kn;
    if (!parent)
    return -ENOENT;
    kobject_get_ownership(kobj, &uid, &gid);
    kn = kernfs_create_dir_ns(parent, kobject_name(kobj), 0755, uid, gid,
    kobj, ns);
    if (IS_ERR(kn)) {
    if (PTR_ERR(kn) == -EEXIST)
    sysfs_warn_dup(parent, kobject_name(kobj));
    return PTR_ERR(kn);
    }
    kobj.sd = kn;
    return 0;
    }
//
// sysfs_remove_dir - remove an object's directory.
// @kobj:	object.
//
// The only thing special about this is that we remove any files in
// the directory before we remove the directory, and we've inlined
// what used to be sysfs_rmdir() below, instead of calling separately.
//
#[no_mangle]
pub unsafe extern "C" fn sysfs_remove_dir(kobj: *mut kobject) {
    void sysfs_remove_dir(struct kobject *kobj)
    {
    struct kernfs_node *kn = kobj.sd;
//
// In general, kobject owner is responsible for ensuring removal
// doesn't race with other operations and sysfs doesn't provide any
// protection; however, when @kobj is used as a symlink target, the
// symlinking entity usually doesn't own @kobj and thus has no
// control over removal.  @kobj->sd may be removed anytime
// and symlink code may end up dereferencing an already freed node.
//
// sysfs_symlink_target_lock synchronizes @kobj->sd
// disassociation against symlink operations so that symlink code
// can safely dereference @kobj->sd.
//
    spin_lock(&sysfs_symlink_target_lock);
    kobj.sd = core::ptr::null_mut();
    spin_unlock(&sysfs_symlink_target_lock);
    if (kn) {
    WARN_ON_ONCE(kernfs_type(kn) != KERNFS_DIR);
    kernfs_remove(kn);
    }
    }
    int sysfs_rename_dir_ns(struct kobject *kobj, const char *new_name,
    const struct ns_common *new_ns)
    {
    struct kernfs_node *parent;
    int ret;
    parent = kernfs_get_parent(kobj.sd);
    ret = kernfs_rename_ns(kobj.sd, parent, new_name, new_ns);
    kernfs_put(parent);
    return ret;
    }
    int sysfs_move_dir_ns(struct kobject *kobj, struct kobject *new_parent_kobj,
    const struct ns_common *new_ns)
    {
    struct kernfs_node *kn = kobj.sd;
    struct kernfs_node *new_parent;
    new_parent = new_parent_kobj && new_parent_kobj.sd ?
    new_parent_kobj.sd : sysfs_root_kn;
    return kernfs_rename_ns(kn, new_parent, core::ptr::null_mut(), new_ns);
    }
//
// sysfs_create_mount_point - create an always empty directory
// @parent_kobj:  kobject that will contain this always empty directory
// @name: The name of the always empty directory to add
//
#[no_mangle]
pub unsafe extern "C" fn sysfs_create_mount_point(parent_kobj: *mut kobject, name: *const c_char) -> c_int {
    int sysfs_create_mount_point(struct kobject *parent_kobj, const char *name)
    {
    struct kernfs_node *kn, *parent = parent_kobj.sd;
    kn = kernfs_create_empty_dir(parent, name);
    if (IS_ERR(kn)) {
    if (PTR_ERR(kn) == -EEXIST)
    sysfs_warn_dup(parent, name);
    return PTR_ERR(kn);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(sysfs_create_mount_point);
//
// sysfs_remove_mount_point - remove an always empty directory.
// @parent_kobj: kobject that will contain this always empty directory
// @name: The name of the always empty directory to remove
//
#[no_mangle]
pub unsafe extern "C" fn sysfs_remove_mount_point(parent_kobj: *mut kobject, name: *const c_char) {
    void sysfs_remove_mount_point(struct kobject *parent_kobj, const char *name)
    {
    struct kernfs_node *parent = parent_kobj.sd;
    kernfs_remove_by_name_ns(parent, name, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(sysfs_remove_mount_point);
