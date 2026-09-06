//! Automatically rewritten from C to Rust
//! Source: fs/sysfs/file.c
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
// fs/sysfs/file.c - sysfs regular (text) file implementation
//
// Copyright (c) 2001-3 Patrick Mochel
// Copyright (c) 2007 SUSE Linux Products GmbH
// Copyright (c) 2007 Tejun Heo <teheo@suse.de>
//
// Please see Documentation/filesystems/sysfs.rst for more information.
//

    static struct kobject *sysfs_file_kobj(struct kernfs_node *kn)
    {
    guard(rcu)();
    return rcu_dereference(kn.__parent).priv;
    }
//
// Determine ktype->sysfs_ops for the given kernfs_node.  This function
// must be called while holding an active reference.
//
    static const struct sysfs_ops *sysfs_file_ops(struct kernfs_node *kn)
    {
    struct kobject *kobj = sysfs_file_kobj(kn);
    if (kn.flags & KERNFS_LOCKDEP)
    lockdep_assert_held(kn);
    return kobj.ktype ? kobj.ktype.sysfs_ops : core::ptr::null_mut();
    }
//
// Reads on sysfs are handled through seq_file, which takes care of hairy
// details like buffering and seeking.  The following function pipes
// sysfs_ops->show() result through seq_file.
//
#[no_mangle]
unsafe extern "C" fn sysfs_kf_seq_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    static int sysfs_kf_seq_show(struct seq_file *sf, void *v)
    {
    struct kernfs_open_file *of = sf.private;
    struct kobject *kobj = sysfs_file_kobj(of.kn);
    const struct sysfs_ops *ops = sysfs_file_ops(of.kn);
    ssize_t count;
    char *buf;
    if (WARN_ON_ONCE(!ops.show))
    return -EINVAL;
// acquire buffer and ensure that it's >= PAGE_SIZE and clear
    count = seq_get_buf(sf, &buf);
    if (count < PAGE_SIZE) {
    seq_commit(sf, -1);
    return 0;
    }
    memset(buf, 0, PAGE_SIZE);
    count = ops.show(kobj, of.kn.priv, buf);
    if (count < 0)
    return count;
//
// The code works fine with PAGE_SIZE return but it's likely to
// indicate truncated result or overflow in normal use cases.
//
    if (count >= PAGE_SIZE) {
    WARN(1, "OOB write or bad count %zd at %pS\n", count, ops.show);
// Try to struggle along
    count = PAGE_SIZE - 1;
    }
    seq_commit(sf, count);
    return 0;
    }
    static ssize_t sysfs_kf_bin_read(struct kernfs_open_file *of, char *buf,
    size_t count, loff_t pos)
    {
    const struct bin_attribute *battr = of.kn.priv;
    struct kobject *kobj = sysfs_file_kobj(of.kn);
    let mut size: loff_t = file_inode(of.file).i_size;
    if (!count)
    return 0;
    if (size) {
    if (pos >= size)
    return 0;
    if (pos + count > size)
    count = size - pos;
    }
    if (!battr.read)
    return -EIO;
    return battr.read(of.file, kobj, battr, buf, pos, count);
    }
// kernfs read callback for regular sysfs files with pre-alloc
    static ssize_t sysfs_kf_read(struct kernfs_open_file *of, char *buf,
    size_t count, loff_t pos)
    {
    const struct sysfs_ops *ops = sysfs_file_ops(of.kn);
    struct kobject *kobj = sysfs_file_kobj(of.kn);
    ssize_t len;
//
// If buf != of->prealloc_buf, we don't know how
// large it is, so cannot safely pass it to ->show
//
    if (WARN_ON_ONCE(buf != of.prealloc_buf))
    return 0;
    len = ops.show(kobj, of.kn.priv, buf);
    if (len < 0)
    return len;
    if (len >= (ssize_t)PAGE_SIZE) {
    printk("fill_read_buffer: %pS returned bad count\n", ops.show);
    len = PAGE_SIZE - 1;
    }
    if (pos) {
    if (len <= pos)
    return 0;
    len -= pos;
    memmove(buf, buf + pos, len);
    }
    return min_t(ssize_t, count, len);
    }
// kernfs write callback for regular sysfs files
    static ssize_t sysfs_kf_write(struct kernfs_open_file *of, char *buf,
    size_t count, loff_t pos)
    {
    const struct sysfs_ops *ops = sysfs_file_ops(of.kn);
    struct kobject *kobj = sysfs_file_kobj(of.kn);
    if (!count)
    return 0;
    return ops.store(kobj, of.kn.priv, buf, count);
    }
// kernfs write callback for bin sysfs files
    static ssize_t sysfs_kf_bin_write(struct kernfs_open_file *of, char *buf,
    size_t count, loff_t pos)
    {
    const struct bin_attribute *battr = of.kn.priv;
    struct kobject *kobj = sysfs_file_kobj(of.kn);
    let mut size: loff_t = file_inode(of.file).i_size;
    if (size) {
    if (size <= pos)
    return -EFBIG;
    count = min_t(ssize_t, count, size - pos);
    }
    if (!count)
    return 0;
    if (!battr.write)
    return -EIO;
    return battr.write(of.file, kobj, battr, buf, pos, count);
    }
    static int sysfs_kf_bin_mmap(struct kernfs_open_file *of,
    struct vm_area_struct *vma)
    {
    const struct bin_attribute *battr = of.kn.priv;
    struct kobject *kobj = sysfs_file_kobj(of.kn);
    return battr.mmap(of.file, kobj, battr, vma);
    }
    static loff_t sysfs_kf_bin_llseek(struct kernfs_open_file *of, loff_t offset,
    int whence)
    {
    const struct bin_attribute *battr = of.kn.priv;
    struct kobject *kobj = sysfs_file_kobj(of.kn);
    if (battr.llseek)
    return battr.llseek(of.file, kobj, battr, offset, whence);
    else
    return generic_file_llseek(of.file, offset, whence);
    }
#[no_mangle]
unsafe extern "C" fn sysfs_kf_bin_open(of: *mut kernfs_open_file) -> c_int {
    static int sysfs_kf_bin_open(struct kernfs_open_file *of)
    {
    const struct bin_attribute *battr = of.kn.priv;
    if (battr.f_mapping)
    of.file.f_mapping = battr.f_mapping();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sysfs_notify(kobj: *mut kobject, dir: *const c_char, attr: *const c_char) {
    void sysfs_notify(struct kobject *kobj, const char *dir, const char *attr)
    {
    struct kernfs_node *kn = kobj.sd, *tmp;
    if (kn && dir)
    kn = kernfs_find_and_get(kn, dir);
    else
    kernfs_get(kn);
    if (kn && attr) {
    tmp = kernfs_find_and_get(kn, attr);
    kernfs_put(kn);
    kn = tmp;
    }
    if (kn) {
    kernfs_notify(kn);
    kernfs_put(kn);
    }
    }
    EXPORT_SYMBOL_GPL(sysfs_notify);
    static const struct kernfs_ops sysfs_file_kfops_empty = {
    };
    static const struct kernfs_ops sysfs_file_kfops_ro = {
    .seq_show	= sysfs_kf_seq_show,
    };
    static const struct kernfs_ops sysfs_file_kfops_wo = {
    .write		= sysfs_kf_write,
    };
    static const struct kernfs_ops sysfs_file_kfops_rw = {
    .seq_show	= sysfs_kf_seq_show,
    .write		= sysfs_kf_write,
    };
    static const struct kernfs_ops sysfs_prealloc_kfops_ro = {
    .read		= sysfs_kf_read,
    .prealloc	= true,
    };
    static const struct kernfs_ops sysfs_prealloc_kfops_wo = {
    .write		= sysfs_kf_write,
    .prealloc	= true,
    };
    static const struct kernfs_ops sysfs_prealloc_kfops_rw = {
    .read		= sysfs_kf_read,
    .write		= sysfs_kf_write,
    .prealloc	= true,
    };
    static const struct kernfs_ops sysfs_bin_kfops_ro = {
    .read		= sysfs_kf_bin_read,
    };
    static const struct kernfs_ops sysfs_bin_kfops_wo = {
    .write		= sysfs_kf_bin_write,
    };
    static const struct kernfs_ops sysfs_bin_kfops_rw = {
    .read		= sysfs_kf_bin_read,
    .write		= sysfs_kf_bin_write,
    };
    static const struct kernfs_ops sysfs_bin_kfops_mmap = {
    .read		= sysfs_kf_bin_read,
    .write		= sysfs_kf_bin_write,
    .mmap		= sysfs_kf_bin_mmap,
    .open		= sysfs_kf_bin_open,
    .llseek		= sysfs_kf_bin_llseek,
    };
    int sysfs_add_file_mode_ns(struct kernfs_node *parent,
    const struct attribute *attr, umode_t mode, kuid_t uid,
    kgid_t gid, const struct ns_common *ns)
    {
    struct kobject *kobj = parent.priv;
    const struct sysfs_ops *sysfs_ops = kobj.ktype.sysfs_ops;
    struct lock_class_key *key = core::ptr::null_mut();
    const struct kernfs_ops *ops = core::ptr::null_mut();
    struct kernfs_node *kn;
// every kobject with an attribute needs a ktype assigned
    if (WARN(!sysfs_ops, KERN_ERR
    "missing sysfs attribute operations for kobject: %s\n",
    kobject_name(kobj)))
    return -EINVAL;
    if (mode & SYSFS_PREALLOC) {
    if (sysfs_ops.show && sysfs_ops.store)
    ops = &sysfs_prealloc_kfops_rw;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_ops->show) -> else {
    else if (sysfs_ops.show)
    ops = &sysfs_prealloc_kfops_ro;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_ops->store) -> else {
    else if (sysfs_ops.store)
    ops = &sysfs_prealloc_kfops_wo;
    } else {
    if (sysfs_ops.show && sysfs_ops.store)
    ops = &sysfs_file_kfops_rw;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_ops->show) -> else {
    else if (sysfs_ops.show)
    ops = &sysfs_file_kfops_ro;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_ops->store) -> else {
    else if (sysfs_ops.store)
    ops = &sysfs_file_kfops_wo;
    }
    if (!ops)
    ops = &sysfs_file_kfops_empty;

    if (!attr.ignore_lockdep)
    key = attr.key ?: (struct lock_class_key *)&attr.skey;

    kn = __kernfs_create_file(parent, attr.name, mode & 0777, uid, gid,
    PAGE_SIZE, ops, (void *)attr, ns, key);
    if (IS_ERR(kn)) {
    if (PTR_ERR(kn) == -EEXIST)
    sysfs_warn_dup(parent, attr.name);
    return PTR_ERR(kn);
    }
    return 0;
    }
    int sysfs_add_bin_file_mode_ns(struct kernfs_node *parent,
    const struct bin_attribute *battr, umode_t mode, size_t size,
    kuid_t uid, kgid_t gid, const struct ns_common *ns)
    {
    const struct attribute *attr = &battr.attr;
    struct lock_class_key *key = core::ptr::null_mut();
    const struct kernfs_ops *ops;
    struct kernfs_node *kn;
    if (battr.mmap)
    ops = &sysfs_bin_kfops_mmap;
#[no_mangle]
pub unsafe extern "C" fn if(battr->write: battr->read &&) -> else {
    else if (battr.read && battr.write)
    ops = &sysfs_bin_kfops_rw;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: battr->read) -> else {
    else if (battr.read)
    ops = &sysfs_bin_kfops_ro;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: battr->write) -> else {
    else if (battr.write)
    ops = &sysfs_bin_kfops_wo;
    else
    ops = &sysfs_file_kfops_empty;

    if (!attr.ignore_lockdep)
    key = attr.key ?: (struct lock_class_key *)&attr.skey;

    kn = __kernfs_create_file(parent, attr.name, mode & 0777, uid, gid,
    size, ops, (void *)attr, ns, key);
    if (IS_ERR(kn)) {
    if (PTR_ERR(kn) == -EEXIST)
    sysfs_warn_dup(parent, attr.name);
    return PTR_ERR(kn);
    }
    return 0;
    }
//
// sysfs_create_file_ns - create an attribute file for an object with custom ns
// @kobj: object we're creating for
// @attr: attribute descriptor
// @ns: namespace the new file should belong to
//
    int sysfs_create_file_ns(struct kobject *kobj, const struct attribute *attr,
    const struct ns_common *ns)
    {
    kuid_t uid;
    kgid_t gid;
    if (WARN_ON(!kobj || !kobj.sd || !attr))
    return -EINVAL;
    kobject_get_ownership(kobj, &uid, &gid);
    return sysfs_add_file_mode_ns(kobj.sd, attr, attr.mode, uid, gid, ns);
    }
    EXPORT_SYMBOL_GPL(sysfs_create_file_ns);
#[no_mangle]
pub unsafe extern "C" fn sysfs_create_files(kobj: *mut kobject, ptr: *const *const attribute) -> c_int {
    int sysfs_create_files(struct kobject *kobj, const struct attribute * const *ptr)
    {
    let mut err: c_int = 0;
    int i;
    for (i = 0; ptr[i] && !err; i++)
    err = sysfs_create_file(kobj, ptr[i]);
    if (err)
    while (--i >= 0)
    sysfs_remove_file(kobj, ptr[i]);
    return err;
    }
    EXPORT_SYMBOL_GPL(sysfs_create_files);
//
// sysfs_add_file_to_group - add an attribute file to a pre-existing group.
// @kobj: object we're acting for.
// @attr: attribute descriptor.
// @group: group name.
//
    int sysfs_add_file_to_group(struct kobject *kobj,
    const struct attribute *attr, const char *group)
    {
    struct kernfs_node *parent;
    kuid_t uid;
    kgid_t gid;
    int error;
    if (group) {
    parent = kernfs_find_and_get(kobj.sd, group);
    } else {
    parent = kobj.sd;
    kernfs_get(parent);
    }
    if (!parent)
    return -ENOENT;
    kobject_get_ownership(kobj, &uid, &gid);
    error = sysfs_add_file_mode_ns(parent, attr, attr.mode, uid, gid,
    core::ptr::null_mut());
    kernfs_put(parent);
    return error;
    }
    EXPORT_SYMBOL_GPL(sysfs_add_file_to_group);
//
// sysfs_chmod_file - update the modified mode value on an object attribute.
// @kobj: object we're acting for.
// @attr: attribute descriptor.
// @mode: file permissions.
//
    int sysfs_chmod_file(struct kobject *kobj, const struct attribute *attr,
    umode_t mode)
    {
    struct kernfs_node *kn;
    struct iattr newattrs;
    int rc;
    kn = kernfs_find_and_get(kobj.sd, attr.name);
    if (!kn)
    return -ENOENT;
    newattrs.ia_mode = (mode & S_IALLUGO) | (kn.mode & ~S_IALLUGO);
    newattrs.ia_valid = ATTR_MODE;
    rc = kernfs_setattr(kn, &newattrs);
    kernfs_put(kn);
    return rc;
    }
    EXPORT_SYMBOL_GPL(sysfs_chmod_file);
//
// sysfs_break_active_protection - break "active" protection
// @kobj: The kernel object @attr is associated with.
// @attr: The attribute to break the "active" protection for.
//
// With sysfs, just like kernfs, deletion of an attribute is postponed until
// all active .show() and .store() callbacks have finished unless this function
// is called. Hence this function is useful in methods that implement self
// deletion.
//
    struct kernfs_node *sysfs_break_active_protection(struct kobject *kobj,
    const struct attribute *attr)
    {
    struct kernfs_node *kn;
    kobject_get(kobj);
    kn = kernfs_find_and_get(kobj.sd, attr.name);
    if (kn)
    kernfs_break_active_protection(kn);
    else
    kobject_put(kobj);
    return kn;
    }
    EXPORT_SYMBOL_GPL(sysfs_break_active_protection);
//
// sysfs_unbreak_active_protection - restore "active" protection
// @kn: Pointer returned by sysfs_break_active_protection().
//
// Undo the effects of sysfs_break_active_protection(). Since this function
// calls kernfs_put() on the kernfs node that corresponds to the 'attr'
// argument passed to sysfs_break_active_protection() that attribute may have
// been removed between the sysfs_break_active_protection() and
// sysfs_unbreak_active_protection() calls, it is not safe to access @kn after
// this function has returned.
//
#[no_mangle]
pub unsafe extern "C" fn sysfs_unbreak_active_protection(kn: *mut kernfs_node) {
    void sysfs_unbreak_active_protection(struct kernfs_node *kn)
    {
    struct kobject *kobj = sysfs_file_kobj(kn);
    kernfs_unbreak_active_protection(kn);
    kernfs_put(kn);
    kobject_put(kobj);
    }
    EXPORT_SYMBOL_GPL(sysfs_unbreak_active_protection);
//
// sysfs_remove_file_ns - remove an object attribute with a custom ns tag
// @kobj: object we're acting for
// @attr: attribute descriptor
// @ns: namespace tag of the file to remove
//
// Hash the attribute name and namespace tag and kill the victim.
//
    void sysfs_remove_file_ns(struct kobject *kobj, const struct attribute *attr,
    const struct ns_common *ns)
    {
    struct kernfs_node *parent = kobj.sd;
    kernfs_remove_by_name_ns(parent, attr.name, ns);
    }
    EXPORT_SYMBOL_GPL(sysfs_remove_file_ns);
//
// sysfs_remove_file_self - remove an object attribute from its own method
// @kobj: object we're acting for
// @attr: attribute descriptor
//
// See kernfs_remove_self() for details.
//
#[no_mangle]
pub unsafe extern "C" fn sysfs_remove_file_self(kobj: *mut kobject, attr: *const attribute) -> bool {
    bool sysfs_remove_file_self(struct kobject *kobj, const struct attribute *attr)
    {
    struct kernfs_node *parent = kobj.sd;
    struct kernfs_node *kn;
    bool ret;
    kn = kernfs_find_and_get(parent, attr.name);
    if (WARN_ON_ONCE(!kn))
    return false;
    ret = kernfs_remove_self(kn);
    kernfs_put(kn);
    return ret;
    }
    EXPORT_SYMBOL_GPL(sysfs_remove_file_self);
#[no_mangle]
pub unsafe extern "C" fn sysfs_remove_files(kobj: *mut kobject, ptr: *const *const attribute) {
    void sysfs_remove_files(struct kobject *kobj, const struct attribute * const *ptr)
    {
    int i;
    for (i = 0; ptr[i]; i++)
    sysfs_remove_file(kobj, ptr[i]);
    }
    EXPORT_SYMBOL_GPL(sysfs_remove_files);
//
// sysfs_remove_file_from_group - remove an attribute file from a group.
// @kobj: object we're acting for.
// @attr: attribute descriptor.
// @group: group name.
//
    void sysfs_remove_file_from_group(struct kobject *kobj,
    const struct attribute *attr, const char *group)
    {
    struct kernfs_node *parent;
    if (group) {
    parent = kernfs_find_and_get(kobj.sd, group);
    } else {
    parent = kobj.sd;
    kernfs_get(parent);
    }
    if (parent) {
    kernfs_remove_by_name(parent, attr.name);
    kernfs_put(parent);
    }
    }
    EXPORT_SYMBOL_GPL(sysfs_remove_file_from_group);
//
// sysfs_create_bin_file - create binary file for object.
// @kobj:	object.
// @attr:	attribute descriptor.
//
    int sysfs_create_bin_file(struct kobject *kobj,
    const struct bin_attribute *attr)
    {
    kuid_t uid;
    kgid_t gid;
    if (WARN_ON(!kobj || !kobj.sd || !attr))
    return -EINVAL;
    kobject_get_ownership(kobj, &uid, &gid);
    return sysfs_add_bin_file_mode_ns(kobj.sd, attr, attr.attr.mode,
    attr.size, uid, gid, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(sysfs_create_bin_file);
//
// sysfs_remove_bin_file - remove binary file for object.
// @kobj:	object.
// @attr:	attribute descriptor.
//
    void sysfs_remove_bin_file(struct kobject *kobj,
    const struct bin_attribute *attr)
    {
    kernfs_remove_by_name(kobj.sd, attr.attr.name);
    }
    EXPORT_SYMBOL_GPL(sysfs_remove_bin_file);
    static int internal_change_owner(struct kernfs_node *kn, kuid_t kuid,
    kgid_t kgid)
    {
    struct iattr newattrs = {
    .ia_valid = ATTR_UID | ATTR_GID,
    .ia_uid = kuid,
    .ia_gid = kgid,
    };
    return kernfs_setattr(kn, &newattrs);
    }
//
// sysfs_link_change_owner - change owner of a sysfs file.
// @kobj:	object of the kernfs_node the symlink is located in.
// @targ:	object of the kernfs_node the symlink points to.
// @name:	name of the link.
// @kuid:	new owner's kuid
// @kgid:	new owner's kgid
//
// This function looks up the sysfs symlink entry @name under @kobj and changes
// the ownership to @kuid/@kgid. The symlink is looked up in the namespace of
// @targ.
//
// Returns 0 on success or error code on failure.
//
    int sysfs_link_change_owner(struct kobject *kobj, struct kobject *targ,
    const char *name, kuid_t kuid, kgid_t kgid)
    {
    struct kernfs_node *kn = core::ptr::null_mut();
    int error;
    if (!name || !kobj.state_in_sysfs || !targ.state_in_sysfs)
    return -EINVAL;
    error = -ENOENT;
    kn = kernfs_find_and_get_ns(kobj.sd, name, targ.sd.ns);
    if (!kn)
    goto out;
    error = -EINVAL;
    if (kernfs_type(kn) != KERNFS_LINK)
    goto out;
    if (kn.symlink.target_kn.priv != targ)
    goto out;
    error = internal_change_owner(kn, kuid, kgid);
    out:
    kernfs_put(kn);
    return error;
    }
//
// sysfs_file_change_owner - change owner of a sysfs file.
// @kobj:	object.
// @name:	name of the file to change.
// @kuid:	new owner's kuid
// @kgid:	new owner's kgid
//
// This function looks up the sysfs entry @name under @kobj and changes the
// ownership to @kuid/@kgid.
//
// Returns 0 on success or error code on failure.
//
    int sysfs_file_change_owner(struct kobject *kobj, const char *name, kuid_t kuid,
    kgid_t kgid)
    {
    struct kernfs_node *kn;
    int error;
    if (!name)
    return -EINVAL;
    if (!kobj.state_in_sysfs)
    return -EINVAL;
    kn = kernfs_find_and_get(kobj.sd, name);
    if (!kn)
    return -ENOENT;
    error = internal_change_owner(kn, kuid, kgid);
    kernfs_put(kn);
    return error;
    }
//
// sysfs_change_owner - change owner of the given object.
// @kobj:	object.
// @kuid:	new owner's kuid
// @kgid:	new owner's kgid
//
// Change the owner of the default directory, files, groups, and attributes of
// @kobj to @kuid/@kgid. Note that sysfs_change_owner mirrors how the sysfs
// entries for a kobject are added by driver core. In summary,
// sysfs_change_owner() takes care of the default directory entry for @kobj,
// the default attributes associated with the ktype of @kobj and the default
// attributes associated with the ktype of @kobj.
// Additional properties not added by driver core have to be changed by the
// driver or subsystem which created them. This is similar to how
// driver/subsystem specific entries are removed.
//
// Returns 0 on success or error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn sysfs_change_owner(kobj: *mut kobject, kuid: kuid_t, kgid: kgid_t) -> c_int {
    int sysfs_change_owner(struct kobject *kobj, kuid_t kuid, kgid_t kgid)
    {
    int error;
    const struct kobj_type *ktype;
    if (!kobj.state_in_sysfs)
    return -EINVAL;
// Change the owner of the kobject itself.
    error = internal_change_owner(kobj.sd, kuid, kgid);
    if (error)
    return error;
    ktype = get_ktype(kobj);
    if (ktype) {
//
// Change owner of the default groups associated with the
// ktype of @kobj.
//
    error = sysfs_groups_change_owner(kobj, ktype.default_groups,
    kuid, kgid);
    if (error)
    return error;
    }
    return 0;
    }
//
// sysfs_emit - scnprintf equivalent, aware of PAGE_SIZE buffer.
// @buf:	start of PAGE_SIZE buffer.
// @fmt:	format
// @...:	optional arguments to @format
//
// Returns number of characters written to @buf.
//
#[no_mangle]
pub unsafe extern "C" fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> c_int {
    int sysfs_emit(char *buf, const char *fmt, ...)
    {
    va_list args;
    int len;
    if (WARN(!buf || offset_in_page(buf),
    "invalid sysfs_emit: buf:%p\n", buf))
    return 0;
    va_start(args, fmt);
    len = vscnprintf(buf, PAGE_SIZE, fmt, args);
    va_end(args);
    return len;
    }
    EXPORT_SYMBOL_GPL(sysfs_emit);
//
// sysfs_emit_at - scnprintf equivalent, aware of PAGE_SIZE buffer.
// @buf:	start of PAGE_SIZE buffer.
// @at:	offset in @buf to start write in bytes
// @at must be >= 0 && < PAGE_SIZE
// @fmt:	format
// @...:	optional arguments to @fmt
//
// Returns number of characters written starting at &@buf[@at].
//
#[no_mangle]
pub unsafe extern "C" fn sysfs_emit_at(buf: *mut c_char, at: c_int, fmt: *const c_char, ...) -> c_int {
    int sysfs_emit_at(char *buf, int at, const char *fmt, ...)
    {
    va_list args;
    int len;
    if (WARN(!buf || offset_in_page(buf) || at < 0 || at >= PAGE_SIZE,
    "invalid sysfs_emit_at: buf:%p at:%d\n", buf, at))
    return 0;
    va_start(args, fmt);
    len = vscnprintf(buf + at, PAGE_SIZE - at, fmt, args);
    va_end(args);
    return len;
    }
    EXPORT_SYMBOL_GPL(sysfs_emit_at);
//
// sysfs_bin_attr_simple_read - read callback to simply copy from memory.
// @file:	attribute file which is being read.
// @kobj:	object to which the attribute belongs.
// @attr:	attribute descriptor.
// @buf:	destination buffer.
// @off:	offset in bytes from which to read.
// @count:	maximum number of bytes to read.
//
// Simple ->read() callback for bin_attributes backed by a buffer in memory.
// The @private and @size members in struct bin_attribute must be set to the
// buffer's location and size before the bin_attribute is created in sysfs.
//
// Bounds check for @off and @count is done in sysfs_kf_bin_read().
// Negative value check for @off is done in vfs_setpos() and default_llseek().
//
// Returns number of bytes written to @buf.
//
    ssize_t sysfs_bin_attr_simple_read(struct file *file, struct kobject *kobj,
    const struct bin_attribute *attr, char *buf,
    loff_t off, size_t count)
    {
    memcpy(buf, attr.private + off, count);
    return count;
    }
    EXPORT_SYMBOL_GPL(sysfs_bin_attr_simple_read);
